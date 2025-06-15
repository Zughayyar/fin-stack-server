use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::pg::PgConnection;
use log;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/database/migrations");

pub fn run_migrations(connection: &mut PgConnection) {
    log::info!("Running database migrations...");
    
    connection.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run database migrations");
    
    log::info!("Database migrations completed successfully");
}
