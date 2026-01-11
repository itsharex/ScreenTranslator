// 从 tauri APIs 中导入所需模块
const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;
const { appWindow } = window.__TAURI__.window;
const { writeText } = window.__TAURI__.clipboard;
const { isPermissionGranted, requestPermission, sendNotification } = window.__TAURI__.notification;

// --- DOM 元素获取 ---
const originalTextEl = document.getElementById('original-text');
const translatedTextEl = document.getElementById('translated-text');
const pinBtn = document.getElementById('pin-btn');
const copyOriginalBtn = document.getElementById('copy-original-btn');
const copyTranslatedBtn = document.getElementById('copy-translated-btn');
const copyImageBtn = document.getElementById('copy-image-btn');
const saveImageBtn = document.getElementById('save-image-btn');
const ttsBtn = document.getElementById('tts-btn');

// --- 状态变量 ---
let isPinned = true;
let originalTextContent = '';
let translatedTextContent = '';
let currentImagePath = '';

// --- 函数定义 (保持不变) ---

function giveFeedback(element) {
    if (!element) return;
    element.classList.add('clicked-feedback');
    setTimeout(() => {
        element.classList.remove('clicked-feedback');
    }, 200);
}

async function notify(title, body) {
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === 'granted';
    }
    if (permissionGranted) {
        sendNotification({ title, body });
    }
}

async function togglePin() {
    isPinned = !isPinned;
    await appWindow.setAlwaysOnTop(isPinned);
    pinBtn.classList.toggle('active', isPinned);
    pinBtn.title = isPinned ? "取消置顶" : "钉在最前";
}

async function copyText(text, type) {
    if (!text) return;
    try {
        await writeText(text);
        await notify('复制成功', `${type}已复制到剪贴板。`);
    } catch (error) {
        console.error(`复制 ${type} 失败:`, error);
        await notify('复制失败', `未能将${type}复制到剪贴板。`);
    }
}

function speakText() {
    if (!translatedTextContent || window.speechSynthesis.speaking) return;
    const utterance = new SpeechSynthesisUtterance(translatedTextContent);
    window.speechSynthesis.speak(utterance);
}

// --- 核心修改：重构后的 UI 更新逻辑 ---

/**
 * 处理 OCR 结果数据并更新 UI
 */
function handleOcrResultPayload(payload) {
    console.log("[RESULTS.JS] 处理 OCR 数据:", payload);

    // 重置译文区域颜色
    translatedTextEl.style.color = 'var(--text-color-bright)';

    // 1. 处理图片路径
    if (payload.image_path) {
        currentImagePath = payload.image_path;
        copyImageBtn.style.display = 'inline-block';
        saveImageBtn.style.display = 'inline-block';
    } else {
        currentImagePath = '';
        copyImageBtn.style.display = 'none';
        saveImageBtn.style.display = 'none';
    }

    // 2. 根据是否有错误来更新文本区域
    if (payload.error_message) {
        originalTextContent = payload.original_text || "错误";
        translatedTextContent = payload.error_message;

        originalTextEl.textContent = originalTextContent;
        translatedTextEl.textContent = translatedTextContent;
        translatedTextEl.style.color = 'var(--error-color)';
    } else {
        originalTextContent = payload.original_text || '';
        // 只有当译文内容为空时（即刚识别完），才显示“翻译中...”
        // 否则保持现有内容（可能是后续推送的译文或从缓存拉取的译文）
        if (!translatedTextContent) {
            translatedTextEl.textContent = '翻译中...';
        }
        originalTextEl.textContent = originalTextContent;
    }
}

/**
 * 处理翻译结果数据并更新 UI
 */
function handleTranslationUpdatePayload(payload) {
    console.log("[RESULTS.JS] 处理翻译数据:", payload);

    if (payload.error_message) {
        translatedTextContent = payload.error_message;
        translatedTextEl.textContent = translatedTextContent;
        translatedTextEl.style.color = 'var(--error-color)';
    } else {
        translatedTextContent = payload.translated_text || '';
        translatedTextEl.textContent = translatedTextContent;
        translatedTextEl.style.color = 'var(--text-color-bright)';
    }
}


// --- 事件监听 ---

listen('ocr_result', (event) => {
    // 重置译文状态，因为这是一次新的识别
    translatedTextContent = '';
    handleOcrResultPayload(event.payload);
});

listen('translation_update', (event) => {
    handleTranslationUpdatePayload(event.payload);
});


// --- 其他交互事件 (保持不变) ---
document.body.addEventListener('dblclick', () => {
    appWindow.close();
});

document.addEventListener('keydown', async (e) => {
    if (e.key === 'Escape') {
        await appWindow.close();
    }
});

pinBtn.addEventListener('click', togglePin);

copyOriginalBtn.addEventListener('click', () => {
    giveFeedback(copyOriginalBtn);
    copyText(originalTextContent, '原文');
});

copyTranslatedBtn.addEventListener('click', () => {
    giveFeedback(copyTranslatedBtn);
    copyText(translatedTextContent, '译文');
});

ttsBtn.addEventListener('click', () => {
    giveFeedback(ttsBtn);
    speakText();
});

copyImageBtn.addEventListener('click', async () => {
    giveFeedback(copyImageBtn);
    if (!currentImagePath) return;
    try {
        await invoke('copy_image_to_clipboard', { path: currentImagePath });
        await notify('复制成功', '截图已复制到剪贴板。');
    } catch (error) {
        console.error("复制图片失败:", error);
        await notify('复制失败', `错误: ${error}`);
    }
});

saveImageBtn.addEventListener('click', async () => {
    giveFeedback(saveImageBtn);
    if (!currentImagePath) return;
    try {
        await invoke('save_image_to_desktop', { path: currentImagePath });
        await notify('保存成功', '截图已保存到桌面。');
    } catch (error) {
        console.error("保存图片失败:", error);
        await notify('保存失败', `错误: ${error}`);
    }
});

// --- 初始化与数据拉取 ---

async function init() {
    pinBtn.classList.add('active');
    pinBtn.title = "取消置顶";
    copyImageBtn.style.display = 'none';
    saveImageBtn.style.display = 'none';

    console.log("[RESULTS.JS] 结果窗口初始化...");

    // 核心修复：主动从后端获取最新的缓存数据
    // 这解决了因窗口重新创建导致的事件丢失问题
    try {
        const cached = await invoke('get_last_ocr_result');
        if (cached) {
            console.log("[RESULTS.JS] 成功拉取缓存数据:", cached);

            // 1. 恢复 OCR 数据
            handleOcrResultPayload({
                original_text: cached.original_text,
                error_message: null,
                image_path: cached.image_path
            });

            // 2. 恢复翻译数据 (如果有)
            if (cached.translated_text) {
                handleTranslationUpdatePayload({
                    translated_text: cached.translated_text,
                    error_message: null
                });
            }
        }
    } catch (err) {
        console.error("[RESULTS.JS] 拉取缓存数据失败:", err);
    }
}

// 执行初始化
init();
