use dotenvy::dotenv;
use std::env;

pub mod errors;

/// Get database URL from environment variable
/// Panics if DATABASE_URL is not set
pub fn get_database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL")
        .expect("❌ DATABASE_URL environment variable is required but not set")
}

/// Get server URL from environment variable
/// Panics if SERVER_URL is not set
pub fn get_server_url() -> String {
    dotenv().ok();
    env::var("SERVER_URL")
        .expect("❌ SERVER_URL environment variable is required but not set")
}

/// Get JWT secret from environment variable
/// Panics if JWT_SECRET is not set
pub fn get_jwt_secret() -> String {
    dotenv().ok();
    let secret = env::var("JWT_SECRET")
        .expect("❌ JWT_SECRET environment variable is required but not set");
    
    if secret.len() < 32 {
        panic!("❌ JWT_SECRET must be at least 32 characters long for security");
    }
    
    secret
}

/// Get JWT expiration hours from environment variable
/// Panics if JWT_EXPIRATION_HOURS is not set
pub fn get_jwt_expiration_hours() -> u64 {
    dotenv().ok();
    let hours_str = env::var("JWT_EXPIRATION_HOURS")
        .expect("❌ JWT_EXPIRATION_HOURS environment variable is required but not set");
    
    hours_str.parse::<u64>()
        .expect("❌ JWT_EXPIRATION_HOURS must be a valid number")
}

/// Get Rust log level from environment variable
/// Panics if RUST_LOG is not set
pub fn get_rust_log() -> String {
    dotenv().ok();
    env::var("RUST_LOG")
        .expect("❌ RUST_LOG environment variable is required but not set")
}

/// Get environment type from environment variable
/// Panics if ENVIRONMENT is not set
pub fn get_environment() -> String {
    dotenv().ok();
    let env_type = env::var("ENVIRONMENT")
        .expect("❌ ENVIRONMENT environment variable is required but not set");
    
    match env_type.as_str() {
        "development" | "staging" | "production" => env_type,
        _ => panic!("❌ ENVIRONMENT must be one of: development, staging, production")
    }
}

/// Validate all required environment variables at startup
/// Call this function early in main() to fail fast if config is invalid
pub fn validate_environment() {
    println!("🔍 Validating environment configuration...");
    
    // Check all required variables
    let _db_url = get_database_url();
    let _server_url = get_server_url();
    let _jwt_secret = get_jwt_secret();
    let _jwt_expiration = get_jwt_expiration_hours();
    let _rust_log = get_rust_log();
    let environment = get_environment();
    
    println!("✅ All environment variables validated successfully");
    println!("🌍 Environment: {}", environment);
    
    // Additional validation for production
    if environment == "production" {
        validate_production_config();
    }
}

/// Additional validation for production environment
fn validate_production_config() {
    println!("🔒 Validating production-specific configuration...");
    
    let jwt_secret = get_jwt_secret();
    if jwt_secret.contains("dev") || jwt_secret.contains("test") || jwt_secret.len() < 64 {
        panic!("❌ Production JWT_SECRET appears to be insecure. Use a long, random string (64+ chars)");
    }
    
    let db_url = get_database_url();
    if db_url.contains("passw0rd") || db_url.contains("password") {
        panic!("❌ Production database appears to use a weak password");
    }
    
    println!("✅ Production configuration validated");
} 