// @generated automatically by Diesel CLI.

diesel::table! {
    gpt_logs (id) {
        id -> Int8,
        ur_id -> Int8,
        code -> Varchar,
        output -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        password -> Varchar,
        token -> Int8,
        is_admin -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    gpt_logs,
    users,
);
