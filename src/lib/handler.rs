use crate::lib::avatars::AvatarFetch;
use crate::lib::filters::ImageFilter;
use actix_web::web::Path;
use actix_web::{web, Resource};
use actix_web::{HttpResponse, HttpResponseBuilder};
use awc::http::StatusCode;

pub fn handler<T, U>(path: &str, fetcher: T, filter: U) -> Resource
where
    T: AvatarFetch + 'static,
    U: ImageFilter + 'static,
{
    web::resource(path).route(web::get().to(move |params: Path<String>| {
        let id = params.into_inner();
        let fetcher = fetcher.clone();
        let filter = filter.clone();

        async move {
            let avatar = fetcher.get_avatar(&id).await;

            if avatar.is_err() {
                return HttpResponse::new(StatusCode::NOT_FOUND);
            }

            let avatar = avatar.unwrap();
            let filtered = filter.convert_bytes(avatar);

            if filtered.is_err() {
                return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR);
            }

            HttpResponseBuilder::new(StatusCode::OK)
                .content_type("image/gif")
                .body(filtered.unwrap())
        }
    }))
}
