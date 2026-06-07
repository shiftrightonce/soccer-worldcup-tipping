use dirtybase_app::{
    axum::Json,
    db::{
        base::paginate_builder::PaginateBuilder,
        types::{ArcUuid7, DateTimeField, IntegerField, LabelField},
    },
};
use dirtybase_contract::{
    http_contract::api::ApiResponse,
    prelude::{CtxExt, IntoResponse, Path, StatusCode},
};
use serde::Deserialize;

use crate::dirtybase_entry::model::game::{Game, GameRepo, GameStatus, Stage};

pub async fn list_handler(
    CtxExt(mut repo): CtxExt<GameRepo>,
    Path(tournament_id): Path<ArcUuid7>,
    page: PaginateBuilder,
) -> impl IntoResponse {
    ApiResponse::from(repo.paginate_by_tournament(tournament_id, Some(page)).await)
}

pub async fn all_handler(
    CtxExt(mut repo): CtxExt<GameRepo>,
    Path(tournament_id): Path<ArcUuid7>,
) -> impl IntoResponse {
    ApiResponse::from(repo.all_by_tournament(tournament_id).await)
}

pub async fn list_for_strategy_handler(
    CtxExt(mut repo): CtxExt<GameRepo>,
    Path(tournament_id): Path<ArcUuid7>,
) -> impl IntoResponse {
    ApiResponse::from(repo.all_unassiged_by_tournament(tournament_id).await)
}

pub async fn by_status_handler(
    CtxExt(mut repo): CtxExt<GameRepo>,
    Path((tournament_id, status)): Path<(ArcUuid7, GameStatus)>,
) -> impl IntoResponse {
    ApiResponse::from(repo.by_tournament_and_status(tournament_id, status).await)
}

pub async fn get_handler(
    CtxExt(mut repo): CtxExt<GameRepo>,
    Path((tournament_id, id)): Path<(ArcUuid7, ArcUuid7)>,
) -> impl IntoResponse {
    ApiResponse::<Game>::from(repo.by_tournament_and_id(tournament_id, id).await)
}

pub async fn create_handler(
    CtxExt(mut repo): CtxExt<GameRepo>,
    Path(tournament_id): Path<ArcUuid7>,
    Json(mut payload): Json<GamePayload>,
) -> impl IntoResponse {
    payload.tournament_id = tournament_id;
    let game = Game::from(payload);
    ApiResponse::<Game>::from(repo.insert(game).await)
}

pub async fn update_handler(
    CtxExt(mut repo): CtxExt<GameRepo>,
    Path((tournament_id, id)): Path<(ArcUuid7, ArcUuid7)>,
    Json(payload): Json<GamePayload>,
) -> impl IntoResponse {
    if let Ok(Some(existing)) = repo.by_id(id).await
        && existing.tournament_id == tournament_id
    {
        let changed_record = payload.merge(existing);
        ApiResponse::<Game>::from(repo.update(changed_record).await)
    } else {
        ApiResponse::error_with_status("Not found", StatusCode::NOT_FOUND)
    }
}

#[derive(Debug, Clone, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
#[ts(export_to = "v1/")]
pub(crate) struct GamePayload {
    #[ts(type = "string")]
    pub(crate) tournament_id: ArcUuid7,
    #[ts(type = "string")]
    pub(crate) label: Option<LabelField>,
    #[ts(type = "number")]
    pub(crate) count: Option<IntegerField>,
    pub(crate) stage: Option<Stage>,
    pub(crate) status: Option<GameStatus>,
    #[ts(type = "string | null")]
    pub(crate) country_a_id: Option<ArcUuid7>,
    #[ts(type = "string | null")]
    pub(crate) country_b_id: Option<ArcUuid7>,
    pub(crate) penalty: Option<bool>,
    #[ts(type = "number | null")]
    pub(crate) country_a_goals: Option<IntegerField>,
    #[ts(type = "number | null")]
    pub(crate) country_b_goals: Option<IntegerField>,
    #[ts(type = "string | null")]
    pub(crate) winner_id: Option<ArcUuid7>,
    #[ts(type = "string | null")]
    pub(crate) date_and_time: Option<DateTimeField>,
    #[ts(type = "number | null")]
    pub(crate) country_a_penalty_goals: Option<IntegerField>,
    #[ts(type = "number | null")]
    pub(crate) country_b_penalty_goals: Option<IntegerField>,
}

impl From<GamePayload> for Game {
    fn from(value: GamePayload) -> Game {
        let game = Self {
            id: Some(ArcUuid7::default()),
            tournament_id: value.tournament_id.clone(),
            ..Default::default()
        };

        value.merge(game)
    }
}

impl GamePayload {
    pub fn merge(self, mut game: Game) -> Game {
        if let Some(label) = self.label {
            game.label = label;
        }

        if let Some(datetime) = self.date_and_time {
            game.date_and_time = Some(datetime);
        }

        if let Some(count) = self.count {
            game.count = count;
        }

        if let Some(stage) = self.stage {
            game.stage = stage;
        }

        if let Some(country_a_id) = self.country_a_id {
            game.country_a_id = country_a_id;
        }

        if let Some(country_b_id) = self.country_b_id {
            game.country_b_id = country_b_id;
        }

        if let Some(penalty) = self.penalty {
            game.penalty = penalty;
        }

        if let Some(country_a_goals) = self.country_a_goals {
            game.country_a_goals = country_a_goals;
        }
        if let Some(country_b_goals) = self.country_b_goals {
            game.country_b_goals = country_b_goals;
        }

        if let Some(country_a_penalty_goals) = self.country_a_penalty_goals {
            game.country_a_penalty_goals = country_a_penalty_goals
        }

        if let Some(country_b_penalty_goals) = self.country_b_penalty_goals {
            game.country_b_penalty_goals = country_b_penalty_goals
        }

        if let Some(winner_id) = self.winner_id {
            game.winner_id = Some(winner_id);
        }

        if let Some(status) = self.status {
            game.status = status;
        }

        game
    }
}
