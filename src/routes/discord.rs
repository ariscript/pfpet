use actix_web::web::Path;
use actix_web::{get, HttpResponse, HttpResponseBuilder, Responder};
use awc::http::StatusCode;

use crate::lib::discord::{DiscordAPIUser, emoji};

#[get("/users/{id}")]
pub async fn discord_user(params: Path<String>) -> impl Responder {
    let id = params.into_inner();
    let avatar = DiscordAPIUser::get(&id).await;

    if let Err(_) = avatar {
        // cba figuring out why i can't use ?
        return HttpResponse::new(StatusCode::NOT_FOUND);
    }

    let avatar = avatar.unwrap();

    HttpResponseBuilder::new(StatusCode::OK)
        .content_type("image/gif")
        .body(avatar)
}

#[get("/emojis/{id}")]
pub async fn discord_emoji(params: Path<String>) -> impl Responder {
    let id: params.into_inner();
    let emoji = emoji(&id).await;

    if let Err(_) = emoji {
        return HttpResponse::new(StatusCode::NOT_FOUND);
    }

    let emoji = emoji.unwrap();

    HttpResponseBuilder::new(StatusCode::OK)
        .content_type("image/gif")
        .body(emoji)
}
