use crate::{RuntimeError, RuntimeResult};
use actix_web::{web, App, FromRequest, Handler, HttpServer, Responder, ResponseError};
use std::{env, str::FromStr};

use tracing::{info, Level};

#[derive(Default)]
pub struct Runtime {
    #[cfg(feature = "ydb")]
    pub(crate) ydb: Option<crate::components::YdbComponent>,
}

pub fn runtime() -> Runtime {
    Runtime::default()
}

const PORT: &str = "PORT";

impl Runtime {
    pub async fn run<F, Args>(self, handler: F) -> RuntimeResult<()>
    where
        F: Handler<Args> + Send,
        Args: FromRequest + 'static,
        F::Output: Responder + 'static,
    {
        init_logs();
        info!("Starting components");

        #[cfg(feature = "ydb")]
        {
            if let Some(ref ydb) = self.ydb {
                ydb.start_up().await?;
            }
        }

        HttpServer::new(move || {
            let mut app = App::new();

            #[cfg(feature = "ydb")]
            {
                if let Some(ydb) = &self.ydb {
                    let ydb = ydb.clone();
                    app = app.app_data(web::Data::new(ydb));
                }
            }

            app.route("/", web::post().to(handler.clone()))
                .route("/", web::get().to(handler.clone()))
        })
        .bind(("0.0.0.0", get_port()))?
        .run()
        .await?;

        Ok(())
    }
}

fn get_port() -> u16 {
    let port = env::var(PORT)
        .unwrap_or("8080".to_owned())
        .parse()
        .unwrap_or(8080);
    info!("Listening on port {}", port);
    port
}
fn init_logs() {
    let level = env::var("RUST_LOG").unwrap_or("INFO".to_string());
    let log_level = Level::from_str(&level).unwrap();
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Error setting subscriber");
}

impl ResponseError for RuntimeError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}
