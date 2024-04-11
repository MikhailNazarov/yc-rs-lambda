#[cfg(feature = "ydb")]
mod ydb;
#[cfg(feature = "ydb")]
pub use self::ydb::YdbComponent;

use std::{ops::Deref, sync::Arc};

use actix_web::{web, FromRequest, HttpRequest};

use futures::future::{ok, Ready};

use crate::RuntimeError;

pub struct Component<T> {
    inner: Arc<T>,
}

impl<T> Deref for Component<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
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
