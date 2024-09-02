use actix_web::{middleware::Logger, App, HttpServer};
use actix_web_lab::web::spa;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(
            spa()
                .index_file("./dist/index.html")
                .static_resources_mount("/static")
                .static_resources_location("./dist")
                .finish(),
        )
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
