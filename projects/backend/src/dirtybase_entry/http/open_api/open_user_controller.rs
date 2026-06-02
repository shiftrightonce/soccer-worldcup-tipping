use dirtybase_app::{axum::Json, db::types::ArcUuid7};
use dirtybase_contract::{
    auth_contract::{
        Actor, ActorPayload, ActorRole, AuthUserStatus, PermStorageProvider, PermissionStorage,
        PersistActorPayload, PersistActorRolePayload,
    },
    http_contract::api::{ApiError, ApiResponse},
    prelude::{Context, CtxExt, IntoResponse, Path},
};
use serde::Deserialize;
use validator::*;

use crate::dirtybase_entry::{
    PLAYER_ROLE,
    model::user::{User, UserRepo},
};

pub async fn sginup_handler(
    CtxExt(mut user_repo): CtxExt<UserRepo>,
    CtxExt(context): CtxExt<Context>,
    CtxExt(storage): CtxExt<PermStorageProvider>,
    Json(mut payload): Json<ActorPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return ApiResponse::validation_error(e);
    }

    payload.status = Some(AuthUserStatus::Pending);
    let email = payload.email.clone().unwrap_or_default();

    let actor_save_result = storage
        .save_actor(PersistActorPayload::Save {
            actor: payload.into(),
        })
        .await;
    if actor_save_result.is_err() {
        return ApiResponse::error("Failed to create user. Username is likely already taken");
    }

    if let Some(actor) = actor_save_result.ok().flatten() {
        if let Ok(Some(role)) = storage
            .fetch_role(
                dirtybase_contract::auth_contract::FetchRolePayload::ByName {
                    name: PLAYER_ROLE.to_string().into(),
                },
                None,
            )
            .await
        {
            let actor_id = actor.id().cloned().unwrap();
            // Assgin default role
            let actor_role_payload = PersistActorRolePayload::Save {
                record: ActorRole::new(actor_id.clone(), role.id().cloned().unwrap()),
            };

            if let Err(e) = storage.save_actor_role(actor_role_payload).await {
                let api_error = ApiError::new(
                    "failed_to_assign_role",
                    "Failed to assign role",
                    "failed to assign role",
                    Some(format!("Failed to assign role: {e}")),
                );
                return ApiResponse::error(api_error);
            }

            let user = User::new(&email, actor_id);
            match user_repo.register(user, context).await {
                Ok(user) => {
                    return ApiResponse::success(user);
                }
                Err(e) => return ApiResponse::error(format!("Failed to create user: {e}")),
            }
        }
    }

    ApiResponse::error("Failed to create user")
}

pub fn verify_email_handler(
    CtxExt(mut user_repo): CtxExt<UserRepo>,
    CtxExt(storage): CtxExt<PermStorageProvider>,
    Path((user_id, token)): Path<(ArcUuid7, String)>,
) -> impl IntoResponse {
    ApiResponse::success("Email verified")
}

pub fn reset_password_request_handler(
    CtxExt(mut user_repo): CtxExt<UserRepo>,
    CtxExt(storage): CtxExt<PermStorageProvider>,
    Json(payload): Json<ResetPasswordRequestPayload>,
) -> impl IntoResponse {
    ApiResponse::success("Password reset requested")
}

#[derive(Debug, Deserialize, Validate, ts_rs::TS)]
#[ts(export_to = "v1/")]
pub struct ResetPasswordRequestPayload {
    #[validate(email(message = "must be a valid email address"))]
    pub email: String,
}
