use assets::Assets;
use gpui::*;

use crate::state::State;
use crate::theme::Theme;
use crate::view::Root;

const APP_WIDTH: f32 = 230.;
const APP_HEIGHT: f32 = 320.;
const APP_TITLE: &str = "GPUI Calculator";

fn get_window_options(bounds: Bounds<DevicePixels>) -> WindowOptions {
    WindowOptions {
        bounds: Some(bounds),
        titlebar: Some(TitlebarOptions {
            title: Some(APP_TITLE.into()),
            appears_transparent: true,
            ..Default::default()
        }),
        focus: true,
        show: true,
        window_background: WindowBackgroundAppearance::Blurred,
        kind: WindowKind::PopUp,
        ..Default::default()
    }
}

fn load_fonts(cx: &AppContext) -> Result<()> {
    let asset_source = cx.asset_source();

    let fonts = asset_source
        .list("fonts")?
        .iter()
        .filter_map(|path| {
            if path.ends_with(".ttf") {
                asset_source.load(path).ok()
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    cx.text_system().add_fonts(fonts)
}

pub fn run_app() {
    App::new().with_assets(Assets).run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.set_global(Theme::default());
        cx.set_global(State::new());
        let _ = load_fonts(cx); // might fail, don't really care yet
        let bounds = Bounds::centered(None, size(px(APP_WIDTH), px(APP_HEIGHT)), cx);
        cx.open_window(get_window_options(bounds), Root::build);
    });
}
