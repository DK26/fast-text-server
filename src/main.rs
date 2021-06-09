#[macro_use]
extern crate lazy_static;

mod cfglib;
mod base64;
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

    println!("Initializing HTTP Listener: {}", CFG.base64.listen);

    HttpServer::new(|| {
        App::new()
            .service(base64::welcome)
            .service(base64::echo)
            .service(base64::unescape)
    })
    .bind(&CFG.base64.listen)?
    .run()
    .await

}