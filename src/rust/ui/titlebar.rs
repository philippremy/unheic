use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div};
use gpui_component::TitleBar;

use crate::resource::info::UNHEIC_APP_NAME;

#[derive(IntoElement)]
pub(crate) struct WindowTitleBar;

impl RenderOnce for WindowTitleBar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .child(TitleBar::new().w_full().border_0().child(UNHEIC_APP_NAME))
    }
}
