#!/bin/bash

# Script to run the full FinStack application with all services
# This includes: PostgreSQL, Rust API server, Angular frontend, and Nginx reverse proxy

set -e

echo "🚀 Starting FinStack Full Application..."

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

# Load environment variables (filtering out comments and empty lines)
if [ -f "env.prod" ]; then
    echo "📁 Loading production environment variables..."
    export $(grep -v '^#' env.prod | grep -v '^$' | xargs)
elif [ -f "env.dev" ]; then
    echo "📁 Loading development environment variables..."
    export $(grep -v '^#' env.dev | grep -v '^$' | xargs)
else
    echo "⚠️  No environment file found. Using default values."
fi

echo "🔨 Building and starting all services..."

# Start all services with frontend profile
docker-compose --profile with-frontend up --build -d

echo "⏳ Waiting for services to be healthy..."

# Wait for database to be healthy
echo "  📊 Waiting for database..."
timeout 60 bash -c 'until docker-compose exec postgres pg_isready -U ${POSTGRES_USER:-finstack_user} -d ${POSTGRES_DB:-finstack_prod}; do sleep 2; done'

# Wait for backend to be healthy
echo "  🦀 Waiting for backend..."
timeout 60 bash -c 'until curl -sf http://localhost:8080/health; do sleep 2; done'

# Wait for frontend to be healthy
echo "  🎨 Waiting for frontend..."
timeout 60 bash -c 'until curl -sf http://localhost:3000; do sleep 2; done'

# Wait for nginx to be healthy
echo "  🌐 Waiting for nginx..."
timeout 60 bash -c 'until curl -sf http://localhost/health; do sleep 2; done'

echo ""
echo "✅ All services are running!"
echo ""
echo "🌐 Access points:"
echo "   • Full Application: http://localhost"
echo "   • Frontend Only: http://localhost:3000"
echo "   • API Only: http://localhost:8080"
echo "   • API Health: http://localhost/api/health"
echo ""
echo "📊 Database access:"
echo "   • Host: localhost"
echo "   • Port: 5432"
echo "   • Database: ${POSTGRES_DB:-finstack_prod}"
echo "   • User: ${POSTGRES_USER:-finstack_user}"
echo ""
echo "🔧 Management commands:"
echo "   • View logs: docker-compose logs -f [service_name]"
echo "   • Stop all: docker-compose --profile with-frontend down"
echo "   • Restart: docker-compose --profile with-frontend restart [service_name]"
echo "" 