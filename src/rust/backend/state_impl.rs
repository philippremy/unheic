use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use gpui::{App, AppContext, EntityId};
use indexmap::IndexSet;

use crate::{
    ThumbnailError, UnHEICError,
    backend::state::{InputListData, InputListEntry, ThumbnailState},
};

impl InputListEntry {
    fn new(path: Arc<PathBuf>) -> Self {
        Self {
            path: path.clone(),
            filename: path
                .file_name()
                .map(|name| Arc::new(name.to_string_lossy().to_string()))
                .unwrap_or(Arc::new("Kein Dateiname".into())),
            thumbnail: ThumbnailState::Requested,
            is_selected: false,
        }
    }

    fn generate_thumbnail(&mut self) -> Result<ThumbnailState, ThumbnailError> {
        Ok(ThumbnailState::Requested)
    }
}

impl InputListData {
    pub(crate) fn new() -> Self {
        Self {
            items: IndexSet::new(),
            list_data: HashMap::new(),
            selected_index: None,
        }
    }

    // Must notify after return!
    pub(crate) fn add_new_entry(&mut self, entry: Arc<PathBuf>, cx: &mut App, entity_id: EntityId) {
        if self.items.contains(&entry) {
            return;
        }
        self.items.insert(Arc::clone(&entry));
        self.list_data.insert(
            Arc::clone(&entry),
            Arc::new(RwLock::new(InputListEntry::new(Arc::clone(&entry)))),
        );
        let list_data = Arc::clone(self.list_data.get_mut(&entry).unwrap());
        cx.spawn(async move |cx| -> crate::Result<()> {
            let list_data_bg = Arc::clone(&list_data);
            let thumbnail_state: Result<ThumbnailState, ThumbnailError> = cx
                .background_spawn(async move {
                    list_data_bg
                        .write()
                        .map_err(|err| ThumbnailError::RwLockPoisoned(err.to_string()))?
                        .generate_thumbnail()
                })
                .await;
            match thumbnail_state {
                Ok(state) => {
                    list_data
                        .write()
                        .map_err(|err| {
                            UnHEICError::ThumbnailGenerationError(ThumbnailError::RwLockPoisoned(
                                err.to_string(),
                            ))
                        })?
                        .thumbnail = state;
                }
                Err(err) => {
                    list_data
                        .write()
                        .map_err(|err| {
                            UnHEICError::ThumbnailGenerationError(ThumbnailError::RwLockPoisoned(
                                err.to_string(),
                            ))
                        })?
                        .thumbnail = ThumbnailState::Error(Arc::new(err));
                }
            }
            cx.update(|cx| cx.notify(entity_id));
            Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub(crate) fn remove_entry(&mut self, entry: Arc<PathBuf>) {
        self.items.shift_remove(&entry);
        self.list_data.remove(&entry);
    }
}
