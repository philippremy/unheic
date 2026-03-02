use std::sync::Arc;

use gpui::{App, Context, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};
use gpui_component::{
    IndexPath, Selectable, Sizable, StyledExt,
    button::{Button, ButtonVariants},
    label::Label,
    list::{ListDelegate, ListState},
};

use crate::{
    backend::state::{InputListData, InputListEntry},
    resource::assets::UnHEICIcons,
    ui::input_image::InputImageThumbnail,
};

impl ListDelegate for InputListData {
    type Item = InputListEntry;

    fn items_count(&self, _: usize, _: &App) -> usize {
        self.items.len()
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _: &mut Window,
        _: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        // Look up the path for the given index
        if let Some(entry) = self.items.get_index(ix.row) {
            if let Some(entry) = self.list_data.get(entry) {
                if let Ok(entry) = entry.read() {
                    return Some(entry.clone());
                }
            }
        }
        None
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }

    // Empty stays empty!
    fn render_empty(
        &mut self,
        _: &mut Window,
        _: &mut Context<ListState<Self>>,
    ) -> impl IntoElement {
        div()
    }
}

impl RenderOnce for InputListEntry {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .h_32()
            .pt_4()
            .pb_4()
            .pr_3()
            .flex()
            .justify_between()
            .items_center()
            .gap_6()
            .child(InputImageThumbnail::new(
                self.thumbnail,
                Arc::clone(&self.path),
            ))
            .child(
                div()
                    .flex_grow()
                    .h_full()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .items_start()
                    .child(
                        Label::new(format!("{}", self.filename))
                            .font_bold()
                            .text_sm(),
                    )
                    .child(
                        Label::new(format!("{}", self.path.display()))
                            .font_light()
                            .text_xs(),
                    ),
            )
            .child(
                Button::new(format!("button-remove-input-{}", self.path.display()))
                    .icon(UnHEICIcons::RemoveImage)
                    .ghost()
                    .with_size(px(24.)),
            )
    }
}

impl Selectable for InputListEntry {
    fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    fn is_selected(&self) -> bool {
        self.is_selected
    }
}
