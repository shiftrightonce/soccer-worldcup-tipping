use dirtybase_app::{
    axum::Json,
    db::{
        base::paginate_builder::PaginateBuilder,
        types::{ArcUuid7, IntegerField},
    },
};
use dirtybase_contract::{
    http_contract::api::ApiResponse,
    prelude::{CtxExt, IntoResponse, Path, Query, StatusCode},
};
use serde::Deserialize;

use crate::dirtybase_entry::model::group::{CountryGroup, CountryGroupRepo};

pub async fn list_handler(
    CtxExt(mut repo): CtxExt<CountryGroupRepo>,
    Path(tournament_id): Path<ArcUuid7>,
    Query(parameters): Query<Parameters>,
    page: PaginateBuilder,
) -> impl IntoResponse {
    parameters.apply(&mut repo);
    ApiResponse::from(repo.paginate_by_tournament(tournament_id, Some(page)).await)
}

pub async fn by_group_handler(
    CtxExt(mut repo): CtxExt<CountryGroupRepo>,
    Path((tournament_id, group_id)): Path<(ArcUuid7, ArcUuid7)>,
) -> impl IntoResponse {
    ApiResponse::from(repo.by_tournament_and_group(tournament_id, group_id).await)
}

pub async fn get_handler(
    CtxExt(mut repo): CtxExt<CountryGroupRepo>,
    Path((tournament_id, id)): Path<(ArcUuid7, ArcUuid7)>,
) -> impl IntoResponse {
    ApiResponse::<CountryGroup>::from(repo.by_tournament_and_id(tournament_id, id).await)
}

pub async fn all_handler(
    CtxExt(mut repo): CtxExt<CountryGroupRepo>,
    Path(tournament_id): Path<ArcUuid7>,
    Query(params): Query<Parameters>,
) -> impl IntoResponse {
    params.apply(&mut repo);
    ApiResponse::from(repo.all_by_tournament(tournament_id).await)
}

pub async fn create_handler(
    CtxExt(mut repo): CtxExt<CountryGroupRepo>,
    Path(tournament_id): Path<ArcUuid7>,
    Json(mut payload): Json<CountryGroupPayload>,
) -> impl IntoResponse {
    payload.tournament_id = tournament_id;
    ApiResponse::from(repo.insert(payload.into()).await)
}

pub async fn update_handler(
    CtxExt(mut repo): CtxExt<CountryGroupRepo>,
    Path((tournament_id, id)): Path<(ArcUuid7, ArcUuid7)>,
    Json(payload): Json<CountryGroupPayload>,
) -> impl IntoResponse {
    if let Ok(Some(existing)) = repo.by_id(id).await
        && existing.tournament_id == tournament_id
    {
        ApiResponse::from(repo.update(payload.merge(existing)).await)
    } else {
        ApiResponse::error_with_status("Not found", StatusCode::NOT_FOUND)
    }
}

#[derive(Debug, Default, Clone, Deserialize, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v1/")]
pub struct CountryGroupPayload {
    #[ts(type = "string | null")]
    tournament_id: ArcUuid7,
    #[ts(type = "string | null")]
    group_id: Option<ArcUuid7>,
    #[ts(type = "string | null")]
    country_id: Option<ArcUuid7>,
    is_out: Option<bool>,
    #[ts(type = "number")]
    points: Option<IntegerField>,
}

impl CountryGroupPayload {
    pub fn merge(self, mut existing: CountryGroup) -> CountryGroup {
        if let Some(group_id) = self.group_id {
            existing.group_id = group_id;
        }

        if let Some(country_id) = self.country_id {
            existing.country_id = country_id;
        }

        if let Some(is_out) = self.is_out {
            existing.is_out = is_out;
        }

        if let Some(points) = self.points {
            existing.points = points;
        }

        existing
    }
}

impl From<CountryGroupPayload> for CountryGroup {
    fn from(value: CountryGroupPayload) -> Self {
        let g = Self {
            id: Some(ArcUuid7::default()),
            tournament_id: value.tournament_id.clone(),
            ..Default::default()
        };

        value.merge(g)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Parameters {
    with: Option<String>,
    filter: Option<String>,
}

impl Parameters {
    pub fn apply(&self, repo: &mut CountryGroupRepo) {
        if let Some(with) = &self.with {
            let withs = with.split(',').map(|s| s.trim()).collect::<Vec<_>>();
            if withs.contains(&"country") {
                repo.with_country();
            }
            if withs.contains(&"group") {
                repo.with_group();
            }
        }

        if let Some(filter) = &self.filter {
            let filters = filter.split(',').map(|s| s.trim()).collect::<Vec<_>>();
            if filters.contains(&"still-in") {
                repo.where_is_out(false);
            }
        }
    }
}
