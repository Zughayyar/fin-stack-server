use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::controllers::expense_controller;
use crate::middleware::auth_middleware::jwt_validator;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(jwt_validator);
    
    cfg.service(
        web::scope("/expenses")
            .wrap(auth)
            .route("", web::get().to(expense_controller::get_all_expenses))
            .route("", web::post().to(expense_controller::create_expense))
            .route("/{user_id}", web::get().to(expense_controller::get_expenses_by_user_id))
            .route("/{expense_id}", web::put().to(expense_controller::update_expense))
            .route("/{expense_id}", web::delete().to(expense_controller::delete_expense))
    );
}
