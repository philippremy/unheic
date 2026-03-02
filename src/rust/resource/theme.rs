use std::{ops::Deref, rc::Rc, sync::LazyLock};

use gpui::{App, WindowAppearance};
use gpui_component::{Theme, ThemeSet};
use log::debug;

use crate::UnHEICError;

static UNHEIC_THEME: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/",
    "res/",
    "theme/",
    "theme.json"
));

static UNHEIC_THEME_SET: LazyLock<crate::Result<ThemeSet>> = LazyLock::new(|| {
    serde_json::from_str::<ThemeSet>(UNHEIC_THEME)
        .map_err(|err| UnHEICError::ThemeParsingError(err.to_string()))
});

pub(crate) fn set_app_theme(cx: &mut App) {
    match UNHEIC_THEME_SET.deref() {
        Ok(theme_set) => match cx.window_appearance() {
            WindowAppearance::Light | WindowAppearance::VibrantLight => {
                Theme::global_mut(cx).apply_config(&Rc::new(
                    theme_set
                        .themes
                        .iter()
                        .find(|theme| !theme.mode.is_dark())
                        .unwrap()
                        .clone(),
                ));
            }
            WindowAppearance::Dark | WindowAppearance::VibrantDark => {
                Theme::global_mut(cx).apply_config(&Rc::new(
                    theme_set
                        .themes
                        .iter()
                        .find(|theme| theme.mode.is_dark())
                        .unwrap()
                        .clone(),
                ));
            }
        },
        Err(err) => {
            panic!("Unable to parse required UnHEIC theme: {err}");
        }
    }
}

pub(crate) fn check_appearance(cx: &mut App) {
    let current_theme_is_dark = Theme::global(cx).mode.is_dark();
    match cx.window_appearance() {
        WindowAppearance::Light | WindowAppearance::VibrantLight => {
            if !current_theme_is_dark {
                return;
            }
            set_app_theme(cx);
        }
        WindowAppearance::Dark | WindowAppearance::VibrantDark => {
            if current_theme_is_dark {
                return;
            }
            set_app_theme(cx);
        }
    }
    debug!("Window appearance changed to: {:?}", cx.window_appearance());
}
