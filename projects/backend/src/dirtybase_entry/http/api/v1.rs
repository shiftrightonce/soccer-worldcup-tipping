mod tip_strategy_controller;
use dirtybase_contract::prelude::RouterManager;

pub fn register_routes(manager: &mut RouterManager) {
    manager.api(Some("/v1"), |router| {
        router.get_x("/strategies", tip_strategy_controller::list_handler);
    });
}
