mod tip_strategy_controller;
mod user_controller;
use dirtybase_contract::prelude::RouterManager;

pub fn register_routes(manager: &mut RouterManager) {
    manager.api(Some("/v1"), |router| {
        // Strategies
        router.nest("/strategies", |router| {
            router.get_x("/", tip_strategy_controller::list_handler);
        });

        // Users
        router.nest("/users", |router| {
            router.get_x("/", user_controller::list_handler);
        });
    });
}
