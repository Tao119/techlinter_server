// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        password -> Varchar,
        token -> Int8,
        is_admin -> Bool,
    }
}
