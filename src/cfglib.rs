use clap::ArgMatches;
use serde_derive::Deserialize;
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

// impl From<std::io::Error> for CfgFileError {
//     fn from(e: std::io::Error) -> Self {
//         CfgFileError::UnknownFailure(e)
//     }
// }

// enum OnParseError<T, F: Fn() -> T> {
//     DefaultValue(T),
//     DefaultFn(F),
//     Panic,
// }

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

// type FilePath<'a> = &'a dyn AsRef<Path>;

// impl<'a> TryFrom<FilePath<'a>> for Config {

//     type Error = CfgFileError;

//     fn try_from(cfg_file: FilePath<'a>) -> Result<Self, Self::Error> {
//         let exe_dir = current_exe()
//             .unwrap()
//             .parent()
//             .unwrap()
//             .to_owned();

//         let toml_path = exe_dir.join(cfg_file);

//         let file = File::open(&toml_path);

//         match file {

//             Ok(mut f) => {

//                 let mut toml_contents= String::new();

//                 if let Err(e) = f.read_to_string(&mut toml_contents) {
//                     return Err(CfgFileError::FailedToReadCfgFile(e))
//                 }

//                 // Returns a `Config` object.
//                 match toml::from_str(&toml_contents) {
//                     Ok(r) => Ok(r),
//                     Err(e) => Err(CfgFileError::FailedToParseCfgFile(e))
//                 }
//             }
//             Err(e) => Err(CfgFileError::FailedToOpenCfgFile(e))
//         }
//     }
// }

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
    fn bitor(self, rhs: Self) -> Self::Output {
        // TODO: If there is None and a None, assign None
        // TODO: If there is None and a Some, assign Some
        // TODO: If there is Some and a Some, assign the first Some
        todo!()
    }
}

impl Config {
    pub fn mix_from_arg_matches(
        arg_matches: ArgMatches,
        base: Config,
    ) -> Result<Self, ConfigError> {
        Ok(
            Self {
                service: ServiceConfig {
                    listen: parse_arg(&arg_matches, "listen", || base.service.listen)?,
                    server_hostname: parse_arg(&arg_matches, "server_hostname", || {
                        base.service.server_hostname
                    })?,
                    workers: parse_arg(&arg_matches, "workers", || base.service.workers)?,
                    backlog: parse_arg(&arg_matches, "backlog", || base.service.backlog)?,
                    max_connections: parse_arg(&arg_matches, "max_connections", || {
                        base.service.max_connections
                    })?,
                    max_connection_rate: parse_arg(&arg_matches, "max_connection_rate", || {
                        base.service.max_connection_rate
                    })?,
                    keep_alive: parse_arg(&arg_matches, "keep_alive", || base.service.keep_alive)?,
                    client_timeout: parse_arg(&arg_matches, "client_timeout", || {
                        base.service.client_timeout
                    })?,
                    client_shutdown: parse_arg(&arg_matches, "client_shutdown", || {
                        base.service.client_shutdown
                    })?,
                    shutdown_timeout: parse_arg(&arg_matches, "shutdown_timeout", || {
                        base.service.shutdown_timeout
                    })?,
                },

                common: CommonConfig {
                    alt_encoding: parse_arg(&arg_matches, "alt_encoding", || {
                        base.common.alt_encoding
                    })?,
                },

                cache: CacheConfig {
                    regex_patterns_limit: parse_arg(&arg_matches, "regex_patterns_limit", || {
                        base.cache.regex_patterns_limit
                    })?,
                    regex_patterns_capacity: parse_arg(
                        &arg_matches,
                        "regex_patterns_capacity",
                        || base.cache.regex_patterns_capacity,
                    )?,
                },

                logger: LoggerConfig {
                    log_level: parse_arg(&arg_matches, "log_level", || base.logger.log_level)?,
                },
            }, // Self
        ) // Ok()
    } // fn
} // impl

// TODO: A new: `impl From<ArgMatches> for Config`
// TODO: `impl From<FilePath> for Config`
// TODO: New Struct: `MixedConfig` (with `base`) or impl Some kind of Operator `+` / `-` for `Config`

// impl From<ArgMatches> for Config {

//     fn from(arg_matches: ArgMatches) -> Self {

//         // let cfg_file_path = "cfg.toml";

//         // // let cfg_file = match Config::from_file(cfg_file_path) {
//         // let cfg_file = match Config::try_from(cfg_file_path.as_ref()) {

//         //     Ok(cfg) => cfg,

//         //     Err(cfg_file_error) => match cfg_file_error {

//         //         CfgFileError::FailedToOpenCfgFile(e) =>  {
//         //             log::warn!("Unable to load '{cfg_file_path}' file: {e}");
//         //             Config::default()
//         //         },

//         //         CfgFileError::FailedToReadCfgFile(e) => {
//         //             log::error!("Unable to load '{cfg_file_path}' contents: {e}");
//         //             std::process::exit(1);
//         //         },

//         //         CfgFileError::FailedToParseCfgFile(e) => {
//         //             log::error!("Failed to parse '{cfg_file_path}': {e}");
//         //             std::process::exit(1);
//         //         },
//         //     }
//         // };

//         Config::mix_from_arg_matches(arg_matches, cfg_file)
//             .unwrap_or_else(|e| {
//                 log::error!("{e}");
//                 std::process::exit(1)
//             })

//     } // fn

// } // impl

fn default_common_config() -> CommonConfig {
    CommonConfig::default()
}
fn default_service_config() -> ServiceConfig {
    ServiceConfig::default()
}
fn default_cache_config() -> CacheConfig {
    CacheConfig::default()
}
fn default_logger_config() -> LoggerConfig {
    LoggerConfig::default()
}

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

fn default_common_alt_encoding() -> String {
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

fn default_service_listen() -> String {
    "127.0.0.1:8080".into()
}
fn default_service_server_hostname() -> String {
    "localhost".into()
}
fn default_service_workers() -> usize {
    num_cpus::get()
}
fn default_service_backlog() -> i32 {
    2048
}
fn default_service_max_connections() -> usize {
    25_000
}
fn default_service_max_connection_rate() -> usize {
    256
}
fn default_service_keep_alive() -> usize {
    5
}
fn default_service_client_timeout() -> u64 {
    5_000
}
fn default_service_client_shutdown() -> u64 {
    5_000
}
fn default_service_shutdown_timeout() -> u64 {
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

fn default_regex_patterns_limit() -> usize {
    10000
}
fn default_regex_patterns_capacity() -> usize {
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

pub fn default_logger_level() -> String {
    "info".into()
}
