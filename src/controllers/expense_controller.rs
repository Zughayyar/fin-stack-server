use actix_web::{web, HttpResponse};
use diesel::PgConnection;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use uuid::Uuid;
use crate::models::expense::{NewExpense, UpdateExpense, Expense};

use crate::config::errors::{AppError, response};
use crate::services::expense_service;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Get all expenses
#[utoipa::path(
    get,
    path = "/api/expenses",
    responses(
        (status = 200, description = "List of expenses", body = Vec<Expense>),
        (status = 500, description = "Internal server error")
    ),
    tag = "expenses"
)]
pub async fn get_all_expenses(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let expenses = expense_service::get_all_expenses(&mut conn)?;
    Ok(response::ok(expenses))
}

/// Get expenses by user ID
#[utoipa::path(
    get,
    path = "/api/expenses/user/{user_id}",
    responses(
        (status = 200, description = "List of expenses for user", body = Vec<Expense>),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    tag = "expenses"
)]
pub async fn get_expenses_by_user_id(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let expenses = expense_service::get_expenses_by_user_id(&mut conn, user_id.into_inner())?;
    Ok(response::ok(expenses))
}

/// Create new expense
#[utoipa::path(
    post,
    path = "/api/expenses",
    request_body = NewExpense,
    responses(
        (status = 201, description = "Expense created successfully", body = Expense),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    tag = "expenses"
)]
pub async fn create_expense(pool: web::Data<DbPool>, new_expense: web::Json<NewExpense>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let expense = expense_service::create_expense(&mut conn, new_expense.into_inner())?;
    Ok(response::created(expense))
}

/// Update expense
#[utoipa::path(
    put,
    path = "/api/expenses/{expense_id}",
    request_body = UpdateExpense,
    responses(
        (status = 200, description = "Expense updated successfully", body = Expense),
        (status = 404, description = "Expense not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("expense_id" = Uuid, Path, description = "Expense ID")
    ),
    tag = "expenses"
)]
pub async fn update_expense(pool: web::Data<DbPool>, expense_id: web::Path<Uuid>, update_expense: web::Json<UpdateExpense>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let expense = expense_service::update_expense(&mut conn, expense_id.into_inner(), update_expense.into_inner())?;
    Ok(response::ok(expense))
}

/// Delete expense
#[utoipa::path(
    delete,
    path = "/api/expenses/{expense_id}",
    responses(
        (status = 200, description = "Expense deleted successfully"),
        (status = 404, description = "Expense not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("expense_id" = Uuid, Path, description = "Expense ID")
    ),
    tag = "expenses"
)]
pub async fn delete_expense(pool: web::Data<DbPool>, expense_id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let expense = expense_service::delete_expense(&mut conn, expense_id.into_inner())?;
    Ok(response::ok(expense))
}
