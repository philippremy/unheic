use std::{ffi::OsStr, path::PathBuf, sync::{Arc, LazyLock}};

use directories::UserDirs;
use gpui::{AsyncApp, IntoElement, ParentElement, RenderImage, WeakEntity, prelude::FluentBuilder};
use image::{Frame, Rgba, RgbaImage};
use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};
use rfd::AsyncFileDialog;

pub(super) trait PlatformConditional
where
    Self: ParentElement + IntoElement + Sized
{
    fn modify_macos(self, f: impl Fn(Self) -> Self) -> Self {
        self.when(cfg!(target_os = "macos"), |this| {
            f(this)
        })
    }
}

impl<T> PlatformConditional for T
where
    Self: ParentElement + IntoElement + Sized
{
}

pub(super) fn user_picture_dir() -> PathBuf {
    UserDirs::new()
        .unwrap()
        .picture_dir()
        .unwrap()
        .to_path_buf()
}

pub(super) async fn open_single_directory() -> Option<PathBuf> {
    AsyncFileDialog::new()
        .set_title("Ausgabeordner auswählen")
        .pick_folder()
        .await
        .map(|fh| {
            fh.path().to_path_buf()
        })
}

pub(super) async fn open_multiple_files() -> Option<Vec<PathBuf>> {
    AsyncFileDialog::new()
        .add_filter("High-Efficiency Image Codec", &["heic", "HEIC"])
        .set_title("HEIC Dateien hinzufügen")
        .pick_files()
        .await
        .map(|fhs| {
            fhs.iter().map(|fh| { fh.path().to_path_buf() }).collect()
        })
}

static LIBHEIF: LazyLock<LibHeif> = LazyLock::new(|| { LibHeif::new() });

pub(super) async fn request_thumbnail_generation(for_path: PathBuf, we: WeakEntity<super::ui::Application>, cx: &mut AsyncApp) {
    if let Some(entity) = we.upgrade() {     
        entity.update(cx, |this, _| {
            this.state.input_image_state.images.get_mut(&for_path).unwrap().state = super::state::ImageLoadingState::InProgress;
        }).unwrap();
        
        let ctx = HeifContext::read_from_file(for_path.to_str().unwrap()).unwrap();
        let handle = ctx.primary_image_handle().unwrap();
        let image = LIBHEIF.decode(
            &handle,
            ColorSpace::Rgb(RgbChroma::Rgba),
            None,
        ).unwrap();
        
        // We assume a fixed height of 500px
        let ratio: f32 = 500. / image.height() as f32;
        let thumbnail = image.scale((image.width() as f32 * ratio) as u32, (image.height() as f32 * ratio) as u32, None).unwrap();
        let plane = thumbnail.planes().interleaved.unwrap();
        let rgba_image = RgbaImage::from_par_fn(thumbnail.width(), thumbnail.height(), |x, y| {
            let x = x as usize;
            let y = y as usize;
    
            let row_start = y * plane.stride;
            let pixel_start = row_start + x * 4;
    
            Rgba([
                plane.data[pixel_start],
                plane.data[pixel_start + 1],
                plane.data[pixel_start + 2],
                plane.data[pixel_start + 3],
            ])
        });
        let render_image = Arc::new(RenderImage::new([Frame::new(rgba_image)]));
        
        entity.update(cx, move |this, _| {
            let entry = this.state.input_image_state.images.get_mut(&for_path).unwrap();
            entry.state = super::state::ImageLoadingState::Done(Arc::new(move |_, _| { Some(Ok(render_image.clone())) }));
        }).unwrap();
    }
}

pub(super) fn file_extension_for_format(format: &super::state::ConversionSettings) -> &'static OsStr {
    match format {
        crate::state::ConversionSettings::JPEG(_, _, _) => OsStr::new("jpg"),
        crate::state::ConversionSettings::PNG(_, _, _) => OsStr::new("png"),
        crate::state::ConversionSettings::TIFF(_, _, _) => OsStr::new("tiff"),
        crate::state::ConversionSettings::WebP(_, _, _) => OsStr::new("webp"),
    }
}
