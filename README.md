# Yandex Cloud Rust Lambda

This project is my attempt to make a minimal framework for creating
serverless containers in Yandex Cloud

## THIS LIBRARY IS UNDER HEAVY DEVELOPMENT

```rust
use yc_runtime::{ydb::YdbComponent, Component, RuntimeResult};

#[tokio::main]
async fn main() -> RuntimeResult<()> {
    yc_runtime::runtime().use_ydb().run(handler).await
}

async fn handler(ydb: Component<YdbComponent>) -> RuntimeResult<String> {
    let _table = ydb.table_client();
    Ok("Hello, World!".to_string())
}

```