use serde::Deserialize;
use actix_web::{
    get,
    post,
    HttpResponse,
    Responder,
    web,
};
use mailparse::parse_header;

use crate::utils;
use crate::DEFAULT_ENCODING;
use crate::PATTERNS_CACHE;

#[derive(Deserialize, Debug)]
pub struct RegexData {
    text: String,
    pattern: String,
    // join: String,
}

#[get("/welcome")]
pub async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome. I am Dr. Samuel Hayden, I'm the head of this facility. 
    I think we can work together and resolve this problem in a way that benefits us both.")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// TODO: Add `uptime()` End-Point
// TODO: Add HTML playground for the API

#[post("/unescape")]
pub async fn unescape(req_body: String) -> impl Responder {

    let unescaped_req_body = utils::unescape_as_bytes(&req_body).expect("Unable to unescape request's body.");

    let response = utils::attempt_decode(&unescaped_req_body, &DEFAULT_ENCODING).unwrap();

    HttpResponse::Ok().body(response)

}

#[post("/unescape/{charset}")]
pub async fn unescape_decode(web::Path((charset,)): web::Path<(String,)>, req_body: String) -> impl Responder {

    let unescaped_req_body = utils::unescape_as_bytes(&req_body).expect("Unable to unescape request's body.");

    let response = utils::attempt_decode(&unescaped_req_body, &charset).unwrap();

    HttpResponse::Ok().body(response)

}

#[post("/decode_base64")]
pub async fn decode_base64(req_body: String) -> impl Responder {

    let raw_payload = base64::decode(&req_body).expect("Unable to decode base64.");

    let response = utils::attempt_decode(&raw_payload, &DEFAULT_ENCODING).unwrap();

    HttpResponse::Ok().body(response)

}

#[post("/decode_base64/{charset}")]
pub async fn decode_base64_charset(web::Path((charset,)): web::Path<(String,)>, req_body: String) -> impl Responder {

    let raw_payload = base64::decode(&req_body).expect("Unable to decode base64.");

    let response = utils::attempt_decode(&raw_payload, &charset).unwrap();

    HttpResponse::Ok().body(response)

}

#[post("/decode_mime_header")]
pub async fn decode_mime_header(req_body: String) -> impl Responder {

    let normalized_req_body = utils::normalize_mime(&req_body)
        .replace(" =?", "\r\n=?")
        .replace("?= ", "?=\r\n");
    
    // let response: String = normalized_req_body.lines()
    //     .map(|x| {
    //         let prefixed_x = format!(":{}", x);
    //         let (parsed, _) = parse_header(prefixed_x.as_bytes()).unwrap();
    //         parsed.get_value()
    //     })
    //     .map(|x| utils::unescape_as_bytes(&x).expect("Unable to unescape request's body.")) 
    //     .map(|x| utils::attempt_decode(&x, &DEFAULT_ENCODING).unwrap())
    //     .collect();
        
    let mut response = String::new();

    for line in normalized_req_body.lines() {
 
        let trimmed_line = line.trim_start();

        if trimmed_line.starts_with("=?") && trimmed_line.ends_with("?=") {

            let prefixed_line = format!(":{}", trimmed_line);
            let (parsed, _) = parse_header(prefixed_line.as_bytes()).unwrap();
            response.push_str(&parsed.get_value())

        } else {
            
            if trimmed_line.contains("\\x") || trimmed_line.contains("\\u") {
                let unescaped_line_bytes = utils::unescape_as_bytes(&trimmed_line).expect("Unable to unescape request's body.");
                let unescaped_line = utils::attempt_decode(&unescaped_line_bytes, &DEFAULT_ENCODING).unwrap();
                response.push_str(&unescaped_line)
            } else { 
                response.push_str(&trimmed_line)
            }
        }
    }

    HttpResponse::Ok().body(response)

}

#[post("/decode_mime_header/rfc822")]
pub async fn decode_mime_header_rfc822(req_body: web::Bytes) -> impl Responder {

    let (parsed, _) = parse_header(&req_body).unwrap();

    HttpResponse::Ok().body(parsed.get_value())

}

#[post("/regex_capture_group")]
pub async fn regex_capture_group(request: web::Json<RegexData>) -> impl Responder { 
    
    let mut patterns_cache = PATTERNS_CACHE.write();

    let re = patterns_cache.get(&request.pattern);

    let caps = re.captures(&request.text).unwrap();

    let response = caps
        .get(1)
        .unwrap()
        .as_str()
        .to_owned();

    HttpResponse::Ok().body(response)
}

// #[derive(Deserialize, Debug)]
// pub struct TestData {
//     payload: String,
//     decode: String,
// }

// #[post("/form_test")]
// pub async fn form_test(request: web::Form<TestData>) -> impl Responder { 
//     format!("{:?}", &request)
// }

// #[post("/json_test")]
// pub async fn json_test(request: web::Json<TestData>) -> impl Responder { 
//     format!("{:?}", &request)
// }