use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use string_analyzer_service::routes::string_routes;
use string_analyzer_service::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let config = Config::from_env();
    let port = config.port;
    
    log::info!("Starting server on port {}", port);
    
    HttpServer::new(|| {
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .configure(string_routes::configure)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}