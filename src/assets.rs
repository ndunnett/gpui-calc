use std::borrow::Cow;

use anyhow::anyhow;
use gpui::*;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "fonts/**/*"]
#[include = "icons/**/*"]
#[exclude = "*.DS_Store"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(f) = Self::get(path) {
            Ok(Some(f.data))
        } else {
            Err(anyhow!("could not find asset at path \"{}\"", path))
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| {
                if p.starts_with(path) {
                    Some(p.into())
                } else {
                    None
                }
            })
            .collect())
    }
}

impl Assets {
    pub fn load_fonts(&self, cx: &AppContext) -> gpui::Result<()> {
        let fonts = self
            .list("fonts")?
            .iter()
            .filter_map(|path| {
                if path.ends_with(".ttf") {
                    cx.asset_source().load(path).unwrap_or(None)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        cx.text_system().add_fonts(fonts)
    }
}
