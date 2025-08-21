use actix_web::{HttpServer, App, web, middleware::Logger, HttpResponse};
use crate::middleware::auth::Authentication;
use crate::utils::Response;

mod router;
mod database;
mod utils;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let port = std::env::var("PORT").unwrap();
    let ip = std::env::var("IP").unwrap();
    println!("Server started on {}:{}", ip, port);

    let db = database::init_database().await.expect("TODO: panic message");
    middleware::logger::init_logger();
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Authentication)
            .app_data(web::Data::new(db.clone()))
            .service(router::user::create_user_service())

            .route("/*", web::route().to(async || Response::<()>::not_found("not found".to_string())))
    })
    .bind(format!("{}:{}", ip, port))?
    .run()
    .await
}
