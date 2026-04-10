use crate::core::{request::Request, response::Response};

pub trait Router: Send + Sync {
    fn route(&self, req: &Request) -> Response;
}


