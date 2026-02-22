#[cfg(target_os = "linux")]
pub mod freedesktop;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub use freedesktop::FreeDesktopTrashManager as CurrentPlatformManager;
#[cfg(target_os = "macos")]
pub use macos::MacosTrashManager as CurrentPlatformManager;
#[cfg(target_os = "windows")]
pub use windows::WindowsTrashManager as CurrentPlatformManager;
