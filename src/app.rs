use gpui::*;

use crate::assets::Assets;
use crate::keybinds::set_keybinds;
use crate::state::StateModel;
use crate::theme::Theme;
use crate::ui::Root;

const WINDOW_SIZE: Size<Pixels> = Size {
    width: px(230.),
    height: px(320.),
};

const MIN_WINDOW_SIZE: Size<Pixels> = Size {
    width: px(200.),
    height: px(280.),
};

const APP_TITLE: &str = "GPUI Calculator";

pub fn run_app() {
    App::new().with_assets(Assets).run(|cx: &mut AppContext| {
        cx.set_global(Theme::default());
        let _ = Assets.load_fonts(cx);
        StateModel::build(cx);
        set_keybinds(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                WINDOW_SIZE,
                cx,
            ))),
            titlebar: Some(TitlebarOptions {
                title: Some(APP_TITLE.into()),
                appears_transparent: true,
                ..Default::default()
            }),
            window_background: WindowBackgroundAppearance::Blurred,
            kind: if cfg!(target_os = "windows") {
                WindowKind::Normal
            } else {
                WindowKind::PopUp
            },
            window_min_size: Some(MIN_WINDOW_SIZE),
            ..Default::default()
        };

        let _ = cx.open_window(window_options, Root::build);
        cx.activate(true);
    });
}
