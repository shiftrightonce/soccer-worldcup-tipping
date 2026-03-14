pub(crate) mod seed_app;
use dirtybase_contract::db_contract::SeederRegisterer;

pub(crate) async fn register_seeders() {
    SeederRegisterer::register("seed_app", |manager, context| {
        Box::pin(async move { seed_app::seed(manager, context).await })
    })
    .await;

    //
}
