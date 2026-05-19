use std::collections::HashSet;

use dirtybase_app::{
    db::types::{ArcUuid7, DateTimeField, IntegerField},
    db_macro::{DirtyEmbedded, DirtyTable},
};
use serde::Serialize;

use crate::dirtybase_entry::model::{
    tip_strategy::{Strategy, StrategyType, TipStrategy},
    tournament::Tournament,
};

// A tip made by a user for a specific strategy.
#[derive(Debug, Clone, Default, Serialize, DirtyTable, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
#[dirty(soft_deletable, timestamps, id_not_auto)]
pub struct Tip {
    #[ts(type = "string")]
    id: Option<ArcUuid7>,
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) tournament: Option<Tournament>,
    #[ts(type = "string")]
    pub(crate) tournament_id: ArcUuid7,
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) tip_strategy: Option<TipStrategy>,
    pub(crate) tip_strategy_pts: Vec<StrategyPoint>,
    #[ts(type = "string")]
    tip_strategy_id: ArcUuid7,
    #[ts(type = "string")]
    user_id: ArcUuid7,
    strategies: HashSet<Strategy>,
    #[ts(type = "number")]
    points: IntegerField,
    #[ts(type = "Date | null")]
    created_at: Option<DateTimeField>,
    #[ts(type = "Date | null")]
    updated_at: Option<DateTimeField>,
    #[ts(type = "Date | null")]
    deleted_at: Option<DateTimeField>,
}

#[derive(Debug, Clone, Default, Serialize, DirtyEmbedded, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct StrategyPoint {
    strategy: StrategyType,
    points: u64,
}
