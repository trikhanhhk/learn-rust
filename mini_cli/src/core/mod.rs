use std::collections::HashMap;

use crate::core::{request::Request, response::Response};

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

pub type Handler = Box<dyn Fn(&Request) -> Response + Send + Sync>;
pub type Routes = HashMap<String, HashMap<Method, Handler>>;
