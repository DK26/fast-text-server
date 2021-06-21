#[macro_use]
extern crate lazy_static;

mod cfglib;
mod services;
mod utils;
use parking_lot::RwLock;

use cfglib::*;
use utils::PatternsCache;
use actix_web::{
    HttpServer,
    App
};
use simple_logger::SimpleLogger;
use clap::{ArgMatches, Arg};

fn arg_matches<'a>() -> ArgMatches<'a> {

    let about = format!("Fast, lightweight RESTful API services for processing & modifying UTF-8 text messages.
    \nAuthor: {}\nSource: https://github.com/DK26/fast-webhooks", env!("CARGO_PKG_AUTHORS"));
 
    clap::App::new("Fast-Webhooks")
        .version(env!("CARGO_PKG_VERSION"))
        .about(about.as_str())
        .arg(
            Arg::with_name("listen")
                .short("l")
                .long("listen")
                .value_name("INTERFACE IP:PORT")
                .takes_value(true)
                .help("Specifies the listening interface for incoming HTTP connections.")
        )
    .get_matches()
}

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

    let matches = arg_matches();

    SimpleLogger::new()
    .with_level(log::LevelFilter::Info)
    .init().unwrap();

    log::info!("Initializing service...");
    
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
    .server_hostname(&CFG.service.server_hostname)
    .workers(CFG.service.workers)
    .backlog(CFG.service.backlog)
    .max_connections(CFG.service.max_connections)
    .keep_alive(CFG.service.keep_alive)
    .client_timeout(CFG.service.client_timeout)
    .client_shutdown(CFG.service.client_shutdown)
    .shutdown_timeout(CFG.service.shutdown_timeout)
    .run()
    .await
}