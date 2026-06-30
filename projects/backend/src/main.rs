use dirtybase_contract::ExtensionSetup;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub mod dirtybase_entry;

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily("./data/logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let app = dirtybase_app::setup()
        .await
        .expect("could not setup application");

    let file_layer = fmt::layer()
        .with_writer(non_blocking) // write to file
        .with_level(true)
        .with_ansi(false)
        .with_filter(EnvFilter::new("error"));

    let stdout_layer = fmt::layer()
        .with_target(false)
        .with_writer(std::io::stdout) // write to console
        .with_ansi(true)
        .with_filter(EnvFilter::new("error"));

    tracing_subscriber::registry()
        .with(file_layer)
        .with(stdout_layer)
        .init();

    dirtybase_entry::Extension.register().await;
    dirtybase_app::run(app)
        .await
        .expect("could not setup and run application");
}
