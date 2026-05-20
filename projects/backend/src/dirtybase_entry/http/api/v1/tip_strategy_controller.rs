use std::collections::HashSet;

use dirtybase_app::{
    axum::Json,
    db::{
        base::paginate_builder::PaginateBuilder,
        types::{ArcStrField, ArcUuid7, DateTimeField, LabelField},
    },
};
use dirtybase_contract::{
    http_contract::api::ApiResponse,
    prelude::{CtxExt, IntoResponse, Path},
};
use serde::Deserialize;

use crate::dirtybase_entry::model::tip_strategy::{StrategyType, TipStrategy, TipStrategyRepo};

pub async fn list_handler(
    CtxExt(mut tip_srategy_repo): CtxExt<TipStrategyRepo>,
    Path(tournament_id): Path<ArcUuid7>,
    page: PaginateBuilder,
) -> impl IntoResponse {
    ApiResponse::from(
        tip_srategy_repo
            .paginate_by_tournament(tournament_id, Some(page))
            .await,
    )
}

pub async fn all_handler(
    CtxExt(mut tip_srategy_repo): CtxExt<TipStrategyRepo>,
    Path(tournament_id): Path<ArcUuid7>,
) -> impl IntoResponse {
    ApiResponse::from(tip_srategy_repo.all_by_tournament_id(tournament_id).await)
}

pub async fn get_handler(
    CtxExt(mut tip_strategy_repo): CtxExt<TipStrategyRepo>,
    Path((tournament_id, id)): Path<(ArcUuid7, ArcUuid7)>,
) -> impl IntoResponse {
    ApiResponse::<TipStrategy>::from(
        tip_strategy_repo
            .by_tournament_and_id(tournament_id, id)
            .await,
    )
}

pub async fn create_handler(
    CtxExt(mut tip_strategy_repo): CtxExt<TipStrategyRepo>,
    Path(tournament_id): Path<ArcUuid7>,
    Json(mut payload): Json<TipStrategyPayload>,
) -> impl IntoResponse {
    payload.tournament_id = Some(tournament_id);
    let new_strategy = TipStrategy::from(payload);
    ApiResponse::<TipStrategy>::from(tip_strategy_repo.insert(new_strategy).await)
}

pub async fn update_handler(
    CtxExt(mut tip_strategy_repo): CtxExt<TipStrategyRepo>,
    Path((tournament_id, id)): Path<(ArcUuid7, ArcUuid7)>,
    Json(payload): Json<TipStrategyPayload>,
) -> impl IntoResponse {
    if let Ok(Some(mut strategy)) = tip_strategy_repo.by_id(id).await {
        strategy.label = payload.label;
        strategy.description = payload.description;
        strategy.game_id = payload.game_id;
        strategy.opens_at = payload.opens_at;
        strategy.ends_at = payload.ends_at;
        strategy.calculate_points_on = payload.calculate_points_on;
        strategy.completed = payload.completed.into();
        strategy.strategy_types = payload.strategy_types;
        return ApiResponse::from(
            tip_strategy_repo
                .update_by_tournament(tournament_id, strategy)
                .await,
        );
    }

    ApiResponse::error("Not found")
}

pub async fn delete_handler(
    CtxExt(mut tip_strategy_repo): CtxExt<TipStrategyRepo>,
    Path((tournament_id, id)): Path<(ArcUuid7, ArcUuid7)>,
) -> impl IntoResponse {
    if let Ok(Some(strategy)) = tip_strategy_repo.by_id(id).await {
        return ApiResponse::from(
            tip_strategy_repo
                .delete_by_tournament(tournament_id, strategy)
                .await,
        );
    }

    ApiResponse::error("Not found")
}

#[derive(Debug, Clone, Deserialize, ts_rs::TS)]
#[ts(export)]
#[ts(export_to = "v1/")]
pub(crate) struct TipStrategyPayload {
    #[ts(type = "string")]
    tournament_id: Option<ArcUuid7>,
    #[ts(type = "string")]
    label: LabelField,
    description: ArcStrField,
    #[ts(type = "string | null")]
    game_id: Option<ArcUuid7>,
    #[ts(type = "string")]
    opens_at: DateTimeField,
    #[ts(type = "string")]
    ends_at: DateTimeField,
    #[ts(type = "string")]
    calculate_points_on: Option<DateTimeField>,
    completed: bool,
    strategy_types: HashSet<StrategyType>,
}

impl From<TipStrategyPayload> for TipStrategy {
    fn from(value: TipStrategyPayload) -> Self {
        Self {
            id: Some(ArcUuid7::default()),
            tournament: None,
            tournament_id: value
                .tournament_id
                .expect("tip strategy payload missing tournament id"),
            label: value.label,
            description: value.description,
            game_id: value.game_id,
            game: None,
            opens_at: value.opens_at,
            ends_at: value.ends_at,
            calculate_points_on: value.calculate_points_on,
            completed: value.completed.into(),
            strategy_types: value.strategy_types,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}
