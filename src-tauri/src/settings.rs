use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, GlobalShortcutManager, PathResolver, State};
use tauri_plugin_autostart::ManagerExt;
use arboard::ImageData;
use image::io::Reader as ImageReader;
use tauri::api::path as tauri_path;
// --- 新增引入 ---
// 引入两个快捷键注册函数，一个用于主功能，一个用于查看图片
use crate::{register_global_shortcut, register_view_image_shortcut};

pub struct AppState {
    pub settings: Mutex<AppSettings>,
    pub last_screenshot_path: Mutex<Option<PathBuf>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub shortcut: String,
    // --- 新增字段 ---
    // 用于存储“查看上次截图”功能的快捷键
    pub view_image_shortcut: String,
    pub target_lang: String,
    pub autostart: bool,
    pub enable_ocr: bool,
    pub enable_translation: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            shortcut: "F1".to_string(),
            // --- 新增字段的默认值 ---
            view_image_shortcut: "F3".to_string(),
            target_lang: "zh".to_string(),
            autostart: false,
            enable_ocr: false,
            enable_translation: false,
        }
    }
}

impl AppSettings {
    fn get_config_path(path_resolver: &PathResolver) -> PathBuf {
        path_resolver.app_config_dir().expect("无法获取应用配置目录").join("settings.json")
    }
    pub fn load(path_resolver: &PathResolver) -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path(path_resolver);
        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }
    pub fn save(&self, path_resolver: &PathResolver) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path(path_resolver);
        if let Some(parent) = config_path.parent() { fs::create_dir_all(parent)?; }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }
}

#[tauri::command]
pub fn get_settings(state: State<AppState>) -> Result<AppSettings, String> {
    Ok(state.settings.lock().unwrap().clone())
}

#[tauri::command]
pub async fn set_settings(app: AppHandle, state: State<'_, AppState>, settings: AppSettings) -> Result<(), String> {
    println!("接收到新设置: {:?}", settings);

    // 1. 保存设置到文件
    settings.save(&app.path_resolver()).map_err(|e| format!("保存设置文件失败: {}", e))?;

    // 2. 更新内存中的应用状态，并记录旧的快捷键
    let old_shortcut;
    // --- 新增：记录旧的“查看截图”快捷键 ---
    let old_view_shortcut;
    {
        let mut app_settings = state.settings.lock().unwrap();
        old_shortcut = app_settings.shortcut.clone();
        // --- 新增：从内存中获取旧值 ---
        old_view_shortcut = app_settings.view_image_shortcut.clone();
        *app_settings = settings.clone();
    }

    let mut shortcut_manager = app.global_shortcut_manager();

    // 3. 更新主截图快捷键 (逻辑保持不变)
    if old_shortcut != settings.shortcut {
        let _ = shortcut_manager.unregister(&old_shortcut);
        println!("主快捷键已更改，注销旧快捷键: {}", old_shortcut);
    }
    if let Err(e) = register_global_shortcut(app.clone(), &settings.shortcut) {
        eprintln!("动态更新主快捷键 {} 失败: {}", &settings.shortcut, e);
        return Err(format!("注册主快捷键失败: {}", e));
    }

    // --- 4. 新增：更新“查看截图”快捷键 ---
    // 如果快捷键字符串发生了变化
    if old_view_shortcut != settings.view_image_shortcut {
        // 先尝试注销旧的快捷键，忽略错误（可能之前就没有注册成功）
        let _ = shortcut_manager.unregister(&old_view_shortcut);
        println!("查看截图快捷键已更改，注销旧快捷键: {}", old_view_shortcut);
    }
    // 统一调用新的注册函数来注册或更新快捷键
    if let Err(e) = register_view_image_shortcut(app.clone(), &settings.view_image_shortcut) {
        eprintln!("动态更新查看截图快捷键 {} 失败: {}", &settings.view_image_shortcut, e);
        return Err(format!("注册查看截图快捷键失败: {}", e));
    }


    // 5. 处理开机自启动（逻辑保持不变）
    let autostart_manager = app.autolaunch();
    let is_enabled = autostart_manager.is_enabled().unwrap_or(false);
    if settings.autostart && !is_enabled {
        autostart_manager.enable().map_err(|e| e.to_string())?;
    } else if !settings.autostart && is_enabled {
        autostart_manager.disable().map_err(|e| e.to_string())?;
    }

    Ok(())
}


// 以下命令保持不变
#[tauri::command]
pub async fn copy_image_to_clipboard(path: String) -> Result<(), String> {
    let img = ImageReader::open(path).map_err(|e| e.to_string())?.decode().map_err(|e| e.to_string())?.to_rgba8();
    let image_data = ImageData { width: img.width() as usize, height: img.height() as usize, bytes: img.into_raw().into() };
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_image(image_data).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn save_image_to_desktop(path: String) -> Result<(), String> {
    let desktop_dir = tauri_path::desktop_dir().ok_or("无法获取桌面路径".to_string())?;
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let new_filename = format!("screenshot-{}.png", timestamp);
    let dest_path = desktop_dir.join(new_filename);
    fs::copy(&path, &dest_path).map_err(|e| format!("保存文件到桌面失败: {}", e))?;
    println!("图片已保存至: {:?}", dest_path);
    Ok(())
}