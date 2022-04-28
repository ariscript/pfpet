use actix_web::web::Path;
use actix_web::{get, HttpResponse, HttpResponseBuilder, Responder};
use awc::http::StatusCode;

use crate::lib::github::get_avatar;
use crate::lib::pet::convert_bytes;

/// Max number of seconds that responses should be cached
pub static MAX_AGE: u64 = 1800;

#[get("/{username}.gif")]
pub async fn github_user(params: Path<String>) -> impl Responder {
    let username = params.into_inner();
    let avatar = get_avatar(&username).await;

    if let Err(_) = avatar {
        // cba figuring out why i can't use ?
        return HttpResponse::new(StatusCode::NOT_FOUND);
    }

    let avatar = avatar.unwrap();
    let petted = convert_bytes(avatar);

    if let Err(_) = petted {
        return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR);
    }

    HttpResponseBuilder::new(StatusCode::OK)
        .content_type("image/gif")
        .body(petted.unwrap())
}
