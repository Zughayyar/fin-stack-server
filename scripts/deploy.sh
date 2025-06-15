#!/bin/bash

# FinStack API Deployment Script
# Usage: ./scripts/deploy.sh [environment] [options]
# Environments: dev, staging, prod
# Options: --rebuild, --clean, --logs

set -e

# Configuration
PROJECT_NAME="finstack_app"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Help function
show_help() {
    echo "FinStack API Deployment Script"
    echo ""
    echo "Usage: $0 [environment] [options]"
    echo ""
    echo "Environments:"
    echo "  dev      - Development environment (default)"
    echo "  staging  - Staging environment"
    echo "  prod     - Production environment"
    echo ""
    echo "Options:"
    echo "  --rebuild    - Force rebuild of images"
    echo "  --clean      - Clean up containers and volumes"
    echo "  --logs       - Show logs after deployment"
    echo "  --help       - Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 dev --rebuild"
    echo "  $0 prod --clean --logs"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed or not in PATH"
        exit 1
    fi
    
    if ! command -v docker-compose &> /dev/null; then
        log_error "Docker Compose is not installed or not in PATH"
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

# Load environment configuration
load_env_config() {
    local env=$1
    local env_file="$PROJECT_DIR/.env.$env"
    
    if [[ -f "$env_file" ]]; then
        log_info "Loading environment configuration from $env_file"
        export $(grep -v '^#' "$env_file" | xargs)
    else
        log_warn "Environment file $env_file not found, using defaults"
        # Set default values
        export POSTGRES_USER="user"
        export POSTGRES_PASSWORD="passw0rd"
        export POSTGRES_DB="finstack"
        export RUST_LOG="info"
    fi
}

# Deploy function
deploy() {
    local environment=$1
    local rebuild=$2
    local clean=$3
    local show_logs=$4
    
    cd "$PROJECT_DIR"
    
    log_info "Deploying FinStack API to $environment environment..."
    
    # Load environment configuration
    load_env_config "$environment"
    
    # Clean up if requested
    if [[ "$clean" == "true" ]]; then
        log_info "Cleaning up existing containers and volumes..."
        docker-compose --project-name "$PROJECT_NAME" down -v --remove-orphans || true
        docker system prune -f || true
    fi
    
    # Build and start services
    local compose_flags=""
    if [[ "$rebuild" == "true" ]]; then
        compose_flags="$compose_flags --build"
    fi
    
    case "$environment" in
        "dev")
            log_info "Starting development environment..."
            docker-compose --project-name "$PROJECT_NAME" up -d postgres
            ;;
        "staging"|"prod")
            log_info "Starting $environment environment with full stack..."
            docker-compose --project-name "$PROJECT_NAME" \
                --profile with-server \
                up -d $compose_flags
            ;;
        *)
            log_error "Unknown environment: $environment"
            exit 1
            ;;
    esac
    
    # Wait for services to be ready
    log_info "Waiting for services to be ready..."
    sleep 10
    
    # Check service health
    check_services_health
    
    log_success "Deployment completed successfully!"
    
    # Show logs if requested
    if [[ "$show_logs" == "true" ]]; then
        log_info "Showing service logs..."
        docker-compose --project-name "$PROJECT_NAME" logs -f
    fi
}

# Check service health
check_services_health() {
    log_info "Checking service health..."
    
    # Check PostgreSQL
    if docker-compose --project-name "$PROJECT_NAME" ps postgres | grep -q "Up"; then
        log_success "PostgreSQL is running"
    else
        log_error "PostgreSQL is not running"
        return 1
    fi
    
    # Check API server (if running)
    if docker-compose --project-name "$PROJECT_NAME" ps server | grep -q "Up"; then
        log_info "Checking API server health..."
        for i in {1..30}; do
            if curl -f http://localhost:8080/health &>/dev/null; then
                log_success "API server is healthy"
                return 0
            fi
            echo -n "."
            sleep 2
        done
        log_error "API server health check failed"
        return 1
    fi
}

# Main script
main() {
    local environment="dev"
    local rebuild="false"
    local clean="false"
    local show_logs="false"
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            dev|staging|prod)
                environment="$1"
                shift
                ;;
            --rebuild)
                rebuild="true"
                shift
                ;;
            --clean)
                clean="true"
                shift
                ;;
            --logs)
                show_logs="true"
                shift
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    log_info "Starting deployment with environment: $environment"
    
    check_prerequisites
    deploy "$environment" "$rebuild" "$clean" "$show_logs"
}

# Run main function with all arguments
main "$@" 