use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use utoipa::ToSchema;
use crate::models::schema::expenses;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, ToSchema)]
#[diesel(table_name = expenses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Expense {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
    #[schema(example = "Groceries")]
    pub item_name: String,
    #[schema(example = "50.00")]
    pub amount: Decimal,
    #[schema(example = "2024-03-20")]
    pub date: chrono::NaiveDate,
    #[schema(example = "Weekly groceries")]
    pub description: Option<String>,
    #[schema(example = "2024-03-20T10:00:00")]
    pub created_at: NaiveDateTime,
    #[schema(example = "2024-03-20T10:00:00")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = expenses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewExpense {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: Uuid,
    #[schema(example = "Groceries")]
    pub item_name: String,
    #[schema(example = "50.00")]
    pub amount: Decimal,
    #[schema(example = "Weekly groceries")]
    pub description: Option<String>,
}

impl NewExpense {
    pub fn into_expense(self) -> Expense {
        let now = chrono::Utc::now().naive_utc();
        Expense {
            id: Uuid::new_v4(),
            user_id: self.user_id,
            item_name: self.item_name,
            amount: self.amount,
            date: now.date(),
            description: self.description,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = expenses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateExpense {
    #[schema(example = "Restaurant")]
    pub item_name: Option<String>,
    #[schema(example = "75.00")]
    pub amount: Option<Decimal>,
    #[schema(example = "2024-03-20")]
    pub date: Option<chrono::NaiveDate>,
    #[schema(example = "Dinner with friends")]
    pub description: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
} 