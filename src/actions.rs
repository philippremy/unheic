use gpui::{App, AppContext, DragMoveEvent, Entity, ExternalPaths, ListAlignment, ListState, px};
use gpui_component::{select::{SelectDelegate, SelectEvent, SelectItem, SelectState}, slider::{SliderEvent, SliderState}};
use image::ImageReader;
use mimetype_detector::{IMAGE_HEIC, match_file};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use smol::channel::{bounded, unbounded};
use std::{collections::HashMap, path::PathBuf};
use gpui::{ClickEvent, Context, Window};

pub(super) fn handle_open_folder_button(_: &mut super::ui::Application, _: &ClickEvent, window: &mut Window, cx: &mut Context<super::ui::Application>) {
    let (sender, receiver) = bounded::<Option<PathBuf>>(1);
    cx.foreground_executor().spawn(async move {
        sender.send(super::utils::open_single_directory().await).await.unwrap();
    }).detach();
    let window_handle = window.window_handle();
    cx.spawn(async move |weak, cx| {
        if let Ok(path_maybe) = receiver.recv().await {
            if let Some(path) = path_maybe {
                cx.update_window(window_handle, move |_, window, cx| {
                    weak
                        .upgrade()
                        .unwrap()
                        .update(cx, move |this, cx| {
                            this.state.output_folder_state.value = path;
                            this.state.output_folder_state.ui_entity.update(cx, |input, cx| {
                                input.set_value(this.state.output_folder_state.value.display().to_string(), window, cx);
                            });   
                        });
                }).unwrap();
            }
        }
    }).detach();
}

pub(super) fn handle_metadata_checkbox_change(this: &mut super::ui::Application, checked: &bool, _: &mut Window, _: &mut Context<super::ui::Application>) {
    match &mut this.state.conversion_settings.settings {
        crate::state::ConversionSettings::JPEG(metadata, _, _) => *metadata = *checked,
        crate::state::ConversionSettings::PNG(metadata, _, _) => *metadata = *checked,
        crate::state::ConversionSettings::TIFF(metadata, _, _) => *metadata = *checked,
        crate::state::ConversionSettings::WebP(metadata, _, _) => *metadata = *checked,
    }
}

pub(super) fn handle_select_event<D: SelectDelegate>(for_target: &Entity<SelectState<D>>, window: &mut Window, cx: &mut Context<super::ui::Application>, setter: impl Fn(&mut App, &mut Window, &mut super::ui::Application, &<<D as SelectDelegate>::Item as SelectItem>::Value) + 'static) {
    let window_handle = window.window_handle();
    cx.subscribe(for_target, move |app, _, ev: &SelectEvent<D>, cx| {
        cx.update_window(window_handle, |_, window, cx| {
            match ev {
                SelectEvent::Confirm(value_opt) => {
                    if let Some(value) = value_opt {
                        setter(cx, window, app, value);
                    }
                },
            }
        }).unwrap();
    }).detach();
}

pub(super) fn handle_slider_event(for_target: &Entity<SliderState>, window: &mut Window, cx: &mut Context<super::ui::Application>, setter: impl Fn(&mut App, &mut Window, &mut super::ui::Application, f32) + 'static) {
    let window_handle = window.window_handle();
    cx.subscribe(for_target, move |app, _, ev: &SliderEvent, cx| {
        cx.update_window(window_handle, |_, window, cx| {
            match ev {
                SliderEvent::Change(val) => setter(cx, window, app, val.end()),
            }
        }).unwrap();
    }).detach();
}

pub(super) fn handle_file_drop(this: &mut super::ui::Application, external_paths: &ExternalPaths, _: &mut Window, cx: &mut Context<super::ui::Application>) {
    if external_paths.paths().is_empty() { return; }
    let input_map = &mut this.state.input_image_state.images;
    for external_path in external_paths.paths() {
        if input_map.contains_key(external_path) { continue; }
        if !match_file(external_path, IMAGE_HEIC).unwrap_or(false) { continue; }
        input_map.insert(external_path.clone(), super::state::InputImage {
            state: super::state::ImageLoadingState::NotStarted,
            path: external_path.clone(),
            name: external_path.file_prefix().map(|filename| { filename.display().to_string() }).unwrap_or("Kein Dateiname".into()),
        });
        let path_async = external_path.clone();
        cx.spawn(async move |we, async_cx| {
            // Request generating the thumbnail image
            super::utils::request_thumbnail_generation(path_async, we, async_cx).await;
        }).detach();
    }
    this.state.input_image_state.total_count = input_map.len() as u16;
    this.state.input_image_state.ui_liststate = ListState::new(input_map.len(), ListAlignment::Top, px(16.));
    cx.stop_propagation();
    cx.notify();
}

pub(super) fn handle_file_drag(_: &mut super::ui::Application, event: &DragMoveEvent<ExternalPaths>, window: &mut Window, cx: &mut Context<super::ui::Application>) {
    if event.bounds.contains(&event.event.position) {
        cx.set_active_drag_cursor_style(gpui::CursorStyle::DragCopy, window);
    } else {
        cx.set_active_drag_cursor_style(gpui::CursorStyle::OperationNotAllowed, window);
    }
    cx.stop_propagation();
}

pub(super) fn handle_add_image_button(_: &mut super::ui::Application, _:&ClickEvent, window: &mut Window, cx: &mut Context<super::ui::Application>) {
    let (sender, receiver) = bounded::<Option<Vec<PathBuf>>>(1);
    cx.foreground_executor().spawn(async move {
        sender.send(super::utils::open_multiple_files().await).await.unwrap();
    }).detach();
    let window_handle = window.window_handle();
    cx.spawn(async move |weak, cx| {
        if let Ok(paths_maybe) = receiver.recv().await {
            if let Some(paths) = paths_maybe {
                cx.update_window(window_handle, move |_, _, cx| {
                    weak
                        .upgrade()
                        .unwrap()
                        .update(cx, move |this, cx| {
                            let input_map = &mut this.state.input_image_state.images;
                            for external_path in paths {
                                if input_map.contains_key(&external_path) { continue; }
                                if !match_file(&external_path, IMAGE_HEIC).unwrap_or(false) { continue; }
                                input_map.insert(external_path.clone(), super::state::InputImage {
                                    state: super::state::ImageLoadingState::NotStarted,
                                    path: external_path.clone(),
                                    name: external_path.file_prefix().map(|filename| { filename.display().to_string() }).unwrap_or("Kein Dateiname".into()),
                                });
                                let path_async = external_path.clone();
                                cx.spawn(async move |we, async_cx| {
                                    // Request generating the thumbnail image
                                    super::utils::request_thumbnail_generation(path_async, we, async_cx).await;
                                }).detach();
                            }
                            this.state.input_image_state.total_count = input_map.len() as u16;
                            this.state.input_image_state.ui_liststate = ListState::new(input_map.len(), ListAlignment::Top, px(16.));
                            cx.notify();
                        });
                }).unwrap();
            }
        }
    }).detach();
}

enum SingleConversionResult {
    Done,
    Error(PathBuf, String),
}

pub(super) fn handle_conversion_start_button<'a>(this: &mut super::ui::Application, _:&ClickEvent, _: &mut Window, cx: &mut Context<super::ui::Application>) {
    
    let total_images = this.state.input_image_state.total_count;
    this.state.conversion_progress = super::state::ConversionProgress::InProgress(0, total_images, 0.);
    let input_image_paths = this.state.input_image_state.images.keys().cloned::<PathBuf>().collect::<Vec<_>>();
    let output_dir = this.state.output_folder_state.value.clone();
    let conversion_settings = this.state.conversion_settings.settings.clone();
    
    // Convert each image in parallel
    cx.spawn(async move |weak, async_app| {
        
        let (sender, receiver) = unbounded::<SingleConversionResult>();
        
        async_app.background_spawn(async move {
            
            input_image_paths.into_par_iter().for_each_with((output_dir, conversion_settings), |(output_dir, conversion_settings), path| {
                
                let img = ImageReader::open(&path);
                if let Err(err) = img { sender.send_blocking(SingleConversionResult::Error(path, err.to_string())).unwrap(); return; }
                let img = img.unwrap().with_guessed_format();
                if let Err(err) = img { sender.send_blocking(SingleConversionResult::Error(path, err.to_string())).unwrap(); return; }
                let img = img.unwrap().decode();
                if let Err(err) = img { sender.send_blocking(SingleConversionResult::Error(path, err.to_string())).unwrap(); return; }
                let img = super::conversion::convert_to_format(img.unwrap().into_rgba8(), conversion_settings);
                if let Err(err) = img { sender.send_blocking(SingleConversionResult::Error(path, err.to_string())).unwrap(); return; }
                
                // Write to file
                let out_file_path = output_dir.join(path.file_prefix().unwrap()).with_added_extension(super::utils::file_extension_for_format(conversion_settings));
                let write_result = std::fs::write(&out_file_path, img.unwrap().as_ref());
                if let Err(err) = write_result { sender.send_blocking(SingleConversionResult::Error(path, err.to_string())).unwrap(); return; }
                
                sender.send_blocking(SingleConversionResult::Done).unwrap();
            });
            
        }).detach();
        
        let mut idx = 1u16;
        while idx != total_images + 1 {
            let recv = receiver.recv().await.unwrap();
            weak.update(async_app, |this, cx| {
                match recv {
                    SingleConversionResult::Done => {
                        match &mut this.state.conversion_progress {
                            super::state::ConversionProgress::InProgress(curr, total, percent) => {
                                *curr = idx;
                                *percent = (*curr as f32 / *total as f32) * 100.;
                            },
                            _ => {}
                        }
                    },
                    SingleConversionResult::Error(path_buf, err) => {
                        match &mut this.state.conversion_progress {
                            super::state::ConversionProgress::Error(err_map) => {
                                err_map.insert(path_buf, err);
                            },
                            _ => {
                                this.state.conversion_progress = super::state::ConversionProgress::Error(HashMap::new());
                                match &mut this.state.conversion_progress {
                                    super::state::ConversionProgress::Error(err_map) => {
                                        err_map.insert(path_buf, err);
                                    },
                                    _ => unreachable!("Logic Error: Got wrong ConversionProgress"),
                                }
                            }
                        }
                    },
                }
                cx.notify();
            }).unwrap();
            idx += 1;
        }
        
        weak.update(async_app, |this, cx| {
            
            match &mut this.state.conversion_progress {
                super::state::ConversionProgress::Inactive => unreachable!("Logic Error: Cannot reach here when the conversion is inactive."),
                super::state::ConversionProgress::InProgress(_, _, _) => {
                    this.state.conversion_progress = super::state::ConversionProgress::Completed(total_images);
                },
                super::state::ConversionProgress::Error(_) => {},
                super::state::ConversionProgress::Completed(_) => unreachable!("Logic Error: Completed cannot be set before"),
            }
            
            cx.notify();
            
        }).unwrap();
        
    }).detach();
    
}
