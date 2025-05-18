// src/utils/routes.rs
use actix_web::web;
use crate::handlers::read::read_handler;

// inside your configure_routes fn:
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::resource("/country")
                    .route(web::get().to(read_handler))
            )
    );
}
