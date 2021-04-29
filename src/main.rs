mod base64;
use std::{fs::File, io::Read};

use serde_derive::Deserialize;

use actix_web::{
    HttpServer,
    App
};

// fn main() {
//     base64::test_hello();
// }

#[derive(Deserialize)]
struct Config {
    base64: Base64Config
}

#[derive(Deserialize)]
struct Base64Config {
    listen: String
}

fn init_cfg() -> Config {

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    println!("Initializing service...");

    let cfg = init_cfg();

    println!("Initializing HTTP Listener: {}", cfg.base64.listen);

    HttpServer::new(|| {
        App::new()
            .service(base64::welcome)
            .service(base64::echo)
    })
    .bind(cfg.base64.listen)?
    .run()
    .await

}