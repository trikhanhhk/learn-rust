use std::collections::HashMap;

use crate::core::Method;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub query: HashMap<String, String>,
    // json body
    pub body: serde_json::Value,
}

pub fn parse_params(req: &str) -> Request {
    let mut sections = req.split("\r\n\r\n");
    let header_part = sections.next().unwrap_or("");
    let body_part = sections.next().unwrap_or("");

    // parse request
    let mut lines = header_part.lines();
    let request_line = lines.next().unwrap_or("");

    let mut parts = request_line.split_whitespace();

    let method_str = parts.next().unwrap_or("").to_uppercase();
    let method = match method_str.as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "PATCH" => Method::PATCH,
        "DELETE" => Method::DELETE,
        _ => Method::GET, // default to GET if unknown
    };

    let full_path = parts.next().unwrap_or("");

    let (path, query) = parse_path_and_query(full_path);

    let headers = parse_headers(lines);

    // parse body
    let body = if let Some(content_type) = headers.get("Content-Type") {
        if content_type == "application/json" {
            serde_json::from_str(body_part).unwrap_or(serde_json::Value::Null)
        } else {
            serde_json::Value::Null
        }
    } else {
        serde_json::Value::Null
    };

    Request {
        method,
        path,
        query,
        body,
    }
}

pub fn parse_path_and_query(full_path: &str) -> (String, HashMap<String, String>) {
    let mut query_map = HashMap::new();

    let mut split = full_path.splitn(2, '?');
    let path = split.next().unwrap_or("").to_string();

    if let Some(query_str) = split.next() {
        for pair in query_str.split('&') {
            let mut kv = pair.splitn(2, '=');
            let key = kv.next().unwrap_or("").to_string();
            let value = kv.next().unwrap_or("").to_string();
            if !key.is_empty() {
                query_map.insert(key.to_string(), value.to_string());
            }
        }
    }

    (path, query_map)
}

pub fn parse_headers(lines: std::str::Lines) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    for line in lines {
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }
    headers
}
