use clap::ArgMatches;
use serde_derive::Deserialize;
use std::usize;
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

    // fn from_arg_matches(&self, arg_matches: ArgMatches) -> Self {
    //     // TODO: Move all match assignments here
    //     todo!()
    // }
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

fn default_cache_regex_patterns_limit() -> usize { 10000 }
fn default_regex_patterns_capacity() -> usize { 10000 }


pub fn init_cfg() -> Config {

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
