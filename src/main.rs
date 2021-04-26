mod base64;

use actix_web::{
    HttpServer,
    App
};

// fn main() {
//     base64::test_hello();
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(base64::welcome)
            .service(base64::echo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}