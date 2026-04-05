use crate::{core::{request::Request, response::Response}, handlers::calc_handler::calc_handler};

pub fn route(req: &Request) -> Response {
    match req.path.as_str() {
        "/calc" => calc_handler(req),
        _ => Response {
            status_code: 404,
            body: "Not Found".to_string(),
        },
    }
}