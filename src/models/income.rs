use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::schema::incomes;
use diesel::{Queryable, Selectable, Insertable, AsChangeset};
use crate::models::user::User;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, ToSchema)]
#[diesel(table_name = incomes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Income {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
    #[schema(example = "Salary")]
    pub source: String,
    #[schema(example = "5000.00")]
    #[serde(with = "rust_decimal::serde::float")]
    pub amount: Decimal,
    #[schema(example = "2024-03-20")]
    pub date: NaiveDate,
    #[schema(example = "Monthly salary")]
    pub description: Option<String>,
    #[schema(example = "2024-03-20T10:00:00")]
    pub created_at: NaiveDateTime,
    #[schema(example = "2024-03-20T10:00:00")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IncomeWithUser {
    #[serde(flatten)]
    pub income: Income,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = incomes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewIncome {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
    #[schema(example = "Salary")]
    pub source: String,
    #[schema(example = "5000.00")]
    #[serde(with = "rust_decimal::serde::float")]
    #[diesel(sql_type = diesel::sql_types::Numeric)]
    pub amount: Decimal,
    #[schema(example = "2024-03-20")]
    pub date: NaiveDate,
    #[schema(example = "Monthly salary")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = incomes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateIncome {
    #[schema(example = "Freelance")]
    pub source: Option<String>,
    #[schema(example = "1000.00")]
    pub amount: Option<Decimal>,
    #[schema(example = "2024-03-20")]
    pub date: Option<NaiveDate>,
    #[schema(example = "Project payment")]
    pub description: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}