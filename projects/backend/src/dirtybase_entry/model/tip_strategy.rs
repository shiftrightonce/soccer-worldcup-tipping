use std::{collections::HashSet, fmt::Display};

use dirtybase_app::{
    db::{
        base::paginate_builder::{PaginateBuilder, PaginateResult},
        field_values::FieldValue,
        types::{ArcStrField, ArcUuid7, BooleanField, DateTimeField, IntegerField, LabelField},
    },
    db_macro::DirtyTable,
};
use dirtybase_common::anyhow;
use serde::{Deserialize, Serialize};

use crate::dirtybase_entry::model::{game::Game, group::Group, tournament::Tournament};

#[derive(Debug, Default, Clone, Serialize, DirtyTable, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
#[dirty(soft_deletable, timestamps, id_not_auto)]
pub struct TipStrategy {
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) game: Option<Game>,
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) tournament: Option<Tournament>,
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) group: Option<Group>,

    #[ts(type = "string")]
    pub(crate) id: Option<ArcUuid7>,
    #[ts(type = "string")]
    pub(crate) tournament_id: ArcUuid7,
    #[ts(type = "string | null")]
    pub(crate) game_id: Option<ArcUuid7>,
    #[ts(type = "string | null")]
    pub(crate) group_id: Option<ArcUuid7>,
    #[ts(type = "string")]
    pub(crate) label: LabelField,
    pub(crate) description: ArcStrField,
    // Date and time when this strategy opens for tips.
    #[ts(type = "string | null")]
    pub(crate) opens_at: DateTimeField,
    // Date and time when this strategy ends for tips.
    #[ts(type = "string | null")]
    pub(crate) ends_at: DateTimeField,
    // Date and time when this strategy calculates points.
    #[ts(type = "string | null")]
    pub(crate) calculate_points_on: Option<DateTimeField>,
    // Indicates whether this strategy has been completed and all points have been calculated.
    #[ts(type = "boolean")]
    pub(crate) completed: BooleanField,
    // Types of strategies to apply.
    pub(crate) strategy_types: HashSet<StrategyType>,
    #[ts(type = "string | null")]
    pub(crate) created_at: Option<DateTimeField>,
    #[ts(type = "string | null")]
    pub(crate) updated_at: Option<DateTimeField>,
    #[ts(type = "string | null")]
    pub(crate) deleted_at: Option<DateTimeField>,
}

impl TipStrategyRepo {
    pub async fn by_tournament_and_id(
        &mut self,
        tournament_id: ArcUuid7,
        id: ArcUuid7,
    ) -> Result<Option<TipStrategy>, anyhow::Error> {
        self.with_tournament().with_game().with_group();
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.by_id(id).await
    }

    pub async fn all_by_tournament_id(
        &mut self,
        tournament_id: ArcUuid7,
    ) -> Result<Vec<TipStrategy>, anyhow::Error> {
        self.with_tournament().with_game().with_group();
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.get().await
    }

    pub async fn paginate_by_tournament(
        &mut self,
        tournament_id: ArcUuid7,
        page: Option<PaginateBuilder>,
    ) -> PaginateResult<TipStrategy> {
        self.with_tournament().with_game().with_group();
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.paginate(page).await
    }

    pub async fn update_by_tournament(
        &mut self,
        tournament_id: ArcUuid7,
        record: TipStrategy,
    ) -> Result<TipStrategy, anyhow::Error> {
        self.with_tournament().with_game().with_group();
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.update(record).await
    }

    pub async fn delete_by_tournament_and_id(
        &mut self,
        tournament_id: ArcUuid7,
        id: ArcUuid7,
    ) -> Result<(), anyhow::Error> {
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.delete_by_id(id).await
    }
    pub async fn delete_by_tournament(
        &mut self,
        tournament_id: ArcUuid7,
        record: TipStrategy,
    ) -> Result<TipStrategy, anyhow::Error> {
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.delete(record).await
    }
}

/// A strategy actual data
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ts_rs::TS)]
#[serde(tag = "kind", content = "entry")]
#[ts(export)]
pub enum Strategy {
    /// The winner of the game. This is per game basis.
    #[serde(rename = "winner")]
    Winner(#[ts(type = "string")] ArcUuid7),
    /// The goals scored by each team. This is per game basis.
    #[serde(rename = "goals")]
    Goals {
        #[ts(type = "number")]
        country_a_goals: IntegerField, // We can determine which team is country A or B based on the game_id and the teams in that game.
        #[ts(type = "number")]
        country_b_goals: IntegerField,
    },
    /// The winner of the cup. This is available before the worldcup starts.
    #[serde(rename = "cup_winner")]
    CupWinner(#[ts(type = "string")] ArcUuid7),
    /// The game is going to penalty shootouts. This is per game basis.
    #[serde(rename = "game_to_penalty")]
    GameToPenalty(bool),
    /// The first red card of the game. This is per game basis.
    #[serde(rename = "first_red_card")]
    FirstRedCard(#[ts(type = "string")] ArcUuid7),
    /// The first yellow card of the game. This is per game basis.
    #[serde(rename = "first_yellow_card")]
    FirstYellowCard(#[ts(type = "string")] ArcUuid7),
    /// The penalty goals scored by each team. This is per game basis.
    #[serde(rename = "penalty_goals")]
    PenaltyGoals {
        #[ts(type = "number")]
        country_a_goals: IntegerField,
        #[ts(type = "number")]
        country_b_goals: IntegerField,
    },
    /// The group ranking of the teams. This is per group basis and only available before the group stage starts.
    #[serde(rename = "group_ranking")]
    GroupRanking(#[ts(type = "string[]")] [ArcUuid7; 4]),
    /// The round of 32 qualifiers. This is per the round of 32 qualifiers.
    #[serde(rename = "round_32_qualifiers")]
    Round32Qualifiers(#[ts(type = "string[]")] [ArcUuid7; 32]),
    /// The round of 16 qualifiers. This is per the round of 16 qualifiers.
    #[serde(rename = "round_16_qualifiers")]
    Round16Qualifiers(#[ts(type = "string[]")] [ArcUuid7; 16]),
    /// The round of 8 qualifiers. This is per the round of 8 qualifiers.
    #[serde(rename = "round_8_qualifiers")]
    Round8Qualifiers(#[ts(type = "string[]")] [ArcUuid7; 8]),
    /// The round of 4 qualifiers. This is per the round of 4 qualifiers.
    #[serde(rename = "round_4_qualifiers")]
    Round4Qualifiers(#[ts(type = "string[]")] [ArcUuid7; 4]),
    /// The third place qualifiers. This will be available at the round of 4 qualifiers.
    #[serde(rename = "third_place_qualifiers")]
    ThirdPlaceQualifiers(#[ts(type = "string[]")] [ArcUuid7; 2]),
    /// The third place qualifiers. This will be available at the round of 4 qualifiers.
    #[serde(rename = "final")]
    Final(#[ts(type = "string[]")] [ArcUuid7; 2]),
}

impl From<Strategy> for FieldValue {
    fn from(value: Strategy) -> Self {
        FieldValue::String(serde_json::to_string(&value).expect("could not serialise strategy"))
    }
}

impl From<FieldValue> for Strategy {
    fn from(value: FieldValue) -> Self {
        serde_json::from_str::<Strategy>(&value.to_string())
            .expect("could not deserialise strategy")
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub enum StrategyType {
    #[default]
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
            FieldValue::String(ref v) => match v.to_lowercase().as_str() {
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
                _ => panic!("Invalid value for strategy type: {}", v),
            },
            _ => panic!("Invalid field value for strategy type"),
        }
    }
}

impl From<StrategyType> for FieldValue {
    fn from(value: StrategyType) -> Self {
        FieldValue::String(value.to_string())
    }
}
