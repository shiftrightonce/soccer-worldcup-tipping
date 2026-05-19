use dirtybase_app::db::{
    base::paginate_builder::PaginateBuilder,
    types::{ArcUuid7, StringField},
};
use dirtybase_contract::{
    http_contract::api::ApiResponse,
    prelude::{CtxExt, IntoResponse, Path},
};

use crate::dirtybase_entry::model::country::{Country, CountryRepo};

pub async fn all_handler(CtxExt(mut repo): CtxExt<CountryRepo>) -> impl IntoResponse {
    ApiResponse::from(repo.get().await)
}

pub async fn list_handler(
    CtxExt(mut repo): CtxExt<CountryRepo>,
    Path(tournament_id): Path<ArcUuid7>,
    page: PaginateBuilder,
) -> impl IntoResponse {
    ApiResponse::from(repo.paginate_by_tournament(tournament_id, Some(page)).await)
}

pub async fn get_handler(
    CtxExt(mut repo): CtxExt<CountryRepo>,
    Path((tournament_id, id)): Path<(ArcUuid7, ArcUuid7)>,
) -> impl IntoResponse {
    ApiResponse::<Country>::from(repo.by_tournament_and_id(tournament_id, id).await)
}

pub async fn create_handler(
    CtxExt(mut repo): CtxExt<CountryRepo>,
    Path(tournament_id): Path<ArcUuid7>,
) {
}

pub struct CountryPayload {
    pub name: Option<StringField>,
    pub short: Option<StringField>,
    pub tournamen_id: ArcUuid7,
    pub is_out: Option<bool>,
}
