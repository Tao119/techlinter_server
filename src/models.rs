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

#[derive(Queryable, Serialize, Deserialize)]
pub struct GRPLogs {
    pub id: i64,
    pub ur_id: i64,
    pub code: String,
    pub output: String,
}

#[derive(Insertable)]
#[table_name = "gpt_logs"]
pub struct NewLog<'a> {
    pub ur_id: i64,
    pub code: &'a str,
    pub output: &'a str,
}
