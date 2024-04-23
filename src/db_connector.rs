use super::models::*;
use super::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error;
use dotenv::dotenv;

// transaction
pub fn create_connection() -> PgConnection {
    dotenv().ok();

    let database_url = "postgres://default:V9qM8aSwlvDm@ep-restless-thunder-a4vfyo6j-pooler.us-east-1.aws.neon.tech:5432/verceldb?sslmode=require";
    PgConnection::establish(database_url)
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

pub fn decrement_user_token(conn: &PgConnection, user_id: i64) -> Result<Users, Error> {
    conn.transaction::<_, Error, _>(|| {
        let user = users::table
            .find(user_id)
            .for_update()
            .first::<Users>(conn)?;
        if user.token > 0 {
            let updated_user = diesel::update(users::table.find(user_id))
                .set(users::token.eq(user.token - 1))
                .get_result::<Users>(conn)?;
            Ok(updated_user)
        } else {
            Err(Error::DatabaseError(
                DatabaseErrorKind::__Unknown,
                Box::new("no token left".to_string()),
            ))
        }
    })
}
