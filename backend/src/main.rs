mod api;

use api::test::test;

use actix_web::{middleware::Logger, web::scope, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(scope("/api").service(test))
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
