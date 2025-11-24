use serde::{Deserialize};
use tauri::AppHandle; // 引入 AppHandle，用于路径解析
use std::process::Command; // 引入用于执行外部命令的模块
use encoding_rs::GBK; // --- 1. 引入 GBK 解码器 ---

// 在 Windows 系统上，需要额外引入此模块来隐藏命令行窗口
#[cfg(windows)]
use std::os::windows::process::CommandExt;

// --- 1. 定义与本地翻译 Sidecar 交互的数据结构 ---

// 用于解析 translate.exe 输出的 JSON 响应
#[derive(Debug, Deserialize)]
struct LocalTranslationResponse {
    code: i32,
    translated_text: Option<String>,
    error_message: Option<String>,
}

// --- 2. 定义统一的翻译器Trait（接口），这部分保持不变 ---
#[async_trait::async_trait]
pub trait Translator {
    async fn translate(
        &self,
        text: &str,
        target_lang: &str,
    ) -> Result<String, String>;
}

// --- 3. 实现本地翻译器 ---

pub struct LocalTranslator {
    // LocalTranslator 需要 AppHandle 来定位打包后的 sidecar 程序
    app_handle: AppHandle,
}

impl LocalTranslator {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }
}

#[async_trait::async_trait]
impl Translator for LocalTranslator {
    /// 实现翻译方法，通过调用外部 translate.exe 程序
    async fn translate(
        &self,
        text: &str,
        target_lang: &str,
    ) -> Result<String, String> {
        // --- a. 定位 sidecar 可执行文件路径 ---
        let translator_exe_path = self.app_handle
            .path_resolver()
            .resolve_resource("external/Translator/translate.exe")
            .ok_or_else(|| "在应用资源中找不到翻译器可执行文件".to_string())?;

        // --- b. 构建命令行 ---
        // --- 核心修改：根据目标语言动态决定源语言 ---
        let source_lang = if target_lang == "en" {
            "zh" // 如果目标是英语，则假定源语言是中文
        } else {
            "en" // 否则，假定源语言是英语（保持原有逻辑）
        };

        println!("翻译请求: 源语言='{}', 目标语言='{}'", source_lang, target_lang);

        let mut command = Command::new(&translator_exe_path);
        command.args(&[
            "--text", text,
            "--source", source_lang, // 使用动态决定的源语言
            "--target", target_lang,
        ]);

        // 在 Windows 上，添加此标志可以在执行命令时不弹出黑色的命令行窗口
        #[cfg(windows)]
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW

        // --- c. 执行命令并捕获输出 ---
        let output = command
            .output()
            .map_err(|e| format!("执行翻译进程失败: {}", e))?;

        // --- d. 处理和解析响应 ---
        if !output.status.success() {
            // --- 2. 使用 GBK 解码 stderr ---
            let stderr = GBK.decode(&output.stderr).0.into_owned();
            return Err(format!("翻译进程执行出错: {}", stderr));
        }

        // --- 2. 使用 GBK 解码 stdout ---
        let (decoded_stdout, _, _) = GBK.decode(&output.stdout);
        let stdout = decoded_stdout.into_owned();


        // --- e. 解析 JSON 并返回结果 ---
        let response: LocalTranslationResponse = serde_json::from_str(&stdout)
            .map_err(|e| format!("解析翻译结果JSON失败: {}. 原始输出: {}", e, stdout))?;

        match response.code {
            200 => response.translated_text.ok_or_else(|| "翻译成功但未返回文本".to_string()),
            _ => Err(response.error_message.unwrap_or_else(|| "翻译器返回未知错误".to_string())),
        }
    }
}

/// 辅助函数，创建一个本地翻译器实例
// 这部分保持不变
pub fn get_translator(app: &AppHandle) -> Box<dyn Translator + Send + Sync> {
    // 创建并返回本地翻译器的实例
    Box::new(LocalTranslator::new(app.clone()))
}