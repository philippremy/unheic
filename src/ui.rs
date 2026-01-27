use std::ops::Range;
use gpui::{App, Context, ElementId, Entity, ExternalPaths, Fill, ImageSource, InteractiveElement, IntoElement, ObjectFit, ParentElement, Render, Styled, StyledImage, Window, div, img, prelude::FluentBuilder, px, uniform_list};
use gpui_component::{ActiveTheme, Disableable, Icon, IconName, Sizable, StyledExt, button::{Button, ButtonCustomVariant, ButtonVariants}, checkbox::Checkbox, input::Input, label::Label, progress::Progress, select::Select, slider::{Slider, SliderState}, spinner::Spinner};
use strum::{EnumMessage, IntoEnumIterator};

pub(super) struct Application {
    pub(super) state: super::state::ApplicationState
}

impl Application {
    pub(super) fn new(mut cx: &mut App, window: &mut Window) -> Self {
        Self { state: super::state::ApplicationState::new(&mut cx, window) }
    }
}

impl Render for Application {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        
        // Force System Theme
        super::theme::theme_fn(window, cx);
        
        // Handle Select Event for Format
        super::actions::handle_select_event(&self.state.conversion_settings.format_dropdown_entity, window, cx, |cx, window, this, value| {
            let variant = super::state::ConversionSettingsDiscriminants::iter().find(|variant| { variant.get_message().unwrap() == value }).unwrap();
            this.state.conversion_settings.variant = variant;
            this.state.conversion_settings.settings = super::state::ConversionSettings::new(cx, window, variant);
        });
        
        // Handle Select Event for TIFF compression
        match &self.state.conversion_settings.settings {
            crate::state::ConversionSettings::TIFF(_, _, entity) => {
                super::actions::handle_select_event(entity, window, cx, |_, _, this, value| {
                    let variant = super::state::TIFFCompression::iter().find(|variant| { variant.get_message().unwrap() == value }).unwrap();
                    match &mut this.state.conversion_settings.settings {
                        crate::state::ConversionSettings::TIFF(_, tiffcompression, _) => *tiffcompression = variant,
                        _ => {}
                    }
                });
            },
            _ => {}
        }
        
        let mut slider_fn = |entity: &Entity<SliderState>| {
            super::actions::handle_slider_event(entity, window, cx, |_, _, this, value| {
                match &mut this.state.conversion_settings.settings {
                    crate::state::ConversionSettings::JPEG(_, comp, _) => *comp = value as u8,
                    crate::state::ConversionSettings::PNG(_, comp, _) => *comp = value as u8,
                    crate::state::ConversionSettings::WebP(_, comp, _) => *comp = value as u8,
                    crate::state::ConversionSettings::TIFF(_, _, _) => {},
                }
            });
        };
        
        // Handle Slider Events
        match &self.state.conversion_settings.settings {
            crate::state::ConversionSettings::JPEG(_, _, entity) => slider_fn(&entity),
            crate::state::ConversionSettings::PNG(_, _, entity) => slider_fn(&entity),
            crate::state::ConversionSettings::WebP(_, _, entity) => slider_fn(&entity),
            _ => {}
        }
        
        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .justify_between()
            .items_center()
            .child(
                crate::utils::PlatformConditional::modify_macos(
                    div()
                        .w_full()
                        .h_12()
                        .border_b_1()
                        .border_color(cx.theme().title_bar_border)
                        .bg(cx.theme().title_bar)
                        .p_3()
                        .flex()
                        .items_center()
                        .justify_between(), |this| { this.pl_24() })
                        .child(
                            div()
                                .h_full()
                                .text_sm()
                                .text_color(cx.theme().secondary_foreground)
                                .child(format!("UnHEIC v{}", env!("CARGO_PKG_VERSION")))
                        )
                        .child(
                            div()
                                .h_full()
                                .flex_grow()
                                .flex()
                                .justify_end()
                                .items_center()
                                .gap_2()
                                .child(
                                    Button::new("UnHEIC.UI.TitleBar.Button.AddImages")
                                        .text_xs()
                                        .tooltip("Bilder hinzufügen")
                                        .icon(
                                            Icon::new(IconName::Plus)
                                                .text_color(cx.theme().secondary_foreground)
                                        )
                                        .on_click(cx.listener(super::actions::handle_add_image_button))
                                )
                                .child(
                                    Button::new("UnHEIC.UI.TitleBar.Button.GitHub")
                                        .text_xs()
                                        .tooltip("Projektseite auf GitHub besuchen")
                                        .icon(
                                            Icon::new(IconName::GitHub)
                                                .text_color(cx.theme().secondary_foreground)
                                        )
                                        .on_click(|_, _, app| { app.open_url("https://github.com/philippremy/UnHEIC"); })
                                )
                                .child(
                                    Button::new("UnHEIC.UI.TitleBar.Button.Info")
                                        .text_xs()
                                        .tooltip("Informationen und Hilfe")
                                        .icon(
                                            Icon::new(IconName::Info)
                                                .text_color(cx.theme().secondary_foreground)
                                        )
                                )
                        )
            )
            .child(
                div()
                    .w_full()
                    .flex_grow()
                    .on_drag_move::<ExternalPaths>(cx.listener(super::actions::handle_file_drag))
                    .on_drop(cx.listener(super::actions::handle_file_drop))
                    .child(
                        div()
                            .pl_4()
                            .pr_4()
                            .size_full()
                            .child(
                                uniform_list(
                                    "UnHEIC.UI.InputArea.UniformList",
                                    self.state.input_image_state.total_count as usize,
                                    cx.processor(move |this, range: Range<usize>, _window, cx| {
                                        let mut items = Vec::new();
                                        for range_idx in range.clone() {
                                            let image = this.state.input_image_state.images.get_index_entry(range_idx).unwrap();
                                            items.push(
                                                div()
                                                    .w_full()
                                                    .h_24()
                                                    .flex()
                                                    .items_center()
                                                    .justify_between()
                                                    .child(
                                                        Label::new(format!("{}", range_idx + 1))
                                                            .text_xl()
                                                            .min_w_12()
                                                            .mr_4()
                                                            .h_full()
                                                            .flex()
                                                            .justify_center()
                                                            .items_center()
                                                    )
                                                    .map(|mut this| {
                                                        match &image.get().state {
                                                            super::state::ImageLoadingState::NotStarted | super::state::ImageLoadingState::InProgress => {
                                                                this = this.child(
                                                                    div()
                                                                    .h_full()
                                                                    .pt_2()
                                                                    .pb_2()
                                                                    .mr_6()
                                                                    .flex()
                                                                    .items_center()
                                                                    .justify_center()
                                                                    .child(
                                                                        Spinner::new()
                                                                    )
                                                                );
                                                            },
                                                            super::state::ImageLoadingState::Done(ri) => {
                                                                this = this.child(
                                                                    div()
                                                                        .h_full()
                                                                        .pt_2()
                                                                        .pb_2()
                                                                        .mr_6()
                                                                        .flex()
                                                                        .items_center()
                                                                        .justify_center()
                                                                        .child(
                                                                            img(ImageSource::Custom(ri.clone()))
                                                                                .max_h_full()
                                                                                .h_full()
                                                                                .object_fit(ObjectFit::Contain)
                                                                        )
                                                                )
                                                            },
                                                            super::state::ImageLoadingState::Failure(_) => todo!(),
                                                        }
                                                        this
                                                    })
                                                    .child(
                                                        div()
                                                            .h_full()
                                                            .flex_grow()
                                                            .flex()
                                                            .flex_col()
                                                            .justify_center()
                                                            .gap_1()
                                                            .child(
                                                                Label::new(format!("{}", image.get().name))
                                                                    .font_semibold() 
                                                            )
                                                            .child(
                                                                Label::new(format!("{}", image.get().path.display()))
                                                                    .text_sm()
                                                                    .italic()
                                                                    .font_light()
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .h_full()
                                                            .min_w_12()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .child(
                                                                Button::new(ElementId::Name(format!("UnHEIC.UI.InputArea.Button.RemoveInput.{}", image.get().path.display()).into()))
                                                                    .icon(Icon::new(IconName::Close))
                                                                    .on_click(cx.listener(move |this, _, _, cx| {
                                                                        this.state.input_image_state.images.remove_index(range_idx);
                                                                        this.state.input_image_state.total_count = this.state.input_image_state.images.len() as u16;
                                                                        cx.notify();
                                                                    }))
                                                            )
                                                    )
                                                    .when(range_idx != range.end - 1, |this| {
                                                        this
                                                            .border_b_1()
                                                            .border_color(cx.theme().title_bar_border)
                                                    })
                                            );
                                        }
                                        items
                                    }),
                                )
                                .h_full(),
                        )
                    )
            )
            .child(
                div()
                    .w_full()
                    .h_32()
                    .border_t_1()
                    .border_color(cx.theme().title_bar_border)
                    .bg(cx.theme().title_bar)
                    .flex()
                    .items_center()
                    .justify_around()
                    .p_3()
                    .child(
                        div()
                            .h_full()
                            .w_1_2()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                Input::new(&self.state.output_folder_state.ui_entity)
                                    .xsmall()
                                    .suffix(
                                        div()
                                            .h_full()
                                            .mr_neg_1()
                                            .border_l_1()
                                            .border_color(cx.theme().title_bar_border)
                                            .hover(|mut sr| {
                                                sr.background = Some(Fill::Color(cx.theme().title_bar.into()));
                                                sr
                                            })
                                            .child(
                                                Button::new("UnHEIC.UI.Footer.Button.OpenOutputFolder")
                                                    .custom(
                                                        ButtonCustomVariant::new(cx)
                                                    )
                                                    .on_click(cx.listener(super::actions::handle_open_folder_button))
                                                    .xsmall()
                                                    .compact()
                                                    .rounded_none()
                                                    .icon(Icon::new(IconName::Folder))
                                                    .tooltip("Ausgabeordner auswählen")
                                            )
                                    )
                            )
                            .child(
                                div()
                                    .w_full()
                                    .flex_grow()
                                    .flex()
                                    .items_center()
                                    .gap_4()
                                    .child(
                                        Label::new("Ausgabeformat")
                                            .text_xs()
                                    )
                                    .child(
                                        Select::new(&self.state.conversion_settings.format_dropdown_entity)
                                            .xsmall()
                                            .flex_grow()
                                    )
                            )
                            .child(
                                div()
                                    .h_full()
                                    .w_full()
                                    .flex()
                                    .items_center()
                                    .child(
                                        Checkbox::new("UnHEIC.UI.Footer.Checkbox.Metadata")
                                            .flex_shrink_0()
                                            .label("Metadaten beibehalten")
                                            .xsmall()
                                            .checked(match self.state.conversion_settings.settings {
                                                crate::state::ConversionSettings::JPEG(metadata, _, _) => metadata,
                                                crate::state::ConversionSettings::PNG(metadata, _, _) => metadata,
                                                crate::state::ConversionSettings::TIFF(metadata, _, _) => metadata,
                                                crate::state::ConversionSettings::WebP(metadata, _, _) => metadata,
                                            })
                                            .on_click(cx.listener(super::actions::handle_metadata_checkbox_change))
                                    )
                                    .child(
                                        div()
                                            .h_full()
                                            .w(px(1.))
                                            .bg(cx.theme().title_bar_border)
                                            .ml_2()
                                            .mr_2()
                                    )
                                    .child(
                                        div()
                                            .when(self.state.conversion_settings.variant == super::state::ConversionSettingsDiscriminants::JPEG, |div| {
                                                div
                                                .flex_grow()
                                                .flex()
                                                .flex_col()
                                                .justify_center()
                                                .items_center()
                                                .gap_1()
                                                .child(
                                                    Slider::new(match &self.state.conversion_settings.settings {
                                                        super::state::ConversionSettings::JPEG(_, _, entity) => entity,
                                                        _ => unreachable!("Logic Error: Encountered different enum variant for ConversionSettings")
                                                    })
                                                    .horizontal()
                                                )
                                                .child(
                                                    Label::new(format!("Qualität ({} %)", match &self.state.conversion_settings.settings {
                                                        super::state::ConversionSettings::JPEG(_, quality, _) => quality,
                                                        _ => unreachable!("Logic Error: Encountered different enum variant for ConversionSettings")
                                                    }))
                                                        .text_xs()
                                                )  
                                            })
                                            .when(self.state.conversion_settings.variant == super::state::ConversionSettingsDiscriminants::PNG, |div| {
                                                div
                                                .flex_grow()
                                                .flex()
                                                .flex_col()
                                                .justify_center()
                                                .items_center()
                                                .gap_1()
                                                .child(
                                                    Slider::new(match &self.state.conversion_settings.settings {
                                                        super::state::ConversionSettings::PNG(_, _, entity) => entity,
                                                        _ => unreachable!("Logic Error: Encountered different enum variant for ConversionSettings")
                                                    })
                                                    .horizontal()
                                                )
                                                .child(
                                                    Label::new(format!("Verlustfreie Kompression ({} %)", match &self.state.conversion_settings.settings {
                                                        super::state::ConversionSettings::PNG(_, compression, _) => compression,
                                                        _ => unreachable!("Logic Error: Encountered different enum variant for ConversionSettings")
                                                    }))
                                                        .text_xs()
                                                )  
                                            })
                                            .when(self.state.conversion_settings.variant == super::state::ConversionSettingsDiscriminants::WebP, |div| {
                                                div
                                                .flex_grow()
                                                .flex()
                                                .flex_col()
                                                .justify_center()
                                                .items_center()
                                                .gap_1()
                                                .child(
                                                    Slider::new(match &self.state.conversion_settings.settings {
                                                        super::state::ConversionSettings::WebP(_, _, entity) => entity,
                                                        _ => unreachable!("Logic Error: Encountered different enum variant for ConversionSettings")
                                                    })
                                                    .horizontal()
                                                )
                                                .child(
                                                    Label::new(format!("Qualität ({} %)", match &self.state.conversion_settings.settings {
                                                        super::state::ConversionSettings::WebP(_, compression, _) => compression,
                                                        _ => unreachable!("Logic Error: Encountered different enum variant for ConversionSettings")
                                                    }))
                                                        .text_xs()
                                                )  
                                            })
                                            .when(self.state.conversion_settings.variant == super::state::ConversionSettingsDiscriminants::TIFF, |div| {
                                                div
                                                .flex_grow()
                                                .flex()
                                                .flex_col()
                                                .justify_center()
                                                .items_center()
                                                .gap_1()
                                                .child(
                                                    Select::new(match &self.state.conversion_settings.settings {
                                                        super::state::ConversionSettings::TIFF(_, _, entity) => entity,
                                                        _ => unreachable!("Logic Error: Encountered different enum variant for ConversionSettings")
                                                    })
                                                    .xsmall()
                                                )
                                                .child(
                                                    Label::new("Verlustfreie Kompressionsart")
                                                        .text_xs()
                                                )  
                                            })
                                    )
                            )
                    )
                    .child(
                        div()
                            .h_full()
                            .w(px(1.))
                            .bg(cx.theme().title_bar_border)
                            .ml_4()
                            .mr_4()
                    )
                    .child(
                        div()
                            .h_full()
                            .w_1_2()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .gap_4()
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_col()
                                    .justify_center()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        Progress::new()
                                            .value(match self.state.conversion_progress {
                                                super::state::ConversionProgress::Completed(_) => 100.0,
                                                super::state::ConversionProgress::Inactive => 0.0,
                                                super::state::ConversionProgress::InProgress(_, _, progress) => progress,
                                                super::state::ConversionProgress::Error(_) => 100.0,
                                            })
                                            .bg(match self.state.conversion_progress {
                                                super::state::ConversionProgress::Completed(_) => cx.theme().success,
                                                super::state::ConversionProgress::Inactive => cx.theme().progress_bar,
                                                super::state::ConversionProgress::InProgress(_, _, _) => cx.theme().blue,
                                                super::state::ConversionProgress::Error(_) => cx.theme().red,
                                            })
                                    )
                                    .when(self.state.conversion_progress != super::state::ConversionProgress::Inactive, |this| {
                                        this.child(
                                            Label::new(match &self.state.conversion_progress {
                                                super::state::ConversionProgress::Inactive => unreachable!("Inactive conversion state has no Label!"),
                                                super::state::ConversionProgress::InProgress(curr, total, progress) => format!("{curr} von {total} Bild(ern) umgewandelt ({progress:.1} %)"),
                                                super::state::ConversionProgress::Error(_) => "Fehler beim Konvertieren".into(),
                                                super::state::ConversionProgress::Completed(total) => format!("Umwandlung abgeschlossen ({total} von {total}) konvertiert"),
                                            })
                                                .text_xs()
                                                .text_color(cx.theme().secondary_foreground)
                                        )
                                    })
                            )
                            .child(
                                Button::new("UnHEIC.UI.Footer.Button.Convert")
                                    .custom(
                                        ButtonCustomVariant::new(cx)
                                            .color(cx.theme().title_bar_border.opacity(0.5))
                                            .hover(cx.theme().title_bar_border.opacity(0.75))
                                            .active(cx.theme().title_bar_border.opacity(1.))
                                    )
                                    .label(match self.state.conversion_progress {
                                        super::state::ConversionProgress::InProgress(_, _, _) => "Umwandlung läuft",
                                        super::state::ConversionProgress::Completed(_) => "Umwandeln",
                                        super::state::ConversionProgress::Inactive => "Umwandeln",
                                        super::state::ConversionProgress::Error(_) => "Umwandeln",
                                    })
                                    .tooltip("Umwandlung starten")
                                    .disabled(match self.state.conversion_progress {
                                        super::state::ConversionProgress::InProgress(_, _, _) => true,
                                        _ => false,
                                    })
                                    .small()
                                    .compact()
                                    .on_click(cx.listener(super::actions::handle_conversion_start_button))
                            )
                    )
            )
    }
}
