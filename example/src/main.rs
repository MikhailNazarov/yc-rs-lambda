use yc_runtime::{ydb::YdbComponent, Component, Context, RuntimeResult};

#[tokio::main]
async fn main() -> RuntimeResult<()> {
    yc_runtime::builder().use_ydb()?.build()?.run(handler).await
}

async fn handler(context: Context, /*, ydb: Component<YdbComponent>*/) -> RuntimeResult<String> {
    Ok("Hello, World3!".to_string())
}
