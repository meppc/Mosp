// 平台无关的通用功能

// 输入设备类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputDeviceType {
    Mouse,
    Touch,
    Pen,
    Unknown,
}

// 输入事件数据
#[derive(Debug, Clone)]
pub struct InputEvent {
    pub x: f32,
    pub y: f32,
    pub pressure: f32,  // 压力值，范围 0.0-1.0
    pub tilt_x: f32,    // X轴倾斜角度，范围 -1.0-1.0
    pub tilt_y: f32,    // Y轴倾斜角度，范围 -1.0-1.0
    pub device_type: InputDeviceType,
}

impl Default for InputEvent {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            pressure: 1.0,
            tilt_x: 0.0,
            tilt_y: 0.0,
            device_type: InputDeviceType::Unknown,
        }
    }
}

// 文件格式处理
pub fn get_supported_formats() -> Vec<&'static str> {
    vec!["mosp", "png", "jpg", "svg"]
}

// 剪贴板操作
pub trait ClipboardOperations {
    fn copy_to_clipboard(&self, data: &[u8]) -> bool;
    fn paste_from_clipboard(&self) -> Option<Vec<u8>>;
}

// 默认实现
pub struct DefaultClipboard;

impl ClipboardOperations for DefaultClipboard {
    fn copy_to_clipboard(&self, _data: &[u8]) -> bool {
        // 默认实现返回失败，需要平台特定代码覆盖
        false
    }

    fn paste_from_clipboard(&self) -> Option<Vec<u8>> {
        // 默认实现返回空，需要平台特定代码覆盖
        None
    }
}