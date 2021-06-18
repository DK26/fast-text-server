use serde_derive::Deserialize;
use std::usize;
use std::{
    fs::File, 
    io::Read
};
use std::env::current_exe;

#[derive(Deserialize)]
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
            common: CommonConfig::default(),
            service: ServiceConfig::default(),
            cache: CacheConfig::default(),
        }
    }
}

fn default_common_config() -> CommonConfig { CommonConfig::default() }
fn default_service_config() -> ServiceConfig { ServiceConfig::default() }
fn default_cache_config() -> CacheConfig { CacheConfig::default() }

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct ServiceConfig {

    #[serde(default = "default_service_listen")]
    pub listen: String

}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            listen: default_service_listen(),
        }
    }
}

fn default_service_listen() -> String { String::from("127.0.0.1:8080") }

#[derive(Deserialize)]
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

    // TODO: Either a - allow running without `cfg.toml` presents or create a `cfg.toml` file when missing.

    let cfg_file = "cfg.toml";

    let exe_dir = current_exe().unwrap().parent().unwrap().to_owned();
    let toml_path = exe_dir.join(cfg_file);

    // println!("{:?}", &toml_path);

    let mut file = File::open(&toml_path).expect("Unable to load `cfg.toml` file.");

    let mut toml_contents= String::new();

    file.read_to_string(&mut toml_contents).expect("Unable to load 'cfg.toml' contents.");

    // Returns a `Config` object.
    toml::from_str(&toml_contents).expect("Failed to parse 'cfg.toml'.")

}
