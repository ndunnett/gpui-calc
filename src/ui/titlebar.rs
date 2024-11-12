#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::Titlebar;

#[cfg(not(target_os = "windows"))]
mod other;
#[cfg(not(target_os = "windows"))]
pub use other::Titlebar;
