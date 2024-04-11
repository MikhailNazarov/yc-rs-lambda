use std::sync::Arc;

use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use futures::future::{ok, Ready};

use crate::{RuntimeError, YcRuntime};

#[derive(Clone)]
pub struct Context {
    pub runtime: Arc<YcRuntime>,
}

impl FromRequest for Context {
    type Error = RuntimeError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let runtime = req.app_data::<web::Data<YcRuntime>>().unwrap();
        let runtime = runtime.clone();
        ok::<Context, Self::Error>(Context {
            runtime: runtime.into_inner(),
        })
    }
}
