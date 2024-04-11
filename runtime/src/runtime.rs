use crate::{components::YcComponent, Context, RuntimeError, RuntimeResult};
use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    get, post, web, App, Error, FromRequest, Handler, HttpServer, Responder, ResponseError,
};
use futures::Future;
use std::{env, str::FromStr, sync::Arc};
use tokio::{runtime::Handle, sync::Mutex};
use tracing::{info, Level};
#[derive(Clone)]
pub struct YcRuntime {
    pub(crate) components: Arc<Mutex<Vec<ComponentRef>>>,
}

pub type ComponentRef = Arc<dyn YcComponent>;

const PORT: &str = "PORT";

impl YcRuntime {
    pub async fn run<F, Args>(self, handler: F) -> RuntimeResult<()>
    where
        F: Handler<Args> + Send,
        Args: FromRequest + 'static,
        F::Output: Responder + 'static,
    {
        init_logs();
        info!("Starting components");
        let components = self.components.clone();

        let tasks = {
            let guard = components.lock().await;
            let mut tasks = vec![];
            for component in guard.iter() {
                let inner = component.clone();
                let task = tokio::spawn(async move {
                    inner.start_up().await.unwrap();
                });
                tasks.push(task);
            }
            tasks
        };
        futures::future::join_all(tasks).await;

        let components = { components.lock().await.clone() };

        HttpServer::new(move || {
            add_components(App::new(), components.clone())
                .app_data(web::Data::new(self.clone()))
                .route("/", web::post().to(handler.clone()))
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

fn add_components<T>(mut app: App<T>, components: Vec<ComponentRef>) -> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    for component in components {
        app = app.app_data(web::Data::new(component));
    }
    app
}
