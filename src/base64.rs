use actix_web::{
    get,
    post,
    HttpResponse,
    Responder
};

#[get("/")]
pub async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome! My name is Dr. Samuel Hayden, I'm the head of this [UAC] corporation.")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// pub fn test_hello() {
//     println!("Hello!");
// }
