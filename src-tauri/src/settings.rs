use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, GlobalShortcutManager, Manager, PathResolver, State};
use tauri_plugin_autostart::ManagerExt;
use arboard::ImageData;
use image::io::Reader as ImageReader;
use tauri::api::path as tauri_path;

pub struct AppState {
    pub settings: Mutex<AppSettings>,
    pub last_screenshot_path: Mutex<Option<PathBuf>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub shortcut: String,
    // pub api_key: String, // <- 移除此行：不再需要API Key
    pub target_lang: String,
    pub autostart: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            shortcut: "F1".to_string(),
            // api_key: "".to_string(), // <- 移除此行
            target_lang: "zh".to_string(),
            autostart: false,
        }
    }
}

// ... [AppSettings 的 load 和 save 方法保持不变] ...
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

// ... [get_settings, set_settings, copy_image_to_clipboard, save_image_to_desktop 命令保持不变] ...
// ... [请保留您原有的这些 tauri::command 函数] ...
#[tauri::command]
pub fn get_settings(state: State<AppState>) -> Result<AppSettings, String> {
    Ok(state.settings.lock().unwrap().clone())
}

#[tauri::command]
pub async fn set_settings(app: AppHandle, state: State<'_, AppState>, settings: AppSettings) -> Result<(), String> {
    println!("接收到新设置: {:?}", settings);
    settings.save(&app.path_resolver()).map_err(|e| format!("保存设置文件失败: {}", e))?;
    let old_shortcut;
    {
        let mut app_settings = state.settings.lock().unwrap();
        old_shortcut = app_settings.shortcut.clone();
        *app_settings = settings.clone();
    }
    if old_shortcut != settings.shortcut {
        let mut shortcut_manager = app.global_shortcut_manager();
        shortcut_manager.unregister_all().map_err(|e| e.to_string())?;
        let app_for_closure = app.clone();
        shortcut_manager.register(&settings.shortcut, move || {
            let app_handle = app_for_closure.clone();
            if let Some(window) = app_handle.get_window("screenshot") {
                window.show().unwrap(); window.set_focus().unwrap();
            } else {
                eprintln!("未找到截图窗口");
            }
        }).map_err(|e| e.to_string())?;
    }
    let autostart_manager = app.autolaunch();
    let is_enabled = autostart_manager.is_enabled().unwrap_or(false);
    if settings.autostart && !is_enabled {
        autostart_manager.enable().map_err(|e| e.to_string())?;
    } else if !settings.autostart && is_enabled {
        autostart_manager.disable().map_err(|e| e.to_string())?;
    }
    Ok(())
}

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