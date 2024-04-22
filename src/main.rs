#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    println!("launching server...");
    techlinter_server::apis::create_app("0.0.0.0", 8080).await
}
