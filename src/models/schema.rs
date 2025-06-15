// @generated automatically by Diesel CLI.

diesel::table! {
    expenses (id) {
        id -> Uuid,
        user_id -> Uuid,
        item_name -> Varchar,
        amount -> Numeric,
        date -> Date,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    incomes (id) {
        id -> Uuid,
        user_id -> Uuid,
        source -> Varchar,
        amount -> Numeric,
        date -> Date,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(expenses -> users (user_id));
diesel::joinable!(incomes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    expenses,
    incomes,
    users,
);
