use clap::ArgMatches;
use serde::Deserialize;
use std::env::current_exe;
use std::ops::BitOr;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fs::File, io::Read};

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum CfgFileError {
    FailedToOpenCfgFile(std::io::Error),
    FailedToReadCfgFile(std::io::Error),
    FailedToParseCfgFile(toml::de::Error),
}

impl std::fmt::Display for CfgFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CfgFileError {}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ConfigError {
    BadArgument(String),
    FileError(CfgFileError),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::BadArgument(e) => write!(f, "{e}"),
            ConfigError::FileError(inner) => write!(f, "{inner}"),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Attempts to parse a given argument into the assigned type. Exists the program on failure.
// fn parse_arg<T: FromStr>(arg_matches: ArgMatches, arg: &str, default: fn() -> T, on_error: OnParseError<T, fn() -> T>) -> T {
// fn parse_arg<T: FromStr>(arg_matches: ArgMatches, arg: &str, default: fn() -> T) -> T {
// fn parse_arg<T: FromStr>(arg_matches: ArgMatches, arg: &str, default: impl Fn() -> T) -> T {
// fn parse_arg<T: FromStr>(arg_matches: &ArgMatches, arg: &str, default: T) -> T {
fn parse_arg<T: FromStr>(
    arg_matches: &ArgMatches,
    arg: &str,
    default: impl FnOnce() -> T,
) -> Result<T, ConfigError> {
    match arg_matches.value_of(arg) {
        Some(value) => match value.parse() {
            Ok(parsed) => Ok(parsed),
            Err(_) => {
                // log::error!("Failed to parse the '{arg}' argument. Incorrect value was given: '{value}'");
                // std::process::exit(1);
                return Err(ConfigError::BadArgument(format!(
                    "Failed to parse the '{arg}' argument. Incorrect value was given: '{value}'"
                )));
            }
        },
        None => {
            Ok(default())
            // default
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_common_config")]
    pub common: CommonConfig,

    #[serde(default = "default_service_config")]
    pub service: ServiceConfig,

    #[serde(default = "default_cache_config")]
    pub cache: CacheConfig,

    #[serde(default = "default_logger_config")]
    pub logger: LoggerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            common: default_common_config(),
            service: default_service_config(),
            cache: default_cache_config(),
            logger: default_logger_config(),
        }
    }
}

/// Automatically produces a full path out of a relative path.
/// e.g. `RelativeFilePath::new("cfg.toml")` allows us to get a reference (a `&Path` from `as_ref()`)
/// which includes the full path to the home directory, joined together with the `cfg.toml` file name.
#[derive(Clone)]
pub struct RelativeFilePath {
    relative_path: PathBuf,
    full_path: PathBuf,
}

impl RelativeFilePath {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let exe_dir = current_exe().unwrap().parent().unwrap().to_owned();

        Self {
            relative_path: path.as_ref().to_owned(),
            full_path: exe_dir.join(path),
        }
    }
}

impl std::fmt::Display for RelativeFilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.relative_path.display())
    }
}

impl std::fmt::Debug for RelativeFilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("RelativeFilePath").field("relative_path", &self.relative_path).field("full_path", &self.full_path).finish()
        write!(f, "{}", self.full_path.display())
    }
}

impl AsRef<Path> for RelativeFilePath {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.full_path.as_ref()
    }
}

impl TryFrom<RelativeFilePath> for Config {
    type Error = CfgFileError;

    fn try_from(cfg_file: RelativeFilePath) -> Result<Self, Self::Error> {
        let file = File::open(&cfg_file);

        match file {
            Ok(mut f) => {
                let mut toml_contents = String::new();

                if let Err(e) = f.read_to_string(&mut toml_contents) {
                    return Err(CfgFileError::FailedToReadCfgFile(e));
                }

                // Returns a `Config` object.
                match toml::from_str(&toml_contents) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(CfgFileError::FailedToParseCfgFile(e)),
                }
            }
            Err(e) => Err(CfgFileError::FailedToOpenCfgFile(e)),
        }
    }
}

impl BitOr for Config {
    type Output = Self;

    /// Usage: `args_config | cfg_file_config | default_config`
    /// Left item as the highest priority
    fn bitor(self, _rhs: Self) -> Self::Output {
        // TODO: If there is None and a None, assign None
        // TODO: If there is None and a Some, assign Some
        // TODO: If there is Some and a Some, assign the first Some
        todo!()
    }
}

impl Config {
    pub fn mix_from_arg_matches(
        arg_matches: &ArgMatches,
        base: Config,
    ) -> Result<Self, ConfigError> {
        Ok(
            Self {
                service: ServiceConfig {
                    listen: parse_arg(arg_matches, "listen", || base.service.listen)?,
                    server_hostname: parse_arg(arg_matches, "server_hostname", || {
                        base.service.server_hostname
                    })?,
                    workers: parse_arg(arg_matches, "workers", || base.service.workers)?,
                    backlog: parse_arg(arg_matches, "backlog", || base.service.backlog)?,
                    max_connections: parse_arg(arg_matches, "max_connections", || {
                        base.service.max_connections
                    })?,
                    max_connection_rate: parse_arg(arg_matches, "max_connection_rate", || {
                        base.service.max_connection_rate
                    })?,
                    keep_alive: parse_arg(arg_matches, "keep_alive", || base.service.keep_alive)?,
                    client_request_timeout: parse_arg(
                        arg_matches,
                        "client_request_timeout",
                        || base.service.client_request_timeout,
                    )?,
                    client_disconnect_timeout: parse_arg(
                        arg_matches,
                        "client_disconnect_timeout",
                        || base.service.client_disconnect_timeout,
                    )?,
                    shutdown_timeout: parse_arg(arg_matches, "shutdown_timeout", || {
                        base.service.shutdown_timeout
                    })?,
                },

                common: CommonConfig {
                    fallback_encoding: parse_arg(arg_matches, "fallback_encoding", || {
                        base.common.fallback_encoding
                    })?,
                },

                cache: CacheConfig {
                    regex_patterns_limit: parse_arg(arg_matches, "regex_patterns_limit", || {
                        base.cache.regex_patterns_limit
                    })?,
                    regex_patterns_capacity: parse_arg(
                        arg_matches,
                        "regex_patterns_capacity",
                        || base.cache.regex_patterns_capacity,
                    )?,
                },

                logger: LoggerConfig {
                    log_level: parse_arg(arg_matches, "log_level", || base.logger.log_level)?,
                },
            }, // Self
        ) // Ok()
    } // fn
} // impl

#[inline]
fn default_common_config() -> CommonConfig {
    CommonConfig::default()
}

#[inline]
fn default_service_config() -> ServiceConfig {
    ServiceConfig::default()
}

#[inline]
fn default_cache_config() -> CacheConfig {
    CacheConfig::default()
}

#[inline]
fn default_logger_config() -> LoggerConfig {
    LoggerConfig::default()
}

#[derive(Deserialize, Debug)]
pub struct CommonConfig {
    #[serde(default = "default_common_fallback_encoding")]
    pub fallback_encoding: String,
}

impl Default for CommonConfig {
    fn default() -> Self {
        Self {
            fallback_encoding: default_common_fallback_encoding(),
        }
    }
}

#[inline]
fn default_common_fallback_encoding() -> String {
    "utf-8".into()
}

#[derive(Deserialize, Debug)]
pub struct ServiceConfig {
    #[serde(default = "default_service_listen")]
    pub listen: String,

    #[serde(default = "default_service_server_hostname")]
    pub server_hostname: String,

    #[serde(default = "default_service_workers")]
    pub workers: usize,

    #[serde(default = "default_service_backlog")]
    pub backlog: u32,

    #[serde(default = "default_service_max_connections")]
    pub max_connections: usize,

    #[serde(default = "default_service_max_connection_rate")]
    pub max_connection_rate: usize,

    #[serde(default = "default_service_keep_alive")]
    pub keep_alive: u64,

    #[serde(default = "default_service_client_request_timeout")]
    pub client_request_timeout: u64,

    #[serde(default = "default_service_client_disconnect_timeout")]
    pub client_disconnect_timeout: u64,

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
            client_request_timeout: default_service_client_request_timeout(),
            client_disconnect_timeout: default_service_client_disconnect_timeout(),
            shutdown_timeout: default_service_shutdown_timeout(),
        }
    }
}

#[inline]
fn default_service_listen() -> String {
    "127.0.0.1:8080".into()
}

#[inline]
fn default_service_server_hostname() -> String {
    "localhost".into()
}

#[inline]
fn default_service_workers() -> usize {
    // num_cpus::get()          // Default for Actix-Web 3 (Virtual CPU count)
    num_cpus::get_physical() // Default for Actix-Web 4 (Physical CPU count or, fallback to Virtual CPU count)
}

#[inline]
const fn default_service_backlog() -> u32 {
    2048
}

#[inline]
const fn default_service_max_connections() -> usize {
    25_000
}

#[inline]
const fn default_service_max_connection_rate() -> usize {
    256
}

#[inline]
const fn default_service_keep_alive() -> u64 {
    5
}

#[inline]
const fn default_service_client_request_timeout() -> u64 {
    5_000
}

#[inline]
const fn default_service_client_disconnect_timeout() -> u64 {
    5_000
}

#[inline]
const fn default_service_shutdown_timeout() -> u64 {
    30
}

#[derive(Deserialize, Debug)]
pub struct CacheConfig {
    #[serde(default = "default_regex_patterns_limit")]
    pub regex_patterns_limit: usize,

    #[serde(default = "default_regex_patterns_capacity")]
    pub regex_patterns_capacity: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            regex_patterns_limit: default_regex_patterns_limit(),
            regex_patterns_capacity: default_regex_patterns_capacity(),
        }
    }
}

#[inline]
const fn default_regex_patterns_limit() -> usize {
    10000
}

#[inline]
const fn default_regex_patterns_capacity() -> usize {
    10000
}

#[derive(Deserialize, Debug)]
pub struct LoggerConfig {
    #[serde(default = "default_logger_level")]
    pub log_level: String,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_level: default_logger_level(),
        }
    }
}

#[inline]
pub fn default_logger_level() -> String {
    "info".into()
}
