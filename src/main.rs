pub mod handlers;

use crate::handlers::index;
use actix_web::{error, middleware::Logger, App, HttpServer};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
#[display("my error: {name}")]
pub struct MyError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe { std::env::set_var("RUST_LOG", "info"); }
    unsafe { std::env::set_var("RUST_BACKTRACE", "1"); }
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
