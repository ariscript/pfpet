use actix_web::web::Bytes;
use async_trait::async_trait;
use std::error::Error;

pub mod discord;

#[async_trait(?Send)]
pub trait AvatarFetch {
    fn cache_max_length() -> u64;
    async fn get_avatar(&self, identifier: &String) -> Result<Bytes, Box<dyn Error>>;
}
