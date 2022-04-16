use actix_web::error::ErrorNotFound;
use actix_web::web::Bytes;
use serde::Deserialize;
use std::env;
use std::error::Error;

use crate::lib::pet::convert_bytes;

/// Struct to represent a user in Discord's API.
/// Only includes `avatar` because other fields don't matter.
#[derive(Debug, Deserialize)]
pub struct DiscordAPIUser {
    pub avatar: Option<String>,
}

impl DiscordAPIUser {
    /// Get a Discord user's avatar and convert to pet.
    /// # Arguments
    /// * `id` - the user's ID
    ///
    /// Currently only gets static avatars.
    /// TODO: get GIF avatars properly
    pub async fn get(id: &String) -> Result<Bytes, Box<dyn Error>> {
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

        Ok(convert_bytes(img)?)
    }
}

pub async fn emoji(id: &String) -> Result<Bytes, Box<dyn Error>> {
    let img = awc::Client::default()
        .get(format!("https://cdn.discordapp.com/emojis/{}.png?size=128", id))
        .send()
        .await?
        .body()
        .await?;

    Ok(convert_bytes(img)?)
}
