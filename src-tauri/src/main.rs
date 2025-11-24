#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod settings;
mod translator;

use tauri::{
    AppHandle, GlobalShortcutManager, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    CustomMenuItem,
};
use tauri_plugin_autostart::MacosLauncher;
use settings::{AppSettings, AppState};

use std::fs;
use base64::{Engine as _, engine::general_purpose};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Serialize)]
struct ImageViewerPayload {
    image_data_url: String,
    image_path: String,
}

fn main() {
    let show_settings = CustomMenuItem::new("show_settings".to_string(), "显示设置");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let tray_menu = SystemTrayMenu::new().add_item(show_settings).add_native_item(tauri::SystemTrayMenuItem::Separator).add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--hidden"])))
        .invoke_handler(tauri::generate_handler![
            commands::process_screenshot_area,
            settings::get_settings,
            settings::set_settings,
            settings::copy_image_to_clipboard,
            settings::save_image_to_desktop
        ])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => std::process::exit(0),
                "show_settings" => { if let Some(window) = app.get_window("main") { window.show().unwrap(); window.set_focus().unwrap(); } }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            // 1. 加载设置或使用默认值
            let settings = AppSettings::load(&app.path_resolver()).unwrap_or_default();

            // 2. 将设置存入 AppState
            app.manage(AppState {
                settings: std::sync::Mutex::new(settings.clone()),
                last_screenshot_path: std::sync::Mutex::new(None),
            });

            // 3. 注册主截图快捷键
            println!("应用启动，注册主截图快捷键: {}", &settings.shortcut);
            register_global_shortcut(app.handle(), &settings.shortcut)
                .unwrap_or_else(|e| eprintln!("启动时注册主截图快捷键失败: {}", e));

            // 4. 从设置中读取并注册“查看截图”快捷键
            println!("应用启动，注册查看截图快捷键: {}", &settings.view_image_shortcut);
            register_view_image_shortcut(app.handle(), &settings.view_image_shortcut)
                .unwrap_or_else(|e| eprintln!("启动时注册查看截图快捷键失败: {}", e));


            // 5. 显示主窗口并通知前端后端已就绪
            let main_window = app.get_window("main").unwrap();
            main_window.show()?;
            main_window.emit("backend-ready", ()).unwrap();
            Ok(())
        })
        .build(tauri::generate_context!()).expect("运行Tauri应用时出错")
        .run(|_app_handle, event| { if let tauri::RunEvent::ExitRequested { api, .. } = event { api.prevent_exit(); } });
}

// 主截图快捷键注册函数 (保持不变)
pub fn register_global_shortcut(app_handle: AppHandle, shortcut: &str) -> Result<(), tauri::Error> {
    let mut manager = app_handle.global_shortcut_manager();

    if manager.is_registered(shortcut)? {
        manager.unregister(shortcut)?;
    }

    let shortcut_for_closure = shortcut.to_string();

    manager.register(shortcut, move || {
        println!("全局快捷键 {} 被按下", shortcut_for_closure);

        let handle_for_closure = app_handle.clone();
        app_handle.run_on_main_thread(move || {
            if let Some(window) = handle_for_closure.get_window("screenshot") {
                window.show().unwrap();
                window.set_focus().unwrap();
            } else {
                tauri::WindowBuilder::new(&handle_for_closure, "screenshot", tauri::WindowUrl::App("screenshot.html".into()))
                    .fullscreen(true)
                    .decorations(false)
                    .transparent(true)
                    .resizable(false)
                    .build()
                    .unwrap();
            }
        }).unwrap();
    }).map_err(Into::into)
}


// “查看截图”快捷键的注册函数
pub fn register_view_image_shortcut(app_handle: AppHandle, shortcut: &str) -> Result<(), tauri::Error> {
    let mut manager = app_handle.global_shortcut_manager();

    if manager.is_registered(shortcut)? {
        let _ = manager.unregister(shortcut);
    }

    // --- 核心修复 ---
    // 1. 创建一个 String 副本，专门用于移动到闭包中。
    //    原始的 `shortcut: &str` 仍然用于 `manager.register` 的第一个参数。
    let shortcut_for_closure = shortcut.to_string();

    // 2. `manager.register` 的第一个参数使用原始的 `shortcut` 引用。
    //    闭包通过 `move` 关键字获取 `shortcut_for_closure` 的所有权。
    //    这样，借用和所有权移动发生在不同的数据上，冲突解决。
    manager.register(shortcut, move || {
        println!("查看截图快捷键 {} 被按下", shortcut_for_closure);

        let handle_for_thread = app_handle.clone();

        std::thread::spawn(move || {
            let path_to_show: Option<PathBuf> = {
                let state: tauri::State<AppState> = handle_for_thread.state();
                let lock = state.last_screenshot_path.lock().unwrap();
                lock.clone()
            };

            if let Some(path) = path_to_show {
                if let Ok(bytes) = fs::read(&path) {
                    let b64 = general_purpose::STANDARD.encode(&bytes);
                    let payload = ImageViewerPayload {
                        image_data_url: format!("data:image/png;base64,{}", b64),
                        image_path: path.to_str().unwrap().to_string(),
                    };

                    let handle_for_main_thread = handle_for_thread.clone();
                    handle_for_thread.run_on_main_thread(move || {
                        if let Some(window) = handle_for_main_thread.get_window("image_viewer") {
                            window.emit("display-image", payload).unwrap();
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        } else {
                            let builder = tauri::WindowBuilder::new(&handle_for_main_thread, "image_viewer", tauri::WindowUrl::App("image_viewer.html".into()))
                                .title("截图预览").decorations(false).transparent(true)
                                .resizable(true).skip_taskbar(true).visible(false);

                            if let Ok(window) = builder.build() {
                                let window_for_closure = window.clone();
                                window.once("tauri://created", move |_| {
                                    window_for_closure.emit("display-image", payload).unwrap();
                                });
                            }
                        }
                    }).unwrap();
                }
            }
        });
    }).map_err(Into::into)
}