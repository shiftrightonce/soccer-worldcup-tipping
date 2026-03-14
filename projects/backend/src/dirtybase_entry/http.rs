mod general;
use dirtybase_contract::prelude::RouterManager;

pub(crate) fn register_routes(manager: &mut RouterManager) {
    // Prefix all routes with the crate's name

    // general routes
    manager.general(None, |router| {
        router.get_x("/", general::index_handler);
    });
}
