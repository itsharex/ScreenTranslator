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
const saveImageBtn = document.getElementById('save-image-btn'); // <-- 新增
const ttsBtn = document.getElementById('tts-btn');

// --- 状态变量 ---
let isPinned = true;
let originalTextContent = '';
let translatedTextContent = '';
let currentImagePath = '';

// --- 函数定义 ---

/**
 * --- 新增：给元素添加短暂的视觉反馈 ---
 * @param {HTMLElement} element - 需要反馈的DOM元素
 */
function giveFeedback(element) {
    if (!element) return;
    element.classList.add('clicked-feedback');
    setTimeout(() => {
        element.classList.remove('clicked-feedback');
    }, 200); // 反馈效果持续 200 毫秒
}


/**
 * 显示系统通知
 * @param {string} title - 通知标题
 * @param {string} body - 通知内容
 */
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

/**
 * 切换窗口的置顶状态
 */
async function togglePin() {
    isPinned = !isPinned;
    await appWindow.setAlwaysOnTop(isPinned);
    pinBtn.classList.toggle('active', isPinned);
    pinBtn.title = isPinned ? "取消置顶" : "钉在最前";
}

/**
 * 复制文本到剪贴板
 * @param {string} text - 要复制的文本
 * @param {string} type - 文本类型 (e.g., "原文", "译文")
 */
async function copyText(text, type) {
    if (!text) return;
    await writeText(text);
    await notify('复制成功', `${type}已复制到剪贴板。`);
}

/**
 * 使用浏览器TTS API朗读文本
 */
function speakText() {
    if (!translatedTextContent || window.speechSynthesis.speaking) return;
    const utterance = new SpeechSynthesisUtterance(translatedTextContent);
    window.speechSynthesis.speak(utterance);
}


// --- 事件监听 ---

listen('translation_result', (event) => {
    const payload = event.payload;
    console.log("接收到翻译结果:", payload);

    if (payload.error_message) {
        originalTextContent = "错误";
        translatedTextContent = payload.error_message;
        originalTextEl.textContent = originalTextContent;
        translatedTextEl.textContent = translatedTextContent;
        translatedTextEl.style.color = 'var(--error-color)';
        currentImagePath = '';
        copyImageBtn.style.display = 'none';
        saveImageBtn.style.display = 'none'; // <-- 新增
    } else {
        originalTextContent = payload.original_text;
        translatedTextContent = payload.translated_text;
        originalTextEl.textContent = originalTextContent;
        translatedTextEl.textContent = translatedTextContent;
        translatedTextEl.style.color = 'var(--text-color-bright)';
        currentImagePath = payload.image_path;
        copyImageBtn.style.display = 'inline-block';
        saveImageBtn.style.display = 'inline-block'; // <-- 新增
    }
});

document.body.addEventListener('dblclick', () => {
    appWindow.close();
});

document.addEventListener('keydown', async (e) => {
    if (e.key === 'Escape') {
        await appWindow.close();
    }
});

// --- 工具栏按钮事件 (核心修改区域) ---
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

// --- 新增：保存图片按钮事件 ---
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

// --- 初始化 ---
pinBtn.classList.add('active');
pinBtn.title = "取消置顶";
copyImageBtn.style.display = 'none';
saveImageBtn.style.display = 'none'; // <-- 新增