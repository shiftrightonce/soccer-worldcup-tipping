mod open_user_controller;

use dirtybase_contract::prelude::RouterManager;

pub fn register_routes(manager: &mut RouterManager) {
    manager.insecure_api(None, |router| {
        router.post_x("/siginup", open_user_controller::sginup_handler);
        // router.get_x("/verify-email", user_controller::verify_email_handler);
        router.get_x("/health", || async { "OK" });
    });
}
