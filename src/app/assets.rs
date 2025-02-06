use std::borrow::Cow;

use anyhow::anyhow;
use gpui::{App, AssetSource, SharedString};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "fonts/**/*"]
#[include = "icons/**/*"]
#[exclude = "*.DS_Store"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if let Some(f) = Self::get(path) {
            Ok(Some(f.data))
        } else {
            Err(anyhow!("could not find asset at path \"{}\"", path))
        }
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
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
    pub fn load_fonts(&self, app: &App) -> gpui::Result<()> {
        let fonts = Self::iter()
            .filter_map(|path| {
                if path.starts_with("fonts") && path.ends_with(".ttf") {
                    app.asset_source().load(&path).unwrap_or(None)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        app.text_system().add_fonts(fonts)
    }
}
