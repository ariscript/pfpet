use crate::{handler, lib::filters::cancel::Cancel, AvatarFetch, Bonk, Pet};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{middleware, web, Error, Scope};

pub fn from_fetcher<T>(
    path: &str,
    fetcher: T,
) -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse,
        Error = Error,
        Config = (),
        InitError = (),
    >,
>
where
    T: AvatarFetch + 'static,
{
    web::scope(path)
        .wrap(middleware::DefaultHeaders::new().add((
            "Cache-Control",
            format!("max-age={}", T::cache_max_length()),
        )))
        .service(handler("/{identifier}.gif", fetcher.clone(), Pet))
        .service(handler("/bonk/{identifier}.gif", fetcher.clone(), Bonk))
        .service(handler("/cancel/{identifier}.gif", fetcher, Cancel))
}
