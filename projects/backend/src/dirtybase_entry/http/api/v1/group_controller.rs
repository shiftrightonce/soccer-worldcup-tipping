use dirtybase_contract::{
    http_contract::api::ApiResponse,
    prelude::{CtxExt, IntoResponse},
};

use crate::dirtybase_entry::model::group::GroupRepo;

pub async fn all_handler(CtxExt(mut repo): CtxExt<GroupRepo>) -> impl IntoResponse {
    ApiResponse::from(repo.get().await)
}
