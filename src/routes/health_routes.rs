use actix_web::{web, HttpResponse, Result};
use serde_json::json;

/// Health check endpoint
/// 
/// Returns a simple health status response for Docker health checks
/// and load balancer health probes.
pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "finstack-api",
        "version": env!("CARGO_PKG_VERSION")
    })))
}

/// Health check with database connectivity test
pub async fn health_check_detailed(
    pool: web::Data<crate::database::db_connection::DbPool>
) -> Result<HttpResponse> {
    use crate::database::db_connection::get_connection;
    
    // Test database connectivity
    let db_status = match get_connection(&pool) {
        Ok(_) => "healthy",
        Err(_) => "unhealthy"
    };
    
    let overall_status = if db_status == "healthy" { "healthy" } else { "unhealthy" };
    
    let response = json!({
        "status": overall_status,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "finstack-api",
        "version": env!("CARGO_PKG_VERSION"),
        "checks": {
            "database": db_status
        }
    });
    
    if overall_status == "healthy" {
        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::ServiceUnavailable().json(response))
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check))
       .route("/health/detailed", web::get().to(health_check_detailed));
} 