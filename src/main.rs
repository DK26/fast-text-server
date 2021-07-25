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
                .help("Sets the listening interface for incoming HTTP connections. (Default: 127.0.0.1:8080)")
        )
        .arg(
            Arg::with_name("server_hostname")
                .short("n")
                .long("server_hostname")
                .value_name("HOSTNAME:PORT")
                .takes_value(true)
                .help("Sets the server hostname. Used by the application router as a hostname for url generation. (Default: localhost:8080)")
        )
        .arg(
            Arg::with_name("workers")
                .short("w")
                .long("workers")
                .value_name("N")
                .takes_value(true)
                .help("Sets the N number of workers. (Default: Logical CPUs count)")
        )
        .arg(
            Arg::with_name("backlog")
                .short("b")
                .long("backlog")
                .value_name("N")
                .takes_value(true)
                .help("Sets the maximum N number of pending connections that can be waiting to be served. (Default: 2048)")
        )
        .arg(
            Arg::with_name("max_connections")
                .short("c")
                .long("max_connections")
                .value_name("N")
                .takes_value(true)
                .help("Sets the maximum per-worker number of N concurrent connections. (Default: 25000)")
        )
        .arg(
            Arg::with_name("max_connection_rate")
                .short("r")
                .long("max_connection_rate")
                .value_name("N")
                .takes_value(true)
                .help("Sets the maximum N per-worker concurrent connection establish process. (Default: 256)")
        )
        .arg(
            Arg::with_name("keep_alive")
                .short("k")
                .long("keep_alive")
                .value_name("N")
                .takes_value(true)
                .help("Sets server keep-alive setting in N seconds. (Default: 5)")
        )
        .arg(
            Arg::with_name("client_timeout")
                .short("t")
                .long("client_timeout")
                .value_name("N")
                .takes_value(true)
                .help("Sets server client timeout in N milliseconds for the first request. To disable timeout set value to 0. (Default: 5000)")
        )
        .arg(
            Arg::with_name("client_shutdown")
                .short("s")
                .long("client_shutdown")
                .value_name("N")
                .takes_value(true)
                .help("Sets server connection shutdown timeout in N milliseconds. To disable timeout set value to 0. (Default: 5000)")
        )
        .arg(
            Arg::with_name("shutdown_timeout")
                .short("d")
                .long("shutdown_timeout")
                .value_name("N")
                .takes_value(true)
                .help("Sets the timeout for graceful workers shutdown in N seconds. (Default: 30)")
        )
        .arg(
            Arg::with_name("alt_encoding")
                .short("a")
                .long("alt_encoding")
                .value_name("ENCODING")
                .takes_value(true)
                .help("Sets the alternative encoding for decoding, in case decoding with the default UTF-8 fails. (Default: UTF-8)")
        )
        .arg(
            Arg::with_name("regex_patterns_limit")
                .long("regex_patterns_limit")
                .value_name("N")
                .takes_value(true)
                .help("Sets the in-memory cached patterns limit. Clears cache after threshold. (Default: 10000)")
        )
        .arg(
            Arg::with_name("regex_patterns_capacity")
                .long("regex_patterns_capacity")
                .value_name("N")
                .takes_value(true)
                .help("Sets the initial amount of N capacity for cached patterns. (Default: 10000)")
        )
    .get_matches()
}

lazy_static! {

    static ref CFG: Config = cfglib::init_cfg(arg_matches());

    static ref PATTERNS_CACHE: RwLock<PatternsCache> = {

        let cache = PatternsCache::with_capacity(CFG.cache.regex_patterns_capacity)
            .limit(CFG.cache.regex_patterns_limit); 

        RwLock::new(cache)
    };

}

pub const DEFAULT_ENCODING : &'static str = "utf-8";

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    SimpleLogger::new()
    .with_level(log::LevelFilter::Debug)
    .init().unwrap();

    log::info!("Initializing service...");

    // Configurations
    // Service
    log::debug!("listen = {}", CFG.service.listen);
    log::debug!("server_hostname = {}", CFG.service.server_hostname);
    log::debug!("workers = {}", CFG.service.workers);
    log::debug!("backlog = {}", CFG.service.backlog);
    log::debug!("max_connections = {}", CFG.service.max_connections);
    log::debug!("max_connection_rate = {}", CFG.service.max_connection_rate);
    log::debug!("keep_alive = {}", CFG.service.keep_alive);
    log::debug!("client_timeout = {}", CFG.service.client_timeout);
    log::debug!("client_shutdown = {}", CFG.service.client_shutdown);
    log::debug!("shutdown_timeout = {}", CFG.service.shutdown_timeout);

    // Common
    log::debug!("alt_encoding = {}", CFG.common.alt_encoding);

    // Cache
    log::debug!("regex_patterns_capacity = {}", CFG.cache.regex_patterns_capacity);
    log::debug!("regex_patterns_limit = {}", CFG.cache.regex_patterns_limit);

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
            .service(services::decode_mime_subject_rfc822)
            // .service(services::try_decode_mime_subject)
            .service(services::regex_capture_group)
    })
    .server_hostname(&CFG.service.server_hostname)
    .workers(CFG.service.workers)
    .backlog(CFG.service.backlog)
    .max_connections(CFG.service.max_connections)
    .max_connection_rate(CFG.service.max_connection_rate)
    .keep_alive(CFG.service.keep_alive)
    .client_timeout(CFG.service.client_timeout)
    .client_shutdown(CFG.service.client_shutdown)
    .shutdown_timeout(CFG.service.shutdown_timeout)
    .bind(&CFG.service.listen)?
    .run()
    .await
}