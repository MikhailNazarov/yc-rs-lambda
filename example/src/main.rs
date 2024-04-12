use yc_runtime::{prelude::*, tracing::info};

#[tokio::main]
async fn main() -> RuntimeResult<()> {
    runtime().use_ydb().run(handler).await
}

async fn handler(
    context: Context,
    event: Option<Event>,
    ydb: Component<YdbComponent>,
) -> RuntimeResult<String> {
    info!("[{}] - {}", context.method, context.path);
    info!("Query: {}", context.query);
    let _table = ydb.table_client();

    if let Some(event) = event {
        info!("Event received: {} messages", event.messages.len());
        for _message in event.messages {

            // store in db
        }
    }

    Ok("Complete!".to_string())
}
