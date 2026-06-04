use dirtybase_app::{
    auth::AuthConfig, axum::Json, clap::error::ErrorKind::ValueValidation, db::types::ArcUuid7,
    helper::time::now,
};
use dirtybase_common::anyhow;
use dirtybase_contract::{
    auth_contract::{
        Actor, ActorPayload, ActorRole, AuthUserStatus, FetchActorOption, FetchActorPayload,
        PermStorageProvider, PermissionManager, PermissionRepo, PermissionStorage,
        PersistActorPayload, PersistActorRolePayload,
    },
    http_contract::api::{ApiError, ApiResponse},
    prelude::{Context, CtxExt, IntoResponse, Path, axum_extra::extract::Query},
};
use serde::Deserialize;
use validator::*;

use crate::dirtybase_entry::{
    PLAYER_ROLE,
    model::{
        user::{User, UserRepo},
        user_validation::{UserValidationRepo, ValidationPurpose},
    },
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

pub async fn verify_email_handler(
    CtxExt(mut repo): CtxExt<UserValidationRepo>,
    CtxExt(auth_config): CtxExt<AuthConfig>,
    CtxExt(storage): CtxExt<PermStorageProvider>,
    Query(data): Query<VerifyEmailQuery>,
) -> impl IntoResponse {
    tracing::info!("token from user request: {}", &data.token);
    match repo.validate(&data.token).await {
        Ok(record) => {
            if let Some(user) = record.user {
                let payload = FetchActorPayload::ById {
                    id: user.auth_actor_id.unwrap(),
                };
                let mut option = FetchActorOption::default();
                option.with_actor_roles = true;
                option.with_roles = true;

                if let Ok(Some(mut actor)) = storage.fetch_actor(payload, Some(option)).await {
                    let mut payload = ActorPayload::new();
                    payload = match record.purpose {
                        ValidationPurpose::Email => {
                            payload.verified_at = Some(now().as_datetime());
                            payload.status = Some(AuthUserStatus::Active);
                            payload.reset_password = Some(false);
                            payload
                        }
                        ValidationPurpose::PasswordReset => {
                            payload.reset_password = Some(true);
                            payload
                        }
                    };

                    actor.merge(payload);
                    let data = PersistActorPayload::Save {
                        actor: actor.clone(),
                    };

                    return ApiResponse::from(
                        save_actor_and_generate_token(actor, data, &storage, &auth_config).await,
                    );
                }
            }
        }
        Err(e) => {
            tracing::error!("error validating: {}", e);
        }
    }
    ApiResponse::forbidden().with_message("validation failed")
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

#[derive(Debug, Deserialize)]
pub(crate) struct VerifyEmailQuery {
    token: String,
}

pub async fn save_actor_and_generate_token(
    mut actor: Actor,
    data: PersistActorPayload,
    storage: &PermStorageProvider,
    auth_config: &AuthConfig,
) -> Result<String, anyhow::Error> {
    _ = storage.save_actor(data).await?;
    if let Some(role) = actor.roles().first().cloned() {
        actor.set_current_role(role);
    }

    if let Ok(token) = actor.generate_signed_jwt(auth_config.jwt_key().as_ref()) {
        return Ok(token);
    }

    Err(anyhow::anyhow!("No role assigned to actor"))
}
