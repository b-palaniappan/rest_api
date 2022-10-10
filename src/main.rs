use actix_web::{get, post, middleware, web, Result, HttpResponse, HttpRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    user_id: u32,
    name: String,
}

/// extract path info using serde
#[get("/users/{user_id}/{name}")] // <- define path parameters
async fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.name, info.user_id
    ))
}

#[get("/users")]
async fn get_users() -> HttpResponse {
    let user = Info{ user_id: 1, name: String::from("John") };
    HttpResponse::Ok().json(&user)
}

#[post("/users")]
async fn add_users(item: web::Json<Info>, req: HttpRequest) -> HttpResponse {
    println!("request: {req:?}");
    println!("model: {item:?}");
    HttpResponse::Ok().json(&item)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new()
    .wrap(middleware::Logger::default())            // add logger
    .wrap(middleware::Compress::default())          // add default compressor
    .wrap(middleware::DefaultHeaders::new().add(("X-Server", "raptor-v1.2")))  //  
    .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
    .service(index)
    .service(get_users)
    .service(add_users))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
