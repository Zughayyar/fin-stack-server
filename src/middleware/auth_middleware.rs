use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;

use crate::services::auth_service::AuthService;

/// JWT token validator middleware
/// 
/// This middleware validates JWT bearer tokens and protects routes that require authentication.
/// When a valid token is provided, it extracts the user claims and adds them to the request
/// extensions for use in route handlers.
/// 
/// Usage example:
/// ```rust
/// use actix_web_httpauth::middleware::HttpAuthentication;
/// use crate::middleware::auth_middleware::jwt_validator;
/// 
/// let auth = HttpAuthentication::bearer(jwt_validator);
/// 
/// cfg.service(
///     web::scope("/api/protected")
///         .wrap(auth)
///         .route("/profile", web::get().to(get_profile))
///         .route("/settings", web::get().to(get_settings))
/// );
/// ```
/// 
/// In your protected route handlers, you can access the user claims like this:
/// ```rust
/// use crate::models::auth::Claims;
/// 
/// pub async fn protected_handler(req: HttpRequest) -> Result<HttpResponse> {
///     let claims = req.extensions().get::<Claims>().unwrap();
///     let user_id = claims.sub;
///     // ... use user_id in your handler
/// }
/// ```
pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    
    match AuthService::validate_token(token) {
        Ok(claims) => {
            // Add user claims to request extensions for use in handlers
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<Config>()
                .cloned()
                .unwrap_or_default()
                .scope("Bearer");
            
            Err((AuthenticationError::from(config).into(), req))
        }
    }
} 