use super::AvatarFetch;
use actix_web::error::ErrorNotFound;
use actix_web::web::Bytes;
use async_trait::async_trait;
use lazy_static::lazy_static;
use retainer::Cache;
use serde::Deserialize;
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

/// Struct to represent a user in Reddit's API.
/// Only includes `icon_img` because other fields don't matter.
#[derive(Debug, Deserialize)]
struct RedditAPIUser {
    data: RedditAPIUserData,
}

#[derive(Debug, Deserialize)]
struct RedditAPIUserData {
    icon_img: Option<String>,
}

#[derive(Clone)]
pub struct Reddit;

#[async_trait(?Send)]
impl AvatarFetch for Reddit {
    fn cache_max_length() -> u64 {
        1800
    }

    async fn get_avatar(&self, username: &String) -> Result<Bytes, Box<dyn Error>> {
        let cache_entry = CACHE.get(username).await;
        if let Some(guard) = cache_entry {
            debug!("reddit: Avatar for user {} in cache.", username);
            let bytes = guard.value().clone();
            return Ok(bytes);
        }

        debug!(
            "reddit: Avatar for user {} not in cache. fetching...",
            &username
        );
        let mut res = awc::Client::default()
            .get(format!(
                "https://www.reddit.com/user/{}/about.json",
                &username
            ))
            .send()
            .await?;

        let user: RedditAPIUser = res.json().await?;
        debug!("{:?}", user);

        if user.data.icon_img.is_none() {
            return Err(Box::new(ErrorNotFound(
                "icon_img not present for this user",
            )));
        }

        let url = user.data.icon_img.unwrap();
        let url = url.split('?').collect::<Vec<_>>()[0];
        debug!("reddit: {user} url: {url}", user = username, url = url);

        let img = awc::Client::default().get(url).send().await?.body().await?;

        CACHE
            .insert(
                username.clone(),
                img.clone(),
                Duration::from_secs(Self::cache_max_length()),
            ) // 30 minutes
            .await;

        Ok(img)
    }
}
