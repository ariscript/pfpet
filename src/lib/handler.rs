use crate::lib::avatars::AvatarFetch;
use crate::lib::filters::ImageFilter;
use actix_web::web::Path;
use actix_web::{web, Resource};
use actix_web::{HttpResponse, HttpResponseBuilder};
use awc::http::StatusCode;

pub fn handler<T, U>(fetcher: T, filter: U, path: &str) -> Resource
where
    T: AvatarFetch + Clone + 'static,
    U: ImageFilter + Clone + 'static,
{
    web::resource(path).route(web::get().to(move |params: Path<String>| {
        let id = params.into_inner();
        let fetcher = fetcher.clone();
        let filter = filter.clone();

        async move {
            let avatar = fetcher.get_avatar(&id).await;

            if let Err(_) = avatar {
                return HttpResponse::new(StatusCode::NOT_FOUND);
            }

            let avatar = avatar.unwrap();
            let filtered = filter.convert_bytes(avatar);

            if let Err(_) = filtered {
                return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR);
            }

            HttpResponseBuilder::new(StatusCode::OK)
                .content_type("image/gif")
                .body(filtered.unwrap())
        }
    }))
}
