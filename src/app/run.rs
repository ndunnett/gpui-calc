use gpui::{
    px, App, Application, Bounds, Pixels, Size, TitlebarOptions, WindowBackgroundAppearance,
    WindowBounds, WindowKind, WindowOptions,
};

use crate::{
    app::{set_keybinds, Assets},
    state::StateEntity,
    ui::{Root, Theme},
};

const WINDOW_SIZE: Size<Pixels> = Size {
    width: px(230.),
    height: px(320.),
};

const MIN_WINDOW_SIZE: Size<Pixels> = Size {
    width: px(200.),
    height: px(280.),
};

const APP_TITLE: &str = "GPUI Calculator";

pub fn run() {
    Application::new().with_assets(Assets).run(|app: &mut App| {
        app.set_global(Theme::default());
        Assets.load_fonts(app).expect("failed to load fonts");
        StateEntity::build(app);
        set_keybinds(app);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                WINDOW_SIZE,
                app,
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

        app.open_window(window_options, Root::build)
            .expect("failed to open window");

        app.activate(true);
    });
}
