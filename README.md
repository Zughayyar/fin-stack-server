# Financial Management API

A comprehensive financial management RESTful API built with Rust, Actix Web, and Diesel ORM. This API allows users to track personal income and expenses.

## Features

- User account management
- Income tracking with source, amount, date and description
- Expense tracking with item name, amount, date and description
- Full CRUD operations for all resources
- Interactive API documentation with Swagger UI

## Setup

1. Make sure you have Docker and Docker Compose installed

2. Create a `.env` file with the following contents:

```env
DATABASE_URL=postgres://user:passw0rd@localhost:5432/finstack
SERVER_URL=127.0.0.1:8080
RUST_LOG=info
```

3.Start the PostgreSQL database with Docker:

```bash
docker-compose up -d
```

4.Run the API server:

```bash
cargo run
```

5.Access the Swagger UI documentation at:

```bash
http://127.0.0.1:8080/swagger-ui/
```

## API Endpoints

### User Management

- `GET /api/users` - Get all users
- `POST /api/users` - Create a new user
- `GET /api/users/{userId}` - Get a specific user
- `PATCH /api/users/{userId}` - Update a user
- `DELETE /api/users/{userId}` - Delete a user

### Income Tracking

- `GET /api/users/{userId}/income` - Get all income records for a user
- `POST /api/users/{userId}/income` - Create a new income record
- `GET /api/users/{userId}/income/{incomeId}` - Get a specific income record
- `PATCH /api/users/{userId}/income/{incomeId}` - Update an income record
- `DELETE /api/users/{userId}/income/{incomeId}` - Delete an income record

### Expense Tracking

- `GET /api/users/{userId}/expenses` - Get all expense records for a user
- `POST /api/users/{userId}/expenses` - Create a new expense record
- `GET /api/users/{userId}/expenses/{expenseId}` - Get a specific expense record
- `PATCH /api/users/{userId}/expenses/{expenseId}` - Update an expense record
- `DELETE /api/users/{userId}/expenses/{expenseId}` - Delete an expense record

## Models

### User

- `id`: UUID - Unique identifier
- `first_name`: String - User's first name
- `last_name`: String - User's last name
- `email`: String - User's email (unique)
- `password`: String - User's password (hashed)
- `created_at`: Timestamp - When account was created
- `updated_at`: Timestamp - When account was last updated

### Income

- `id`: UUID - Unique identifier
- `user_id`: UUID - Reference to user
- `source`: String - Source of income
- `amount`: Decimal - Amount of income
- `date`: Date - When income was received
- `description`: Optional String - Additional details
- `created_at`: Timestamp - When record was created
- `updated_at`: Timestamp - When record was last updated

### Expense

- `id`: UUID - Unique identifier
- `user_id`: UUID - Reference to user
- `item_name`: String - Name of expense
- `amount`: Decimal - Amount of expense
- `date`: Date - When expense occurred
- `description`: Optional String - Additional details
- `created_at`: Timestamp - When record was created
- `updated_at`: Timestamp - When record was last updated

## Technologies

- **Rust** - Programming language
- **Actix Web** - Web framework
- **Diesel ORM** - Database ORM
- **PostgreSQL** - Database
- **utoipa** - OpenAPI documentation
- **utoipa-swagger-ui** - Interactive API browser
