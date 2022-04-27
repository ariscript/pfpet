/// Code adapted from https://github.com/poly000/petpet-rs, licensed under the MIT License.
use actix_web::web::Bytes;
use image::codecs::gif::{GifEncoder, Repeat};
use image::error::ImageResult;
use image::imageops::{overlay, resize, FilterType};
use image::{load_from_memory_with_format, Delay, Frame, ImageError, ImageFormat, Rgba, RgbaImage};
use lazy_static::lazy_static;
use std::error::Error;

const FRAMES: u32 = 8;
const RESOLUTION: (u32, u32) = (128, 115);

mod hand_raw {
    pub static BONK_0: &[u8; 15117] = include_bytes!("../res/bonk-0.png");
    pub static BONK_1: &[u8; 19351] = include_bytes!("../res/bonk-1.png");
    pub static BONK_2: &[u8; 18317] = include_bytes!("../res/bonk-2.png");
    pub static BONK_3: &[u8; 17831] = include_bytes!("../res/bonk-3.png");
}

lazy_static! {
    static ref BONKS: Vec<RgbaImage> = vec![
        load_png(hand_raw::BONK_0).unwrap(),
        load_png(hand_raw::BONK_1).unwrap(),
        load_png(hand_raw::BONK_2).unwrap(),
        load_png(hand_raw::BONK_3).unwrap(),
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

        let offset_x = (((1.0 - width_scale) * 0.3 - 0.05) * RESOLUTION.0 as f64) as i64;
        let offset_y = (((1.0 - height_scale) - 0.08) * RESOLUTION.1 as f64) as i64;

        let calculate_then_resize = resize(&image, width, height, filter);

        let mut resize_then_overlay = RgbaImage::new(RESOLUTION.0, RESOLUTION.1);

        overlay(
            &mut resize_then_overlay,
            &calculate_then_resize,
            offset_x,
            offset_y,
        );

        for (pixel_bonk, pixel_canvas) in BONKS[i as usize / 4]
            .pixels()
            .zip(resize_then_overlay.pixels_mut())
        {
            if !matches!(pixel_bonk, Rgba([_, _, _, 0])) {
                *pixel_canvas = *pixel_bonk;
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

pub fn convert_bytes(png: Bytes) -> Result<Bytes, Box<dyn Error>> {
    let loaded = load_png(&png[..])?;
    let bonked = encode_gif(generate(loaded, FilterType::Lanczos3)?, 5)?;

    Ok(Bytes::from(bonked))
}
