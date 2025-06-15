use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use actix_web::error::JsonPayloadError;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Standardized API error response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub error: Option<String>,
    pub status: u16,
}

/// Custom error types for the application
#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    /// Database errors (connection, query, etc.)
    Database(String),
    /// Validation errors (invalid input data)
    Validation(String),
    /// Not found errors (resource doesn't exist)
    NotFound(String),
    /// Authorization errors (permission denied)
    Unauthorized(String),
    /// Bad request errors (invalid parameters)
    BadRequest(String),
    /// Server errors (internal issues)
    InternalServer(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::InternalServer(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let error_msg = self.to_string();
        let message = match self {
            AppError::Database(_) => "Database operation failed",
            AppError::Validation(_) => "Validation failed",
            AppError::NotFound(_) => "Resource not found",
            AppError::Unauthorized(_) => "Unauthorized access",
            AppError::BadRequest(_) => "Invalid request",
            AppError::InternalServer(_) => "Internal server error",
        };

        HttpResponse::build(status).json(ErrorResponse {
            message: message.to_string(),
            error: Some(error_msg),
            status: status.as_u16(),
        })
    }
}

/// Convert Diesel errors to our AppError
impl From<DieselError> for AppError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => AppError::NotFound("Resource not found".to_string()),
            _ => {
                log::error!("Database error: {:?}", error);
                AppError::Database(error.to_string())
            }
        }
    }
}

/// Convert r2d2 errors to our AppError
impl From<r2d2::Error> for AppError {
    fn from(error: r2d2::Error) -> Self {
        log::error!("Database connection error: {:?}", error);
        AppError::Database("Failed to get database connection".to_string())
    }
}

/// Convert JSON payload errors to our AppError
impl From<JsonPayloadError> for AppError {
    fn from(error: JsonPayloadError) -> Self {
        match &error {
            JsonPayloadError::ContentType => {
                AppError::BadRequest("Invalid content type. Expected application/json".to_string())
            }
            JsonPayloadError::Deserialize(err) => {
                if err.is_data() {
                    let err_string = err.to_string();
                    let field_name = err_string
                        .split("field `")
                        .nth(1)
                        .and_then(|s| s.split('`').next())
                        .unwrap_or("unknown");
                    
                    AppError::Validation(format!("Missing or invalid field: {}", field_name))
                } else {
                    AppError::BadRequest("Invalid JSON format".to_string())
                }
            }
            _ => AppError::BadRequest("Error processing JSON data".to_string()),
        }
    }
}

/// Convert uuid parsing errors to our AppError
impl From<uuid::Error> for AppError {
    fn from(_: uuid::Error) -> Self {
        AppError::BadRequest("Invalid UUID format".to_string())
    }
}

/// Helper function to create JSON config with our error handler
pub fn json_error_handler() -> actix_web::web::JsonConfig {
    actix_web::web::JsonConfig::default()
        .error_handler(|err, _| {
            let error_response = AppError::from(err).error_response();
            actix_web::error::InternalError::from_response(
                "Json payload error",
                error_response
            )
            .into()
        })
}

/// Helper functions to create error responses
#[allow(dead_code)]
pub mod response {
    use super::*;
    
    /// Create a bad request error response
    pub fn bad_request(message: &str, error: Option<&str>) -> HttpResponse {
        let status = StatusCode::BAD_REQUEST;
        HttpResponse::BadRequest().json(ErrorResponse {
            message: message.to_string(),
            error: error.map(|e| e.to_string()),
            status: status.as_u16(),
        })
    }
    
    /// Create a not found error response
    pub fn not_found(message: &str, error: Option<&str>) -> HttpResponse {
        let status = StatusCode::NOT_FOUND;
        HttpResponse::NotFound().json(ErrorResponse {
            message: message.to_string(),
            error: error.map(|e| e.to_string()),
            status: status.as_u16(),
        })
    }
    
    /// Create an internal server error response
    pub fn server_error(message: &str, error: Option<&str>) -> HttpResponse {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        HttpResponse::InternalServerError().json(ErrorResponse {
            message: message.to_string(),
            error: error.map(|e| e.to_string()),
            status: status.as_u16(),
        })
    }
    
    /// Create an unauthorized error response
    pub fn unauthorized(message: &str, error: Option<&str>) -> HttpResponse {
        let status = StatusCode::UNAUTHORIZED;
        HttpResponse::Unauthorized().json(ErrorResponse {
            message: message.to_string(),
            error: error.map(|e| e.to_string()),
            status: status.as_u16(),
        })
    }
    
    /// Create a success response
    pub fn ok<T: Serialize>(data: T) -> HttpResponse {
        HttpResponse::Ok().json(data)
    }
    
    /// Create a created response
    pub fn created<T: Serialize>(data: T) -> HttpResponse {
        HttpResponse::Created().json(data)
    }
} 