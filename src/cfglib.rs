use serde_derive::Deserialize;
use std::{
    fs::File, 
    io::Read
};
use std::{
    env::current_exe, 
    path::PathBuf
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

    // TODO: Verify the existence of a `cfg.toml` file. 
    // TODO: - Create a default `cfg.toml` file; serializing a default `Config` object. 
    // TODO: - Use return the default values in a `Config` object.

    let cfg_file = "cfg.toml";

    let exe_dir = current_exe().unwrap().parent().unwrap().to_owned();
    let toml_path = exe_dir.join(cfg_file);

    // println!("{:?}", &toml_path);

    let mut file = File::open(&toml_path).expect("Unable to load `cfg.toml` file.");

    let mut toml_contents= String::new();

    file.read_to_string(&mut toml_contents).expect("Unable to load 'cfg.toml' contents.");

    // Return a `Config` object.
    toml::from_str(&toml_contents).expect("Failed to parse 'cfg.toml'.")

    // Config {
    //     base64: Base64Config {
    //         listen: "127.0.0.1:8080".to_owned()
    //     }
    // }
}
