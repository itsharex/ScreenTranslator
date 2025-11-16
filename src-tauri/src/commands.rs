use serde::{Serialize};
use tauri::{Manager, State, AppHandle};
use image::{ImageBuffer, Rgba};
use std::process::Command as StdCommand;
use encoding_rs::GBK;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

use crate::settings::{AppSettings, AppState};
use crate::translator;

// --- 新的事件 Payload 结构 ---

/// 事件 'ocr_result': 在 OCR 完成后立即发送
/// 包含识别的原文（或错误）和截图路径
#[derive(Debug, Serialize, Clone)]
struct OcrPayload {
    original_text: Option<String>,
    error_message: Option<String>,
    image_path: String,
}

/// 事件 'translation_update': 在翻译完成后发送
/// 仅包含译文或翻译错误
#[derive(Debug, Serialize, Clone)]
struct TranslationUpdatePayload {
    translated_text: Option<String>,
    error_message: Option<String>,
}

// --- 主命令（保持不变） ---

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

    let settings_clone = state.settings.lock().unwrap().clone();
    let app_for_task = app.clone();

    tokio::spawn(async move {
        create_and_show_results_window(&app_for_task);

        // 调用重构后的核心处理函数
        // 如果截图本身失败，发送一个初始错误事件
        if let Err(e) = capture_ocr_translate(&app_for_task, settings_clone, x, y, width, height).await {
            eprintln!("处理流程出现严重错误 (截图阶段): {}", e);
            // 注意：这里我们复用 OcrPayload 来报告初始错误，因为此时还没有图片路径
            // 前端需要能处理 image_path 为空字符串的情况
            let error_payload = OcrPayload {
                original_text: Some("处理失败".to_string()),
                error_message: Some(e),
                image_path: String::new(), // 明确表示没有图片
            };
            app_for_task.emit_all("ocr_result", error_payload).unwrap();
        }
    });

    Ok(())
}


/// 核心处理流程：截图 -> OCR -> （发送事件） -> 翻译 -> （发送事件）
/// [核心修改] 此函数现在分两步发送事件，以实现即时UI更新
async fn capture_ocr_translate(
    app: &AppHandle,
    settings: AppSettings,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> Result<(), String> {
    // --- 步骤 1: 截图并保存。这是关键步骤，如果失败则无法继续。 ---
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

    // 将有效的图片路径存入 AppState，并转换为字符串
    let state: State<AppState> = app.state();
    {
        let mut last_path = state.last_screenshot_path.lock().unwrap();
        *last_path = Some(image_path.clone());
    }
    let image_path_str = image_path.to_str().unwrap().to_string();


    // --- 步骤 2: 执行 OCR。 ---
    let ocr_result: Result<String, String> = (|| {
        // ... (内部 OCR 逻辑保持不变) ...
        let ocr_exe_path = app
            .path_resolver()
            .resolve_resource("external/PaddleOCR-json/PaddleOCR-json.exe")
            .ok_or_else(|| "在应用资源中找不到 OCR 可执行文件路径".to_string())?
            .canonicalize()
            .map_err(|e| format!("无法找到 or 规范化 OCR 可执行文件路径: {}. 请确认 external/PaddleOCR-json/PaddleOCR-json.exe 文件存在。", e))?;

        if !ocr_exe_path.exists() { return Err(format!("错误: OCR 可执行文件在路径 {:?} 下不存在!", ocr_exe_path)); }
        let ocr_dir = ocr_exe_path.parent().ok_or("无法获取OCR程序的父目录")?;
        let args = vec![format!("--image_path={}", &image_path_str)];
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
    })();


    // --- 步骤 3: 根据 OCR 结果，立即发送第一个事件并决定是否继续翻译 ---
    match ocr_result {
        // OCR 成功
        Ok(original_text) => {
            // a. 立即发送 OCR 成功事件
            let ocr_payload = OcrPayload {
                original_text: Some(original_text.clone()),
                error_message: None,
                image_path: image_path_str,
            };
            app.emit_all("ocr_result", ocr_payload).unwrap();

            // b. 在后台继续进行翻译
            let translator = translator::get_translator(app);
            let translation_result = translator.translate(&original_text, &settings.target_lang).await;

            // c. 根据翻译结果发送第二个更新事件
            let update_payload = match translation_result {
                Ok(translated_text) => {
                    println!("翻译结果: {}", translated_text);
                    TranslationUpdatePayload {
                        translated_text: Some(translated_text),
                        error_message: None,
                    }
                },
                Err(e) => {
                    eprintln!("翻译失败: {}", e);
                    TranslationUpdatePayload {
                        translated_text: None,
                        error_message: Some(e),
                    }
                }
            };
            app.emit_all("translation_update", update_payload).unwrap();
        },
        // OCR 失败
        Err(e) => {
            eprintln!("OCR 失败: {}", e);
            // 发送 OCR 失败事件，流程终止
            let ocr_payload = OcrPayload {
                original_text: Some("识别失败".to_string()),
                error_message: Some(e),
                image_path: image_path_str, // 即使失败，也发送图片路径
            };
            app.emit_all("ocr_result", ocr_payload).unwrap();
        }
    };

    Ok(())
}


// --- 辅助函数（保持不变） ---
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