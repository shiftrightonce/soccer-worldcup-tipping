use std::fmt::Display;

use dirtybase_app::{
    db::{
        field_values::FieldValue,
        types::{DateTimeField, IntegerField, StringField},
    },
    db_macro::DirtyTable,
};
use dirtybase_contract::db_contract::types::ArcUuid7;
use serde::{Deserialize, Serialize};

use crate::dirtybase_entry::model::country::Country;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum GameStatus {
    #[default]
    #[serde(alias = "pending")]
    /// The game is not ready for anything
    Pending,
    /// The game if open fo tipping
    #[serde(alias = "open")]
    Open,
    /// The game is closed for tipping
    #[serde(alias = "closed")]
    Closed,
    /// The game has been scored
    #[serde(alias = "scored")]
    Scored,
    /// The game has been completed
    #[serde(alias = "completed")]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum Stage {
    #[default]
    #[serde(alias = "group")]
    Group,
    #[serde(alias = "round_32")]
    Round32,
    #[serde(alias = "round_16")]
    Round16,
    #[serde(alias = "round_8")]
    Round8,
    #[serde(alias = "round_4")]
    Round4,
    #[serde(alias = "third_place")]
    ThirdPlace,
    #[serde(alias = "final")]
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

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, DirtyTable)]
#[dirty(table = "games", timestamps, soft_delete)]
pub struct Game {
    id: Option<ArcUuid7>,
    status: GameStatus,
    stage: Stage,
    year: IntegerField,
    label: StringField, // TODO: This should be a type
    country_a: Option<Country>,
    country_b: Option<Country>,
    penalty: bool,
    country_a_goals: IntegerField,
    country_b_goals: IntegerField,
    country_a_penalty_goals: IntegerField,
    country_b_penalty_goals: IntegerField,
    winner: Option<Country>,
    to_configure_on: Option<DateTimeField>,
    created_at: Option<DateTimeField>,
    updated_at: Option<DateTimeField>,
    deleted_at: Option<DateTimeField>,
}
