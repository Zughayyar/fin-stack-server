-- Database initialization script
-- This script runs when PostgreSQL container starts for the first time

-- Create the database if it doesn't exist (though this should be handled by POSTGRES_DB)
-- SELECT 'CREATE DATABASE finstack' WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'finstack')\gexec

-- Create any additional users or configurations if needed
-- For now, this is just a placeholder

-- Log the initialization
\echo 'Database initialization completed' 