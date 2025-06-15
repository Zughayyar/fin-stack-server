mod income_routes;
mod expense_routes;
mod health_routes;
mod auth_routes;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(health_routes::configure)
        .service(
            web::scope("/api")
                .configure(auth_routes::configure)
                .configure(income_routes::configure)
                .configure(expense_routes::configure)
        );
} 