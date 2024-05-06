use gpui::*;

use crate::state::State;
use crate::theme::Theme;
use crate::view::Root;
use crate::assets::Assets;

const WINDOW_SIZE: Size<Pixels> = Size {
    width: px(230.),
    height: px(320.),
};

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

pub fn run_app() {
    App::new().with_assets(Assets).run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.set_global(Theme::default());
        cx.set_global(State::new());
        let _ = Assets.load_fonts(cx);
        let bounds = Bounds::centered(None, WINDOW_SIZE, cx);
        cx.open_window(get_window_options(bounds), Root::build);
    });
}
