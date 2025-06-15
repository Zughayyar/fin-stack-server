use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::{web};
use actix_cors::Cors;
use dotenvy::dotenv;
use std::io;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod controllers;
mod middleware;
mod models;
mod routes;
mod services;
mod database;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::auth_controller::register,
        controllers::auth_controller::login,
        controllers::auth_controller::me,
        controllers::auth_controller::logout,
        controllers::income_controller::get_all_incomes,
        controllers::income_controller::get_incomes_by_user_id,
        controllers::income_controller::create_income,
        controllers::income_controller::update_income,
        controllers::income_controller::delete_income,
        controllers::expense_controller::get_all_expenses,
        controllers::expense_controller::get_expenses_by_user_id,
        controllers::expense_controller::create_expense,
        controllers::expense_controller::update_expense,
        controllers::expense_controller::delete_expense,
    ),
    components(
        schemas(
            models::auth::LoginRequest,
            models::auth::RegisterRequest,
            models::auth::TokenResponse,
            models::auth::UserInfo,
            models::auth::AuthError,

            models::income::Income,
            models::income::NewIncome,
            models::income::UpdateIncome,
            models::income::IncomeWithUser,
            models::expense::Expense,
            models::expense::NewExpense,
            models::expense::UpdateExpense
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "incomes", description = "Income management endpoints"),
        (name = "expenses", description = "Expense management endpoints")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize environment
    dotenv().ok();
    
    // Validate all required environment variables first
    config::validate_environment();
    
    // Initialize logger after validation
    env_logger::init();

    let database_url = config::get_database_url();
    let server_url = config::get_server_url();
    log::info!("Starting server at: {}", server_url);
    log::info!("Swagger UI available at: {}/swagger-ui/", server_url);

    let pool = database::db_connection::create_connection_pool(&database_url);
    let mut conn = database::db_connection::get_connection(&pool)
        .expect("Failed to get connection from pool");
    database::db_migrations::run_migrations(&mut conn);

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        // Configure custom logger
        let logger = Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T");

        // Configure CORS with more comprehensive settings
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![
                "content-type", 
                "authorization", 
                "accept",
                "origin",
                "x-requested-with",
                "access-control-request-method",
                "access-control-request-headers"
            ])
            .expose_headers(vec!["content-type", "x-total-count"])
            .max_age(3600)
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(logger)
            .app_data(config::errors::json_error_handler())
            .configure(routes::configure)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(server_url)?
    .run()
    .await
}
