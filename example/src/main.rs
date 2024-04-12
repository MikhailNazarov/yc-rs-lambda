use yc_runtime::prelude::*;

#[tokio::main]
async fn main() -> RuntimeResult<()> {
    runtime().use_ydb().run(handler).await
}

async fn handler(event: Event, ydb: Component<YdbComponent>) -> RuntimeResult<String> {
    let _table = ydb.table_client();

    for _message in event.messages {
        // store in db
    }

    Ok("Complete!".to_string())
}
