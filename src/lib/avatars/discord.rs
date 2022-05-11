use super::AvatarFetch;
use actix_web::web::Bytes;
use async_trait::async_trait;
use lazy_static::lazy_static;
use retainer::Cache;
use serde::Deserialize;
use std::env;
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

/// Struct to represent a user in Discord's API.
/// Only includes `avatar` because other fields don't matter.
#[derive(Debug, Deserialize)]
struct DiscordAPIUser {
    avatar: Option<String>,
    discriminator: String,
}

#[derive(Clone)]
pub struct Discord;

#[async_trait(?Send)]
impl AvatarFetch for Discord {
    fn cache_max_length() -> u64 {
        1800
    }

    async fn get_avatar(&self, id: &String) -> Result<Bytes, Box<dyn Error>> {
        let cache_entry = CACHE.get(id).await;
        if let Some(guard) = cache_entry {
            debug!("discord: Avatar for user {} in cache.", id);
            let bytes = guard.value().clone();
            return Ok(bytes);
        }

        debug!("discord: Avatar for user {} not in cache. fetching...", &id);
        let mut res = awc::Client::default()
            .get(format!("https://discord.com/api/v9/users/{}", &id))
            .insert_header((
                "Authorization",
                format!(
                    "Bot {}",
                    env::var("DISCORD_TOKEN")
                        .expect("DISCORD_TOKEN environment variable must be set")
                ),
            ))
            .send()
            .await?;

        let user = res.json::<DiscordAPIUser>().await?;

        let url = match user.avatar {
            Some(hash) => format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=128", &id, &hash),
            None => format!("https://cdn.discordapp.com/embed/avatars/{}.png?size=128", user.discriminator.parse::<u16>()? % 5),
        };

        let img = awc::Client::default()
            .get(url)
            .send()
            .await?
            .body()
            .await?;

        CACHE
            .insert(id.clone(), img.clone(), Duration::from_secs(Self::cache_max_length())) // 30 minutes
            .await;

        Ok(img)
    }
}
