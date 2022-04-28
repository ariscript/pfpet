use actix_web::error::ErrorNotFound;
use actix_web::web::Bytes;
use lazy_static::lazy_static;
use retainer::Cache;
use tracing::debug;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

lazy_static! {
    static ref CACHE: Arc<Cache<String, Bytes>> = {
        let cache = Arc::new(Cache::new());
        let clone = cache.clone();

        tokio::spawn(async move { clone.monitor(4, 0.25, Duration::from_secs(3)).await });

        cache
    };
}

/// Get a Github user's / organization's avatar.
/// # Arguments
/// * `username` - the user's username
///
pub async fn get_avatar(username: &String) -> Result<Bytes, Box<dyn Error>> {
    let cache_entry = CACHE.get(username).await;
    if let Some(guard) = cache_entry {
        debug!("Avatar for user {} in cache.", username);
        let bytes = guard.value().clone();
        return Ok(bytes);
    }

    debug!("Avatar for user {} not in cache. fetching...", username);

    let avatar = awc::Client::default()
        .get(format!(
            "https://github.com/{}.png?size=128",
            username
        ))
        .send()
        .await?
        .body()
        .await?;

    if avatar.is_empty() {
        return Err(Box::new(ErrorNotFound("User does not exist")));
    }

    CACHE
        .insert(username.clone(), avatar.clone(), Duration::from_secs(1800)) // 30 minutes
        .await;

    Ok(avatar)
}
