use actix_web::{
    get,
    post,
    HttpResponse,
    Responder
};

#[get("/")]
pub async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome. I am Dr. Samuel Hayden, I'm the head of this facility. 
    I think we can work together and resolve this problem in a way that benefits us both.")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// pub fn test_hello() {
//     println!("Hello!");
// }
