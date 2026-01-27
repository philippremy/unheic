use std::{io::Cursor};
use image::{EncodableLayout, ImageBuffer, Rgba};
use tiff::encoder::colortype::RGBA8;

pub(super) fn convert_to_format(input: ImageBuffer<Rgba<u8>, Vec<u8>>, output_format: &super::state::ConversionSettings) -> Result<Box<dyn AsRef<[u8]> + Send + Sync>, String> {
    
    match output_format {
        crate::state::ConversionSettings::JPEG(_, _, _) => convert_to_jpeg(input, output_format),
        crate::state::ConversionSettings::PNG(_, _, _) => convert_to_png(input, output_format),
        crate::state::ConversionSettings::TIFF(_, _, _) => convert_to_tiff(input, output_format),
        crate::state::ConversionSettings::WebP(_, _, _) => convert_to_webp(input, output_format),
    }
    
}

fn convert_to_jpeg(input: ImageBuffer<Rgba<u8>, Vec<u8>>, settings: &super::state::ConversionSettings) -> Result<Box<dyn AsRef<[u8]> + Send + Sync>, String> {
    use turbojpeg::{Compressor, Image, PixelFormat};
    
    let quality = match settings {
        super::state::ConversionSettings::JPEG(_, quality, _) => quality,
        _ => unreachable!("Logic Error: Found different ConversionSettings")
    };
    
    let mut compressor = Compressor::new().map_err(|err| { err.to_string() })?;
    if *quality == 100 {
        compressor.set_lossless(true).map_err(|err| { err.to_string() })?;
    } else {
        compressor.set_lossless(false).map_err(|err| { err.to_string() })?;
        compressor.set_quality(*quality as i32).map_err(|err| { err.to_string() })?;
    }
    
    let img = Image { pixels: input.as_bytes(), width: input.width() as usize, pitch: input.width() as usize * PixelFormat::RGBA.size(), height: input.height() as usize, format: PixelFormat::RGBA };
    let out_buf = compressor.compress_to_vec(img).map_err(|err| { err.to_string() })?;
    Ok(Box::new(out_buf))
}

fn convert_to_png(input: ImageBuffer<Rgba<u8>, Vec<u8>>, settings: &super::state::ConversionSettings) -> Result<Box<dyn AsRef<[u8]> + Send + Sync>, String> {
    use png::{BitDepth, ColorType, Compression, Encoder};
    
    let compression = match settings {
        super::state::ConversionSettings::PNG(_, compression, _) => compression,
        _ => unreachable!("Logic Error: Found different ConversionSettings")
    };
    
    let mut out_vec = Vec::new();
    let mut encoder = Encoder::new(&mut out_vec, input.width(), input.height());
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);
    
    match *compression {
        0 => encoder.set_compression(Compression::NoCompression),
        1..=25 => encoder.set_compression(Compression::Fastest),
        26..=50 => encoder.set_compression(Compression::Fast),
        51..=75 => encoder.set_compression(Compression::Balanced),
        76.. => encoder.set_compression(Compression::High),
    }
    
    let mut writer = encoder.write_header().map_err(|err| { err.to_string() })?;
    writer.write_image_data(input.as_bytes()).map_err(|err| { err.to_string() })?;
    writer.finish().map_err(|err| { err.to_string() })?;
    
    Ok(Box::new(out_vec))
}

fn convert_to_tiff(input: ImageBuffer<Rgba<u8>, Vec<u8>>, settings: &super::state::ConversionSettings) -> Result<Box<dyn AsRef<[u8]> + Send + Sync>, String> {
    use tiff::encoder::{compression::DeflateLevel, Compression, TiffEncoder};
    
    let compression = match settings {
        super::state::ConversionSettings::TIFF(_, compression, _) => compression,
        _ => unreachable!("Logic Error: Found different ConversionSettings")
    };
    
    let mut out_vec = Vec::new();
    let mut cursor = Cursor::new(&mut out_vec);
    let mut encoder = TiffEncoder::new(&mut cursor).map_err(|err| { err.to_string() })?;
    
    encoder = match compression {
        crate::state::TIFFCompression::None => encoder.with_compression(Compression::Uncompressed),
        crate::state::TIFFCompression::LZW => encoder.with_compression(Compression::Lzw),
        crate::state::TIFFCompression::Deflate => encoder.with_compression(Compression::Deflate(DeflateLevel::Balanced)),
    };
    
    encoder.write_image::<RGBA8>(input.width(), input.height(), input.as_bytes()).map_err(|err| { err.to_string() })?;
    
    Ok(Box::new(out_vec))
}

fn convert_to_webp(input: ImageBuffer<Rgba<u8>, Vec<u8>>, settings: &super::state::ConversionSettings) -> Result<Box<dyn AsRef<[u8]> + Send + Sync>, String> {
    use webp::Encoder;
    
    let compression = match settings {
        super::state::ConversionSettings::WebP(_, compression, _) => compression,
        _ => unreachable!("Logic Error: Found different ConversionSettings")
    };
    
    let encoder = Encoder::from_rgba(input.as_bytes(), input.width(), input.height());
    
    let encoded_mem = match *compression {
        0 => encoder.encode_simple(false, *compression as f32).map_err(|err| { format!("{err:?}") })?,
        1..=25 => encoder.encode_simple(false, *compression as f32).map_err(|err| { format!("{err:?}") })?,
        26..=50 => encoder.encode_simple(false, *compression as f32).map_err(|err| { format!("{err:?}") })?,
        51..=75 => encoder.encode_simple(false, *compression as f32).map_err(|err| { format!("{err:?}") })?,
        76..100 => encoder.encode_simple(false, *compression as f32).map_err(|err| { format!("{err:?}") })?,
        100.. => encoder.encode_simple(true, 100.).map_err(|err| { format!("{err:?}") })?,
    };
    
    Ok(Box::new(encoded_mem.to_vec()))
}
