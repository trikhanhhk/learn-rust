use std::{collections::HashMap, sync::Arc};

use crate::{
    core::{Method, Routes, request::Request, response::Response},
    handlers::calc_handler::{calc_handler_get, calc_handler_post},
};

pub struct App {
    routes: Routes,
}

impl App {
    pub fn new() -> Self {
        App {
            routes: HashMap::new(),
        }
    }

    pub fn get<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        let method_map = self
            .routes
            .entry(path.to_string())
            .or_insert_with(HashMap::new);
        method_map.insert(Method::GET, Box::new(handler));
        self
    }

    pub fn post<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        let method_map = self
            .routes
            .entry(path.to_string())
            .or_insert_with(HashMap::new);
        method_map.insert(Method::POST, Box::new(handler));
        self
    }

    pub fn build(self) -> Arc<Routes> {
        Arc::new(self.routes)
    }
}

pub fn create_app() -> App {
    App::new()
        .get("/calc", calc_handler_get)
        .post("/calc", calc_handler_post)
}
