use gpui::*;

use crate::assets::Assets;
use crate::state::StateModel;
use crate::theme::Theme;
use crate::ui::Root;

const WINDOW_SIZE: Size<Pixels> = Size {
    width: px(230.),
    height: px(320.),
};

const APP_TITLE: &str = "GPUI Calculator";

fn get_window_options(bounds: Bounds<DevicePixels>) -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
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
        cx.set_global(Theme::default());
        let _ = Assets.load_fonts(cx);
        StateModel::build(cx);
        let bounds = Bounds::centered(None, WINDOW_SIZE, cx);
        cx.open_window(get_window_options(bounds), Root::build);
        cx.activate(true);
    });
}
