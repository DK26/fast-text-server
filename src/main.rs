#[macro_use]
extern crate lazy_static;

mod cfglib;
mod services;
mod utils;
use parking_lot::RwLock;
use log::info;

use cfglib::*;
use utils::PatternsCache;
use actix_web::{
    HttpServer,
    App
};

lazy_static! {

    static ref CFG: Config = init_cfg();

    static ref PATTERNS_CACHE: RwLock<PatternsCache> = {

        let cache = PatternsCache::with_capacity(CFG.cache.regex_patterns_capacity)
            .limit(CFG.cache.regex_patterns_limit); 

        RwLock::new(cache)
    };

}

pub const DEFAULT_ENCODING : &'static str = "utf-8";

// TODO: Implement `clap` arguments for configurations. Passed arguments override `cfg.toml` configurations.

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // TODO: Implement with logger
    // println!("Initializing service...");
    log::info!("Initializing service...");
    
    // println!("Initializing HTTP Listener: {}", CFG.service.listen);
    log::info!("Initializing HTTP Listener: {}", CFG.service.listen);

    HttpServer::new(|| {
        App::new()
            .service(services::welcome)
            .service(services::echo)
            .service(services::unescape)
            // .service(services::form_test)
            // .service(services::json_test)
            .service(services::unescape_decode)
            .service(services::decode_base64)
            .service(services::decode_base64_encoding)
            .service(services::decode_mime_subject)
            .service(services::regex_capture_group)
    })
    .bind(&CFG.service.listen)?
    .run()
    .await
}