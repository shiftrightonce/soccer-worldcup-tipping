mod open_user_controller;

use dirtybase_contract::prelude::RouterManager;

pub(crate) use open_user_controller::SignInReponse;

pub fn register_routes(manager: &mut RouterManager) {
    manager.insecure_api(Some("/v1"), |router| {
        router.post_x("/signup", open_user_controller::sginup_handler);
        router.post_x("/login", open_user_controller::login_handler);
        router.get(
            "/verify-email",
            open_user_controller::verify_email_handler,
            "verify_email",
        );
        router.get_x("/health", || async { "OK" });
    });
}
