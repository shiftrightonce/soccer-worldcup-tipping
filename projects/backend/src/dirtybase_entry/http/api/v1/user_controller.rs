use dirtybase_app::{auth::AuthConfig, axum::Json, db::types::ArcUuid7};
use dirtybase_contract::{
    auth_contract::{
        Actor, ActorPayload, FetchActorOption, FetchActorPayload, PermStorageProvider,
        PermissionStorage, PersistActorPayload,
    },
    http_contract::api::ApiResponse,
    prelude::{CtxExt, IntoResponse, Path},
};
use validator::Validate;

use crate::dirtybase_entry::{
    http::open_api::SignInReponse,
    model::user::{User, UserRepo},
};

pub async fn list_handler(CtxExt(mut user_repo): CtxExt<UserRepo>) -> impl IntoResponse {
    let result = ApiResponse::from(user_repo.with_actor().paginate(None).await);
    result
}

pub async fn get_me_handler(
    CtxExt(mut user_repo): CtxExt<UserRepo>,
    CtxExt(actor): CtxExt<Actor>,
) -> impl IntoResponse {
    if let Some(id) = actor.id().cloned() {
        return ApiResponse::<User>::from(
            user_repo
                .with_actor()
                .with_actor_roles()
                // .with_roles()
                .by_actor_id(id)
                .await,
        );
    }
    ApiResponse::not_found()
}

/// A user view another user
pub async fn view_player_handler(
    CtxExt(mut user_repo): CtxExt<UserRepo>,
    Path(user_id): Path<ArcUuid7>,
) -> impl IntoResponse {
    ApiResponse::<User>::from(user_repo.by_id(user_id).await)
}

// Admin viewing a user
pub async fn get_user_handler(
    CtxExt(mut user_repo): CtxExt<UserRepo>,
    Path(user_id): Path<ArcUuid7>,
) -> impl IntoResponse {
    ApiResponse::<User>::from(
        user_repo
            .with_actor()
            .with_trashed()
            .with_actor_roles()
            // .with_roles()
            .by_id(user_id)
            .await,
    )
}

pub async fn update_credential_handler(
    CtxExt(mut user_repo): CtxExt<UserRepo>,
    CtxExt(actor): CtxExt<Actor>,
    CtxExt(auth_config): CtxExt<AuthConfig>,
    CtxExt(storage): CtxExt<PermStorageProvider>,
    Json(mut cred): Json<ActorPayload>,
) -> impl IntoResponse {
    if let Err(e) = cred.validate() {
        return ApiResponse::validation_error(e);
    }

    let mut option = FetchActorOption::default();
    option.with_roles = true;
    option.with_actor_roles = true;

    let payload = FetchActorPayload::by_id(actor.id().cloned().unwrap());

    if let Ok(Some(mut actor)) = storage.fetch_actor(payload.clone(), None).await {
        cred.rotate_salt = true;
        cred.id = actor.id().cloned();
        let email = cred.email.clone();
        actor.merge(cred);
        if let Ok(Some(actor)) = storage
            .save_actor(PersistActorPayload::Save { actor: actor })
            .await
        {
            if email.is_some() {
                if let Ok(Some(mut user)) =
                    user_repo.by_actor_id(actor.id().cloned().unwrap()).await
                {
                    user.email = email.unwrap().into();
                    if let Err(e) = user_repo.update(user).await {
                        return ApiResponse::internal_error().with_message(&format!("{}", e));
                    }
                } else {
                    return ApiResponse::not_found().with_message("could not find user");
                }
            }

            if let Ok(Some(actor)) = storage.fetch_actor(payload, Some(option)).await {
                user_repo.with_actor_roles();
                let user = if let Ok(Some(u)) =
                    user_repo.by_actor_id(actor.id().cloned().unwrap()).await
                {
                    u
                } else {
                    return ApiResponse::bad_request()
                        .with_message("could not handled your request");
                };

                if let Ok(token) = actor.generate_signed_jwt(auth_config.jwt_key().as_ref()) {
                    return ApiResponse::success(SignInReponse {
                        user,
                        token,
                        roles: actor.roles().into(),
                    });
                }
            }
        }
    }

    ApiResponse::bad_request()
        .with_message("could not save your chagnes. Username may not be available")
}
