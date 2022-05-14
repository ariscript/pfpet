use super::AvatarFetch;
use actix_web::error::ErrorNotFound;
use actix_web::web::Bytes;
use async_trait::async_trait;
use lazy_static::lazy_static;
use md5;
use retainer::Cache;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tracing::debug;

lazy_static! {
    static ref CACHE: Arc<Cache<String, Bytes>> = {
        let cache = Arc::new(Cache::new());
        let clone = cache.clone();

        tokio::spawn(async move { clone.monitor(4, 0.25, Duration::from_secs(3)).await });

        cache
    };
}

#[derive(Clone)]
pub struct Gravatar;

#[async_trait(?Send)]
impl AvatarFetch for Gravatar {
    fn cache_max_length() -> u64 {
        1800
    }

    async fn get_avatar(&self, email: &String) -> Result<Bytes, Box<dyn Error>> {
        let cache_entry = CACHE.get(email).await;
        if let Some(guard) = cache_entry {
            debug!("gravatar: Avatar for user {} in cache.", email);
            let bytes = guard.value().clone();
            return Ok(bytes);
        }

        debug!(
            "gravatar: Avatar for user {} not in cache. fetching...",
            &email
        );

        let digest = md5::compute(email.trim().to_lowercase().as_bytes());

        let mut res = awc::Client::default()
            .get(format!(
                "https://www.gravatar.com/avatar/{:x}.jpg?size=128",
                &digest
            ))
            .send()
            .await?;

        let img = res.body().await?;

        if img.is_empty() {
            return Err(Box::new(ErrorNotFound("User doesn't exist")));
        }

        CACHE
            .insert(
                format!("{:x}", digest.clone()),
                img.clone(),
                Duration::from_secs(Self::cache_max_length()),
            ) // 30 minutes
            .await;

        Ok(img)
    }
}
