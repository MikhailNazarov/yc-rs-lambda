use std::{env, future::Ready, sync::Arc, time::Duration};

use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use tokio::time::timeout;
use ydb::{FromEnvCredentials, YdbError};

use crate::{RuntimeBuilder, RuntimeError, RuntimeResult};

use super::YcComponent;

impl RuntimeBuilder {
    pub fn use_ydb(self) -> RuntimeResult<Self> {
        Ok(self.add_component(Arc::new(YdbComponent::new()?)))
    }
}

pub struct YdbComponent {
    client: ydb::Client,
    connect_timeout: Duration,
}

const YDB_CONNECTION_STRING: &str = "YDB_CONNECTION_STRING";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

impl YdbComponent {
    fn new() -> RuntimeResult<Self> {
        let connection_string = env::var(YDB_CONNECTION_STRING)
            .map_err(|_| RuntimeError::RequiredEnv(YDB_CONNECTION_STRING.to_string()))?;
        Ok(Self {
            connect_timeout: DEFAULT_TIMEOUT,
            client: ydb::ClientBuilder::new_from_connection_string(connection_string)?
                .with_credentials(FromEnvCredentials::new()?)
                .client()?,
        })
    }
}
#[async_trait::async_trait]
impl YcComponent for YdbComponent {
    async fn start_up(&self) -> RuntimeResult<()> {
        timeout(self.connect_timeout, self.client.wait()).await??;
        Ok(())
    }
}

impl From<YdbError> for RuntimeError {
    fn from(err: YdbError) -> Self {
        RuntimeError::ComponentError(err.to_string())
    }
}
