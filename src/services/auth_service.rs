use actix_web::{web, HttpRequest};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::env;
use uuid::Uuid;

use crate::models::auth::{AuthError, Claims, LoginRequest, RegisterRequest, TokenResponse, UserInfo};
use crate::models::schema::users;
use crate::models::user::{NewUser, User};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct AuthService;

impl AuthService {
    /// Hash a password using bcrypt
    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }

    /// Verify a password against a hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, hash)
    }

    /// Generate JWT token for user
    pub fn generate_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims::new(user.id, user.email.clone(), expiration);

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }

    /// Validate JWT token and extract claims
    pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
        let validation = Validation::new(Algorithm::HS256);

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )
        .map(|data| data.claims)
    }

    /// Register a new user
    pub async fn register_user(
        pool: web::Data<DbPool>,
        register_data: RegisterRequest,
    ) -> Result<TokenResponse, AuthError> {
        let mut conn = pool.get().map_err(|_| AuthError {
            message: "Database connection failed".to_string(),
            code: "DB_CONNECTION_ERROR".to_string(),
        })?;

        // Validate password confirmation
        if register_data.password != register_data.confirm_password {
            return Err(AuthError {
                message: "Passwords do not match".to_string(),
                code: "PASSWORD_MISMATCH".to_string(),
            });
        }

        // Check if email already exists
        let existing_user = users::table
            .filter(users::email.eq(&register_data.email))
            .first::<User>(&mut conn)
            .optional()
            .map_err(|_| AuthError {
                message: "Database query failed".to_string(),
                code: "DB_QUERY_ERROR".to_string(),
            })?;

        if existing_user.is_some() {
            return Err(AuthError {
                message: "Email already exists".to_string(),
                code: "EMAIL_EXISTS".to_string(),
            });
        }

        // Hash password
        let hashed_password = Self::hash_password(&register_data.password)
            .map_err(|_| AuthError {
                message: "Password hashing failed".to_string(),
                code: "HASH_ERROR".to_string(),
            })?;

        // Create new user
        let new_user = NewUser::new(
            register_data.first_name,
            register_data.last_name,
            register_data.email,
            hashed_password,
        );

        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .map_err(|_| AuthError {
                message: "Failed to create user".to_string(),
                code: "USER_CREATION_ERROR".to_string(),
            })?;

        // Generate token
        let token = Self::generate_token(&user)
            .map_err(|_| AuthError {
                message: "Token generation failed".to_string(),
                code: "TOKEN_ERROR".to_string(),
            })?;

        Ok(TokenResponse {
            token,
            token_type: "Bearer".to_string(),
            expires_in: 24 * 3600, // 24 hours
            user: UserInfo {
                id: user.id,
                first_name: user.first_name,
                last_name: user.last_name,
                email: user.email,
            },
        })
    }

    /// Login user
    pub async fn login_user(
        pool: web::Data<DbPool>,
        login_data: LoginRequest,
    ) -> Result<TokenResponse, AuthError> {
        let mut conn = pool.get().map_err(|_| AuthError {
            message: "Database connection failed".to_string(),
            code: "DB_CONNECTION_ERROR".to_string(),
        })?;

        // Find user by email
        let user = users::table
            .filter(users::email.eq(&login_data.email))
            .first::<User>(&mut conn)
            .optional()
            .map_err(|_| AuthError {
                message: "Database query failed".to_string(),
                code: "DB_QUERY_ERROR".to_string(),
            })?;

        let user = user.ok_or_else(|| AuthError {
            message: "Invalid credentials".to_string(),
            code: "INVALID_CREDENTIALS".to_string(),
        })?;

        // Verify password
        let is_valid = Self::verify_password(&login_data.password, &user.password)
            .map_err(|_| AuthError {
                message: "Password verification failed".to_string(),
                code: "VERIFICATION_ERROR".to_string(),
            })?;

        if !is_valid {
            return Err(AuthError {
                message: "Invalid credentials".to_string(),
                code: "INVALID_CREDENTIALS".to_string(),
            });
        }

        // Generate token
        let token = Self::generate_token(&user)
            .map_err(|_| AuthError {
                message: "Token generation failed".to_string(),
                code: "TOKEN_ERROR".to_string(),
            })?;

        Ok(TokenResponse {
            token,
            token_type: "Bearer".to_string(),
            expires_in: 24 * 3600, // 24 hours
            user: UserInfo {
                id: user.id,
                first_name: user.first_name,
                last_name: user.last_name,
                email: user.email,
            },
        })
    }

    /// Get current user from token
    pub async fn get_current_user(
        pool: web::Data<DbPool>,
        req: HttpRequest,
    ) -> Result<User, AuthError> {
        let token = Self::extract_token_from_request(&req)?;
        let claims = Self::validate_token(&token)
            .map_err(|_| AuthError {
                message: "Invalid token".to_string(),
                code: "INVALID_TOKEN".to_string(),
            })?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AuthError {
                message: "Invalid user ID in token".to_string(),
                code: "INVALID_USER_ID".to_string(),
            })?;

        let mut conn = pool.get().map_err(|_| AuthError {
            message: "Database connection failed".to_string(),
            code: "DB_CONNECTION_ERROR".to_string(),
        })?;

        let user = users::table
            .find(user_id)
            .first::<User>(&mut conn)
            .optional()
            .map_err(|_| AuthError {
                message: "Database query failed".to_string(),
                code: "DB_QUERY_ERROR".to_string(),
            })?;

        user.ok_or_else(|| AuthError {
            message: "User not found".to_string(),
            code: "USER_NOT_FOUND".to_string(),
        })
    }

    /// Extract token from Authorization header
    fn extract_token_from_request(req: &HttpRequest) -> Result<String, AuthError> {
        let auth_header = req
            .headers()
            .get("Authorization")
            .ok_or_else(|| AuthError {
                message: "Missing Authorization header".to_string(),
                code: "MISSING_AUTH_HEADER".to_string(),
            })?;

        let auth_str = auth_header
            .to_str()
            .map_err(|_| AuthError {
                message: "Invalid Authorization header".to_string(),
                code: "INVALID_AUTH_HEADER".to_string(),
            })?;

        if !auth_str.starts_with("Bearer ") {
            return Err(AuthError {
                message: "Invalid Authorization format".to_string(),
                code: "INVALID_AUTH_FORMAT".to_string(),
            });
        }

        Ok(auth_str[7..].to_string())
    }
} 