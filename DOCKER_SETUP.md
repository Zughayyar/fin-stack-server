# FinStack Docker Setup Guide

This guide explains how to run the complete FinStack application using Docker, including the Angular frontend, Rust API backend, PostgreSQL database, and Nginx reverse proxy.

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │   Backend       │    │   Database      │
│   (Angular)     │    │   (Rust API)    │    │   (PostgreSQL)  │
│   Port: 3000    │    │   Port: 8080    │    │   Port: 5432    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
          │                        │                        │
          └────────────┬───────────┘                        │
                       │                                     │
          ┌─────────────────────────────┐                   │
          │     Nginx Reverse Proxy     │                   │
          │         Port: 80           │                   │
          │  Routes:                   │                   │
          │  /api/* → Backend          │                   │
          │  /*     → Frontend         │                   │
          └─────────────────────────────┘                   │
                       │                                     │
                       └─────────────────────────────────────┘
```

## 📦 Services

### 1. PostgreSQL Database (`postgres`)

- **Image**: `postgres:16-alpine`
- **Port**: `5432`
- **Health Check**: Built-in PostgreSQL health check
- **Data**: Persistent volume storage

### 2. Rust API Server (`server`)

- **Build**: From local Dockerfile (multi-stage Rust build)
- **Port**: `8080`
- **Health Check**: `GET /health`
- **Dependencies**: PostgreSQL database

### 3. Angular Frontend (`frontend`)

- **Build**: From `../Web/Dockerfile` (multi-stage Node.js build + Nginx serve)
- **Port**: `3000` (mapped from internal port 80)
- **Health Check**: `GET /health`

### 4. Nginx Reverse Proxy (`nginx`)

- **Image**: `nginx:alpine`
- **Port**: `80`
- **Configuration**: Routes `/api/*` to backend, everything else to frontend
- **Dependencies**: Both frontend and backend services

### 5. PgAdmin (Optional) (`pgadmin`)

- **Image**: `dpage/pgadmin4`
- **Port**: `8081`
- **Profile**: `with-pgadmin`

## 🚀 Quick Start

### Full Stack (Recommended)

```bash
# From the server directory
cd server
./scripts/run-full-stack.sh
```

### Manual Docker Compose

```bash
# From the server directory
cd server

# Start all services
docker-compose --profile with-server up --build -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f

# Stop all services
docker-compose --profile with-server down
```

## 🔧 Different Deployment Scenarios

### 1. Full Stack with Nginx

```bash
docker-compose --profile with-server up -d
```

**Access**: <http://localhost> (Frontend via Nginx proxy)

### 2. Frontend Only (for development)

```bash
./scripts/run-frontend-only.sh
# or
docker-compose --profile with-frontend up -d frontend
```

**Access**: <http://localhost:3000>

### 3. Backend + Database Only

```bash
docker-compose up -d postgres server
```

**Access**: <http://localhost:8080>

### 4. Database Only

```bash
docker-compose up -d postgres
```

**Access**: localhost:5432

### 5. With Database Admin

```bash
docker-compose --profile with-server --profile with-pgadmin up -d
```

**Additional Access**: <http://localhost:8081> (PgAdmin)

## 🌐 Access Points

| Service | Direct Access | Via Nginx Proxy |
|---------|---------------|------------------|
| Frontend | <http://localhost:3000> | <http://localhost> |
| API | <http://localhost:8080> | <http://localhost/api/> |
| Database | localhost:5432 | - |
| PgAdmin | <http://localhost:8081> | - |

## 📊 Environment Variables

Create `env.dev` or `env.prod` files in the `server` directory:

```bash
# Database Configuration
POSTGRES_USER=your_user
POSTGRES_PASSWORD=your_password
POSTGRES_DB=finstack

# API Configuration  
RUST_LOG=info

# PgAdmin Configuration (optional)
PGADMIN_EMAIL=admin@finstack.local
PGADMIN_PASSWORD=admin123
```

## 🔍 Health Checks & Monitoring

### Check Service Status

```bash
# All services
docker-compose ps

# Specific service health
docker-compose exec postgres pg_isready
curl http://localhost:8080/health
curl http://localhost:3000/health
curl http://localhost/health
```

### View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f frontend
docker-compose logs -f server
docker-compose logs -f postgres
docker-compose logs -f nginx
```

## 🐛 Troubleshooting

### Common Issues

1. **Port conflicts**

   ```bash
   # Check what's using the ports
   lsof -i :80 :3000 :8080 :5432
   ```

2. **Services not starting**

   ```bash
   # Check service logs
   docker-compose logs [service_name]
   
   # Rebuild specific service
   docker-compose build [service_name]
   ```

3. **Database connection issues**

   ```bash
   # Check database is ready
   docker-compose exec postgres pg_isready -U user -d finstack
   
   # Connect to database
   docker-compose exec postgres psql -U user -d finstack
   ```

4. **Frontend build issues**

   ```bash
   # Rebuild frontend with no cache
   docker-compose build --no-cache frontend
   ```

### Reset Everything

```bash
# Stop and remove all containers, networks, and volumes
docker-compose --profile with-server down -v
docker system prune -f

# Rebuild from scratch
docker-compose --profile with-server up --build -d
```

## 🔧 Development Workflow

### Frontend Development

```bash
# Start backend services only
docker-compose up -d postgres server

# Run frontend locally for development
cd ../Web
pnpm install
pnpm start

# Or run frontend in Docker for testing
./scripts/run-frontend-only.sh
```

### Backend Development

```bash
# Start database only
docker-compose up -d postgres

# Run backend locally for development
cargo run

# Or run everything in Docker
docker-compose --profile with-server up -d
```

## 📁 File Structure

```
server/
├── docker-compose.yml          # Main orchestration file
├── Dockerfile                  # Backend container definition
├── nginx/
│   └── nginx.conf             # Nginx reverse proxy config
├── scripts/
│   ├── run-full-stack.sh      # Full stack startup script
│   └── run-frontend-only.sh   # Frontend-only script
├── env.dev                    # Development environment
├── env.prod                   # Production environment
└── DOCKER_SETUP.md           # This file

Web/
├── Dockerfile                 # Frontend container definition
├── nginx.conf                 # Frontend nginx config
├── .dockerignore             # Docker ignore file
└── ... (Angular app files)
```

## 🏷️ Docker Profiles

- **Default**: Only PostgreSQL database
- **`with-server`**: Database + API + Frontend + Nginx (full stack)
- **`with-frontend`**: Only frontend service
- **`with-nginx`**: Include Nginx reverse proxy
- **`with-pgadmin`**: Include database administration tool

## 🔒 Security Considerations

1. **Environment Variables**: Use proper env files for sensitive data
2. **Nginx Security**: Security headers and rate limiting configured
3. **Database**: PostgreSQL runs with custom user, not root
4. **Frontend**: Served via Nginx with proper caching headers
5. **API**: Health checks ensure services are actually ready

## 📈 Performance Optimizations

1. **Multi-stage builds**: Smaller production images
2. **Layer caching**: Dependencies installed before copying source
3. **Gzip compression**: Enabled in Nginx
4. **Static asset caching**: Long-term caching for JS/CSS files
5. **Health checks**: Prevent routing to unhealthy services
