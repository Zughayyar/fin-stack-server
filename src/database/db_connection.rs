use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use std::time::Duration;
use log;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_connection_pool(database_url: &str) -> DbPool {
    create_connection_pool_with_retries(database_url, 5)
}

pub fn create_connection_pool_with_retries(database_url: &str, max_retries: u32) -> DbPool {
    for attempt in 1..=max_retries {
        log::info!("Attempting to create database connection pool (attempt {}/{})", attempt, max_retries);
        
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        
        match Pool::builder()
            .max_size(15)
            .min_idle(Some(5))
            .connection_timeout(Duration::from_secs(30))
            .idle_timeout(Some(Duration::from_secs(300)))
            .test_on_check_out(true)
            .build(manager)
        {
            Ok(pool) => {
                log::info!("Database connection pool created successfully");
                return pool;
            }
            Err(e) => {
                log::warn!("Failed to create connection pool (attempt {}): {}", attempt, e);
                
                if attempt < max_retries {
                    log::info!("Retrying in 5 seconds...");
                    std::thread::sleep(Duration::from_secs(5));
                } else {
                    log::error!("Failed to create connection pool after {} attempts", max_retries);
                    panic!("Failed to create connection pool: {}", e);
                }
            }
        }
    }
    
    unreachable!("This should never be reached")
}

pub fn get_connection(pool: &DbPool) -> Result<DbConnection, r2d2::Error> {
    pool.get().map_err(|e| {
        log::error!("Failed to get database connection: {}", e);
        e
    })
}

// pub fn test_connection(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
//     use diesel::prelude::*;
    
//     let mut conn = get_connection(pool)?;
    
//     // Simple query to test connectivity
//     diesel::sql_query("SELECT 1").execute(&mut conn)?;
    
//     Ok(())
// } 