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

// TODO: Create a const for each configuration default value, to make it consistent among all references.

fn arg_matches<'a>() -> ArgMatches<'a> {

    let about = format!("Fast, lightweight RESTful API services for processing & modifying UTF-8 text messages.
    \nAuthor: {}\nSource: https://github.com/DK26/fast-webhooks", env!("CARGO_PKG_AUTHORS"));
 
    // TODO: Continue the rest of the configurations
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
                .short("s")
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
    .get_matches()
}

lazy_static! {

    static ref CFG: Config = init_cfg();

    static ref ARGS: ArgMatches<'static> = arg_matches();

    static ref PATTERNS_CACHE: RwLock<PatternsCache> = {

        // TODO: Configurations to ARGS!
        let cache = PatternsCache::with_capacity(CFG.cache.regex_patterns_capacity)
            .limit(CFG.cache.regex_patterns_limit); 

        RwLock::new(cache)
    };

}

pub const DEFAULT_ENCODING : &'static str = "utf-8";

// TODO: Implement `clap` arguments for configurations. Passed arguments override `cfg.toml` configurations.

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    SimpleLogger::new()
    .with_level(log::LevelFilter::Info)
    .init().unwrap();

    log::info!("Initializing service...");

    let cfg_bind = ARGS.value_of("listen")
        .unwrap_or(&CFG.service.listen);

    let cfg_server_hostname = ARGS.value_of("server_hostname")
        .unwrap_or(&CFG.service.server_hostname);

    let cfg_workers = match ARGS.value_of("workers") {
        // Some(w) => w.parse().unwrap_or(CFG.service.workers),
        Some(w) => w.parse().expect(&format!("Unable to parse '{}' as workers number.", w)),
        None => CFG.service.workers
    };

    let cfg_backlog = match ARGS.value_of("backlog") {
        Some(w) => w.parse().expect(&format!("Unable to parse '{}' as backlog number.", w)),
        None => CFG.service.backlog
    };

    let cfg_max_connections = match ARGS.value_of("max_connections") {
        Some(w) => w.parse().expect(&format!("Unable to parse '{}' as max_connections number.", w)),
        None => CFG.service.max_connections
    };
    
    let cfg_max_connection_rate = match ARGS.value_of("max_connection_rate") {
        Some(w) => w.parse().expect(&format!("Unable to parse '{}' as max_connection_rate number.", w)),
        None => CFG.service.max_connection_rate
    };

    let cfg_keep_alive = match ARGS.value_of("keep_alive") {
        Some(w) => w.parse().expect(&format!("Unable to parse '{}' as keep_alive number.", w)),
        None => CFG.service.keep_alive
    };

    let cfg_client_timeout = match ARGS.value_of("client_timeout") {
        Some(w) => w.parse().expect(&format!("Unable to parse '{}' as client_timeout number.", w)),
        None => CFG.service.client_timeout
    };

    let cfg_client_shutdown = match ARGS.value_of("client_shutdown") {
        Some(w) => w.parse().expect(&format!("Unable to parse '{}' as client_shutdown number.", w)),
        None => CFG.service.client_shutdown
    };

    let cfg_shutdown_timeout = match ARGS.value_of("shutdown_timeout") {
        Some(w) => w.parse().expect(&format!("Unable to parse '{}' as shutdown_timeout number.", w)),
        None => CFG.service.shutdown_timeout
    };
    
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
    .server_hostname(cfg_server_hostname)
    .workers(cfg_workers)
    .backlog(cfg_backlog)
    .max_connections(cfg_max_connections)
    .max_connection_rate(cfg_max_connection_rate)
    .keep_alive(cfg_keep_alive)
    .client_timeout(cfg_client_timeout)
    .client_shutdown(cfg_client_shutdown)
    .shutdown_timeout(cfg_shutdown_timeout)
    .bind(cfg_bind)?
    .run()
    .await
}