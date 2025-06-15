use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

use crate::models::expense::{Expense, NewExpense, UpdateExpense};
use crate::models::schema::expenses;
use crate::database::db_connection::DbConnection;

pub fn get_all_expenses(connection: &mut DbConnection) -> Result<Vec<Expense>, diesel::result::Error> {
    expenses::table
        .select(Expense::as_select())
        .load::<Expense>(connection)
}

pub fn get_expenses_by_user_id(connection: &mut DbConnection, user_id: Uuid) -> Result<Vec<Expense>, diesel::result::Error> {
    expenses::table
        .filter(expenses::user_id.eq(user_id))
        .select(Expense::as_select())
        .load::<Expense>(connection)
}

pub fn create_expense(connection: &mut DbConnection, new_expense: NewExpense) -> Result<Expense, diesel::result::Error> {
    let now = Utc::now().naive_utc();
    let expense = diesel::insert_into(expenses::table)
        .values((
            expenses::id.eq(Uuid::new_v4()),
            expenses::user_id.eq(new_expense.user_id),
            expenses::item_name.eq(new_expense.item_name),
            expenses::amount.eq(new_expense.amount),
            expenses::date.eq(now.date()),
            expenses::description.eq(new_expense.description),
            expenses::created_at.eq(now),
            expenses::updated_at.eq(now),
        ))
        .get_result::<Expense>(connection)?;

    Ok(expense)
}

pub fn update_expense(connection: &mut DbConnection, expense_id: Uuid, mut update_expense: UpdateExpense) -> Result<Expense, diesel::result::Error> {
    connection.transaction(|connection| {
        update_expense.updated_at = Some(Utc::now().naive_utc());
        diesel::update(expenses::table.find(expense_id))
            .set(update_expense)
            .get_result(connection)
    })
}

pub fn delete_expense(connection: &mut DbConnection, expense_id: Uuid) -> Result<Expense, diesel::result::Error> {
    connection.transaction(|connection| {
        diesel::delete(expenses::table.find(expense_id))
            .get_result(connection)
    })
}