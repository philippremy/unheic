use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use gpui::{IntoElement, RenderImage};
use gpui_component::IndexPath;
use indexmap::IndexSet;
use strum::{EnumIs, EnumTryAs};

use crate::{ConversionError, ThumbnailError};

#[derive(EnumIs, EnumTryAs)]
pub(crate) enum ConversionState {
    Idle,
    Error(HashMap<PathBuf, ConversionError>),
    InProgress(f32),
    Done,
}

#[derive(Clone, EnumIs, EnumTryAs)]
pub(crate) enum ThumbnailState {
    Requested,
    Done(Arc<RenderImage>),
    Error(Arc<ThumbnailError>),
}

#[derive(Clone, IntoElement)]
pub(crate) struct InputListEntry {
    pub(crate) path: Arc<PathBuf>,
    pub(crate) filename: Arc<String>,
    pub(crate) thumbnail: ThumbnailState,
    pub(crate) is_selected: bool,
}

pub(crate) struct InputListData {
    pub(crate) selected_index: Option<IndexPath>,
    pub(crate) items: IndexSet<Arc<PathBuf>>,
    pub(crate) list_data: HashMap<Arc<PathBuf>, Arc<RwLock<InputListEntry>>>,
}
