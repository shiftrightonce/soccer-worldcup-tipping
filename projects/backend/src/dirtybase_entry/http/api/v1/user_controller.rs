use dirtybase_app::db::types::ArcUuid7;
use dirtybase_contract::{
    auth_contract::Actor,
    http_contract::api::ApiResponse,
    prelude::{CtxExt, IntoResponse, Path},
};

use crate::dirtybase_entry::model::user::{User, UserRepo};

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
