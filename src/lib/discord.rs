use actix_web::error::ErrorNotFound;
use actix_web::web::Bytes;
use lazy_static::lazy_static;
use retainer::Cache;
use serde::Deserialize;
use tracing::debug;
use std::env;
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

/// Struct to represent a user in Discord's API.
/// Only includes `avatar` because other fields don't matter.
#[derive(Debug, Deserialize)]
struct DiscordAPIUser {
    pub avatar: Option<String>,
}

/// Get a Discord user's avatar.
/// # Arguments
/// * `id` - the user's ID
///
/// Currently only gets static avatars.
/// TODO: get GIF avatars properly
pub async fn get_avatar(id: &String) -> Result<Bytes, Box<dyn Error>> {
    let cache_entry = CACHE.get(id).await;
    if let Some(guard) = cache_entry {
        debug!("Avatar for user {} in cache.", id);
        let bytes = guard.value().clone();
        return Ok(bytes);
    }

    debug!("Avatar for user {} not in cache. fetching...", id);
    let avatar = awc::Client::default()
        .get(format!("https://discord.com/api/v9/users/{}", id))
        .insert_header((
            "Authorization",
            format!("Bot {}", env::var("DISCORD_TOKEN").unwrap()),
        ))
        .send()
        .await?
        .json::<DiscordAPIUser>()
        .await?
        .avatar;

    if let None = avatar {
        return Err(Box::new(ErrorNotFound("User has no avatar.")));
    }

    let avatar = avatar.unwrap();

    let img = awc::Client::default()
        .get(format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png?size=128",
            id, avatar
        ))
        .send()
        .await?
        .body()
        .await?;

    CACHE
        .insert(id.clone(), img.clone(), Duration::from_secs(1800)) // 30 minutes
        .await;

    Ok(img)
}
