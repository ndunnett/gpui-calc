use gpui::*;

pub struct Colors {
    pub bg_window: Rgba,
    pub bg_button: Rgba,
    pub bg_button_hover: Rgba,
    pub bg_close_button_hover: Rgba,
    pub text: Rgba,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            bg_window: rgba(0xcccccc07),
            bg_button: rgba(0x45454535),
            bg_button_hover: rgba(0x55555555),
            bg_close_button_hover: rgba(0xcc2222cc),
            text: rgba(0xffffffcc),
        }
    }
}

pub struct Fonts {
    pub main_family: String,
}

impl Default for Fonts {
    fn default() -> Self {
        Self {
            main_family: "zed-sans".into(),
        }
    }
}

#[derive(Default)]
pub struct Theme {
    pub colors: Colors,
    pub fonts: Fonts,
}

impl Global for Theme {}
