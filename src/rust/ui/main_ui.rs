use std::{path::PathBuf, sync::Arc, time::SystemTime};

use gpui::{
    AnchoredPositionMode, App, AppContext, Context, Entity, IntoElement, ParentElement, Render,
    Size, Styled, Window, anchored, deferred, div, prelude::FluentBuilder, px, rems,
};
use gpui_component::{
    ActiveTheme, TITLE_BAR_HEIGHT,
    button::Button,
    list::{List, ListState},
};
use log::trace;

use crate::{
    backend::state::{ConversionState, InputListData},
    resource::theme::check_appearance,
    ui::{titlebar::WindowTitleBar, toolbar::Toolbar},
};

pub(crate) struct MainUI {
    conversion_state: Entity<ConversionState>,
    input_list_state: Entity<ListState<InputListData>>,
}

impl MainUI {
    pub(crate) fn new(window: &mut Window, cx: &mut App) -> Self {
        Self {
            conversion_state: cx.new(|_| ConversionState::Idle),
            input_list_state: cx.new(|cx| ListState::new(InputListData::new(), window, cx)),
        }
    }
}

impl Render for MainUI {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Trace rendering
        trace!("MainUI: Rerendering...");

        // The paintable area of the viewport
        // It deducts the 3rem padding from each size
        let viewport = window.viewport_size() - Size::new(px(24.), px(12.));
        let list_width_accounted_neg_m = viewport.width + px(12.);

        // Trace theme handling
        trace!("Checking if the Window appearance changed...");
        check_appearance(cx);
        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_start()
            .bg(cx.theme().background)
            .when(
                cfg!(target_os = "windows") || cfg!(target_os = "linux"),
                |this| this.child(WindowTitleBar),
            )
            .child(
                div()
                    .w_full()
                    .flex_grow()
                    .pl_3()
                    .pr_3()
                    .pt_3()
                    .child(deferred(
                        anchored().position_mode(AnchoredPositionMode::Local).child(
                            div()
                                .when(!cfg!(target_os = "macos"), |this| this.pt(TITLE_BAR_HEIGHT))
                                .w(viewport.width)
                                .h(viewport.height)
                                .child(Toolbar::with_entity(self.conversion_state.clone())),
                        ),
                    ))
                    .child(deferred(anchored().child(
                        div().child(Button::new("test-button").label("New Item!").on_click(
                            cx.listener(|this, _, _, cx| {
                                this.input_list_state.update(cx, |entity, cx| {
                                    let rstr = format!("{:?}", SystemTime::now());
                                    entity.delegate_mut().add_new_entry(
                                        Arc::new(PathBuf::from(rstr)),
                                        cx,
                                        this.input_list_state.entity_id(),
                                    );
                                });
                                cx.notify();
                            }),
                        )),
                    )))
                    .child(
                        div()
                            .w(list_width_accounted_neg_m)
                            .h_full()
                            .flex()
                            .flex_col()
                            .justify_start()
                            .items_center()
                            .child(
                                List::new(&self.input_list_state)
                                    .pt(rems(3.5))
                                    .mr_neg_5()
                                    .ml_neg_5(),
                            ),
                    ),
            )
    }
}
