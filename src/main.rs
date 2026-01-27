use std::error::Error;

use gpui::{AppContext, Application, Global, KeyBinding, TitlebarOptions, WindowBounds, WindowOptions, actions, point, px, size};
use gpui_component::Root;
use gpui_component_assets::Assets;
use libheif_rs::integration::image::register_heic_decoding_hook;

mod actions;
mod conversion;
mod state;
mod ui;
mod utils;
mod theme;

/// This is currently required to access a regular App instance from
/// AsyncApp where needed
struct GlobalStub;
impl Global for GlobalStub {}

actions!(window, [Quit]);

fn main() {
    
    // Register libHEIF image hooks
    register_heic_decoding_hook();
    
    let app = Application::new()
        .with_assets(Assets);
    app.run(move |cx_sync| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx_sync);
        
        // Register global stub object
        cx_sync.set_global(GlobalStub);

        cx_sync.spawn(async move |cx_async| {
            let window_options = cx_async.read_global(|_: &GlobalStub, app| {
                WindowOptions {
                    window_bounds: WindowBounds::centered(size(px(800.), px(600.)), app).into(),
                    titlebar: TitlebarOptions { title: Some(format!("UnHEIC v{}", env!("CARGO_PKG_VERSION")).into()), appears_transparent: true, traffic_light_position: Some(point(px(16.), px(16.))) }.into(),
                    window_min_size: size(px(800.), px(600.)).into(),
                    ..Default::default()
                }
            }).unwrap();
            cx_async.open_window(window_options, |window, cx_async| {
                let view = cx_async.new(|cx| ui::Application::new(cx, window));
                // This first level on the window, should be a Root.
                cx_async.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, Box<dyn Error>>(())
        })
        .detach();
        
        cx_sync.activate(true);
        cx_sync.on_action(|_: &Quit, cx| cx.quit());
        cx_sync.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    });
}
