use std::fmt::Display;

use dirtybase_app::{
    db::{
        base::paginate_builder::{PaginateBuilder, PaginateResult},
        field_values::FieldValue,
        types::{
            CreatedAtField, DateTimeField, DeletedAtField, IntegerField, LabelField, UpdatedAtField,
        },
    },
    db_macro::DirtyTable,
};
use dirtybase_common::anyhow;
use dirtybase_contract::db_contract::types::ArcUuid7;
use serde::{Deserialize, Serialize};

use crate::dirtybase_entry::model::{country::Country, tournament::Tournament};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub enum GameStatus {
    #[default]
    #[serde(rename = "pending")]
    /// The game is not ready for anything
    Pending,
    /// The game if open fo tipping
    #[serde(rename = "open")]
    Open,
    /// The game is closed for tipping
    #[serde(rename = "closed")]
    Closed,
    /// The game has been scored
    #[serde(rename = "scored")]
    Scored,
    /// The game has been completed
    #[serde(rename = "completed")]
    Completed,
}

impl Display for GameStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameStatus::Pending => "pending",
                GameStatus::Open => "open",
                GameStatus::Closed => "closed",
                GameStatus::Scored => "scored",
                GameStatus::Completed => "completed",
            }
        )
    }
}

impl From<FieldValue> for GameStatus {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::String(s) => match s.as_str() {
                "pending" => GameStatus::Pending,
                "open" => GameStatus::Open,
                "closed" => GameStatus::Closed,
                "scored" => GameStatus::Scored,
                "completed" => GameStatus::Completed,
                _ => GameStatus::Pending,
            },
            _ => GameStatus::Pending,
        }
    }
}

impl From<GameStatus> for FieldValue {
    fn from(value: GameStatus) -> Self {
        match value {
            GameStatus::Pending => FieldValue::String("pending".to_string()),
            GameStatus::Open => FieldValue::String("open".to_string()),
            GameStatus::Closed => FieldValue::String("closed".to_string()),
            GameStatus::Scored => FieldValue::String("scored".to_string()),
            GameStatus::Completed => FieldValue::String("completed".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, ts_rs::TS)]
#[ts(export)]
pub enum Stage {
    #[default]
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "round_32")]
    Round32,
    #[serde(rename = "round_16")]
    Round16,
    #[serde(rename = "round_8")]
    Round8,
    #[serde(rename = "round_4")]
    Round4,
    #[serde(rename = "third_place")]
    ThirdPlace,
    #[serde(rename = "final")]
    Final,
}

impl Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stage::Group => write!(f, "group"),
            Stage::Round32 => write!(f, "round_32"),
            Stage::Round16 => write!(f, "round_16"),
            Stage::Round8 => write!(f, "round_8"),
            Stage::Round4 => write!(f, "round_4"),
            Stage::ThirdPlace => write!(f, "third_place"),
            Stage::Final => write!(f, "final"),
        }
    }
}

impl From<FieldValue> for Stage {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::String(s) => match s.as_str() {
                "group" => Stage::Group,
                "round_32" => Stage::Round32,
                "round_16" => Stage::Round16,
                "round_8" => Stage::Round8,
                "round_4" => Stage::Round4,
                "third_place" => Stage::ThirdPlace,
                "final" => Stage::Final,
                _ => Stage::Group,
            },
            _ => Stage::Group,
        }
    }
}

impl From<Stage> for FieldValue {
    fn from(value: Stage) -> Self {
        match value {
            Stage::Group => FieldValue::String("group".to_string()),
            Stage::Round32 => FieldValue::String("round_32".to_string()),
            Stage::Round16 => FieldValue::String("round_16".to_string()),
            Stage::Round8 => FieldValue::String("round_8".to_string()),
            Stage::Round4 => FieldValue::String("round_4".to_string()),
            Stage::ThirdPlace => FieldValue::String("third_place".to_string()),
            Stage::Final => FieldValue::String("final".to_string()),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, DirtyTable, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
#[dirty(table = "games", timestamps, soft_delete, id_not_auto)]
pub struct Game {
    #[ts(type = "string")]
    pub(crate) id: Option<ArcUuid7>,
    pub(crate) status: GameStatus,
    pub(crate) stage: Stage,
    #[ts(type = "string")]
    pub(crate) label: LabelField,
    #[ts(type = "number")]
    pub(crate) count: IntegerField,
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) tournament: Option<Tournament>,
    #[ts(type = "string")]
    pub(crate) tournament_id: ArcUuid7,
    #[dirty(rel(kind = "belongs_to", column = "country_a_id"))]
    pub(crate) country_a: Option<Country>,
    #[dirty(rel(kind = "belongs_to", column = "country_b_id"))]
    pub(crate) country_b: Option<Country>,
    #[ts(type = "string")]
    pub(crate) country_a_id: ArcUuid7,
    #[ts(type = "string")]
    pub(crate) country_b_id: ArcUuid7,
    pub(crate) penalty: bool,
    #[ts(type = "number")]
    pub(crate) country_a_goals: IntegerField,
    #[ts(type = "number")]
    pub(crate) country_b_goals: IntegerField,
    #[ts(type = "number")]
    pub(crate) country_a_penalty_goals: IntegerField,
    #[ts(type = "number")]
    pub(crate) country_b_penalty_goals: IntegerField,
    #[dirty(rel(kind = "belongs_to", column = "winner_id"))]
    pub(crate) winner: Option<Country>,
    #[ts(type = "string")]
    pub(crate) winner_id: Option<ArcUuid7>,
    #[ts(type = "Date | null")]
    pub(crate) to_configure_on: Option<DateTimeField>,
    #[ts(type = "Date | null")]
    pub(crate) created_at: CreatedAtField,
    #[ts(type = "Date | null")]
    pub(crate) updated_at: UpdatedAtField,
    #[ts(type = "Date | null")]
    pub(crate) deleted_at: DeletedAtField,
}

impl Game {
    pub fn new(
        tournament_id: ArcUuid7,
        label: String,
        country_a: ArcUuid7,
        country_b: ArcUuid7,
    ) -> Self {
        Self {
            id: None,
            status: GameStatus::Pending,
            stage: Stage::Group,
            tournament_id: tournament_id,
            label: label.into(),
            country_a_id: country_a,
            country_b_id: country_b,
            penalty: false,
            created_at: Some(DateTimeField::default()),
            ..Default::default()
        }
    }

    pub fn id(&self) -> Option<&ArcUuid7> {
        self.id.as_ref()
    }

    pub fn status(&self) -> &GameStatus {
        &self.status
    }

    pub fn set_status(&mut self, status: GameStatus) -> &mut Self {
        self.status = status;
        self
    }

    pub fn stage(&self) -> &Stage {
        &self.stage
    }

    pub fn set_stage(&mut self, stage: Stage) -> &mut Self {
        self.stage = stage;
        self
    }

    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    pub fn count(&self) -> IntegerField {
        self.count
    }

    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label = label.to_string().into();
        self
    }

    pub fn country_a(&self) -> Option<&Country> {
        self.country_a.as_ref()
    }

    pub fn country_b(&self) -> Option<&Country> {
        self.country_b.as_ref()
    }

    pub fn penalty(&self) -> bool {
        self.penalty
    }

    pub fn set_penalty(&mut self, penalty: bool) -> &mut Self {
        self.penalty = penalty;
        self
    }

    pub fn country_a_goals(&self) -> i64 {
        self.country_a_goals
    }

    pub fn set_country_a_goals(&mut self, country_a_goals: i64) -> &mut Self {
        self.country_a_goals = country_a_goals;
        self
    }

    pub fn country_b_goals(&self) -> i64 {
        self.country_b_goals
    }

    pub fn set_country_b_goals(&mut self, country_b_goals: i64) -> &mut Self {
        self.country_b_goals = country_b_goals;
        self
    }

    pub fn country_a_penalty_goals(&self) -> i64 {
        self.country_a_penalty_goals
    }

    pub fn set_country_a_penalty_goals(&mut self, goals: i64) -> &mut Self {
        self.country_a_penalty_goals = goals;
        self
    }

    pub fn country_b_penalty_goals(&self) -> i64 {
        self.country_b_penalty_goals
    }

    pub fn set_country_b_penalty_goals(&mut self, goals: i64) -> &mut Self {
        self.country_b_penalty_goals = goals;
        self
    }

    pub fn winner(&self) -> Option<&Country> {
        self.winner.as_ref()
    }

    pub fn winner_id(&self) -> Option<&ArcUuid7> {
        self.winner_id.as_ref()
    }

    pub fn set_winner_id(&mut self, winner_id: ArcUuid7) -> &mut Self {
        if self.country_a_id == winner_id || self.country_b_id == winner_id {
            self.winner_id = Some(winner_id);
        } else {
            tracing::debug!(
                "winder must be one of the countries: {winner_id}, game: {:#?}",
                &self.id
            );
        }
        self
    }

    pub fn to_configure_on(&self) -> Option<&DateTimeField> {
        self.to_configure_on.as_ref()
    }

    pub fn set_to_configure_on(&mut self, to_configure_on: DateTimeField) -> &mut Self {
        self.to_configure_on = Some(to_configure_on);
        self
    }

    pub fn created_at(&self) -> Option<&DateTimeField> {
        self.created_at.as_ref()
    }

    pub fn updated_at(&self) -> Option<&DateTimeField> {
        self.updated_at.as_ref()
    }

    pub fn deleted_at(&self) -> Option<&DateTimeField> {
        self.deleted_at.as_ref()
    }
}

impl GameRepo {
    pub async fn by_tournament_and_id(
        &mut self,
        tournament_id: ArcUuid7,
        id: ArcUuid7,
    ) -> Result<Option<Game>, anyhow::Error> {
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.by_id(id).await
    }

    pub async fn all_by_tournament(
        &mut self,
        tournament_id: ArcUuid7,
    ) -> Result<Vec<Game>, anyhow::Error> {
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.get().await
    }

    pub async fn by_tournament_and_status(
        &mut self,
        tournament_id: ArcUuid7,
        status: GameStatus,
    ) -> Result<Vec<Game>, anyhow::Error> {
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.builder.is_eq(Self::col_status(), status);
        self.get().await
    }

    pub async fn paginate_by_tournament(
        &mut self,
        tournament_id: ArcUuid7,
        page: Option<PaginateBuilder>,
    ) -> PaginateResult<Game> {
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.paginate(page).await
    }
}
