use serde::{Deserialize, Serialize};

use super::schema::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Users {
    pub id: i64,
    pub name: String,
    pub password: String,
    pub token: i64,
    pub is_admin: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub password: &'a str,
    pub token: i64,
    pub is_admin: bool,
}
