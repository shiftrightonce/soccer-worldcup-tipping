use std::collections::HashMap;

use dirtybase_app::{auth::AuthConfig, axum::Json, helper::time::now};
use dirtybase_common::anyhow;
use dirtybase_contract::{
    auth_contract::{
        Actor, ActorPayload, ActorRole, AuthUserStatus, FetchActorOption, FetchActorPayload,
        PermStorageProvider, PermissionStorage, PersistActorPayload, PersistActorRolePayload, Role,
    },
    http_contract::api::{ApiError, ApiResponse},
    prelude::{Context, CtxExt, IntoResponse, axum_extra::extract::Query},
};
use serde::{Deserialize, Serialize};
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
    CtxExt(auth_config): CtxExt<AuthConfig>,
    CtxExt(context): CtxExt<Context>,
    CtxExt(storage): CtxExt<PermStorageProvider>,
    Json(payload): Json<SignupPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return ApiResponse::validation_error(e);
    }
    let email = payload.email.clone();

    let actor_save_result = storage
        .save_actor(PersistActorPayload::Save {
            actor: ActorPayload::from(payload).into(),
        })
        .await;
    if actor_save_result.is_err() {
        return ApiResponse::error("Failed to create user. Username is likely already taken");
    }

    if let Some(mut actor) = actor_save_result.ok().flatten() {
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
                    actor.set_current_role(role.clone());
                    if let Ok(token) = actor.generate_signed_jwt(auth_config.jwt_key().as_ref()) {
                        return ApiResponse::success(SignInReponse {
                            token,
                            user,
                            roles: vec![role],
                        });
                    }
                }
                Err(e) => return ApiResponse::error(format!("Failed to create user: {e}")),
            }
        }
    }

    ApiResponse::error("Failed to create user")
}

pub async fn login_handler(
    CtxExt(mut user_repo): CtxExt<UserRepo>,
    CtxExt(auth_config): CtxExt<AuthConfig>,
    CtxExt(storage): CtxExt<PermStorageProvider>,
    Json(cred): Json<SignInPayload>,
) -> impl IntoResponse {
    if let Err(e) = cred.validate() {
        return ApiResponse::validation_error(e);
    }

    let mut option = FetchActorOption::default();
    option.with_roles = true;
    option.with_actor_roles = true;

    let result = if cred.email.is_some() {
        let payload = FetchActorPayload::by_email(&cred.email.clone().unwrap());
        storage.fetch_actor(payload, Some(option)).await
    } else {
        let payload = FetchActorPayload::by_username(&cred.username.clone().unwrap());
        storage.fetch_actor(payload, Some(option)).await
    };

    if let Ok(Some(mut actor)) = result
        && actor.verify_password(&cred.password)
    {
        user_repo.with_actor_roles();
        let user = if let Ok(Some(u)) = user_repo.by_actor_id(actor.id().cloned().unwrap()).await {
            u
        } else {
            return ApiResponse::bad_request().with_message("could not handled your request");
        };

        if let Some(role) = actor.roles().first().cloned() {
            actor.set_current_role(role);
        }

        if let Ok(token) = actor.generate_signed_jwt(auth_config.jwt_key().as_ref()) {
            return ApiResponse::success(SignInReponse {
                user,
                token,
                roles: actor.roles().clone().into(),
            });
        }
    }

    ApiResponse::bad_request().with_message("could not authenticate user")
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

#[derive(Debug, Deserialize, ts_rs::TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v1/")]
pub(crate) struct SignupPayload {
    #[validate(length(
        min = 3,
        max = 16,
        message = "username must be between 3 to 16 characters"
    ))]
    pub(crate) username: String,
    #[validate(email)]
    pub(crate) email: String,
    #[validate(must_match(other = "confirm_password"), length(min = 8))]
    pub(crate) password: String,
    pub(crate) confirm_password: String,
}

impl From<SignupPayload> for ActorPayload {
    fn from(value: SignupPayload) -> Self {
        let mut payload = Self::default();
        payload.email = value.email.into();
        payload.username = value.username.into();
        payload.password = value.password.into();
        payload.status = Some(AuthUserStatus::Active);
        payload
    }
}

#[derive(Debug, Deserialize, ts_rs::TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v1/")]
#[validate(schema(function = "validate_signin_payload", skip_on_field_errors = false))]
pub(crate) struct SignInPayload {
    pub(crate) username: Option<String>,
    #[validate(email)]
    pub(crate) email: Option<String>,
    pub(crate) password: String,
}

#[allow(dead_code)]
fn validate_signin_payload(payload: &SignInPayload) -> Result<(), ValidationError> {
    if payload.username.is_none() && payload.email.is_none() {
        return Err(ValidationError {
            code: "missing_credential".into(),
            message: Some("Username or Email is required".into()),
            params: HashMap::default(),
        });
    }
    Ok(())
}

#[derive(Debug, Serialize, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v1/")]
pub(crate) struct SignInReponse {
    pub user: User,
    pub token: String,
    #[ts(type = "[{name: string, label: string}]")]
    pub roles: Vec<Role>,
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
