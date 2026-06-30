mod open_user_controller;

use dirtybase_app::axum::Json;
use dirtybase_contract::prelude::{CtxExt, RouterManager};

pub(crate) use open_user_controller::SignInReponse;

use crate::dirtybase_entry::TipConfig;

pub fn register_routes(manager: &mut RouterManager) {
    manager.insecure_api(Some("/v1"), |router| {
        router.post_x("/signup", open_user_controller::sginup_handler);
        router.post_x("/login", open_user_controller::login_handler);
        router.get(
            "/verify-email",
            open_user_controller::verify_email_handler,
            "verify_email",
        );
        router.get_x("/health", |CtxExt(config): CtxExt<TipConfig>| async move {
            let mut response = serde_json::Map::<String, serde_json::value::Value>::new();
            response.insert(
                "status".to_string(),
                serde_json::value::Value::String("ok".to_string()),
            );
            response.insert(
                "version".to_string(),
                serde_json::value::Value::String(config.to_owned().version.to_string()),
            );

            Json(response)
        });
    });
}
