use dirtybase_app::{
    axum::Json,
    db::{
        base::paginate_builder::PaginateBuilder,
        types::{ArcStrField, ArcUuid7, LabelField},
    },
};
use dirtybase_contract::{
    http_contract::api::ApiResponse,
    prelude::{CtxExt, IntoResponse, Path},
};
use serde::Deserialize;

use crate::dirtybase_entry::model::tournament::{Tournament, TournamentRepo, TournamentStatus};

pub async fn get_by_id_handler(
    CtxExt(mut repo): CtxExt<TournamentRepo>,
    Path(id): Path<ArcUuid7>,
) -> impl IntoResponse {
    ApiResponse::<Tournament>::from(repo.by_id(id).await)
}

pub async fn create_handler(
    CtxExt(mut repo): CtxExt<TournamentRepo>,
    Json(payload): Json<TournamentPayload>,
) -> impl IntoResponse {
    let record = Tournament::from(payload);
    ApiResponse::<Tournament>::from(repo.insert(record).await)
}

pub async fn update_handler(
    CtxExt(mut repo): CtxExt<TournamentRepo>,
    Path(tournament_id): Path<ArcUuid7>,
    Json(payload): Json<TournamentPayload>,
) -> impl IntoResponse {
    let mut record = Tournament::from(payload);
    record.id = Some(tournament_id);
    ApiResponse::<Tournament>::from(repo.update(record).await)
}

pub async fn list_handler(
    CtxExt(mut repo): CtxExt<TournamentRepo>,
    page: PaginateBuilder,
) -> impl IntoResponse {
    ApiResponse::from(repo.paginate(Some(page)).await)
}

#[derive(Debug, Clone, Deserialize, ts_rs::TS)]
#[ts(export)]
#[ts(export_to = "v1/")]
pub(crate) struct TournamentPayload {
    #[ts(type = "string")]
    label: LabelField,
    description: ArcStrField,
    status: TournamentStatus,
}

impl From<TournamentPayload> for Tournament {
    fn from(value: TournamentPayload) -> Self {
        Self {
            id: Some(ArcUuid7::default()),
            name: value.label.to_string().into(),
            label: value.label,
            description: value.description,
            status: value.status,
            ..Default::default()
        }
    }
}
