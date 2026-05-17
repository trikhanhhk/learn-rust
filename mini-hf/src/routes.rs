use std::path::PathBuf;

use axum::{Json, Router, extract::Query, routing::get};
use mini_hf::services::build::{Entry, process_node};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ScanQuery {
    pub path: String,
    pub max_depth: usize,
}

pub fn create_router() -> Router {
    Router::new()
    .route("/health", get(health_handler))
    .route("/scan", get(hf_handler))
}

async fn health_handler () -> &'static str {
    "Welcome to Bolt"
}

async fn hf_handler (Query(query): Query<ScanQuery>) -> Json<Entry>{
    let result = process_node(
        PathBuf::from(query.path),
        0,
        query.max_depth,
    )
    .await;

    Json(result)
}

