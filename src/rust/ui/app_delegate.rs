use gpui::{
    App, AppContext, KeyBinding, Point, Size, TitlebarOptions, WindowBounds, WindowOptions,
    actions, px,
};
use gpui_component::Root;
use log::debug;

use crate::{
    GPUIError, UnHEICError,
    resource::{
        info::{UNHEIC_APP_LOCALE, UNHEIC_APP_NAME, UNHEIC_APP_ORGANIZATION},
        theme::set_app_theme,
    },
    ui::main_ui::MainUI,
};

pub(crate) fn app_did_finish_launching(cx: &mut App) {
    debug!("Application finished launching. Initializing UI...");

    // Initialize keybindings
    register_keybindings(cx);

    debug!("Initialized keybindings.");

    // Register actions
    register_actions(cx);

    debug!("Registered global actions");

    // Initialize gpui-component
    gpui_component::init(cx);

    debug!("Initialized gpui-component.");

    cx.spawn(async move |cx| -> Result<(), UnHEICError> {
        let window_options = cx.update(|cx| WindowOptions {
            window_bounds: Some(WindowBounds::centered(Size::new(px(800.), px(600.)), cx)),
            titlebar: Some(TitlebarOptions {
                title: Some(UNHEIC_APP_NAME.into()),
                appears_transparent: true,
                traffic_light_position: Some(Point::new(px(24.), px(28.))),
            }),
            focus: true,
            show: true,
            app_id: Some(format!(
                "{}.{}.{}",
                UNHEIC_APP_LOCALE, UNHEIC_APP_ORGANIZATION, UNHEIC_APP_NAME
            )),
            window_min_size: Some(Size::new(px(800.), px(600.))),
            tabbing_identifier: Some(format!(
                "{}.{}.{}",
                UNHEIC_APP_LOCALE, UNHEIC_APP_ORGANIZATION, UNHEIC_APP_NAME
            )),
            ..Default::default()
        });

        debug!("Created WindowOptions.");

        cx.open_window(window_options, |window, cx| {
            let main_ui = cx.new(|cx| MainUI::new(window, cx));
            cx.new(|cx| Root::new(main_ui, window, cx))
        })
        .map_err(|err| UnHEICError::GPUIError(GPUIError::OpenWindowFailed(err.to_string())))?;

        debug!("Initial window opened.");

        Ok(())
    })
    .detach_and_log_err(cx);

    // Set initial theme
    debug!("UI initialized. Setting UnHEIC theme...");
    set_app_theme(cx);

    debug!("Theme set. Activating UI...");

    // Activate Window
    cx.activate(true);

    debug!("UI activated.");
}

actions!(window, [Quit]);

fn register_keybindings(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
}

fn register_actions(cx: &mut App) {
    cx.on_action(|_: &Quit, cx| cx.quit());
}
