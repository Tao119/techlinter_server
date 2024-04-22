use crate::db_connector;
use crate::models::Users;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tera::{Context, Tera};

pub async fn create_app(addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_user_page)
            .service(add_user)
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

#[post("/users/add/{ur_name}/{password}")]
async fn add_user(path: web::Path<(String, String)>) -> impl Responder {
    let (ur_name, password) = path.into_inner();
    let conn = db_connector::create_connection();

    match db_connector::insert_user(&conn, &ur_name, &password) {
        Ok(user) => HttpResponse::Ok().json(user.name),
        Err(_) => HttpResponse::InternalServerError().finish(),
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
}

#[derive(Serialize)]
struct GptResponse {
    response: serde_json::Value,
}

#[post("/analyze")]
async fn analyze(input: web::Json<Input>) -> impl Responder {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "text-davinci-003",
            "messages": {"role": "user", "content": input.prompt},
            "temperature": 0
        }))
        .send()
        .await;

    match response {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(body) => HttpResponse::Ok().json(GptResponse { response: body }),
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
