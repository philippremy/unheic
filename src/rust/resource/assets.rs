use std::borrow::Cow;

use anyhow::anyhow;
use gpui::{App, AssetSource, IntoElement, RenderOnce, SharedString, Window};
use gpui_component::{Icon, IconNamed, icon_named};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/res"]
#[include = "*.svg"]
pub(crate) struct UnHEICAssets;

impl AssetSource for UnHEICAssets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        if let Some(data) = Self::get(path).map(|file| file.data) {
            return Ok(Some(data));
        }

        Err(anyhow!("Path does not exist: {}", path))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

icon_named!(UnHEICIcons, "res/icons");

impl RenderOnce for UnHEICIcons {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        Icon::new(self)
    }
}

pub(crate) struct MultiAssetsSource {
    providers: Vec<Box<dyn AssetSource>>,
}

impl MultiAssetsSource {
    pub(crate) fn new() -> Self {
        Self { providers: vec![] }
    }
    pub(crate) fn append_provider(mut self, provider: impl AssetSource) -> Self {
        self.providers.push(Box::new(provider));
        self
    }
}

impl AssetSource for MultiAssetsSource {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        for provider in self.providers.iter() {
            if let Ok(maybe) = provider.load(path) {
                if let Some(data) = maybe {
                    return Ok(Some(data));
                }
            }
        }
        Err(anyhow!("Path does not exist: {}", path))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(self
            .providers
            .iter()
            .map(|provider| provider.list(path).ok())
            .collect::<Vec<Option<Vec<SharedString>>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>())
    }
}
