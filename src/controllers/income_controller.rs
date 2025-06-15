use actix_web::{web, HttpResponse};
use diesel::PgConnection;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use uuid::Uuid;
use crate::models::income::{NewIncome, UpdateIncome, Income, IncomeWithUser};

use crate::config::errors::{AppError, response};
use crate::services::income_service;


type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Get all incomes
#[utoipa::path(
    get,
    path = "/api/incomes",
    responses(
        (status = 200, description = "List of incomes", body = Vec<IncomeWithUser>),
        (status = 500, description = "Internal server error")
    ),
    tag = "incomes"
)]
pub async fn get_all_incomes(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let incomes = income_service::get_all_incomes(&mut conn)?;
    Ok(response::ok(incomes))
}

/// Get incomes by user ID
#[utoipa::path(
    get,
    path = "/api/incomes/user/{user_id}",
    responses(
        (status = 200, description = "List of incomes for user", body = Vec<Income>),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    tag = "incomes"
)]
pub async fn get_incomes_by_user_id(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let incomes = income_service::get_incomes_by_user_id(&mut conn, user_id.into_inner())?;
    Ok(response::ok(incomes))
}

/// Create new income
#[utoipa::path(
    post,
    path = "/api/incomes",
    request_body = NewIncome,
    responses(
        (status = 201, description = "Income created successfully", body = Income),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    tag = "incomes"
)]
pub async fn create_income(pool: web::Data<DbPool>, new_income: web::Json<NewIncome>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let income = income_service::create_income(&mut conn, new_income.into_inner())?;
    Ok(response::created(income))
}

/// Update income
#[utoipa::path(
    put,
    path = "/api/incomes/{income_id}",
    request_body = UpdateIncome,
    responses(
        (status = 200, description = "Income updated successfully", body = Income),
        (status = 404, description = "Income not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("income_id" = Uuid, Path, description = "Income ID")
    ),
    tag = "incomes"
)]
pub async fn update_income(pool: web::Data<DbPool>, income_id: web::Path<Uuid>, update_income: web::Json<UpdateIncome>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let income = income_service::update_income(&mut conn, income_id.into_inner(), update_income.into_inner())?;
    Ok(response::ok(income))
}

/// Delete income
#[utoipa::path(
    delete,
    path = "/api/incomes/{income_id}",
    responses(
        (status = 200, description = "Income deleted successfully"),
        (status = 404, description = "Income not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("income_id" = Uuid, Path, description = "Income ID")
    ),
    tag = "incomes"
)]
pub async fn delete_income(pool: web::Data<DbPool>, income_id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    let income = income_service::delete_income(&mut conn, income_id.into_inner())?;
    Ok(response::ok(income))
}