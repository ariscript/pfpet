use actix_web::web::Bytes;
use std::error::Error;

pub mod pet;

pub trait ImageFilter {
    fn convert_bytes(&self, png: Bytes) -> Result<Bytes, Box<dyn Error>>;
}
