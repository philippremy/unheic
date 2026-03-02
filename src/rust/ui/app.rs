use gpui_component_assets::Assets;
use gpui_platform::application;
use log::debug;

use crate::{
    resource::assets::{MultiAssetsSource, UnHEICAssets},
    ui::app_delegate::app_did_finish_launching,
};

pub(crate) fn start_app() -> crate::Result<()> {
    // Get application from gpui_platform
    let app = application().with_assets(
        MultiAssetsSource::new()
            .append_provider(UnHEICAssets)
            .append_provider(Assets),
    );

    debug!("Starting application...");

    // Run the App
    app.run(app_did_finish_launching);

    Ok(())
}
