use crate::db_connector::{self};
use crate::models::Users;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::result::Error;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

pub async fn create_app(addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_user_page)
            .service(signup)
            .service(login)
            .service(analyze)
            .service(fixed)
    })
    .bind((addr, port))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    let conn = db_connector::create_connection();

    let users = match db_connector::get_users(&conn).await {
        // Assuming get_users is async
        Ok(users) if !users.is_empty() => users,
        Ok(_) => return HttpResponse::NotFound().body("No users found."),
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Database error: {}", e))
        }
    };

    // Using user data to construct a response
    let name = &users[0].name; // Safely assuming there is at least one user due to the check above
    HttpResponse::Ok().body(format!(
        "
    <div>Title</div>
    Hello, {}
    ",
        name
    ))
}

#[get("/users/{ur_name}")]
async fn get_user_page(ur_name: web::Path<String>) -> impl Responder {
    let conn = db_connector::create_connection();
    let path = ur_name.to_string();
    let user = match db_connector::get_user_by_name(&conn, &path) {
        Ok(result) => result,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    match render_user_page(user) {
        Ok(contents) => HttpResponse::Ok().content_type("text/html").body(contents),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
struct SignInInput {
    name: String,
    password: String,
}

#[post("/signup")]
async fn signup(info: web::Json<SignInInput>) -> impl Responder {
    let input = info.into_inner();
    let ur_name = input.name;
    let password = input.password;
    let conn = db_connector::create_connection();

    let hashed_password = match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
        Ok(hp) => hp,
        Err(_) => return HttpResponse::InternalServerError().body("Error hashing password"),
    };

    match db_connector::insert_user(&conn, &ur_name, &hashed_password) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(Error::NotFound) => HttpResponse::BadRequest().body("Username already exists"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/login")]
async fn login(info: web::Json<SignInInput>) -> impl Responder {
    let input = info.into_inner();
    let ur_name = input.name;
    let password = input.password;
    let conn = db_connector::create_connection();

    match db_connector::get_user_by_name(&conn, &ur_name) {
        Ok(user) => {
            if let Ok(matches) = bcrypt::verify(password, &user.password) {
                if matches {
                    HttpResponse::Ok().json(user)
                } else {
                    HttpResponse::BadRequest().body("Invalid password")
                }
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
        Err(_) => HttpResponse::BadRequest().body("User not found"),
    }
}

fn render_user_page(user: Users) -> Result<String, tera::Error> {
    let tmpl = Tera::new("templates/**/*").unwrap();
    let mut ctx = Context::new();
    ctx.insert("user", &user.name);

    tmpl.render("user.html", &ctx)
}

#[derive(Deserialize)]
struct Input {
    prompt: String,
    ur_id: i64,
}

#[derive(Serialize)]
struct GptResponse {
    response: serde_json::Value,
}

#[post("/analyze")]
async fn analyze(input: web::Json<Input>) -> impl Responder {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = reqwest::Client::new();
    let conn = db_connector::create_connection();

    if let Err(e) = db_connector::decrement_user_token(&conn, input.ur_id) {
        return HttpResponse::InternalServerError()
            .body(format!("Error updating user token: {}", e));
    }

    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-4",
            "messages":
            [
                {"role": "user", "content": input.prompt}
            ],
            "n":1,
            "stop":null,
            "temperature":0,
        }))
        .send()
        .await;

    match response {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(body) => {
                if let Err(e) = db_connector::insert_gpt_log(
                    &conn,
                    input.ur_id,
                    &input.prompt,
                    &body.to_string(),
                ) {
                    HttpResponse::InternalServerError()
                        .body(format!("Failed to log GPT response: {}", e))
                } else {
                    HttpResponse::Ok().json(GptResponse { response: body })
                }
            }
            Err(_) => HttpResponse::InternalServerError().json("Failed to parse response"),
        },
        Err(_) => HttpResponse::InternalServerError().json("Failed to contact API"),
    }
}

#[derive(Serialize, Deserialize)]
struct Issue {
    severity: String,
    message: String,
    line: u32,
    end_line: u32,
    column: u32,
    end_column: u32,
}

#[derive(Serialize, Deserialize)]
struct Response {
    response: Vec<Issue>,
}

#[post("/fix")]
async fn fixed() -> impl Responder {
    let issues = vec![Issue {
        severity: "Warning".to_string(),
        message: "テストです".to_string(),
        line: 1,
        end_line: 2,
        column: 1,
        end_column: 10,
    }];

    let message = Response { response: issues };

    HttpResponse::Ok().json(message)
}
