use crate::api::controllers::user_address_handler::{
    create_user_address_handler,
    update_user_address_handler,
    delete_user_address_handler,
    get_user_address_handler,
    list_user_addresss_handler,
};

use actix_web::{
    middleware::from_fn,
    web::{self, ServiceConfig},
};

pub fn user_address_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/user_addresss")
            // Uncomment if you need a middleware for this scope
            // .wrap(from_fn(|req, next| async move {
            //     check_permission_middleware(req, next, "admin").await
            // }))
            .service(
                web::resource("")
                    .route(web::post().to(create_user_address_handler))
                    .route(web::get().to(list_user_addresss_handler)),
            )
            .service(web::resource("/update").route(web::post().to(update_user_address_handler)))
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_user_address_handler))
                    .route(web::delete().to(delete_user_address_handler)),
            ),
    );
}
