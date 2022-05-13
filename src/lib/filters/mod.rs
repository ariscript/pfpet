use actix_web::web::Bytes;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Frame, ImageError};
use std::error::Error;

pub mod bonk;
pub mod pet;

pub trait ImageFilter: Clone {
    fn convert_bytes(&self, png: Bytes) -> Result<Bytes, Box<dyn Error>>;
}

pub fn encode_gif(
    frames: impl IntoIterator<Item = Frame>,
    speed: i32,
) -> Result<Vec<u8>, ImageError> {
    let mut buf: Vec<u8> = vec![];

    {
        let mut encoder = GifEncoder::new_with_speed(&mut buf, speed);
        encoder.set_repeat(Repeat::Infinite)?;
        encoder.encode_frames(frames)?;
    }

    Ok(buf)
}
