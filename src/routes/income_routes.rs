use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::controllers::income_controller;
use crate::middleware::auth_middleware::jwt_validator;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(jwt_validator);
    
    cfg.service(
        web::scope("/incomes")
            .wrap(auth)
            .route("", web::get().to(income_controller::get_all_incomes))
            .route("/{user_id}", web::get().to(income_controller::get_incomes_by_user_id))
            .route("", web::post().to(income_controller::create_income))
            .route("/{income_id}", web::put().to(income_controller::update_income))
            .route("/{income_id}", web::delete().to(income_controller::delete_income))
    );
} 