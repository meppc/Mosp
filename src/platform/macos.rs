use super::common::{ClipboardOperations, InputDeviceType, InputEvent};
use core_graphics::event::{CGEvent, CGEventType};

#[cfg(feature = "apple-pencil")]
use core_graphics::event::{CGEventTapLocation, CGEventField};

pub struct MacOSPlatform;

impl MacOSPlatform {
    pub fn new() -> Self {
        MacOSPlatform {}
    }

    pub fn get_pressure_sensitivity(&self, event: &CGEvent) -> f32 {
        #[cfg(feature = "apple-pencil")]
        {
            // 获取压感数据，范围从0-1
            if let Some(pressure) = event.get_doublevalue_field(CGEventField::TabletPressure) {
                return pressure as f32;
            }
        }
        
        // 默认返回最大压力
        1.0
    }
    
    pub fn get_tilt_data(&self, event: &CGEvent) -> (f32, f32) {
        #[cfg(feature = "apple-pencil")]
        {
            // 获取Apple Pencil的倾斜数据
            let tilt_x = event.get_doublevalue_field(CGEventField::TabletTiltX).unwrap_or(0.0) as f32;
            let tilt_y = event.get_doublevalue_field(CGEventField::TabletTiltY).unwrap_or(0.0) as f32;
            return (tilt_x, tilt_y);
        }
        
        // 默认返回无倾斜
        (0.0, 0.0)
    }
    
    pub fn detect_input_device(&self, event: &CGEvent) -> InputDeviceType {
        #[cfg(feature = "apple-pencil")]
        {
            // 检测是否为Apple Pencil输入
            if event.get_integer_value_field(CGEventField::TabletPointerType) > 0 {
                return InputDeviceType::Pen;
            }
        }
        
        // 根据事件类型判断
        match event.get_type() {
            CGEventType::LeftMouseDown | CGEventType::LeftMouseUp | CGEventType::RightMouseDown 
            | CGEventType::RightMouseUp | CGEventType::MouseMoved => {
                InputDeviceType::Mouse
            },
            CGEventType::OtherMouseDown | CGEventType::OtherMouseUp => {
                // 在macOS上，触摸事件通常映射为其他鼠标事件
                // 这里需要更复杂的逻辑来区分，简化处理
                InputDeviceType::Touch
            },
            _ => InputDeviceType::Unknown,
        }
    }
    
    pub fn process_input_event(&self, event: &CGEvent) -> InputEvent {
        let device_type = self.detect_input_device(event);
        let pressure = self.get_pressure_sensitivity(event);
        let (tilt_x, tilt_y) = self.get_tilt_data(event);
        
        let location = event.location();
        
        InputEvent {
            x: location.x as f32,
            y: location.y as f32,
            pressure,
            tilt_x,
            tilt_y,
            device_type,
        }
    }
}

pub struct MacOSClipboard;

impl ClipboardOperations for MacOSClipboard {
    fn copy_to_clipboard(&self, data: &[u8]) -> bool {
        // macOS特定的剪贴板实现
        // 实际应用中，这里会使用macOS的Pasteboard API
        println!("复制到macOS剪贴板: {} 字节", data.len());
        true
    }

    fn paste_from_clipboard(&self) -> Option<Vec<u8>> {
        // macOS特定的剪贴板实现
        // 实际应用中，这里会使用macOS的Pasteboard API
        println!("从macOS剪贴板粘贴");
        Some(vec![0; 10]) // 示例数据
    }
}