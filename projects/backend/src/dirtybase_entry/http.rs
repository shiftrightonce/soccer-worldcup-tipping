mod api;
mod general;
mod open_api;

use dirtybase_contract::prelude::RouterManager;

pub(crate) fn register_routes(manager: &mut RouterManager) {
    // Prefix all routes with the crate's name

    // general routes
    manager.general(None, |router| {
        router.get_x("/", general::index_handler);
    });

    // v1 routes
    api::v1::register_routes(manager);

    // open api routes
    open_api::register_routes(manager);
}
