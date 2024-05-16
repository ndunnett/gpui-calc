use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::theme::Theme;
use crate::ui::components::*;

#[derive(Clone, Copy)]
pub struct Titlebar;

impl Titlebar {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for Titlebar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if cfg!(target_os = "windows") {
            let theme = cx.global::<Theme>();

            div()
                .flex()
                .flex_row()
                .h((2. * cx.rem_size()).max(px(32.)))
                .justify_end()
                .children([
                    windows_button(
                        WindowsButtonType::Minimize.code(),
                        theme.colors.bg_button_hover,
                    ),
                    if cx.is_maximized() {
                        windows_button(
                            WindowsButtonType::Restore.code(),
                            theme.colors.bg_button_hover,
                        )
                    } else {
                        windows_button(
                            WindowsButtonType::Maximize.code(),
                            theme.colors.bg_button_hover,
                        )
                    },
                    windows_button(
                        WindowsButtonType::Close.code(),
                        theme.colors.bg_close_button_hover,
                    ),
                ])
                .when(cx.is_maximized(), |this| this.mt_2())
        } else {
            div().h(rems(1.5))
        }
    }
}
