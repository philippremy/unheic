use std::{collections::HashMap, path::PathBuf, sync::{Arc, RwLock}};

use gpui::{App, AppContext, Entity, ImageCacheError, ListAlignment, ListState, RenderImage, Window, px};
use gpui_component::{IndexPath, input::InputState, select::SelectState, slider::SliderState};
use ordermap::OrderMap;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

#[derive(Default, PartialEq)]
pub(crate) enum ConversionProgress {
    #[default]
    Inactive,
    InProgress(u16, u16, f32),
    Error(HashMap<PathBuf, String>),
    Completed(u16),
}

#[derive(Clone, Copy, Default, EnumIter, EnumMessage)]
pub(crate) enum TIFFCompression {
    #[strum(message = "Keine Kompression")]
    #[default]
    None,
    #[strum(message = "Lempel-Ziv-Welch (LZW)")]
    LZW,
    #[strum(message = "Deflate (LZ77 / Huffman)")]
    Deflate,
}

#[derive(Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter, EnumMessage))]
pub(crate) enum ConversionSettings {
    #[strum_discriminants(strum(message = "JPEG (.jpg/.jpeg)"))]
    JPEG(bool, u8, Entity<SliderState>),
    #[strum_discriminants(strum(message = "PNG (.png)"))]
    PNG(bool, u8, Entity<SliderState>),
    #[strum_discriminants(strum(message = "TIFF (.tif/.tiff)"))]
    TIFF(bool, TIFFCompression, Entity<SelectState<Vec<String>>>),
    #[strum_discriminants(strum(message = "WebP (.webp)"))]
    WebP(bool, u8, Entity<SliderState>),
}

impl ConversionSettings {
    pub(super) fn new(cx: &mut App, window: &mut Window, variant: ConversionSettingsDiscriminants) -> Self {
        match variant {
            ConversionSettingsDiscriminants::JPEG => Self::JPEG(true, 90, cx.new(|_| { SliderState::new().max(100.).min(0.).step(1.).default_value(90.) })),
            ConversionSettingsDiscriminants::PNG => Self::PNG(true, 75, cx.new(|_| { SliderState::new().max(100.).min(0.).step(1.).default_value(75.) })),
            ConversionSettingsDiscriminants::TIFF => Self::TIFF(true, TIFFCompression::None, cx.new(|cx| { SelectState::new(TIFFCompression::iter().map(|variant| { variant.get_message().unwrap().to_string() }).collect::<Vec<String>>(), Some(IndexPath::default()), window, cx) })),
            ConversionSettingsDiscriminants::WebP => Self::WebP(true, 80, cx.new(|_| { SliderState::new().max(100.).min(0.).step(1.).default_value(80.) })),
        }
    }
}

pub(super) struct OutputFolderState {
    pub(super) ui_entity: Entity<InputState>,
    pub(super) value: PathBuf,
}

impl OutputFolderState {
    fn new(cx: &mut App, window: &mut Window) -> Self {
        // Get default output folder
        let picture_dir = Arc::new(RwLock::new(super::utils::user_picture_dir()));
        Self {
            // Output Folder Input
            ui_entity: cx.new(|cx| {
                InputState::new(window, cx)
                    .default_value(picture_dir.read().map(|path| { path.display().to_string() }).unwrap_or("Failed to lock RwLock for Reading".into()))
                    .placeholder("Ausgabeordner")
            }),
            value: super::utils::user_picture_dir(),
        }
    }
}

pub(super) struct ConversionSettingsState {
    pub(super) format_dropdown_entity: Entity<SelectState<Vec<String>>>,
    pub(super) settings: ConversionSettings,
    pub(super) variant: ConversionSettingsDiscriminants,
}

impl ConversionSettingsState {
    pub(super) fn new(cx: &mut App, window: &mut Window) -> Self {
        Self { 
            format_dropdown_entity: cx.new(|cx| {
                SelectState::new(
                    ConversionSettingsDiscriminants::iter().map(|variant| { format!("{}", variant.get_message().unwrap()) }).collect::<Vec<_>>(), 
                    Some(IndexPath::default()), 
                    window, 
                    cx
                )
            }), 
            settings: ConversionSettings::new(cx, window, ConversionSettingsDiscriminants::JPEG),
            variant: ConversionSettingsDiscriminants::JPEG
        }
    }
}

#[derive(Default)]
pub(super) enum ImageLoadingState {
    #[default]
    NotStarted,
    InProgress,
    Done(Arc<dyn Fn(&mut Window, &mut App) -> Option<Result<Arc<RenderImage>, ImageCacheError>>>),
    Failure(String),
}

pub(super) struct InputImage {
    pub(super) state: ImageLoadingState,
    pub(super) path: PathBuf,
    pub(super) name: String,
}

pub(super) struct InputImageState {
    pub(super) total_count: u16,
    pub(super) images: OrderMap<PathBuf, InputImage>,
    pub(super) ui_liststate: ListState,
}

impl Default for InputImageState {
    fn default() -> Self {
        Self {
            total_count: Default::default(),
            images: Default::default(),
            ui_liststate: ListState::new(0, ListAlignment::Top, px(16.))
        }
    }
}

pub(super) struct ApplicationState {
    pub(super) conversion_progress: ConversionProgress,
    pub(super) conversion_settings: ConversionSettingsState,
    pub(super) output_folder_state: OutputFolderState,
    pub(super) input_image_state: InputImageState,
}

impl ApplicationState {
    pub(super) fn new(cx: &mut App, window: &mut Window) -> Self {
        ApplicationState {
            conversion_progress: Default::default(),
            input_image_state: Default::default(),
            conversion_settings: ConversionSettingsState::new(cx, window),
            output_folder_state: OutputFolderState::new(cx, window),
        }
    }
}
