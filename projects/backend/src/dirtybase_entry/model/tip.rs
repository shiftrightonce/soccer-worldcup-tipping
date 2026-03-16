use std::{fmt::Display, str::FromStr};

use dirtybase_app::{
    db::{
        field_values::FieldValue,
        types::{ArcUuid7, DateTimeField, IntegerField},
    },
    db_macro::DirtyTable,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum TippingStatus {
    #[default]
    #[serde(alias = "open")]
    Open,
    #[serde(alias = "closed")]
    Closed,
    #[serde(alias = "scored")]
    Scored,
}

impl Display for TippingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TippingStatus::Open => "open",
                TippingStatus::Closed => "closed",
                TippingStatus::Scored => "scored",
            }
        )
    }
}

impl FromStr for TippingStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "open" => Ok(TippingStatus::Open),
            "closed" => Ok(TippingStatus::Closed),
            "scored" => Ok(TippingStatus::Scored),
            _ => Ok(Self::default()),
        }
    }
}

impl From<TippingStatus> for FieldValue {
    fn from(value: TippingStatus) -> Self {
        FieldValue::String(value.to_string())
    }
}

impl From<FieldValue> for TippingStatus {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::String(s) => TippingStatus::from_str(&s).unwrap_or_default(),
            _ => TippingStatus::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, DirtyTable)]
pub struct Tip {
    id: Option<ArcUuid7>,
    game_id: Option<ArcUuid7>,
    user_id: ArcUuid7,
    status: TippingStatus,
    strategy: Vec<Strategy>,
    closed_at: DateTimeField,
    points: IntegerField,
    created_at: Option<DateTimeField>,
    updated_at: Option<DateTimeField>,
    deleted_at: Option<DateTimeField>,
}

#[cfg(test)]
mod test {

    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_tipping_status_from_str() {
        assert_eq!(
            TippingStatus::from_str("open").unwrap(),
            TippingStatus::Open
        );
        assert_eq!(
            TippingStatus::from_str("closed").unwrap(),
            TippingStatus::Closed
        );
        assert_eq!(
            TippingStatus::from_str("scored").unwrap(),
            TippingStatus::Scored
        );
        assert_eq!(
            TippingStatus::from_str("invalid").unwrap(),
            TippingStatus::default()
        );
    }

    #[test]
    fn test_tip() {
        let mut tip = Tip {
            id: None,
            game_id: None,
            user_id: ArcUuid7::default(),
            status: TippingStatus::Open,
            strategy: vec![Strategy::GroupRanking(None)],
            closed_at: DateTimeField::default(),
            points: IntegerField::default(),
            created_at: None,
            updated_at: None,
            deleted_at: None,
        };

        if let Some(strategy) = tip.strategy.first_mut() {
            if let Strategy::GroupRanking(ranking) = strategy {
                if ranking.is_none() {
                    let r = (0..4)
                        .into_iter()
                        .map(|_| ArcUuid7::default())
                        .collect::<Vec<ArcUuid7>>();
                    *ranking = Some(r.try_into().unwrap());
                }
            }
        }

        let json = serde_json::to_string(&tip).unwrap();
        let back = serde_json::from_str::<Tip>(&json).unwrap();
        println!("{:#?}", json);
        println!("{:#?}", back);
    }
}
