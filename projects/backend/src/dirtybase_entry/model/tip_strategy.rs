use std::{collections::HashSet, fmt::Display};

use dirtybase_app::{
    db::{
        field_values::FieldValue,
        types::{ArcUuid7, DateTimeField},
    },
    db_macro::DirtyTable,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrategyType {
    #[default]
    None,
    #[serde(rename = "winner")]
    Winner,
    #[serde(rename = "goals")]
    Goals,
    #[serde(rename = "cup_winner")]
    CupWinner,
    #[serde(rename = "game_to_penalty")]
    GameToPenalty,
    #[serde(rename = "first_red_card")]
    FirstRedCard,
    #[serde(rename = "first_yellow_card")]
    FirstYellowCard,
    #[serde(rename = "penalty_goals")]
    PenaltyGoals,
    #[serde(rename = "group_ranking")]
    GroupRanking,
    #[serde(rename = "round_32_qualifiers")]
    Round32Qualifiers,
    #[serde(rename = "round_16_qualifiers")]
    Round16Qualifiers,
    #[serde(rename = "round_8_qualifiers")]
    Round8Qualifiers,
    #[serde(rename = "round_4_qualifiers")]
    Round4Qualifiers,
    #[serde(rename = "third_qualifiers")]
    ThirdQualifiers,
    #[serde(rename = "final")]
    Final,
}

impl Display for StrategyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrategyType::None => write!(f, "none"),
            StrategyType::Winner => write!(f, "winner"),
            StrategyType::Goals => write!(f, "goals"),
            StrategyType::CupWinner => write!(f, "cup_winner"),
            StrategyType::GameToPenalty => write!(f, "game_to_penalty"),
            StrategyType::FirstRedCard => write!(f, "first_red_card"),
            StrategyType::FirstYellowCard => write!(f, "first_yellow_card"),
            StrategyType::PenaltyGoals => write!(f, "penalty_goals"),
            StrategyType::GroupRanking => write!(f, "group_ranking"),
            StrategyType::Round32Qualifiers => write!(f, "round_32_qualifiers"),
            StrategyType::Round16Qualifiers => write!(f, "round_16_qualifiers"),
            StrategyType::Round8Qualifiers => write!(f, "round_8_qualifiers"),
            StrategyType::Round4Qualifiers => write!(f, "round_4_qualifiers"),
            StrategyType::ThirdQualifiers => write!(f, "third_qualifiers"),
            StrategyType::Final => write!(f, "final"),
        }
    }
}

impl From<FieldValue> for StrategyType {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::String(ref v) => match v.as_str() {
                "none" => StrategyType::None,
                "winner" => StrategyType::Winner,
                "goals" => StrategyType::Goals,
                "cup_winner" => StrategyType::CupWinner,
                "game_to_penalty" => StrategyType::GameToPenalty,
                "first_red_card" => StrategyType::FirstRedCard,
                "first_yellow_card" => StrategyType::FirstYellowCard,
                "penalty_goals" => StrategyType::PenaltyGoals,
                "group_ranking" => StrategyType::GroupRanking,
                "round_32_qualifiers" => StrategyType::Round32Qualifiers,
                "round_16_qualifiers" => StrategyType::Round16Qualifiers,
                "round_8_qualifiers" => StrategyType::Round8Qualifiers,
                "round_4_qualifiers" => StrategyType::Round4Qualifiers,
                "third_qualifiers" => StrategyType::ThirdQualifiers,
                "final" => StrategyType::Final,
                _ => StrategyType::None,
            },
            _ => StrategyType::None,
        }
    }
}

impl From<StrategyType> for FieldValue {
    fn from(value: StrategyType) -> Self {
        FieldValue::String(value.to_string())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, DirtyTable)]
pub struct TipStrategy {
    id: Option<ArcUuid7>,
    game_id: Option<ArcUuid7>,
    opens_at: DateTimeField,
    ends_at: DateTimeField,
    strategies: HashSet<StrategyType>,
    created_at: Option<DateTimeField>,
    updated_at: Option<DateTimeField>,
    deleted_at: Option<DateTimeField>,
}
