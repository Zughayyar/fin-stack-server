use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = "password123")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "John")]
    pub first_name: String,
    #[schema(example = "Doe")]
    pub last_name: String,
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = "password123")]
    pub password: String,
    #[schema(example = "password123")]
    pub confirm_password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    #[schema(example = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...")]
    pub token: String,
    #[schema(example = "Bearer")]
    pub token_type: String,
    #[schema(example = 3600)]
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserInfo {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    #[schema(example = "John")]
    pub first_name: String,
    #[schema(example = "Doe")]
    pub last_name: String,
    #[schema(example = "john@example.com")]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub exp: usize, // Expiration time
    pub iat: usize, // Issued at
}

impl Claims {
    pub fn new(user_id: Uuid, email: String, exp: usize) -> Self {
        Self {
            sub: user_id.to_string(),
            email,
            exp,
            iat: chrono::Utc::now().timestamp() as usize,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthError {
    #[schema(example = "Invalid credentials")]
    pub message: String,
    #[schema(example = "INVALID_CREDENTIALS")]
    pub code: String,
} 