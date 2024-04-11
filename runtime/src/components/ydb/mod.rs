use std::{env, ops::Deref, sync::Arc, time::Duration};

use tokio::time::timeout;
use ydb::{FromEnvCredentials, YdbError};

use crate::{Runtime, RuntimeError, RuntimeResult};

#[derive(Clone)]
pub struct YdbComponent {
    client: Arc<ydb::Client>,
    connect_timeout: Duration,
}

impl Deref for YdbComponent {
    type Target = Arc<ydb::Client>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

const YDB_CONNECTION_STRING: &str = "YDB_CONNECTION_STRING";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

impl Runtime {
    pub fn use_ydb(mut self) -> Self {
        self.ydb = Some(YdbComponent::new().unwrap());
        self
    }
}

impl YdbComponent {
    fn new() -> RuntimeResult<Self> {
        let connection_string = env::var(YDB_CONNECTION_STRING)
            .map_err(|_| RuntimeError::RequiredEnv(YDB_CONNECTION_STRING.to_string()))?;
        Ok(Self {
            connect_timeout: DEFAULT_TIMEOUT,
            client: Arc::new(
                ydb::ClientBuilder::new_from_connection_string(connection_string)?
                    .with_credentials(FromEnvCredentials::new()?)
                    .client()?,
            ),
        })
    }
}

impl YdbComponent {
    pub(crate) async fn start_up(&self) -> RuntimeResult<()> {
        timeout(self.connect_timeout, self.client.wait()).await??;
        Ok(())
    }
}

impl From<YdbError> for RuntimeError {
    fn from(err: YdbError) -> Self {
        RuntimeError::ComponentError(err.to_string())
    }
}
