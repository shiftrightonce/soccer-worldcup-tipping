use dirtybase_contract::{
    http_contract::api::ApiResponse,
    prelude::{CtxExt, IntoResponse},
};

use crate::dirtybase_entry::model::tip_strategy::TipStrategyRepo;

pub async fn list_handler(
    CtxExt(mut tip_srategy_repo): CtxExt<TipStrategyRepo>,
) -> impl IntoResponse {
    let result = ApiResponse::from(tip_srategy_repo.paginate(None).await);
    result
}
