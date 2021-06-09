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

    println!("Initializing service...");

    // let cfg = init_cfg();

    println!("Initializing HTTP Listener: {}", CFG.service.listen);

    HttpServer::new(|| {
        App::new()
            .service(services::welcome)
            .service(services::echo)
            .service(services::unescape)
    })
    .bind(&CFG.service.listen)?
    .run()
    .await

}