use gpui::{App, Window};
use gpui_component::Theme;

pub(super) fn theme_fn(window: &mut Window, cx: &mut App) {
    Theme::sync_system_appearance(window.into(), cx);
}
