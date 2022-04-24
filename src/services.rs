use actix_web::{get, post, web, HttpResponse, Responder};
use mailparse::parse_header;
use serde::Deserialize;

use crate::utils;
use crate::DEFAULT_CHARSET;
use crate::PATTERNS_CACHE;

#[derive(Deserialize, Debug)]
pub struct RegexData {
    text: String,
    pattern: String,
    // join: String,
}

#[get("/welcome")]
pub async fn welcome() -> impl Responder {
    HttpResponse::Ok().body(
        "Welcome. I am Dr. Samuel Hayden, I'm the head of this facility. 
    I think we can work together and resolve this problem in a way that benefits us both.",
    )
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// TODO: Get statistics: uptime, concurrent connections, bandwidth usage, CPU & RAM.
// TODO: Add HTML playground for the API

#[post("/unescape")]
pub async fn unescape(req_body: String) -> impl Responder {
    let unescaped_req_body =
        utils::unescape_as_bytes(&req_body).expect("Unable to unescape request's body.");

    let response = utils::attempt_decode(&unescaped_req_body, DEFAULT_CHARSET).unwrap();

    HttpResponse::Ok().body(response.into_owned())
}

#[post("/unescape/{charset}")]
pub async fn unescape_charset(path: web::Path<(String,)>, req_body: String) -> impl Responder {
    let (charset,) = path.into_inner();
    let unescaped_req_body =
        utils::unescape_as_bytes(&req_body).expect("Unable to unescape request's body.");

    let response = utils::attempt_decode(&unescaped_req_body, &charset).unwrap();

    HttpResponse::Ok().body(response.into_owned())
}

#[post("/decode_quoted_printable")]
pub async fn decode_quoted_printable(req_body: String) -> impl Responder {
    // let response = match quoted_printable::decode(&req_body, quoted_printable::ParseMode::Robust) {
    //     Ok(v) => {
    //         utils::attempt_decode(&v, &DEFAULT_CHARSET).unwrap()
    //     },
    //     Err(_) => {
    //         return HttpResponse::Ok().body(req_body)
    //     }
    // };

    let response = utils::decode_quoted_printable(&req_body, DEFAULT_CHARSET).unwrap();

    HttpResponse::Ok().body(response.into_owned())
}

#[post("/decode_quoted_printable/{charset}")]
pub async fn decode_quoted_printable_charset(
    path: web::Path<(String,)>,
    req_body: String,
) -> impl Responder {
    let (charset,) = path.into_inner();
    let response = match quoted_printable::decode(&req_body, quoted_printable::ParseMode::Robust) {
        Ok(v) => v,
        Err(_) => return HttpResponse::Ok().body(req_body),
    };

    let response = utils::attempt_decode(&response, &charset).unwrap();

    HttpResponse::Ok().body(response.into_owned())
}

#[post("/decode_base64")]
pub async fn decode_base64(req_body: String) -> impl Responder {
    let raw_payload = base64::decode(&req_body).expect("Unable to decode base64.");

    let response = utils::attempt_decode(&raw_payload, DEFAULT_CHARSET).unwrap();

    HttpResponse::Ok().body(response.into_owned())
}

#[post("/decode_base64/{charset}")]
pub async fn decode_base64_charset(path: web::Path<(String,)>, req_body: String) -> impl Responder {
    let (charset,) = path.into_inner();
    let raw_payload = base64::decode(&req_body).expect("Unable to decode base64.");

    let response = utils::attempt_decode(&raw_payload, &charset).unwrap();

    HttpResponse::Ok().body(response.into_owned())
}

#[post("/decode_mime_header")]
pub async fn decode_mime_header(req_body: String) -> impl Responder {
    let normalized_req_body = utils::normalize_str(&req_body);

    // let response: String = normalized_req_body.lines()
    //     .map(|x| {
    //         let prefixed_x = format!(":{x}");
    //         let (parsed, _) = parse_header(prefixed_x.as_bytes()).unwrap();
    //         parsed.get_value()
    //     })
    //     .map(|x| utils::unescape_as_bytes(&x).expect("Unable to unescape request's body."))
    //     .map(|x| utils::attempt_decode(&x, &DEFAULT_ENCODING).unwrap())
    //     .collect();

    let response = utils::decode_mime_header(&normalized_req_body).unwrap();

    HttpResponse::Ok().body(response.into_owned())
}

#[post("/decode_mime_header/rfc822")]
pub async fn decode_mime_header_rfc822(req_body: web::Bytes) -> impl Responder {
    let (parsed, _) = parse_header(&req_body).unwrap();

    HttpResponse::Ok().body(parsed.get_value())
}

#[post("/decode_auto")]
pub async fn decode_auto(req_body: String) -> impl Responder {
    let response = utils::auto_decode(&req_body, DEFAULT_CHARSET).unwrap();

    HttpResponse::Ok().body(response.into_owned())
}

#[post("/decode_auto/{charset}")]
pub async fn decode_auto_charset(path: web::Path<(String,)>, req_body: String) -> impl Responder {
    let (charset,) = path.into_inner();
    let response = utils::auto_decode(&req_body, &charset).unwrap();

    HttpResponse::Ok().body(response.into_owned())
}

#[post("/regex_capture_group")]
pub async fn regex_capture_group(request: web::Json<RegexData>) -> impl Responder {
    let mut patterns_cache = PATTERNS_CACHE.write();

    let re = patterns_cache.get(&request.pattern);

    let caps = re.captures(&request.text).unwrap();

    let response = caps.get(1).unwrap().as_str().to_owned();

    HttpResponse::Ok().body(response)
}

#[post("/regex_to_json")]
pub async fn regex_to_json(request: web::Json<RegexData>) -> impl Responder {
    // TODO: Return a JSON in the response with all Regex fields e.g. `(?P<year>\d+)` may return `{"year": 2022}`
    // TODO: Consider how to use `read` on RWLock and `write` only when needed (maybe move sync stuff into the PatternsCache?)
    // let mut patterns_cache = PATTERNS_CACHE.read();

    let mut patterns_cache = PATTERNS_CACHE.write();

    let re = patterns_cache.get(&request.pattern);

    let caps = re.captures(&request.text).unwrap();

    let mut response = String::from('{');

    for key in re.capture_names().flatten() {
        if let Some(value) = caps.name(key) {
            response.push_str(&format!("\"{}\":\"{}\",", key, value.as_str()));
        }
    }

    // Remove last `,`
    // response.pop();
    // response.push('}');

    response.replace_range(response.len() - 1.., "}"); // `-1` for the `,` byte size

    HttpResponse::Ok().body(response)
}

// #[derive(Deserialize, Debug)]
// pub struct TestData {
//     payload: String,
//     decode: String,
// }

// #[post("/form_test")]
// pub async fn form_test(request: web::Form<TestData>) -> impl Responder {
//     format!("{request:?}")
// }

// #[post("/json_test")]
// pub async fn json_test(request: web::Json<TestData>) -> impl Responder {
//     format!("{request:?}")
// }
