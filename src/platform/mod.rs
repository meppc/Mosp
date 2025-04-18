#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "ios")]
pub mod ios;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_arch = "wasm32")]
pub mod web;

// 平台无关的通用功能
pub mod common;

// 平台特性检测
pub fn has_pressure_sensitivity() -> bool {
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "windows"))]
    {
        return true;
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "windows")))]
    {
        return false;
    }
}

pub fn has_tilt_support() -> bool {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        return true;
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    {
        return false;
    }
}