use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use crate::models::user::User;
use diesel::result::Error;

use crate::models::income::{Income, NewIncome, UpdateIncome, IncomeWithUser};
use crate::models::schema::{incomes, users};
use crate::database::db_connection::DbConnection;

pub fn get_all_incomes(connection: &mut DbConnection) -> Result<Vec<IncomeWithUser>, Error> {
    incomes::table
        .inner_join(users::table)
        .select((incomes::all_columns, users::all_columns))
        .load::<(Income, User)>(connection)
        .map(|results| {
            results
                .into_iter()
                .map(|(income, user)| IncomeWithUser {
                    income,
                    user,
                })
                .collect()
        })
}

pub fn get_incomes_by_user_id(connection: &mut DbConnection, user_id: Uuid) -> Result<Vec<Income>, diesel::result::Error> {
    incomes::table
        .filter(incomes::user_id.eq(user_id))
        .select(Income::as_select())
        .load(connection)
}

pub fn create_income(connection: &mut DbConnection, new_income: NewIncome) -> Result<Income, diesel::result::Error> {
    let now = Utc::now().naive_utc();
    let income = diesel::insert_into(incomes::table)
        .values((
            incomes::id.eq(Uuid::new_v4()),
            incomes::user_id.eq(new_income.user_id),
            incomes::source.eq(new_income.source),
            incomes::amount.eq(new_income.amount),
            incomes::date.eq(new_income.date),
            incomes::description.eq(new_income.description),
            incomes::created_at.eq(now),
            incomes::updated_at.eq(now),
        ))
        .get_result::<Income>(connection)?;

    Ok(income)
}

pub fn update_income(connection: &mut DbConnection, income_id: Uuid, update_income: UpdateIncome) -> Result<Income, diesel::result::Error> {
    diesel::update(incomes::table)
        .filter(incomes::id.eq(income_id))
        .set(update_income)
        .get_result(connection)
}

pub fn delete_income(connection: &mut DbConnection, income_id: Uuid) -> Result<Income, diesel::result::Error> {
    diesel::delete(incomes::table)
        .filter(incomes::id.eq(income_id))
        .get_result(connection)
}