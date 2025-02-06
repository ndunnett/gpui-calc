use gpui::{App, AppContext, Entity};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(target_os = "windows"))]
mod other;

#[derive(Clone, Copy)]
pub struct Titlebar;

impl Titlebar {
    pub fn build(app: &mut App) -> Entity<Self> {
        app.new(|_cx| Self)
    }
}
