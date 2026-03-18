use dirtybase_contract::{
    http_contract::api::ApiResponse,
    prelude::{CtxExt, IntoResponse},
};

use crate::dirtybase_entry::model::user::UserRepo;

pub async fn list_handler(CtxExt(mut user_repo): CtxExt<UserRepo>) -> impl IntoResponse {
    let result = ApiResponse::from(user_repo.with_actor().paginate(None).await);
    result
}
