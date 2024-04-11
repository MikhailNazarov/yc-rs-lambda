use yc_runtime::{ydb::YdbComponent, Component, RuntimeResult};

#[tokio::main]
async fn main() -> RuntimeResult<()> {
    yc_runtime::runtime().use_ydb().run(handler).await
}

async fn handler(ydb: Component<YdbComponent>) -> RuntimeResult<String> {
    let _table = ydb.table_client();
    Ok("Hello, World!".to_string())
}
