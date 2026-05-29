use std::collections::HashSet;

use dirtybase_app::{
    axum::Json,
    db::{
        base::paginate_builder::PaginateBuilder,
        types::{ArcUuid7, ToColumnAndValue},
    },
};
use dirtybase_contract::{
    auth_contract::Actor,
    http_contract::api::ApiResponse,
    prelude::{CtxExt, IntoResponse, Path},
};
use serde::Deserialize;

use crate::dirtybase_entry::model::{
    tip::{Tip, TipRepo},
    tip_strategy::{Strategy, TipStrategyRepo},
    user::UserRepo,
};

pub async fn list_handler(
    CtxExt(mut tip_repo): CtxExt<TipRepo>,
    Path(tournament_id): Path<ArcUuid7>,
    page: PaginateBuilder,
) -> impl IntoResponse {
    ApiResponse::from(
        tip_repo
            .paginate_by_tournament_id(tournament_id, Some(page))
            .await,
    )
}

pub async fn my_tips_handler(
    CtxExt(mut tip_repo): CtxExt<TipRepo>,
    CtxExt(mut user_repo): CtxExt<UserRepo>,
    CtxExt(actor): CtxExt<Actor>,
    Path(tournament_id): Path<ArcUuid7>,
) -> impl IntoResponse {
    if let Ok(Some(user)) = user_repo.by_actor_id(actor.id().cloned().unwrap()).await {
        ApiResponse::from(
            tip_repo
                .all_by_tournament_and_user(tournament_id, user.id.clone().unwrap())
                .await,
        )
    } else {
        ApiResponse::<Vec<Tip>>::not_found()
    }
}

pub async fn create_handler(
    CtxExt(mut tip_repo): CtxExt<TipRepo>,
    CtxExt(mut user_repo): CtxExt<UserRepo>,
    CtxExt(mut tip_strategy_repo): CtxExt<TipStrategyRepo>,
    CtxExt(actor): CtxExt<Actor>,
    Path(tournament_id): Path<ArcUuid7>,
    Json(payload): Json<TipPayload>,
) -> impl IntoResponse {
    if let Ok(Some(user)) = user_repo.by_actor_id(actor.id().cloned().unwrap()).await {
        let tip_strategy = tip_strategy_repo
            .by_id(payload.tip_strategy_id.clone())
            .await
            .ok()
            .flatten();
        if let Some(tip_strategy) = tip_strategy {
            if tip_strategy.tournament_id != tournament_id {
                return ApiResponse::<Tip>::bad_request()
                    .with_message("Tip strategy does not belong to the tournament");
            }
        } else {
            return ApiResponse::<Tip>::not_found().with_message("Tip strategy not found");
        }

        let mut editing = false;
        let mut tip: Tip = match tip_repo
            .one_by_tournament_strategy_and_user(
                tournament_id.clone(),
                payload.tip_strategy_id.clone(),
                user.id.clone().unwrap(),
            )
            .await
        {
            Ok(Some(existing)) => {
                editing = true;
                payload.merge(existing)
            }
            Ok(None) => payload.into(),
            Err(e) => return ApiResponse::internal_error().with_message(&e.to_string()),
        };

        tip.user_id = user.id.clone().unwrap();
        tip.tournament_id = tournament_id;
        tracing::warn!("{:#?}", tip.to_column_value());
        ApiResponse::from(if editing {
            tip_repo.update(tip).await
        } else {
            tracing::warn!("inserting new user tip");
            tip_repo.insert(tip).await
        })
    } else {
        ApiResponse::<Tip>::not_found()
    }
}

pub async fn update_handler(
    CtxExt(mut tip_repo): CtxExt<TipRepo>,
    CtxExt(mut user_repo): CtxExt<UserRepo>,
    CtxExt(mut tip_strategy_repo): CtxExt<TipStrategyRepo>,
    CtxExt(actor): CtxExt<Actor>,
    Path((tournament_id, tip_id)): Path<(ArcUuid7, ArcUuid7)>,
    Json(payload): Json<TipPayload>,
) -> impl IntoResponse {
    if let Ok(Some(user)) = user_repo.by_actor_id(actor.id().cloned().unwrap()).await {
        if let Ok(Some(existing)) = tip_repo
            .by_tournament_and_id(tournament_id.clone(), tip_id)
            .await
        {
            if existing.user_id != user.id.clone().unwrap() {
                return ApiResponse::<Tip>::forbidden()
                    .with_message("You can only update your own tips");
            }

            let tip_strategy = tip_strategy_repo
                .by_id(payload.tip_strategy_id.clone())
                .await
                .ok()
                .flatten();
            if let Some(tip_strategy) = tip_strategy {
                if tip_strategy.tournament_id != tournament_id {
                    return ApiResponse::<Tip>::bad_request()
                        .with_message("Tip strategy does not belong to the tournament");
                }
            } else {
                return ApiResponse::<Tip>::not_found().with_message("Tip strategy not found");
            }

            let changed_record = payload.merge(existing);
            ApiResponse::from(tip_repo.update(changed_record).await)
        } else {
            ApiResponse::<Tip>::not_found()
        }
    } else {
        ApiResponse::<Tip>::not_found()
    }
}

#[derive(Debug, Clone, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
#[ts(export_to = "v1/")]
pub struct TipPayload {
    #[ts(type = "string | null")]
    pub id: Option<ArcUuid7>,
    #[ts(type = "string")]
    pub tip_strategy_id: ArcUuid7,
    #[ts(type = "string")]
    pub tournament_id: ArcUuid7,
    #[ts(type = "string | null")]
    pub user_id: Option<ArcUuid7>,
    pub strategies: HashSet<Strategy>,
}

impl From<TipPayload> for Tip {
    fn from(payload: TipPayload) -> Self {
        Self {
            id: Some(ArcUuid7::default()),
            tip_strategy_id: payload.tip_strategy_id,
            tournament_id: payload.tournament_id,
            user_id: payload.user_id.unwrap_or_default(),
            strategies: payload.strategies,
            ..Default::default()
        }
    }
}

impl TipPayload {
    pub fn merge(self, mut existing: Tip) -> Tip {
        existing.strategies = self.strategies;
        existing
    }
}
