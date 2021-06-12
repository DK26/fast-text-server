#[macro_use]
extern crate lazy_static;

mod cfglib;
mod services;
mod utils;

use cfglib::*;
use actix_web::{
    HttpServer,
    App
};

lazy_static! {
    static ref CFG: Config = init_cfg();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // TODO: Implement with logger
    println!("Initializing service...");

    println!("Initializing HTTP Listener: {}", CFG.service.listen);

    HttpServer::new(|| {
        App::new()
            .service(services::welcome)
            .service(services::echo)
            .service(services::unescape)
            // .service(services::form_test)
            // .service(services::json_test)
            .service(services::unescape_decode)
    })
    .bind(&CFG.service.listen)?
    .run()
    .await

}