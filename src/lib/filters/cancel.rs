use super::{encode_gif, ImageFilter};
/// Code adapted from https://github.com/poly000/petpet-rs, licensed under the MIT License.
use actix_web::web::Bytes;
use image::error::ImageResult;
use image::imageops::{overlay, resize, FilterType};
use image::{load_from_memory_with_format, Delay, Frame, ImageError, ImageFormat, Rgba, RgbaImage};
use lazy_static::lazy_static;
use std::error::Error;

const RESOLUTION: (u32, u32) = (113, 113);

mod no_raw {
    pub static CANCEL_0: &[u8; 4145] = include_bytes!("../../res/cancel/0.png");
}

lazy_static! {
    static ref CANCEL: RgbaImage = load_png(no_raw::CANCEL_0).unwrap();
}

fn load_png(buf: &[u8]) -> Result<RgbaImage, ImageError> {
    let dyn_image = load_from_memory_with_format(buf, ImageFormat::Png)?;
    Ok(dyn_image.to_rgba8())
}

fn generate(image: RgbaImage, filter: FilterType) -> ImageResult<impl IntoIterator<Item = Frame>> {
    let mut frames = Vec::<Frame>::new();

    let width = (RESOLUTION.0 as f64) as u32;
    let height = (RESOLUTION.1 as f64) as u32;

    let calculate_then_resize = resize(&image, width, height, filter);
    let mut overlaything = RgbaImage::new(RESOLUTION.0, RESOLUTION.1);

    overlay(&mut overlaything, &calculate_then_resize, 0, 0);
    for (pixel_bonk, pixel_canvas) in CANCEL.pixels().zip(overlaything.pixels_mut()) {
        if !matches!(pixel_bonk, Rgba([_, _, _, 0])) {
            *pixel_canvas = *pixel_bonk;
        }
    }
    let overlay_then_delay =
        Frame::from_parts(overlaything, 0, 0, Delay::from_numer_denom_ms(1, 1));
    frames.push(overlay_then_delay);
    Ok(frames)
}

#[derive(Clone)]
pub struct Cancel;

impl ImageFilter for Cancel {
    fn convert_bytes(&self, png: Bytes) -> Result<Bytes, Box<dyn Error>> {
        let loaded = load_png(&png[..])?;
        let canceled = encode_gif(generate(loaded, FilterType::Lanczos3)?, 5)?;

        Ok(Bytes::from(canceled))
    }
}
