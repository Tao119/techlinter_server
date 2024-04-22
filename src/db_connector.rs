use super::models::*;
use super::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

// transaction
pub fn create_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// insert
pub fn insert_user(conn: &PgConnection, name: &str, password: &str) -> Result<Users, Error> {
    let new_user = NewUser {
        name,
        password,
        token: 0, // Assuming a default token value of 0
        is_admin: false,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

// get
pub fn get_user_by_name(conn: &PgConnection, ur_name: &str) -> Result<Users, Error> {
    users::table.filter(users::name.eq(ur_name)).first(conn)
}

pub async fn get_users(conn: &PgConnection) -> Result<Vec<Users>, Error> {
    users::table.load::<Users>(conn)
}

// test transaction
pub fn test_transaction<F, R>(test_fn: F) -> Result<R, Error>
where
    F: FnOnce(&PgConnection) -> Result<R, Error>,
{
    let connection = create_connection();
    Ok(connection.test_transaction(|| test_fn(&connection)))
}

// #[cfg(test)]
// #[allow(non_snake_case)]
// mod unit_DBテスト {
//     use super::*;

//     #[test]
//     fn get_user_by_id関数() {
//         test_transaction(|conn| {
//             insert_user(conn, "testUser", "12345678")?;

//             let users = get_user_by_name(conn, "testUser")?;

//             let user = &users;
//             assert_eq!(user.name, "testUser");
//             assert_eq!(user.password, "12345678");
//             assert_eq!(user.token, 0);

//             Ok(())
//         })
//         .unwrap();
//     }
// }
