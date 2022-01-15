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

fn init_arg_matches() -> ArgMatches {

    let about = format!("{description}\n\n Author: {author}\n Source: {source}", 
        description = "Fast Webhooks is a lightweight, high capacity and reliable remote function server which provides REST API services for processing, modifying, re-encoding and matching on UTF-8 data.",
        author = env!("CARGO_PKG_AUTHORS"),
        source = "https://github.com/DK26/fast-webhooks",
    );
 
    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(about.as_str())
        .arg(
            Arg::new("listen")
                .short('l')
                .long("listen")
                .value_name("INTERFACE IP:PORT")
                .takes_value(true)
                .help("Sets the listening interface for incoming HTTP connections. (Default: 127.0.0.1:8080)")
        )
        .arg(
            Arg::new("server_hostname")
                .short('n')
                .long("server_hostname")
                .value_name("HOSTNAME:PORT")
                .takes_value(true)
                .help("Sets the server hostname. Used by the application router as a hostname for url generation. (Default: localhost:8080)")
        )
        .arg(
            Arg::new("workers")
                .short('w')
                .long("workers")
                .value_name("N")
                .takes_value(true)
                .help("Sets the N number of workers. (Default: Logical CPUs count)")
        )
        .arg(
            Arg::new("backlog")
                .short('b')
                .long("backlog")
                .value_name("N")
                .takes_value(true)
                .help("Sets the maximum N number of pending connections that can be waiting to be served. (Default: 2048)")
        )
        .arg(
            Arg::new("max_connections")
                .short('c')
                .long("max_connections")
                .value_name("N")
                .takes_value(true)
                .help("Sets the maximum per-worker number of N concurrent connections. (Default: 25000)")
        )
        .arg(
            Arg::new("max_connection_rate")
                .short('r')
                .long("max_connection_rate")
                .value_name("N")
                .takes_value(true)
                .help("Sets the maximum N per-worker concurrent connection establish process. (Default: 256)")
        )
        .arg(
            Arg::new("keep_alive")
                .short('k')
                .long("keep_alive")
                .value_name("N")
                .takes_value(true)
                .help("Sets server keep-alive setting in N seconds. (Default: 5)")
        )
        .arg(
            Arg::new("client_timeout")
                .short('t')
                .long("client_timeout")
                .value_name("N")
                .takes_value(true)
                .help("Sets server client timeout in N milliseconds for the first request. To disable timeout set value to 0. (Default: 5000)")
        )
        .arg(
            Arg::new("client_shutdown")
                .short('s')
                .long("client_shutdown")
                .value_name("N")
                .takes_value(true)
                .help("Sets server connection shutdown timeout in N milliseconds. To disable timeout set value to 0. (Default: 5000)")
        )
        .arg(
            Arg::new("shutdown_timeout")
                .short('d')
                .long("shutdown_timeout")
                .value_name("N")
                .takes_value(true)
                .help("Sets the timeout for graceful workers shutdown in N seconds. (Default: 30)")
        )
        .arg(
            Arg::new("alt_encoding")
                .short('a')
                .long("alt_encoding")
                .value_name("ENCODING")
                .takes_value(true)
                .help("Sets the alternative encoding for decoding, in case decoding with the default UTF-8 fails. (Default: UTF-8 [lossy])")
        )
        .arg(
            Arg::new("regex_patterns_limit")
                .long("regex_patterns_limit")
                .value_name("N")
                .takes_value(true)
                .help("Sets the in-memory cached patterns limit. Clears cache after threshold. (Default: 10000)")
        )
        .arg(
            Arg::new("regex_patterns_capacity")
                .long("regex_patterns_capacity")
                .value_name("N")
                .takes_value(true)
                .help("Sets the initial amount of N capacity for cached patterns. (Default: 10000)")
        ).arg(
            Arg::new("log_level")
                .short('L')
                .long("log_level")
                .value_name("LEVEL")
                .takes_value(true)
                .help(r#"Sets the log level for the logger. (Available levels: "OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE")"#)
        )
    .get_matches()
}

lazy_static! {

    static ref CFG: Config = {

        // We set the logger with a default `Trace` level to catch any kind of log
        // before we change the log level to the one from the configurations.
        SimpleLogger::new()
            .with_level(log::LevelFilter::Trace).init().unwrap();

        // First match and check against arguments
        // as this may exit the program with a help menu.
        let arg_matches = init_arg_matches();

        // Then, if there are no problems, continue from here

        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

        log::info!("Initializing service...");
        
        arg_matches.into()

    };

    static ref PATTERNS_CACHE: RwLock<PatternsCache> = {

        let cache = PatternsCache::with_capacity(CFG.cache.regex_patterns_capacity)
            .limit(CFG.cache.regex_patterns_limit); 

        RwLock::new(cache)
    };

}

pub const DEFAULT_CHARSET : &str = "utf-8";

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let log_level =  match CFG.logger.log_level.parse() {
        Ok(level) => level,
        Err(e) => {
            let default_log_level = cfglib::default_logger_level().to_uppercase();
            log::warn!("Log level: {e}. Continuing with log level `{default_log_level}`.");
            default_log_level.parse().unwrap()
        }
    };

    log::set_max_level(log_level);

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

    // Logger
    log::debug!("log_level = {}", CFG.logger.log_level);

    HttpServer::new(|| {
        App::new()
            .service(services::welcome)
            .service(services::echo)
            .service(services::unescape)
            // .service(services::form_test)
            // .service(services::json_test)
            .service(services::unescape_charset)
            .service(services::decode_base64)
            .service(services::decode_base64_charset)
            .service(services::decode_mime_header)
            .service(services::decode_mime_header_rfc822)
            .service(services::decode_quoted_printable)
            .service(services::decode_quoted_printable_charset)
            .service(services::decode_auto)
            .service(services::decode_auto_charset)
            .service(services::regex_capture_group)
            .service(services::regex_to_json)
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