mod cfglib;
mod base64;
mod utils;

use cfglib::*;
use actix_web::{
    HttpServer,
    App
};

// fn main() {
//     base64::test_hello();
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Initializing service...");

    let cfg = init_cfg();

    println!("Initializing HTTP Listener: {}", cfg.base64.listen);

    HttpServer::new(|| {
        App::new()
            .service(base64::welcome)
            .service(base64::echo)
    })
    .bind(cfg.base64.listen)?
    .run()
    .await

}