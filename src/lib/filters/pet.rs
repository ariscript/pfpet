use super::ImageFilter;
/// Code adapted from https://github.com/poly000/petpet-rs, licensed under the MIT License.
use actix_web::web::Bytes;
use image::codecs::gif::{GifEncoder, Repeat};
use image::error::ImageResult;
use image::imageops::{overlay, resize, FilterType};
use image::{load_from_memory_with_format, Delay, Frame, ImageError, ImageFormat, Rgba, RgbaImage};
use lazy_static::lazy_static;
use std::error::Error;

const FRAMES: u32 = 10;
const RESOLUTION: (u32, u32) = (112, 112);

mod hand_raw {
    pub static HAND_0: &[u8; 15758] = include_bytes!("../../res/0.png");
    pub static HAND_1: &[u8; 16013] = include_bytes!("../../res/1.png");
    pub static HAND_2: &[u8; 16284] = include_bytes!("../../res/2.png");
    pub static HAND_3: &[u8; 16199] = include_bytes!("../../res/3.png");
    pub static HAND_4: &[u8; 14816] = include_bytes!("../../res/4.png");
}

lazy_static! {
    static ref HANDS: Vec<RgbaImage> = vec![
        load_png(hand_raw::HAND_0).unwrap(),
        load_png(hand_raw::HAND_1).unwrap(),
        load_png(hand_raw::HAND_2).unwrap(),
        load_png(hand_raw::HAND_3).unwrap(),
        load_png(hand_raw::HAND_4).unwrap(),
    ];
}

fn load_png(buf: &[u8]) -> Result<RgbaImage, ImageError> {
    let dyn_image = load_from_memory_with_format(buf, ImageFormat::Png)?;
    Ok(dyn_image.to_rgba8())
}

fn generate(image: RgbaImage, filter: FilterType) -> ImageResult<impl IntoIterator<Item = Frame>> {
    let mut frames = Vec::<Frame>::new();

    for i in 0..FRAMES {
        let squeeze = if i < FRAMES / 2 { i } else { FRAMES - i } as f64;

        let width_scale = 0.8 + squeeze * 0.02;
        let height_scale = 0.8 - squeeze * 0.05;

        let width = (width_scale * RESOLUTION.0 as f64) as u32;
        let height = (height_scale * RESOLUTION.1 as f64) as u32;

        let offset_x = (((1.0 - width_scale) * 0.5 + 0.1) * RESOLUTION.0 as f64) as i64;
        let offset_y = (((1.0 - height_scale) - 0.08) * RESOLUTION.1 as f64) as i64;

        let calculate_then_resize = resize(&image, width, height, filter);

        let mut resize_then_overlay = RgbaImage::new(RESOLUTION.0, RESOLUTION.1);

        overlay(
            &mut resize_then_overlay,
            &calculate_then_resize,
            offset_x,
            offset_y,
        );

        for (pixel_hand, pixel_canvas) in HANDS[i as usize / 2]
            .pixels()
            .zip(resize_then_overlay.pixels_mut())
        {
            if !matches!(pixel_hand, Rgba([_, _, _, 0])) {
                *pixel_canvas = *pixel_hand;
            }
        }

        const DELAY: u32 = 20;
        let overlay_then_delay = Frame::from_parts(
            resize_then_overlay,
            0,
            0,
            Delay::from_numer_denom_ms(DELAY, 1),
        );

        frames.push(overlay_then_delay);
    }
    Ok(frames)
}

fn encode_gif(frames: impl IntoIterator<Item = Frame>, speed: i32) -> Result<Vec<u8>, ImageError> {
    let mut buf: Vec<u8> = vec![];

    {
        let mut encoder = GifEncoder::new_with_speed(&mut buf, speed);
        encoder.set_repeat(Repeat::Infinite)?;
        encoder.encode_frames(frames)?;
    }

    Ok(buf)
}

#[derive(Clone)]
pub struct Pet;

impl ImageFilter for Pet {
    fn convert_bytes(&self, png: Bytes) -> Result<Bytes, Box<dyn Error>> {
        let loaded = load_png(&png[..])?;
        let petted = encode_gif(generate(loaded, FilterType::Lanczos3)?, 5)?;

        Ok(Bytes::from(petted))
    }
}
