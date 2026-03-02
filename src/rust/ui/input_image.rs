use std::{path::PathBuf, sync::Arc};

use gpui::{
    App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, img, prelude::FluentBuilder,
};
use gpui_component::{
    ActiveTheme, Disableable, Icon,
    button::{Button, ButtonVariants},
    skeleton::Skeleton,
};

use crate::{backend::state::ThumbnailState, resource::assets::UnHEICIcons};

#[derive(IntoElement)]
pub(crate) struct InputImageThumbnail {
    path: Arc<PathBuf>,
    source: ThumbnailState,
}

impl InputImageThumbnail {
    pub(crate) fn new(thumbnail_state: ThumbnailState, path: Arc<PathBuf>) -> Self {
        Self {
            path: path,
            source: thumbnail_state,
        }
    }
}

impl RenderOnce for InputImageThumbnail {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut image_container = div();
        image_container.style().aspect_ratio = Some(3. / 2.); // 3:2 Image by default
        image_container
            .h_full()
            .flex()
            .justify_center()
            .items_center()
            .rounded_md()
            .when(self.source.is_done(), |this| {
                this.child(img(Arc::clone(self.source.try_as_done_ref().unwrap())).h_full())
            })
            .when(self.source.is_requested(), |this| {
                this.border_1()
                    .border_color(cx.theme().foreground.opacity(0.5))
                    .child(Skeleton::new().h_full())
            })
            .when(self.source.is_error(), |this| {
                this.border_1()
                    .border_color(cx.theme().foreground.opacity(0.5))
                    .child(
                        Button::new(format!("button-thumbnail-failure-{}", self.path.display()))
                            .icon(
                                Icon::new(UnHEICIcons::ImageError)
                                    .h_1_5()
                                    .text_color(cx.theme().yellow),
                            )
                            .disabled(true)
                            .ghost()
                            .tooltip(format!(
                                "Unable to load image: {}",
                                self.source.try_as_error_ref().unwrap()
                            )),
                    )
            })
    }
}
