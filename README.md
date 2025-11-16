# ScreenTranslator - 屏幕截图翻译工具

![GitHub license](https://img.shields.io/github/license/git-hub-cc/ScreenTranslator)
![GitHub stars](https://img.shields.io/github/stars/git-hub-cc/ScreenTranslator?style=social)
![GitHub forks](https://img.shields.io/github/forks/git-hub-cc/ScreenTranslator?style=social)

一款基于 [Tauri](https://tauri.app/) 框架开发的现代化、轻量级的桌面端截图翻译工具。它允许用户通过一个简单的全局快捷键，快速捕捉屏幕上的任意区域，自动识别其中的文本并进行翻译。

## ✨ 核心功能

-   ✅ **全局快捷键**: 在任何应用中，通过 `Alt+Q` (可自定义) 一键呼出截图功能。
-   🖼️ **精准截图**: 拖拽鼠标即可选择需要翻译的屏幕区域，支持 `ESC` 键取消。
-   🚀 **高效识别**: 集成离线 OCR 引擎 ([PaddleOCR-json](https://github.com/hiroi-sora/PaddleOCR-json))，无需联网即可快速、准确地识别截图中的文字。
-   🌐 **强大翻译**: 对接 [DeepL API](https://www.deepl.com/pro-api)，提供高质量、自然的翻译结果（支持 Free 和 Pro 账户）。
-   📋 **便捷操作**:
    -   **置顶窗口**: 将翻译结果窗口钉在屏幕最前端。
    -   **一键复制**: 快速复制原文或译文到剪贴板。
    -   **朗读译文**: 调用系统 TTS 引擎朗读翻译结果。
-   ⚙️ **高度可配**:
    -   自定义截图快捷键。
    -   配置 DeepL API Key。
    -   选择多种目标翻译语言。
    -   设置是否开机自启动。
-   🖥️ **跨平台设计**: 基于 Tauri 构建，具备打包为多平台应用（Windows, macOS, Linux）的潜力。

## 🛠️ 技术栈

-   **核心框架**: [Tauri](https://tauri.app/) (使用 Rust 作为后端，Web 技术作为前端)
-   **后端**:
    -   [Rust](https://www.rust-lang.org/)
    -   [Tokio](https://tokio.rs/): 用于异步处理耗时任务（如 OCR 和 API 请求）。
    -   [Reqwest](https://github.com/seanmonstar/reqwest): 用于向 DeepL API 发送 HTTP 请求。
    -   [Serde](https://serde.rs/): 用于 JSON 序列化与反序列化。
    -   [screenshots](https://crates.io/crates/screenshots): 用于截取屏幕。
-   **前端**:
    -   HTML5, CSS3, Vanilla JavaScript
-   **OCR 引擎**: [PaddleOCR-json](https://github.com/hiroi-sora/PaddleOCR-json) (作为 sidecar 集成)

## 🚀 安装与启动

### 对于普通用户

1.  前往 [GitHub Releases](https://github.com/git-hub-cc/ScreenTranslator/releases) 页面。
2.  下载适用于您操作系统的最新版本安装包（例如 `.msi` for Windows）。
3.  运行安装程序即可。

### 对于开发者

请确保您的开发环境已安装 [Rust](https://www.rust-lang.org/tools/install) 和 [Node.js](https://nodejs.org/)。

1.  **克隆仓库**:
    ```bash
    git clone https://github.com/git-hub-cc/ScreenTranslator.git
    cd ScreenTranslator
    ```

2.  **设置 OCR 引擎** (⚠️ **关键步骤**):
    本项目依赖 `PaddleOCR-json` 作为离线的 OCR 引擎。
    -   访问 [PaddleOCR-json Releases](https://github.com/hiroi-sora/PaddleOCR-json/releases) 页面，下载最新版的 `PaddleOCR-json-v...-win-....zip`。
    -   在项目根目录下创建一个 `external` 文件夹，然后在其中再创建一个 `PaddleOCR-json` 文件夹。
    -   将下载的压缩包解压，并将其中的**所有文件**复制到 `external/PaddleOCR-json/` 目录下。
    -   完成后的目录结构应如下所示：
        ```
        ScreenTranslator/
        ├── external/
        │   └── PaddleOCR-json/
        │       ├── PaddleOCR-json.exe
        │       ├── models/
        │       └── ... (其他所有文件和文件夹)
        ├── src/
        └── src-tauri/
        ```
    > **注意**: 由于当前依赖 `.exe` 文件，该项目在未作修改的情况下**仅支持 Windows**。要在其他平台运行，需要替换为对应平台的 OCR 解决方案。

3.  **安装前端依赖**:
    ```bash
    npm install
    ```

4.  **启动开发环境**:
    ```bash
    npm run tauri dev
    ```

5.  **构建应用**:
    ```bash
    npm run tauri build
    ```

## 📖 使用指南

1.  **初次配置**:
    -   首次启动应用时，会显示设置窗口。
    -   在 "DeepL API Key" 输入框中填入您的 Key。您可以从 [DeepL 官网](https://www.deepl.com/pro-api) 获取。Free 版本的 Key 以 `:fx` 结尾。
    -   根据需要修改快捷键、目标语言和开机自启选项。
    -   点击 "保存设置"。应用会自动隐藏到系统托盘。

2.  **触发截图**:
    -   在需要翻译的界面，按下您设置的全局快捷键（默认为 `Alt+Q`）。
    -   屏幕会变暗，鼠标变为十字准星。

3.  **选择区域**:
    -   按住鼠标左键并拖动，选择您想要翻译的文本区域。
    -   选区右下角会实时显示截图尺寸。
    -   松开鼠标左键即可完成截图。
    -   如果想取消，请按 `ESC` 键。

4.  **查看结果**:
    -   截图完成后，屏幕上会自动弹出一个半透明的结果窗口。
    -   窗口上方显示 OCR 识别出的原文，下方显示 DeepL 翻译后的译文。
    -   您可以使用窗口顶部的工具栏进行置顶、复制、朗读等操作。
    -   双击窗口或按 `ESC` 键可关闭结果窗口。

## 📂 项目结构

```
.
├── external/               # 存放外部依赖，如 OCR 引擎
├── icons/                  # 应用图标
├── src/                    # 前端代码 (HTML, CSS, JS)
│   ├── css/
│   ├── js/
│   ├── index.html          # 设置窗口
│   ├── results.html        # 结果窗口
│   └── screenshot.html     # 截图窗口
├── src-tauri/              # 后端 Rust 代码
│   ├── src/
│   │   ├── commands.rs     # 截图、OCR 和翻译的核心逻辑
│   │   ├── settings.rs     # 设置的加载、保存及相关指令
│   │   ├── translator.rs   # DeepL 翻译器实现
│   │   └── main.rs         # 应用主入口、系统托盘、快捷键管理
│   ├── build.rs
│   └── Cargo.toml          # Rust 依赖配置
└── tauri.conf.json         # Tauri 应用配置
```

## 🤝 贡献

欢迎对本项目进行贡献！如果您有任何想法或建议，请随时提交 Pull Request 或创建 Issue。

1.  Fork 本仓库
2.  创建您的特性分支 (`git checkout -b feature/AmazingFeature`)
3.  提交您的更改 (`git commit -m 'Add some AmazingFeature'`)
4.  将您的分支推送到远程 (`git push origin feature/AmazingFeature`)
5.  打开一个 Pull Request

## 📄 许可证

本项目基于 [MIT License](./LICENSE) 开源。

---