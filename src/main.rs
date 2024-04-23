use std::env;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let port_key = "PORT";
    let default_port = 8080;
    let port = match env::var(port_key) {
        Ok(val) => match val.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                println!(
                    "the port number \"{}\" is invalid. default port will be used.",
                    val
                );
                default_port
            }
        },
        Err(_) => {
            println!(
                "\"{}\" is not defined in environment variables. default port will be used.",
                port_key
            );
            default_port
        }
    };
    println!("launching server...");
    techlinter_server::apis::create_app("0.0.0.0", port).await
}
