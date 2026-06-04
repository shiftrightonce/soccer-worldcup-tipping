mod open_user_controller;

use dirtybase_contract::prelude::RouterManager;

pub fn register_routes(manager: &mut RouterManager) {
    manager.insecure_api(None, |router| {
        router.post_x("/siginup", open_user_controller::sginup_handler);
        router.get(
            "/verify-email",
            open_user_controller::verify_email_handler,
            "verify_email",
        );
        router.get_x("/health", || async { "OK" });
    });
}
