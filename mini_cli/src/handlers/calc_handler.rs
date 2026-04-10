use std::path::Path;

use crate::{
    core::{request::Request, response::Response}, router::router::Router, services::build::build_rayon, utils::formatter::format_tree
};

pub struct CalcHandler;

impl Router for CalcHandler {
    fn route(&self, req: &Request) -> Response {
        match req.method {
            crate::core::Method::POST => calc_handler_post(req),
            crate::core::Method::GET => calc_handler_get(req),
            _ => Response {
                status_code: 405,
                body: "Method Not Allowed".to_string(),
            },
        }
    }
}

fn calc(max_depth: usize, path_raw: &str) -> String {
    let mut result = String::new();
    if !path_raw.is_empty() {
        let path: &Path = Path::new(&path_raw);
        println!("Processing path: {:?}", path.to_path_buf());
        let entry = build_rayon(path.to_path_buf(), max_depth);

        result = format_tree(&entry).to_string().into();
    }
    result
}

pub fn calc_handler_post(req: &Request) -> Response {
    let body = req.body.clone();
    let max_depth = req.query.get("max_depth").and_then(|v| v.parse::<usize>().ok()).unwrap_or(3);
    let path_raw = body.get("path").and_then(|v| v.as_str()).unwrap_or("");

    let result = calc(max_depth, path_raw);

    Response {
        status_code: 200,
        body: format!("<pre>{}</pre>", result),
    }
}

pub fn calc_handler_get(req: &Request) -> Response {
    println!("Received /calc GET request with params: {:?}", req);
    let max_depth = req.query.get("max_depth").and_then(|v| v.parse::<usize>().ok()).unwrap_or(3);
    let path_raw = req.query.get("path").map(|s| s.as_str()).unwrap_or("");
    println!("Received path: {}", path_raw);
    let result = calc(max_depth, path_raw);
    Response {
        status_code: 200,
        body: format!("<pre>{}</pre>", result),
    }
}
