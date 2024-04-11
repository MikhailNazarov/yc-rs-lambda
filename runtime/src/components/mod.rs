#[cfg(feature = "ydb")]
pub mod ydb;

use std::sync::Arc;

use actix_web::{web, FromRequest, HttpRequest};
use async_trait::async_trait;
use futures::future::{ok, Ready};

use crate::{RuntimeError, RuntimeResult};

#[async_trait]
pub trait YcComponent
where
    Self: Send + Sync,
{
    async fn start_up(&self) -> RuntimeResult<()> {
        Ok(())
    }
}

pub struct Component<T> {
    inner: Arc<T>,
}

impl<T: Sized + 'static> FromRequest for Component<T> {
    type Error = RuntimeError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let component = req.app_data::<web::Data<T>>().unwrap();
        let component = component.clone();
        ok(Component {
            inner: component.into_inner(),
        })
    }
}
