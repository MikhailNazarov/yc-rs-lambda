# Yandex Cloud Rust Lambda

This project is my attempt to make a minimal framework for creating
serverless containers in Yandex Cloud

## THIS LIBRARY IS UNDER HEAVY DEVELOPMENT

```rust
use yc_runtime::prelude::*;

#[tokio::main]
async fn main() -> RuntimeResult<()> {
    runtime().use_ydb().run(handler).await
}

async fn handler(ydb: Component<YdbComponent>) -> RuntimeResult<String> {
    let _table = ydb.table_client();
    // do work with _table client

    Ok("Hello, World!".to_string())
}


```