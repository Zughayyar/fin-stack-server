# 🦀 FinStack API

> A high-performance financial management RESTful API built with Rust, Actix Web, and PostgreSQL

[![Rust](https://img.shields.io/badge/Rust-1.70+-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Actix Web](https://img.shields.io/badge/Actix%20Web-4.0+-8B5A3C?style=for-the-badge&logo=actix&logoColor=white)](https://actix.rs/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-15+-316192?style=for-the-badge&logo=postgresql&logoColor=white)](https://www.postgresql.org/)
[![Docker](https://img.shields.io/badge/Docker-Ready-2496ED?style=for-the-badge&logo=docker&logoColor=white)](https://www.docker.com/)

## 🎯 Overview

FinStack API is a robust, secure, and lightning-fast backend service for personal finance management. Built with Rust's memory safety and performance guarantees, it provides a comprehensive set of endpoints for user management, income tracking, and expense monitoring.

## ✨ Key Features

- **🚀 High Performance** - Built with Rust for maximum speed and efficiency
- **🔒 Memory Safe** - Zero-cost abstractions with compile-time safety
- **📊 Complete CRUD Operations** - Full resource management for users, income, and expenses
- **🗄️ PostgreSQL Integration** - Robust database operations with Diesel ORM
- **📚 Interactive Documentation** - Swagger UI with live API testing
- **🔐 Secure Architecture** - Input validation and error handling
- **⚡ Async Processing** - Non-blocking I/O with Tokio runtime

## 🛠️ Tech Stack

| Technology | Purpose | Version |
|------------|---------|---------|
| **Rust** | Systems programming language | 1.70+ |
| **Actix Web** | High-performance web framework | 4.0+ |
| **Diesel ORM** | Safe, extensible ORM and query builder | 2.0+ |
| **PostgreSQL** | Advanced relational database | 15+ |
| **utoipa** | OpenAPI documentation generator | Latest |
| **Tokio** | Asynchronous runtime | Latest |

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ with Cargo
- Docker & Docker Compose
- PostgreSQL (or use Docker setup)

### Development Setup

```bash
# Clone and navigate to project
git clone <repository-url>
cd finstack-api

# Set up environment
cp .env.example .env
# Edit .env with your database configuration

# Start PostgreSQL with Docker
docker-compose up -d

# Install dependencies and run
cargo run

# Access API documentation
open http://127.0.0.1:8080/swagger-ui/
```

### Environment Configuration

```env
DATABASE_URL=postgres://user:password@localhost:5432/finstack
SERVER_URL=127.0.0.1:8080
RUST_LOG=info
JWT_SECRET=your-secret-key
```

## 📊 API Architecture

### Core Endpoints

| Resource | Endpoints | Description |
|----------|-----------|-------------|
| **Users** | `/api/users/*` | Complete user lifecycle management |
| **Income** | `/api/users/{id}/income/*` | Income tracking and analytics |
| **Expenses** | `/api/users/{id}/expenses/*` | Expense monitoring and categorization |
| **Auth** | `/api/auth/*` | Authentication and authorization |

### Authentication Flow

```text
Client → Login → JWT Token → Authorized Requests → Protected Resources
```

## 🗃️ Data Models

### User Entity

```rust
pub struct User {
    pub id: Uuid,           // Unique identifier
    pub first_name: String, // User's first name
    pub last_name: String,  // User's last name  
    pub email: String,      // Unique email address
    pub password: String,   // Bcrypt hashed password
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Income Entity

```rust
pub struct Income {
    pub id: Uuid,                    // Unique identifier
    pub user_id: Uuid,              // Foreign key to user
    pub source: String,             // Income source description
    pub amount: BigDecimal,         // Precise monetary amount
    pub date: NaiveDate,           // Income date
    pub description: Option<String>, // Optional details
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Expense Entity

```rust
pub struct Expense {
    pub id: Uuid,                    // Unique identifier
    pub user_id: Uuid,              // Foreign key to user
    pub item_name: String,          // Expense description
    pub amount: BigDecimal,         // Precise monetary amount
    pub date: NaiveDate,           // Expense date
    pub description: Option<String>, // Optional details
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## 🔧 API Endpoints

### User Management

```http
GET    /api/users              # List all users
POST   /api/users              # Create new user
GET    /api/users/{id}          # Get user by ID
PATCH  /api/users/{id}          # Update user
DELETE /api/users/{id}          # Delete user
```

### Income Operations

```http
GET    /api/users/{id}/income              # Get user's income records
POST   /api/users/{id}/income              # Create income record
GET    /api/users/{id}/income/{income_id}  # Get specific income
PATCH  /api/users/{id}/income/{income_id}  # Update income
DELETE /api/users/{id}/income/{income_id}  # Delete income
```

### Expense Operations

```http
GET    /api/users/{id}/expenses                # Get user's expenses
POST   /api/users/{id}/expenses                # Create expense
GET    /api/users/{id}/expenses/{expense_id}   # Get specific expense
PATCH  /api/users/{id}/expenses/{expense_id}   # Update expense
DELETE /api/users/{id}/expenses/{expense_id}   # Delete expense
```

## 🏗️ Architecture Highlights

- **Clean Architecture** - Separation of concerns with modular design
- **Error Handling** - Comprehensive error types and responses
- **Input Validation** - Request validation with detailed feedback
- **Database Migrations** - Version-controlled schema management
- **Logging** - Structured logging with configurable levels
- **Health Checks** - Application monitoring endpoints

## 📈 Performance Benefits

- **Memory Efficiency** - Zero-cost abstractions
- **Concurrency** - Async/await with Tokio runtime  
- **Type Safety** - Compile-time error prevention
- **Database Performance** - Optimized queries with Diesel
- **Minimal Runtime** - Small binary footprint

## 🧪 Testing & Development

```bash
# Run tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt

# Build for production
cargo build --release
```

## 📚 Documentation

- **Live API Docs**: `/swagger-ui/` - Interactive API browser
- **Health Check**: `/health` - Service status endpoint
- **OpenAPI Spec**: Auto-generated from code annotations

## 🐳 Docker Support

```bash
# Build Docker image
docker build -t finstack-api .

# Run with Docker Compose
docker-compose up -d
```

---

### Powered by 🦀 Rust for blazing fast, memory-safe financial data processing
