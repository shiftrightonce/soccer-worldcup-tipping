use dirtybase_contract::ExtensionSetup;
use tracing_subscriber::EnvFilter;

pub mod dirtybase_entry;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        // .with_max_level(Level::DEBUG)
        .try_init()
        .expect("could not setup tracing");

    dirtybase_entry::Extension.register().await;
    dirtybase_app::setup_and_run()
        .await
        .expect("could not setup and run application");
}
