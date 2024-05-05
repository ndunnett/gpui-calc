use gpui::*;

pub struct Palette {
    pub red: Rgba,
}

impl Default for Palette {
    fn default() -> Self {
        Self { red: rgb(0xfa1010) }
    }
}

pub struct Colors {
    pub bg_window: Rgba,
    pub bg_primary: Rgba,
    pub bg_secondary: Rgba,
    pub text: Rgba,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            bg_window: rgba(0xcccccc08),
            bg_primary: rgb(0x3a3a3a),
            bg_secondary: rgb(0x313131),
            text: rgb(0xFFFFFF),
        }
    }
}

#[derive(Default)]
pub struct Theme {
    pub colors: Colors,
    pub palette: Palette,
}

impl Global for Theme {}
