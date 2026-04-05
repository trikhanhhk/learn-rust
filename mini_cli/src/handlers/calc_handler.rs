use std::path::Path;

use crate::{
    core::{request::Request, response::Response},
    services::build::build_rayon, utils::formatter::format_tree,
};

pub fn calc_handler(req: &Request) -> Response {
    // let result = "Calculation result".to_string(); // Placeholder for actual calculation logic

    // let query = req.query.clone();
    let body = req.body.clone();
    let max_depth = req.query.get("max_depth").and_then(|v| v.parse::<usize>().ok()).unwrap_or(3);
    let path_raw = body.get("path").and_then(|v| v.as_str()).unwrap_or("");
    println!("Received path: {}", path_raw);
    let mut result = String::new();
        if !path_raw.is_empty() {
            let path: &Path = Path::new(&path_raw);
            println!("Processing path: {:?}", path.to_path_buf());
            let entry = build_rayon(path.to_path_buf(), max_depth);

            result = format_tree(&entry).to_string().into();
            println!("Built result: {:?}", &result);
        }

    println!("Received /calc request with params: {:?}", req);
    Response {
        status_code: 200,
        body: format!("<pre>{}</pre>", result),
    }
}
