use serde::{Serialize};
use tauri::{Manager, State, AppHandle};
use image::{ImageBuffer, Rgba};
use std::process::Command as StdCommand;
use encoding_rs::GBK;
// --- 新增引入 ---
use std::fs;
use base64::{Engine as _, engine::general_purpose};
use crate::ImageViewerPayload; // 从 main.rs 引入 ImageViewerPayload 结构体

#[cfg(windows)]
use std::os::windows::process::CommandExt;

use crate::settings::{AppSettings, AppState};
use crate::translator;

// 事件 Payload 结构保持不变
#[derive(Debug, Serialize, Clone)]
struct OcrPayload {
    original_text: Option<String>,
    error_message: Option<String>,
    image_path: String,
}

#[derive(Debug, Serialize, Clone)]
struct TranslationUpdatePayload {
    translated_text: Option<String>,
    error_message: Option<String>,
}

#[tauri::command]
pub async fn process_screenshot_area(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> Result<(), String> {
    println!("接收到截图区域: x={}, y={}, width={}, height={}", x, y, width, height);

    let settings = state.settings.lock().unwrap().clone();
    let app_for_task = app.clone();

    tokio::spawn(async move {
        if let Err(e) = capture_ocr_translate(&app_for_task, settings, x, y, width, height).await {
            eprintln!("处理流程出现严重错误 (截图阶段): {}", e);
        }
    });

    Ok(())
}

async fn capture_ocr_translate(
    app: &AppHandle,
    settings: AppSettings,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> Result<(), String> {
    // 步骤 1: 截图并保存
    let screen = screenshots::Screen::from_point(x as i32, y as i32)
        .map_err(|e| format!("无法找到屏幕: {}", e))?;
    let capture = screen.capture_area(x as i32, y as i32, width as u32, height as u32)
        .map_err(|e| format!("截图失败: {}", e))?;
    let temp_dir = app.path_resolver().app_cache_dir().unwrap().join("tmp");
    tokio::fs::create_dir_all(&temp_dir).await
        .map_err(|e| format!("创建临时目录失败: {}", e))?;
    let image_path = temp_dir.join("screenshot.png");
    let img_buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(capture.width(), capture.height(), capture.rgba().to_vec())
        .ok_or_else(|| "无法从原始数据创建图像缓冲区".to_string())?;
    img_buffer.save(&image_path)
        .map_err(|e| format!("保存截图文件失败: {}", e))?;

    let state: State<AppState> = app.state();
    {
        let mut last_path = state.last_screenshot_path.lock().unwrap();
        *last_path = Some(image_path.clone());
    }
    let image_path_str = image_path.to_str().unwrap().to_string();

    // 步骤 2: 根据 OCR 开关状态执行不同逻辑
    if settings.enable_ocr {
        println!("OCR 功能已开启，执行识别流程...");

        create_and_show_results_window(app);

        let ocr_result = perform_ocr(app, &image_path_str);

        match ocr_result {
            Ok(original_text) => {
                let ocr_payload = OcrPayload {
                    original_text: Some(original_text.clone()),
                    error_message: None,
                    image_path: image_path_str,
                };
                app.emit_all("ocr_result", ocr_payload).unwrap();

                if settings.enable_translation {
                    println!("翻译功能已开启，开始翻译...");
                    let translator = translator::get_translator(app);
                    let translation_result = translator.translate(&original_text, &settings.target_lang).await;

                    let update_payload = match translation_result {
                        Ok(translated_text) => TranslationUpdatePayload {
                            translated_text: Some(translated_text),
                            error_message: None,
                        },
                        Err(e) => TranslationUpdatePayload {
                            translated_text: None,
                            error_message: Some(e),
                        }
                    };
                    app.emit_all("translation_update", update_payload).unwrap();
                } else {
                    println!("翻译功能已关闭，跳过翻译步骤。");
                    let update_payload = TranslationUpdatePayload {
                        translated_text: None,
                        error_message: Some("翻译功能已关闭".to_string()),
                    };
                    app.emit_all("translation_update", update_payload).unwrap();
                }
            },
            Err(e) => { // OCR 失败
                eprintln!("OCR 失败: {}", e);
                let ocr_payload = OcrPayload {
                    original_text: Some("识别失败".to_string()),
                    error_message: Some(e),
                    image_path: image_path_str,
                };
                app.emit_all("ocr_result", ocr_payload).unwrap();
            }
        };

    } else {
        println!("OCR 功能已关闭，仅显示截图预览。");

        let bytes = fs::read(&image_path).map_err(|e| format!("读取截图文件失败: {}", e))?;
        let b64 = general_purpose::STANDARD.encode(&bytes);
        let payload = ImageViewerPayload {
            image_data_url: format!("data:image/png;base64,{}", b64),
            image_path: image_path_str,
        };
        create_and_show_image_viewer_window(app, payload);
    }

    Ok(())
}


// --- 辅助函数 ---

fn perform_ocr(app: &AppHandle, image_path_str: &str) -> Result<String, String> {
    let ocr_exe_path = app
        .path_resolver()
        .resolve_resource("external/PaddleOCR-json/PaddleOCR-json.exe")
        .ok_or_else(|| "在应用资源中找不到 OCR 可执行文件路径".to_string())?
        .canonicalize()
        .map_err(|e| format!("无法找到 or 规范化 OCR 可执行文件路径: {}. 请确认 external/PaddleOCR-json/PaddleOCR-json.exe 文件存在。", e))?;

    if !ocr_exe_path.exists() { return Err(format!("错误: OCR 可执行文件在路径 {:?} 下不存在!", ocr_exe_path)); }
    let ocr_dir = ocr_exe_path.parent().ok_or("无法获取OCR程序的父目录")?;
    let args = vec![format!("--image_path={}", image_path_str)];
    #[cfg(windows)] const CREATE_NO_WINDOW: u32 = 0x08000000;
    let mut command = StdCommand::new(&ocr_exe_path);
    command.args(&args).current_dir(&ocr_dir);
    #[cfg(windows)] command.creation_flags(CREATE_NO_WINDOW);
    let ocr_output = command.output().map_err(|e| format!("执行 OCR 进程失败: {}", e))?;
    if !ocr_output.status.success() {
        let stderr = GBK.decode(&ocr_output.stderr).0.into_owned();
        return Err(format!("OCR 进程执行出错: {}", stderr));
    }
    let stdout = GBK.decode(&ocr_output.stdout).0.into_owned();
    let json_str = stdout.lines().find(|line| line.starts_with('{')).unwrap_or("{}");
    let ocr_value: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| format!("解析 OCR JSON 失败: {}. 原始输出: {}", e, stdout))?;
    let code = ocr_value["code"].as_i64().unwrap_or(0);
    let original_text = match code {
        100 => ocr_value["data"].as_array().unwrap_or(&vec![]).iter()
            .filter_map(|item| item["text"].as_str()).map(|s| s.to_string())
            .collect::<Vec<String>>().join(" "),
        101 => return Err("未识别到任何文字".to_string()),
        _ => return Err(ocr_value["data"].as_str().unwrap_or("OCR 返回未知错误").to_string()),
    };
    if original_text.trim().is_empty() { return Err("未识别到任何文字".to_string()); }
    println!("OCR 识别原文: {}", original_text);
    Ok(original_text)
}

fn create_and_show_results_window(app: &AppHandle) {
    let handle = app.clone();
    if let Some(window) = handle.get_window("results") {
        window.show().unwrap();
        window.set_focus().unwrap();
    } else {
        tauri::WindowBuilder::new(&handle, "results", tauri::WindowUrl::App("results.html".into()))
            .build()
            .expect("无法创建结果窗口");
    }
}

fn create_and_show_image_viewer_window(app: &AppHandle, payload: ImageViewerPayload) {
    let handle = app.clone();

    // --- 核心修复 1: 克隆 handle 供闭包使用 ---
    // 创建一个 handle 的副本，这个副本的所有权将被移动到闭包中。
    // 原始的 `handle` 变量可以安全地被 `run_on_main_thread` 借用。
    let handle_for_closure = handle.clone();
    handle.run_on_main_thread(move || {
        // 在闭包内部，我们使用克隆出来的 `handle_for_closure`
        if let Some(window) = handle_for_closure.get_window("image_viewer") {
            window.emit("display-image", payload).unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        } else {
            let builder = tauri::WindowBuilder::new(&handle_for_closure, "image_viewer", tauri::WindowUrl::App("image_viewer.html".into()))
                .title("截图预览").decorations(false).transparent(true)
                .resizable(true).skip_taskbar(true).visible(false);

            if let Ok(window) = builder.build() {
                // --- 核心修复 2: 克隆 window 供闭包使用 ---
                // 创建一个 window 的副本，它的所有权将被移动到 `once` 的闭包中。
                // 原始的 `window` 变量可以安全地被 `.once()` 方法借用。
                let window_for_closure = window.clone();
                window.once("tauri://created", move |_| {
                    window_for_closure.emit("display-image", payload).unwrap();
                });
            }
        }
    }).unwrap();
}