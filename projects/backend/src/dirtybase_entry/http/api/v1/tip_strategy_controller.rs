use std::collections::HashSet;

use dirtybase_app::{
    axum::Json,
    db::{
        base::paginate_builder::PaginateBuilder,
        types::{ArcStrField, ArcUuid7, DateTimeField, LabelField},
    },
};
use dirtybase_contract::{
    auth_contract::Actor,
    http_contract::api::ApiResponse,
    prelude::{CtxExt, IntoResponse, Path, Query},
};
use serde::Deserialize;

use crate::dirtybase_entry::model::{
    strategy_result::{StrategyResult, StrategyResultRepo},
    tip::TipRepo,
    tip_strategy::{Strategy, StrategyType, TipStrategy, TipStrategyRepo},
    user::{User, UserRepo},
};

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

pub async fn all_open_handler(
    CtxExt(mut tip_srategy_repo): CtxExt<TipStrategyRepo>,
    CtxExt(mut user_repo): CtxExt<UserRepo>,
    CtxExt(actor): CtxExt<Actor>,
    Path(tournament_id): Path<ArcUuid7>,
    Query(params): Query<Parameters>,
) -> impl IntoResponse {
    if let Ok(Some(user)) = user_repo
        .by_actor_id(actor.id().cloned().unwrap_or_default())
        .await
    {
        params.apply(&mut tip_srategy_repo, user);
    }

    ApiResponse::from(
        tip_srategy_repo
            .all_open_by_tournament_id(tournament_id)
            .await,
    )
}

pub async fn all_closed_handler(
    CtxExt(mut tip_srategy_repo): CtxExt<TipStrategyRepo>,
    Path(tournament_id): Path<ArcUuid7>,
) -> impl IntoResponse {
    ApiResponse::from(
        tip_srategy_repo
            .all_closed_by_tournament_id(tournament_id)
            .await,
    )
}

pub async fn create_result_handler(
    CtxExt(mut strategy_result_repo): CtxExt<StrategyResultRepo>,
    CtxExt(mut tip_strategy_repo): CtxExt<TipStrategyRepo>,
    Path((tournament_id, id)): Path<(ArcUuid7, ArcUuid7)>,
    Json(payload): Json<StrategyResultPayload>,
) -> impl IntoResponse {
    match tip_strategy_repo
        .by_tournament_and_id(tournament_id.clone(), id.clone())
        .await
    {
        Ok(Some(existing)) => {
            if existing.tournament_id != tournament_id {
                return ApiResponse::error(
                    "Tip strategy does not belong to the specified tournament",
                );
            }
        }
        Ok(None) => return ApiResponse::error("Tip strategy not found"),
        Err(e) => return ApiResponse::error(format!("Error fetching tip strategy: {e}")),
    }

    let strategy_result = StrategyResult {
        id: None,
        tip_strategy_id: id,
        strategy_results: payload.strategy_results,
        created_at: None,
        updated_at: None,
        deleted_at: None,
    };

    ApiResponse::from(strategy_result_repo.save(strategy_result).await)
}

pub async fn get_handler(
    CtxExt(mut tip_strategy_repo): CtxExt<TipStrategyRepo>,
    CtxExt(mut user_repo): CtxExt<UserRepo>,
    CtxExt(actor): CtxExt<Actor>,
    Path((tournament_id, id)): Path<(ArcUuid7, ArcUuid7)>,
    Query(parameters): Query<Parameters>,
) -> impl IntoResponse {
    if let Ok(Some(user)) = user_repo
        .by_actor_id(actor.id().cloned().unwrap_or_default())
        .await
    {
        parameters.apply(&mut tip_strategy_repo, user);
    }

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
#[serde(rename_all = "camelCase")]
pub(crate) struct TipStrategyPayload {
    #[ts(type = "string")]
    tournament_id: Option<ArcUuid7>,
    #[ts(type = "string")]
    label: LabelField,
    description: ArcStrField,
    #[ts(type = "string | null")]
    game_id: Option<ArcUuid7>,
    #[ts(type = "string | null")]
    group_id: Option<ArcUuid7>,
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
            tips: None,
            result: None,
            tournament_id: value
                .tournament_id
                .expect("tip strategy payload missing tournament id"),
            label: value.label,
            description: value.description,
            game_id: value.game_id,
            group_id: value.group_id,
            group: None,
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Parameters {
    with: Option<String>,
    // filter: Option<String>,
}

impl Parameters {
    pub fn apply(&self, repo: &mut TipStrategyRepo, user: User) {
        if let Some(with) = &self.with {
            with.split(',').for_each(|relation| match relation.trim() {
                "my_tips" => {
                    let id = user.id.clone().unwrap_or_default();
                    repo.with_tips_where(|q| {
                        q.query_mut().is_eq(TipRepo::col_user_id(), id);
                    });
                }
                _ => {}
            })
        }

        // if let Some(filter) = &self.filter {
        //     if filter.contains("open") {
        //         repo.open();
        //     } else if filter.contains("closed") {
        //         repo.closed();
        //     }
        // }
    }
}

#[derive(Debug, Clone, Deserialize, ts_rs::TS)]
#[ts(export)]
#[ts(export_to = "v1/")]
#[serde(rename_all = "camelCase")]
pub(crate) struct StrategyResultPayload {
    strategy_results: HashSet<Strategy>,
}
