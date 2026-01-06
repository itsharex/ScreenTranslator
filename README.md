# ScreenTranslator - 高效离线截图翻译工具

![GitHub license](https://img.shields.io/github/license/git-hub-cc/ScreenTranslator)
![GitHub stars](https://img.shields.io/github/stars/git-hub-cc/ScreenTranslator?style=social)
![GitHub forks](https://img.shields.io/github/forks/git-hub-cc/ScreenTranslator?style=social)

一款基于 [Tauri](https://tauri.app/) 框架开发的现代化、轻量级的桌面端截图翻译工具。它采用完全离线的设计，通过简单的全局快捷键，即可快速捕捉屏幕区域、识别文字并进行翻译，保障您的数据隐私和使用便捷性。

## ✨ 核心功能

-   ✅ **全局快捷键**: 在任何应用中，通过 `F1` (可自定义) 一键呼出截图功能，`F3` (可自定义) 快速查看上次结果。
-   🖼️ **精准截图**: 拖拽鼠标即可选择屏幕区域，支持放大镜、尺寸提示，`ESC` 或右键轻松取消。
-   🔒 **完全离线**: 首次配置后，文字识别 (OCR) 和翻译过程均在本地完成，无需联网，确保数据安全和响应速度。
-   🚀 **自动化工作流**:
    -   **多模式选择**: 可根据习惯设置截图后的首要动作，包括：
        -   **仅识别文字 (OCR)**: 结果直接复制到剪贴板。
        -   **识别并翻译**: 原文和译文先后复制到剪贴板。
        -   **立即复制图片**: 将截图直接复制到剪贴板。
        -   **立即保存图片**: 将截图直接保存到桌面。
        -   **预览与手动操作**: 弹出预览窗口，手动选择后续操作。
-   ⚙️ **一体化引擎管理**:
    -   **内置下载器**: 在设置界面一键下载、更新离线 OCR 和翻译引擎包，无需手动配置。
    -   **状态清晰**: 实时显示引擎包的安装状态。
-   📋 **便捷的结果处理**:
    -   **置顶窗口**: 将结果窗口钉在屏幕最前端，方便对照。
    -   **一键复制**: 快速复制原文、译文或原始截图。
    -   **朗读译文**: 调用系统 TTS 引擎朗读翻译结果。
-   🖥️ **跨平台设计**: 基于 Tauri 构建，具备打包为多平台应用（Windows, macOS, Linux）的潜力。

## 🛠️ 技术栈

-   **核心框架**: [Tauri](https://tauri.app/) (使用 Rust 作为后端，Web 技术作为前端)
-   **后端**:
    -   [Rust](https://www.rust-lang.org/)
    -   [Tokio](https://tokio.rs/): 用于异步处理耗时任务（如截图、OCR、翻译和下载）。
    -   [Reqwest](https://github.com/seanmonstar/reqwest): 用于从 GitHub Releases 下载引擎包。
    -   [Serde](https://serde.rs/): 用于 JSON 序列化与反序列化。
    -   [xcap](https://crates.io/crates/xcap): 用于高效、跨平台的屏幕捕获。
    -   [zip](https://crates.io/crates/zip) / [sevenz-rust](https://crates.io/crates/sevenz-rust): 用于解压引擎包。
-   **前端**:
    -   HTML5, CSS3, Vanilla JavaScript
-   **OCR 引擎**: [RapidOCR-json](https://github.com/hiroi-sora/RapidOCR-json) (通过应用内下载器集成)
-   **翻译引擎**: [LocalTranslator](https://github.com/git-hub-cc/LocalTranslator) (通过应用内下载器集成)

## 🚀 安装与启动

### 对于普通用户

1.  前往 [GitHub Releases](https://github.com/git-hub-cc/ScreenTranslator/releases) 页面。
2.  下载适用于您操作系统的最新版本安装包（例如 `.msi` for Windows）。
3.  运行安装程序。首次启动后，请根据应用内的提示下载所需的离线引擎包。

### 对于开发者

请确保您的开发环境已安装 [Rust](https://www.rust-lang.org/tools/install) 和 [Node.js](https://nodejs.org/)。

1.  **克隆仓库**:
    ```bash
    git clone https://github.com/git-hub-cc/ScreenTranslator.git
    cd ScreenTranslator
    ```

2.  **安装前端依赖**:
    ```bash
    npm install
    ```

3.  **启动开发环境**:
    ```bash
    npm run tauri dev
    ```
    应用启动后，会自动打开设置窗口。您需要在此界面**点击下载按钮**来安装离线 OCR 和翻译引擎，之后所有功能才能正常使用。

4.  **构建应用**:
    ```bash
    npm run tauri build
    ```

> **注意**: 当前集成的离线引擎为 Windows `.exe` 版本，因此项目在未作修改的情况下主要支持 **Windows** 平台。

## 📖 使用指南

1.  **初次配置**:
    -   首次启动应用，会显示设置窗口。
    -   在 "本地识别引擎 (OCR)" 和 "本地翻译引擎" 区域，点击下载按钮。应用会自动下载并安装所需的模型包（首次下载需要一些时间）。
    -   安装完成后，状态会显示为 "已安装"。

2.  **选择工作模式**:
    -   在 "截图后的首要动作" 部分，选择最适合您的工作流的模式。例如，如果您只做文字摘录，选择 "识别文字 (OCR)"。

3.  **自定义设置**:
    -   根据需要修改截图快捷键、目标语言等选项。所有设置都会自动保存。
    -   完成后，您可以关闭设置窗口，应用会自动隐藏到系统托盘。

4.  **触发截图**:
    -   在任何界面，按下您设置的全局快捷键（默认为 `F1`）。
    -   屏幕会变暗，鼠标变为十字准星。

5.  **选择区域**:
    -   按住鼠标左键并拖动，选择您想要处理的区域。
    -   松开鼠标左键即可完成截图并触发您选择的动作。
    -   如果想取消，请按 `ESC` 键或单击鼠标右键。

6.  **获取结果**:
    -   根据您选择的模式，应用会执行相应操作：
        -   **自动模式**: 您会收到一个系统通知，提示内容已复制或保存。
        -   **预览模式**: 会弹出一个图片预览窗口，供您手动操作。
        -   **识别/翻译模式**: 如果您未选择自动复制，可以按 `F3` 快捷键随时调出上一次的结果窗口。

## 📂 项目结构

```
.
├── icons/                  # 应用图标
├── src/                    # 前端代码 (HTML, CSS, JS)
│   ├── css/
│   ├── js/
│   ├── index.html          # 设置窗口
│   ├── results.html        # 结果窗口
│   ├── screenshot.html     # 截图窗口
│   ├── image_viewer.html   # 图片预览窗口
│   └── loading.html        # 处理中提示窗口
├── src-tauri/              # 后端 Rust 代码
│   ├── src/
│   │   ├── capture.rs      # 屏幕捕获与图像编码
│   │   ├── commands.rs     # 核心指令：截图处理、引擎下载、OCR调用等
│   │   ├── settings.rs     # 设置的加载、保存及状态管理
│   │   ├── translator.rs   # 本地翻译器实现
│   │   └── main.rs         # 应用主入口、系统托盘、快捷键管理
│   ├── build.rs
│   └── Cargo.toml          # Rust 依赖配置
└── tauri.conf.json         # Tauri 应用配置
```

## 📄 许可证

本项目基于 [MIT License](./LICENSE) 开源。