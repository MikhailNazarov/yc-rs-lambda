use actix_web::{FromRequest, HttpRequest};
use futures::future::{ok, Ready};

use crate::RuntimeError;

pub struct Context {
    pub method: String,
    pub path: String,
    pub query: String,
}

impl FromRequest for Context {
    type Error = RuntimeError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let method = req.method().as_str().to_owned();
        let path = req.path().to_owned();
        let query = req.query_string().to_owned();
        ok(Context {
            method,
            path,
            query,
        })
    }
}
