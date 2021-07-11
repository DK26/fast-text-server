use clap::ArgMatches;
use serde_derive::Deserialize;
use std::default::default;
use std::path::Path;
// use std::usize;
use std::{
    fs::File, 
    io::Read
};
use std::env::current_exe;

#[derive(Deserialize, Debug)]
pub struct Config {

    #[serde(default = "default_common_config")]
    pub common: CommonConfig,

    #[serde(default = "default_service_config")]
    pub service: ServiceConfig,

    #[serde(default = "default_cache_config")]
    pub cache: CacheConfig,
}

impl Default for Config {

    fn default() -> Self {
        Self {
            common: default_common_config(),
            service: default_service_config(),
            cache: default_cache_config(),
        }
    }

}

enum CfgFileError {
    FailedToOpenCfgFile(std::io::Error),
    FailedToReadCfgFile(std::io::Error),
    FailedToParseCfgFile(toml::de::Error)
}


impl Config {

    fn from_file<P: AsRef<Path>>(&self, cfg_file: P) -> Result<Self, CfgFileError> {

        let exe_dir = current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_owned();
            
        let toml_path = exe_dir.join(cfg_file);
    
        let file = File::open(&toml_path);
    
        match file {
    
            Ok(mut f) => {
    
                let mut toml_contents= String::new();
    
                // match f.read_to_string(&mut toml_contents) {
                //     Err(e) => {
                //         log::error!("Unable to load 'cfg.toml' contents: {}", e);
                //         std::process::exit(1)
                //     }
                //     _ => {}
                // }

                if let Err(e) = f.read_to_string(&mut toml_contents) {
                    return Err(CfgFileError::FailedToReadCfgFile(e))
                }
            
                // Returns a `Config` object.
                match toml::from_str(&toml_contents) {
                    Ok(r) => Ok(r),
                    Err(e) => {
                        // log::error!("Failed to parse 'cfg.toml': {}", e);
                        // std::process::exit(1);
                        return Err(CfgFileError::FailedToParseCfgFile(e))
                    }
                }
                // toml::from_str(&toml_contents)
                //     .expect("Failed to parse 'cfg.toml'.")
            }
            Err(e) => {
                // log::warn!("Unable to load `cfg.toml` file: {}", e);
                // Config::default()
                Err(CfgFileError::FailedToOpenCfgFile(e))
            }
        }
    }

    fn from_arg_matches(&self, arg_matches: ArgMatches) -> Self {

        Self {

            service : ServiceConfig {

                listen: arg_matches
                    .value_of("listen")
                    .unwrap_or_default()
                    .to_owned(),

                server_hostname: arg_matches
                    .value_of("server_hostname")
                    .unwrap_or_default()
                    .to_owned(),

                workers: arg_matches
                    .value_of("workers")
                    .unwrap_or_default()
                    .parse().unwrap_or_default(),

                backlog: arg_matches
                    .value_of("backlog")
                    .unwrap_or(|| default_service_backlog()),

                max_connections: arg_matches
                    .value_of("max_connections")
                    .unwrap_or(|| default_service_max_connections()),

                max_connection_rate: arg_matches
                    .value_of("max_connection_rate")
                    .unwrap_or(|| default_service_max_connection_rate()),

                keep_alive: arg_matches
                    .value_of("keep_alive")
                    .unwrap_or(|| default_service_keep_alive()),

                client_timeout: arg_matches
                    .value_of("client_timeout")
                    .unwrap_or(|| default_service_client_timeout()),

                client_shutdown: arg_matches
                    .value_of("client_shutdown")
                    .unwrap_or(|| default_service_client_shutdown()),

                shutdown_timeout: arg_matches
                    .value_of("shutdown_timeout")
                    .unwrap_or(|| default_service_shutdown_timeout()),

            },

            common : CommonConfig {

                alt_encoding: arg_matches
                    .value_of("alt_encoding")
                    .unwrap_or(|| default_common_alt_encoding()),

            },

            cache: CacheConfig {

                regex_patterns_limit: arg_matches
                    .value_of("regex_patterns_limit")
                    .unwrap_or(|| default_regex_patterns_limit()),

                regex_patterns_capacity: arg_matches
                    .value_of("regex_patterns_capacity")
                    .unwrap_or(|| default_regex_patterns_capacity()),
      
            },

        }

    // Service Configurations
    // let cfg_bind = ARGS.value_of("listen")
    //     .unwrap_or(&CFG.service.listen);
    // log::debug!("bind = {}", cfg_bind);

    // let cfg_server_hostname = ARGS.value_of("server_hostname")
    //     .unwrap_or(&CFG.service.server_hostname);
    // log::debug!("server_hostname = {}", cfg_server_hostname);

    // let cfg_workers = match ARGS.value_of("workers") {
    //     // Some(w) => w.parse().unwrap_or(CFG.service.workers),
    //     Some(w) => w.parse().expect(&format!("Unable to parse '{}' as workers number.", w)),
    //     None => CFG.service.workers
    // };
    // log::debug!("workers = {}", cfg_workers);

    // let cfg_backlog = match ARGS.value_of("backlog") {
    //     Some(w) => w.parse().expect(&format!("Unable to parse '{}' as backlog number.", w)),
    //     None => CFG.service.backlog
    // };
    // log::debug!("backlog = {}", cfg_backlog);

    // let cfg_max_connections = match ARGS.value_of("max_connections") {
    //     Some(w) => w.parse().expect(&format!("Unable to parse '{}' as max_connections number.", w)),
    //     None => CFG.service.max_connections
    // };
    // log::debug!("max_connections = {}", cfg_max_connections);

    // let cfg_max_connection_rate = match ARGS.value_of("max_connection_rate") {
    //     Some(w) => w.parse().expect(&format!("Unable to parse '{}' as max_connection_rate number.", w)),
    //     None => CFG.service.max_connection_rate
    // };
    // log::debug!("max_connection_rate = {}", cfg_max_connection_rate);

    // let cfg_keep_alive = match ARGS.value_of("keep_alive") {
    //     Some(w) => w.parse().expect(&format!("Unable to parse '{}' as keep_alive number.", w)),
    //     None => CFG.service.keep_alive
    // };
    // log::debug!("keep_alive = {}", cfg_keep_alive);

    // let cfg_client_timeout = match ARGS.value_of("client_timeout") {
    //     Some(w) => w.parse().expect(&format!("Unable to parse '{}' as client_timeout number.", w)),
    //     None => CFG.service.client_timeout
    // };
    // log::debug!("client_timeout = {}", cfg_client_timeout);

    // let cfg_client_shutdown = match ARGS.value_of("client_shutdown") {
    //     Some(w) => w.parse().expect(&format!("Unable to parse '{}' as client_shutdown number.", w)),
    //     None => CFG.service.client_shutdown
    // };
    // log::debug!("client_shutdown = {}", cfg_client_shutdown);

    // let cfg_shutdown_timeout = match ARGS.value_of("shutdown_timeout") {
    //     Some(w) => w.parse().expect(&format!("Unable to parse '{}' as shutdown_timeout number.", w)),
    //     None => CFG.service.shutdown_timeout
    // };
    // log::debug!("shutdown_timeout = {}", cfg_shutdown_timeout);

    // Common Configurations
    // log::debug!("alt_encoding = {}", *utils::CFG_ALT_ENCODING);

    // Cache Configurations
    // log::debug!("regex_patterns_capacity = {}", CFG.cache.regex_patterns_capacity);
    // log::debug!("regex_patterns_limit = {}", CFG.cache.regex_patterns_limit);
    }
}

fn default_common_config() -> CommonConfig { CommonConfig::default() }
fn default_service_config() -> ServiceConfig { ServiceConfig::default() }
fn default_cache_config() -> CacheConfig { CacheConfig::default() }

#[derive(Deserialize, Debug)]
pub struct CommonConfig {

    #[serde(default = "default_common_alt_encoding")]
    pub alt_encoding: String,

}

impl Default for CommonConfig {
    fn default() -> Self {
        Self {
            alt_encoding: default_common_alt_encoding(),
        }
    }
}

fn default_common_alt_encoding() -> String { String::from("utf-8") }

#[derive(Deserialize, Debug)]
pub struct ServiceConfig {

    #[serde(default = "default_service_listen")]
    pub listen: String,

    #[serde(default = "default_service_server_hostname")]
    pub server_hostname: String,

    #[serde(default = "default_service_workers")]
    pub workers: usize,

    #[serde(default = "default_service_backlog")]
    pub backlog: i32,

    #[serde(default = "default_service_max_connections")]
    pub max_connections: usize,

    #[serde(default = "default_service_max_connection_rate")]
    pub max_connection_rate: usize,

    #[serde(default = "default_service_keep_alive")]
    pub keep_alive: usize,

    #[serde(default = "default_service_client_timeout")]
    pub client_timeout: u64,

    #[serde(default = "default_service_client_shutdown")]
    pub client_shutdown: u64,

    #[serde(default = "default_service_shutdown_timeout")]
    pub shutdown_timeout: u64,

}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            listen: default_service_listen(),
            server_hostname: default_service_server_hostname(),
            workers: default_service_workers(),
            backlog: default_service_backlog(),
            max_connections: default_service_max_connections(),
            max_connection_rate: default_service_max_connection_rate(),
            keep_alive: default_service_keep_alive(),
            client_timeout: default_service_client_timeout(),
            client_shutdown: default_service_client_shutdown(),
            shutdown_timeout: default_service_shutdown_timeout(),
        }
    }
}

fn default_service_listen() -> String { String::from("127.0.0.1:8080") }
fn default_service_server_hostname() -> String { String::from("localhost") }
fn default_service_workers() -> usize { num_cpus::get() }
fn default_service_backlog() -> i32 { 2048 }
fn default_service_max_connections() -> usize { 25_000 }
fn default_service_max_connection_rate() -> usize { 256 }
fn default_service_keep_alive() -> usize { 5 }
fn default_service_client_timeout() -> u64 { 5_000 }
fn default_service_client_shutdown() -> u64 { 5_000 }
fn default_service_shutdown_timeout() -> u64 { 30 }

#[derive(Deserialize, Debug)]
pub struct CacheConfig {

    #[serde(default = "default_cache_regex_patterns_limit")]
    pub regex_patterns_limit: usize,

    #[serde(default = "default_regex_patterns_capacity")]
    pub regex_patterns_capacity: usize,

}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            regex_patterns_limit: default_cache_regex_patterns_limit(),
            regex_patterns_capacity: default_regex_patterns_capacity(),
        }
    }
}

fn default_regex_patterns_limit() -> usize { 10000 }
fn default_regex_patterns_capacity() -> usize { 10000 }


pub fn load_cfg_file() -> Config {

    let cfg_file = "cfg.toml";

    let exe_dir = current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();
        
    let toml_path = exe_dir.join(cfg_file);

    let file = File::open(&toml_path);

    match file {

        Ok(mut f) => {

            let mut toml_contents= String::new();

            match f.read_to_string(&mut toml_contents) {
                Err(e) => {
                    log::error!("Unable to load 'cfg.toml' contents: {}", e);
                    std::process::exit(1);
                }
                _ => {}
            }
        
            // Returns a `Config` object.
            match toml::from_str(&toml_contents) {
                Ok(r) => r,
                Err(e) => {
                    log::error!("Failed to parse 'cfg.toml': {}", e);
                    std::process::exit(1);
                }
            }
            // toml::from_str(&toml_contents)
            //     .expect("Failed to parse 'cfg.toml'.")
        }
        Err(e) => {
            log::warn!("Unable to load `cfg.toml` file: {}", e);
            Config::default()
        }
    }
}

fn init_cfg() -> Config {

    let cfg_file = load_cfg_file();

    Config {

    }

}