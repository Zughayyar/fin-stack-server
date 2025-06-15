# FinStack API Docker Deployment Guide

This guide provides comprehensive instructions for deploying the FinStack API using Docker and Docker Compose.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Quick Start](#quick-start)
3. [Environment Configuration](#environment-configuration)
4. [Deployment Options](#deployment-options)
5. [Health Checks](#health-checks)
6. [Troubleshooting](#troubleshooting)
7. [Best Practices](#best-practices)
8. [Production Considerations](#production-considerations)

## Prerequisites

- Docker Engine 20.10+
- Docker Compose 2.0+
- At least 2GB RAM available
- Ports 5432, 8080, 80 available (or configure different ports)

## Quick Start

### Development Environment (Database Only)

```bash
# Start PostgreSQL only for local development
./scripts/deploy.sh dev

# Or manually:
docker-compose up -d postgres
```

### Full Stack Deployment

```bash
# Production deployment with all services
./scripts/deploy.sh prod --rebuild

# Or manually:
docker-compose --profile with-server up -d --build
```

## Environment Configuration

### Environment Files

Create environment-specific configuration files:

- `env.dev` - Development settings
- `env.prod` - Production settings
- `env.example` - Template file

### Key Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `POSTGRES_USER` | Database username | `user` |
| `POSTGRES_PASSWORD` | Database password | `passw0rd` |
| `POSTGRES_DB` | Database name | `finstack` |
| `RUST_LOG` | Logging level | `info` |
| `SERVER_URL` | Server bind address | `0.0.0.0:8080` |

## Deployment Options

### Using Deployment Script

```bash
# Development environment
./scripts/deploy.sh dev

# Production with rebuild
./scripts/deploy.sh prod --rebuild

# Clean deployment (removes volumes)
./scripts/deploy.sh prod --clean --rebuild

# Show logs after deployment
./scripts/deploy.sh prod --logs
```

### Manual Docker Compose

```bash
# Database only
docker-compose up -d postgres

# Full stack
docker-compose --profile with-server up -d

# With Nginx reverse proxy
docker-compose --profile with-server --profile with-nginx up -d

# With PgAdmin
docker-compose --profile with-pgadmin up -d
```

## Health Checks

### Application Health Endpoints

- `GET /health` - Basic health check
- `GET /health/detailed` - Health check with database connectivity

### Docker Health Checks

All services include health checks:

- **PostgreSQL**: `pg_isready` command
- **API Server**: HTTP health endpoint
- **Nginx**: Depends on API server health

### Monitoring Health

```bash
# Check service status
docker-compose ps

# View health check logs
docker-compose logs postgres
docker-compose logs server

# Manual health check
curl http://localhost:8080/health
```

## Troubleshooting

### Common Issues and Solutions

#### 1. Database Connection Errors

**Symptoms:**

- "Failed to create connection pool"
- "Connection refused"

**Solutions:**

```bash
# Check PostgreSQL is running
docker-compose ps postgres

# Check PostgreSQL logs
docker-compose logs postgres

# Restart PostgreSQL
docker-compose restart postgres

# Clean restart
docker-compose down postgres
docker-compose up -d postgres
```

#### 2. Migration Failures

**Symptoms:**

- "Failed to run database migrations"
- Migration timeout errors

**Solutions:**

```bash
# Check database is ready
docker-compose exec postgres pg_isready -U $POSTGRES_USER

# Manual migration (if needed)
docker-compose exec server ./server migrate

# Reset database (CAUTION: destroys data)
docker-compose down -v
docker-compose up -d postgres
```

#### 3. Port Conflicts

**Symptoms:**

- "Port already in use"
- "Address already in use"

**Solutions:**

```bash
# Check what's using the port
lsof -i :5432
lsof -i :8080

# Use different ports in docker-compose.yml
ports:
  - "5433:5432"  # PostgreSQL
  - "8081:8080"  # API Server
```

#### 4. Build Failures

**Symptoms:**

- Rust compilation errors
- Dependency resolution failures

**Solutions:**

```bash
# Clean build
docker-compose build --no-cache server

# Check Dockerfile syntax
docker build -t finstack-api .

# View build logs
docker-compose build server
```

#### 5. Memory Issues

**Symptoms:**

- Container killed (exit code 137)
- Out of memory errors

**Solutions:**

```bash
# Increase Docker memory limit
# Docker Desktop: Settings > Resources > Memory

# Monitor memory usage
docker stats

# Reduce connection pool size in code
```

### Debugging Commands

```bash
# View all logs
docker-compose logs

# Follow logs in real-time
docker-compose logs -f

# View specific service logs
docker-compose logs postgres
docker-compose logs server

# Execute commands in containers
docker-compose exec postgres psql -U $POSTGRES_USER -d $POSTGRES_DB
docker-compose exec server /bin/bash

# Inspect container configuration
docker inspect finstack_postgres
docker inspect finstack_api
```

## Best Practices

### Security

1. **Change Default Passwords**

   ```bash
   # Use strong passwords in production
   POSTGRES_PASSWORD=your_very_secure_password_here
   ```

2. **Use Environment Files**

   ```bash
   # Never commit .env files with secrets
   echo ".env*" >> .gitignore
   ```

3. **Network Isolation**
   - Services communicate through internal Docker network
   - Only necessary ports exposed to host

### Performance

1. **Database Optimization**

   ```yaml
   # Adjust PostgreSQL settings
   environment:
     POSTGRES_SHARED_PRELOAD_LIBRARIES: pg_stat_statements
     POSTGRES_MAX_CONNECTIONS: 100
   ```

2. **Connection Pooling**

   ```rust
   // Adjust pool settings in code
   .max_size(15)
   .min_idle(Some(5))
   ```

3. **Resource Limits**

   ```yaml
   # Add resource limits
   deploy:
     resources:
       limits:
         memory: 512M
         cpus: '0.5'
   ```

### Monitoring

1. **Health Checks**
   - All services have health checks
   - Automatic restart on failure

2. **Logging**

   ```bash
   # Centralized logging
   docker-compose logs > deployment.log
   ```

3. **Metrics**

   ```bash
   # Monitor resource usage
   docker stats --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}"
   ```

## Production Considerations

### SSL/TLS

1. **Generate SSL Certificates**

   ```bash
   # Create SSL directory
   mkdir -p nginx/ssl
   
   # Generate self-signed certificate (for testing)
   openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
     -keyout nginx/ssl/finstack.key \
     -out nginx/ssl/finstack.crt
   ```

2. **Update Nginx Configuration**

   ```nginx
   server {
       listen 443 ssl;
       ssl_certificate /etc/nginx/ssl/finstack.crt;
       ssl_certificate_key /etc/nginx/ssl/finstack.key;
   }
   ```

### Backup Strategy

1. **Database Backups**

   ```bash
   # Create backup
   docker-compose exec postgres pg_dump -U $POSTGRES_USER $POSTGRES_DB > backup.sql
   
   # Restore backup
   docker-compose exec -T postgres psql -U $POSTGRES_USER $POSTGRES_DB < backup.sql
   ```

2. **Automated Backups**

   ```bash
   # Add to crontab
   0 2 * * * /path/to/backup-script.sh
   ```

### Scaling

1. **Horizontal Scaling**

   ```yaml
   # Scale API servers
   docker-compose up -d --scale server=3
   ```

2. **Load Balancing**
   - Configure Nginx upstream with multiple servers
   - Use external load balancer

### Monitoring and Alerting

1. **Health Check Monitoring**

   ```bash
   # External monitoring
   curl -f http://your-domain.com/health || alert
   ```

2. **Log Aggregation**
   - Use ELK stack or similar
   - Centralized log management

### Environment Separation

1. **Multiple Environments**

   ```bash
   # Different compose files
   docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
   ```

2. **CI/CD Integration**

   ```yaml
   # GitHub Actions example
   - name: Deploy to production
     run: ./scripts/deploy.sh prod --rebuild
   ```

## Support

For issues and questions:

1. Check the [troubleshooting section](#troubleshooting)
2. Review Docker and application logs
3. Verify environment configuration
4. Check system resources

## Version History

- v1.0.0 - Initial Docker deployment setup
- v1.1.0 - Added health checks and monitoring
- v1.2.0 - Production optimizations and security improvements
