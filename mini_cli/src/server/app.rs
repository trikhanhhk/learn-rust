use std::{collections::HashMap, sync::Arc};

use crate::{core::{Method, Routes}, handlers::calc_handler::CalcHandler, router::router::Router};

pub struct App {
    routes: Routes,
}

impl App {
    pub fn new() -> Self {
        App {
            routes: HashMap::new(),
        }
    }
    
    pub fn get(mut self, path: &str, handler: Box<dyn Router>) -> Self {
        let method_map = self.routes.entry(path.to_string())
            .or_insert_with(HashMap::new);
        method_map.insert(Method::GET, handler);
        self
    }

    pub fn post(mut self, path: &str, handler: Box<dyn Router>) -> Self {
        let method_map = self.routes.entry(path.to_string())
            .or_insert_with(HashMap::new);
        method_map.insert(Method::POST, handler);
        self
    }

    pub fn build(self) -> Arc<Routes> {
        Arc::new(self.routes)
    }
}

pub fn create_app() -> App {
    App::new()
        .get("/calc", Box::new(CalcHandler))
        .post("/calc", Box::new(CalcHandler))
}