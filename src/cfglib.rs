use serde_derive::Deserialize;
use std::{
    fs::File, 
    io::Read
};

#[derive(Deserialize)]
pub struct Config {
    pub common: Common,
    pub base64: Base64Config
}

#[derive(Deserialize)]
pub struct Common {
    pub alt_encoding: String
}

#[derive(Deserialize)]
pub struct Base64Config {
    pub listen: String
}

pub fn init_cfg() -> Config {

    let toml_path = "cfg.toml";

    let mut file = File::open(&toml_path).expect("Unable to load `cfg.toml` file.");

    let mut toml_contents= String::new();

    file.read_to_string(&mut toml_contents).expect("Unable to load 'cfg.toml' contents.");

    toml::from_str(&toml_contents).expect("Failed to parse 'cfg.toml'.")

    // Config {
    //     base64: Base64Config {
    //         listen: "127.0.0.1:8080".to_owned()
    //     }
    // }
}
