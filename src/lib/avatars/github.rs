use super::AvatarFetch;
use actix_web::error::ErrorNotFound;
use actix_web::web::Bytes;
use async_trait::async_trait;
use lazy_static::lazy_static;
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
pub struct Github;

#[async_trait(?Send)]
impl AvatarFetch for Github {
    fn cache_max_length() -> u64 {
        1800
    }

    async fn get_avatar(&self, username: &String) -> Result<Bytes, Box<dyn Error>> {
        let cache_entry = CACHE.get(username).await;
        if let Some(guard) = cache_entry {
            debug!("github: Avatar for user {} in cache.", username);
            let bytes = guard.value().clone();
            return Ok(bytes);
        }

        debug!("github: Avatar for user {} not in cache. fetching...", &username);
        let mut res = awc::Client::default()
            .get(format!(
                "https://github.com/{}.png?size=128",
                &username
            ))
            .send()
            .await?;

        let img = res.body().await?;

        if img.is_empty() {
            return Err(Box::new(ErrorNotFound("User doesn't exist")))
        }

        CACHE
            .insert(username.clone(), img.clone(), Duration::from_secs(1800)) // 30 minutes
            .await;

        Ok(img)
    }
}

