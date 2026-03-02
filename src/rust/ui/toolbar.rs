use gpui::{
    App, Entity, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder, px, rems,
};
use gpui_component::{
    ActiveTheme, Sizable,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    progress::Progress,
};

use crate::{backend::state::ConversionState, resource::assets::UnHEICIcons};

#[derive(IntoElement)]
pub(crate) struct Toolbar {
    conversion_state: Entity<ConversionState>,
}

impl Toolbar {
    pub(crate) fn with_entity(entity: Entity<ConversionState>) -> Self {
        Self {
            conversion_state: entity,
        }
    }
}

impl RenderOnce for Toolbar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let toolbar_button_style = ButtonCustomVariant::new(cx)
            .color(cx.theme().transparent)
            .foreground(cx.theme().accent)
            .hover(cx.theme().primary_foreground.opacity(0.05))
            .active(cx.theme().primary_foreground.opacity(0.15));

        div()
            .w_full()
            .h_12()
            .rounded_xl()
            .bg(cx.theme().accent)
            .shadow_md()
            .flex()
            .items_center()
            .justify_between()
            .when(cfg!(target_os = "macos"), |this| this.pl(rems(5.)))
            .when(!cfg!(target_os = "macos"), |this| this.pl_2())
            .child(
                Button::new("button-settings")
                    .with_size(px(24.))
                    .p_4()
                    .ghost()
                    .text_color(cx.theme().foreground)
                    .icon(UnHEICIcons::Gear),
            )
            .child(
                div()
                    .h_full()
                    .flex_grow()
                    .pl_24()
                    .pr_24()
                    .flex()
                    .justify_center()
                    .items_center()
                    .when(self.conversion_state.read(cx).is_in_progress(), |this| {
                        this.child(
                            Progress::new("progress-conversion")
                                .value(
                                    *self
                                        .conversion_state
                                        .read(cx)
                                        .try_as_in_progress_ref()
                                        .unwrap(),
                                )
                                .color(cx.theme().green),
                        )
                    }),
            )
            .child(
                div()
                    .h_full()
                    .pl_6()
                    .pr_2()
                    .overflow_hidden()
                    .rounded_l_full()
                    .rounded_r_xl()
                    .bg(cx.theme().green)
                    .flex()
                    .justify_end()
                    .items_center()
                    .gap_1()
                    .text_color(cx.theme().accent)
                    .child(
                        Button::new("button-add-image")
                            .with_size(px(24.))
                            .p_4()
                            .custom(toolbar_button_style)
                            .text_color(cx.theme().accent)
                            .icon(UnHEICIcons::AddImage),
                    )
                    .child(
                        Button::new("button-set-output-folder")
                            .with_size(px(24.))
                            .p_4()
                            .custom(toolbar_button_style)
                            .text_color(cx.theme().accent)
                            .icon(UnHEICIcons::SetOutputFolder),
                    )
                    .when_else(
                        self.conversion_state.read(cx).is_in_progress(),
                        |this_if| {
                            this_if.child(
                                Button::new("button-stop-conversion")
                                    .with_size(px(24.))
                                    .p_4()
                                    .custom(toolbar_button_style)
                                    .text_color(cx.theme().accent)
                                    .icon(UnHEICIcons::StopConversion),
                            )
                        },
                        |this_else| {
                            this_else.child(
                                Button::new("button-start-conversion")
                                    .with_size(px(24.))
                                    .p_4()
                                    .custom(toolbar_button_style)
                                    .text_color(cx.theme().accent)
                                    .icon(UnHEICIcons::StartConversion),
                            )
                        },
                    ),
            )
    }
}
