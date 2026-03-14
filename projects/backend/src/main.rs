use dirtybase_contract::ExtensionSetup;

pub mod dirtybase_entry;

#[tokio::main]
async fn main() {
    dirtybase_entry::Extension.register().await;
    dirtybase_app::setup_and_run()
        .await
        .expect("could not setup and run application");
}
