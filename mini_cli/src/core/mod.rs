use std::collections::HashMap;

use crate::router::router::Router;

pub mod request;
pub mod response;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

pub type Routes = HashMap<String, HashMap<Method, Box<dyn Router>>>;