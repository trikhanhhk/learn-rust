use crate::core::{Routes, response::Response};

pub fn route_registry(routes: &Routes, req: &crate::core::request::Request) -> Response {
    if let Some(methods) = routes.get(&req.path) {
        if let Some(handler) = methods.get(&req.method) {
            return handler.route(req);
        } else {
            return Response {
                status_code: 405,
                body: "Method Not Allowed".to_string(),
            };
        }
    } else {
        return Response {
            status_code: 404,
            body: "Not Found".to_string(),
        };

    }

}