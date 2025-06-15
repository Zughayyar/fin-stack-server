use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use crate::models::schema::users;
use crate::models::income::Income;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, ToSchema)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    #[schema(example = "John")]
    pub first_name: String,
    #[schema(example = "Doe")]
    pub last_name: String,
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = "hashed_password_here")]
    pub password: String,
    #[schema(example = "2024-03-20T10:00:00")]
    pub created_at: NaiveDateTime,
    #[schema(example = "2024-03-20T10:00:00")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserWithIncomes {
    #[serde(flatten)]
    pub user: User,
    pub incomes: Vec<Income>,
}

#[derive(Debug, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub id: Uuid,
    #[schema(example = "John")]
    pub first_name: String,
    #[schema(example = "Doe")]
    pub last_name: String,
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = "password123")]
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NewUser {
    pub fn new(first_name: String, last_name: String, email: String, password: String) -> Self {
        let now = chrono::Utc::now().naive_utc();
        NewUser {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            email,
            password,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn into_user(self) -> User {
        User {
            id: self.id,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            password: self.password,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateUser {
    #[schema(example = "John")]
    pub first_name: Option<String>,
    #[schema(example = "Doe")]
    pub last_name: Option<String>,
    #[schema(example = "john@example.com")]
    pub email: Option<String>,
    #[schema(example = "newpassword123")]
    pub password: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}