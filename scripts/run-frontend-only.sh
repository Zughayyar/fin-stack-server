#!/bin/bash

# Script to run only the frontend service for development
# Useful when you're developing the frontend and have the API running separately

set -e

echo "🎨 Starting FinStack Frontend Only..."

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

echo "🔨 Building and starting frontend service..."

# Start only the frontend service
docker-compose --profile with-frontend up --build -d frontend

echo "⏳ Waiting for frontend to be healthy..."

# Wait for frontend to be healthy
timeout 60 bash -c 'until curl -sf http://localhost:3000/health; do sleep 2; done'

echo ""
echo "✅ Frontend is running!"
echo ""
echo "🌐 Access points:"
echo "   • Frontend: http://localhost:3000"
echo ""
echo "📊 To view logs:"
echo "   docker-compose logs -f frontend"
echo ""
echo "🛑 To stop frontend:"
echo "   docker-compose --profile with-frontend down"
echo ""
echo "📋 Running containers:"
docker-compose ps 