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

pub fn run_app(app: App) {
    app.run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.set_global(Theme::default());
        cx.set_global(State::new());
        let bounds = Bounds::centered(None, size(px(APP_WIDTH), px(APP_HEIGHT)), cx);
        cx.open_window(get_window_options(bounds), Root::build);
    });
}
