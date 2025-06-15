use actix_web::{web, HttpRequest, HttpResponse, Result};

use crate::models::auth::{AuthError, LoginRequest, RegisterRequest, TokenResponse};
use crate::services::auth_service::{AuthService, DbPool};

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = TokenResponse),
        (status = 400, description = "Registration failed", body = AuthError)
    )
)]
pub async fn register(
    pool: web::Data<DbPool>,
    register_data: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    match AuthService::register_user(pool, register_data.into_inner()).await {
        Ok(token_response) => Ok(HttpResponse::Created().json(token_response)),
        Err(error) => match error.code.as_str() {
            "EMAIL_EXISTS" => Ok(HttpResponse::Conflict().json(error)),
            "PASSWORD_MISMATCH" => Ok(HttpResponse::BadRequest().json(error)),
            _ => Ok(HttpResponse::InternalServerError().json(error)),
        },
    }
}

/// Login user
#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = TokenResponse),
        (status = 401, description = "Invalid credentials", body = AuthError)
    )
)]
pub async fn login(
    pool: web::Data<DbPool>,
    login_data: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    match AuthService::login_user(pool, login_data.into_inner()).await {
        Ok(token_response) => Ok(HttpResponse::Ok().json(token_response)),
        Err(error) => match error.code.as_str() {
            "INVALID_CREDENTIALS" => Ok(HttpResponse::Unauthorized().json(error)),
            _ => Ok(HttpResponse::InternalServerError().json(error)),
        },
    }
}

/// Get current user profile
#[utoipa::path(
    get,
    path = "/api/auth/me",
    tag = "auth",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Current user profile", body = crate::models::user::User),
        (status = 401, description = "Unauthorized", body = AuthError)
    )
)]
pub async fn me(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    match AuthService::get_current_user(pool, req).await {
        Ok(user) => {
            // Don't return the password in the response
            let safe_user = crate::models::auth::UserInfo {
                id: user.id,
                first_name: user.first_name,
                last_name: user.last_name,
                email: user.email,
            };
            Ok(HttpResponse::Ok().json(safe_user))
        }
        Err(error) => match error.code.as_str() {
            "MISSING_AUTH_HEADER" | "INVALID_TOKEN" | "INVALID_AUTH_HEADER" | "INVALID_AUTH_FORMAT" => {
                Ok(HttpResponse::Unauthorized().json(error))
            }
            "USER_NOT_FOUND" => Ok(HttpResponse::NotFound().json(error)),
            _ => Ok(HttpResponse::InternalServerError().json(error)),
        },
    }
}

/// Logout user (client-side token deletion)
#[utoipa::path(
    post,
    path = "/api/auth/logout",
    tag = "auth",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn logout() -> Result<HttpResponse> {
    // Since we're using stateless JWT tokens, logout is handled client-side
    // The client should delete the token from storage
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Logout successful. Please delete the token from client storage."
    })))
} 