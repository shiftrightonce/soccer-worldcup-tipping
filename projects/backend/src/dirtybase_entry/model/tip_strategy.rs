use std::{collections::HashSet, fmt::Display};

use dirtybase_app::{
    db::{
        field_values::FieldValue,
        types::{ArcUuid7, BooleanField, DateTimeField},
    },
    db_macro::DirtyTable,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, DirtyTable)]
#[dirty(soft_deletable, timestamps)]
pub struct TipStrategy {
    id: Option<ArcUuid7>,
    game_id: Option<ArcUuid7>,
    // Date and time when this strategy opens for tips.
    opens_at: DateTimeField,
    // Date and time when this strategy ends for tips.
    ends_at: DateTimeField,
    // Date and time when this strategy calculates points.
    calculate_points_on: Option<DateTimeField>,
    // Indicates whether this strategy has been completed and all points have been calculated.
    completed: BooleanField,
    // Types of strategies to apply.
    strategy_types: HashSet<StrategyType>,
    created_at: Option<DateTimeField>,
    updated_at: Option<DateTimeField>,
    deleted_at: Option<DateTimeField>,
}

/// A strategy actual data
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Strategy {
    /// No strategy. This is the default.
    #[default]
    None,
    /// The winner of the game. This is per game basis.
    #[serde(alias = "winner")]
    Winner(Option<ArcUuid7>),
    /// The goals scored by each team. This is per game basis.
    #[serde(alias = "goals")]
    Goals {
        country_a: Option<ArcUuid7>,
        country_b: Option<ArcUuid7>,
    },
    /// The winner of the cup. This is available before the worldcup starts.
    #[serde(alias = "cup_winner")]
    CupWinner(Option<ArcUuid7>),
    /// The game is going to penalty shootouts. This is per game basis.
    #[serde(alias = "game_to_penalty")]
    GameToPenalty(bool),
    /// The first red card of the game. This is per game basis.
    #[serde(alias = "first_red_card")]
    FirstRedCard(Option<ArcUuid7>),
    /// The first yellow card of the game. This is per game basis.
    #[serde(alias = "first_yellow_card")]
    FirstYellowCard(Option<ArcUuid7>),
    /// The penalty goals scored by each team. This is per game basis.
    #[serde(alias = "penalty_goals")]
    PenaltyGoals {
        country_a: Option<ArcUuid7>,
        country_b: Option<ArcUuid7>,
    },
    /// The group ranking of the teams. This is per group basis and only available before the group stage starts.
    #[serde(alias = "group_ranking")]
    GroupRanking(Option<[ArcUuid7; 4]>),
    /// The round of 32 qualifiers. This is per the round of 32 qualifiers.
    #[serde(alias = "round_32_qualifiers")]
    Round32Qualifiers([ArcUuid7; 32]),
    /// The round of 16 qualifiers. This is per the round of 16 qualifiers.
    #[serde(alias = "round_16_qualifiers")]
    Round16Qualifiers([ArcUuid7; 16]),
    /// The round of 8 qualifiers. This is per the round of 8 qualifiers.
    #[serde(alias = "round_8_qualifiers")]
    Round8Qualifiers([ArcUuid7; 8]),
    /// The round of 4 qualifiers. This is per the round of 4 qualifiers.
    #[serde(alias = "round_4_qualifiers")]
    Round4Qualifiers([ArcUuid7; 4]),
    /// The third place qualifiers. This will be available after the round of 4 qualifiers.
    #[serde(alias = "third_place_qualifiers")]
    ThirdPlaceQualifiers([ArcUuid7; 2]),
    /// The third place qualifiers. This will be available after the round of 4 qualifiers.
    #[serde(alias = "final")]
    Final([ArcUuid7; 2]),
}

impl From<Strategy> for FieldValue {
    fn from(value: Strategy) -> Self {
        FieldValue::String(serde_json::to_string(&value).expect("could not serialise strategy"))
    }
}

impl From<FieldValue> for Strategy {
    fn from(value: FieldValue) -> Self {
        serde_json::from_str::<Strategy>(&value.to_string()).unwrap_or_default()
    }
}

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
