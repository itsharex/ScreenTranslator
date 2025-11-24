// 导入Tauri API
const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;
const { listen } = window.__TAURI__.event;

// --- DOM 元素获取 ---
const shortcutInput = document.getElementById('shortcut-input');
const viewShortcutInput = document.getElementById('view-shortcut-input');
const targetLangSelect = document.getElementById('target-lang-select');
const autostartCheckbox = document.getElementById('autostart-checkbox');
const saveBtn = document.getElementById('save-btn');
const statusMessage = document.getElementById('status-message');
const ocrCheckbox = document.getElementById('ocr-checkbox');
const translationCheckbox = document.getElementById('translation-checkbox');

// --- 最小侵入修复：为输入框设置初始默认值 ---
// 这可以防止在从后端加载真实设置值之前的短暂空白状态。
shortcutInput.value = 'F1';
viewShortcutInput.value = 'F3';


// --- 状态变量 ---
// isRecording 状态需要为每个输入框单独管理，因此使用对象
let isRecording = {
    main: false,
    view: false,
};
let currentSettings = {};

// --- 函数定义 ---

/**
 * 从后端加载设置并更新UI
 */
async function loadSettings() {
    try {
        const settings = await invoke('get_settings');
        currentSettings = settings;
        console.log("加载到设置:", settings);

        // 更新UI元素的值
        shortcutInput.value = settings.shortcut;
        viewShortcutInput.value = settings.view_image_shortcut;
        targetLangSelect.value = settings.target_lang;
        autostartCheckbox.checked = settings.autostart;
        ocrCheckbox.checked = settings.enable_ocr;
        translationCheckbox.checked = settings.enable_translation;

    } catch (error) {
        console.error("加载设置失败:", error);
        showStatusMessage("加载设置失败!", true);
    }
}

/**
 * 保存当前UI上的设置到后端
 */
async function saveSettings() {
    const shortcutValue = shortcutInput.value.trim();
    if (!shortcutValue) {
        showStatusMessage("截图快捷键不能为空！", true);
        shortcutInput.focus();
        return;
    }

    const viewShortcutValue = viewShortcutInput.value.trim();
    if (!viewShortcutValue) {
        showStatusMessage("查看截图快捷键不能为空！", true);
        viewShortcutInput.focus();
        return;
    }

    // 从UI元素收集最新的设置值
    const newSettings = {
        shortcut: shortcutValue,
        view_image_shortcut: viewShortcutValue,
        target_lang: targetLangSelect.value,
        autostart: autostartCheckbox.checked,
        enable_ocr: ocrCheckbox.checked,
        enable_translation: translationCheckbox.checked,
    };

    try {
        await invoke('set_settings', { settings: newSettings });
        showStatusMessage("设置已保存!", false);
        currentSettings = newSettings; // 更新本地缓存的设置
    } catch (error) {
        console.error("保存设置失败:", error);
        showStatusMessage(`保存设置失败! ${error}`, true);
    }
}

/**
 * 在界面上显示状态消息
 */
function showStatusMessage(msg, isError = false) {
    statusMessage.textContent = msg;
    statusMessage.style.color = isError ? 'var(--error-color)' : 'var(--accent-color)';
    setTimeout(() => {
        statusMessage.textContent = '';
    }, 4000);
}

/**
 * 格式化并显示快捷键
 */
function formatShortcut(e) {
    const parts = [];
    if (e.ctrlKey) parts.push('Ctrl');
    if (e.altKey) parts.push('Alt');
    if (e.shiftKey) parts.push('Shift');
    if (e.metaKey) parts.push('Super');

    const key = e.key.toLowerCase();
    if (!['control', 'alt', 'shift', 'meta'].includes(key)) {
        parts.push(e.code.replace('Key', '').replace('Digit', ''));
    }

    return parts.join('+');
}


// --- 事件监听 ---

saveBtn.addEventListener('click', saveSettings);

// -- 主截图快捷键的监听 --
shortcutInput.addEventListener('focus', () => {
    isRecording.main = true;
    shortcutInput.value = '请按下快捷键...';
});

shortcutInput.addEventListener('blur', () => {
    isRecording.main = false;
    if (shortcutInput.value === '请按下快捷键...') {
        shortcutInput.value = currentSettings.shortcut || 'F1';
    }
});

shortcutInput.addEventListener('keydown', (e) => {
    if (isRecording.main) {
        e.preventDefault();
        const formatted = formatShortcut(e);
        if (formatted && (formatted.includes('+') || formatted.startsWith('F'))) {
            shortcutInput.value = formatted;
            shortcutInput.blur(); // 录制成功后自动失焦
        }
    }
});

// -- “查看截图快捷键”的监听 --
viewShortcutInput.addEventListener('focus', () => {
    isRecording.view = true;
    viewShortcutInput.value = '请按下快捷键...';
});

viewShortcutInput.addEventListener('blur', () => {
    isRecording.view = false;
    // 如果用户什么都没按就离开，恢复为旧的有效快捷键
    if (viewShortcutInput.value === '请按下快捷键...') {
        viewShortcutInput.value = currentSettings.view_image_shortcut || 'F3';
    }
});

viewShortcutInput.addEventListener('keydown', (e) => {
    if (isRecording.view) {
        e.preventDefault();
        const formatted = formatShortcut(e);
        // 确保是有效的组合键或功能键
        if (formatted && (formatted.includes('+') || formatted.startsWith('F'))) {
            viewShortcutInput.value = formatted;
            viewShortcutInput.blur(); // 录制成功后自动失焦
        }
    }
});


// -- 功能开关联动逻辑 --
translationCheckbox.addEventListener('change', () => {
    if (translationCheckbox.checked) {
        ocrCheckbox.checked = true;
    }
});

ocrCheckbox.addEventListener('change', () => {
    if (!ocrCheckbox.checked) {
        translationCheckbox.checked = false;
    }
});

// -- 初始化 --
listen('backend-ready', () => {
    console.log("接收到 'backend-ready' 事件，开始加载设置...");
    loadSettings();
});