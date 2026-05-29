use std::collections::HashSet;

use dirtybase_app::{
    db::{
        base::paginate_builder::{PaginateBuilder, PaginateResult},
        field_values::FieldValue,
        types::{ArcUuid7, DateTimeField, IntegerField},
    },
    db_macro::DirtyTable,
};
use dirtybase_common::anyhow;
use serde::{Deserialize, Serialize};

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
    pub(crate) id: Option<ArcUuid7>,
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) tournament: Option<Tournament>,
    #[ts(type = "string")]
    pub(crate) tournament_id: ArcUuid7,
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) tip_strategy: Option<TipStrategy>,
    pub(crate) tip_strategy_pts: HashSet<StrategyPoint>,
    #[ts(type = "string")]
    pub(crate) tip_strategy_id: ArcUuid7,
    #[ts(type = "string")]
    pub(crate) user_id: ArcUuid7,
    pub(crate) strategies: HashSet<Strategy>,
    #[ts(type = "number")]
    pub(crate) points: IntegerField,
    #[ts(type = "Date | null")]
    pub(crate) created_at: Option<DateTimeField>,
    #[ts(type = "Date | null")]
    pub(crate) updated_at: Option<DateTimeField>,
    #[ts(type = "Date | null")]
    pub(crate) deleted_at: Option<DateTimeField>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct StrategyPoint {
    strategy: StrategyType,
    points: u64,
}

impl From<StrategyPoint> for FieldValue {
    fn from(value: StrategyPoint) -> Self {
        FieldValue::String(
            serde_json::to_string(&value).expect("could not serialise strategy point type"),
        )
    }
}

impl From<FieldValue> for StrategyPoint {
    fn from(value: FieldValue) -> Self {
        serde_json::from_str::<StrategyPoint>(&value.to_string())
            .expect("could not deserialise strategy point type")
    }
}

impl TipRepo {
    pub async fn paginate_by_tournament_id(
        &mut self,
        tournament_id: ArcUuid7,
        page: Option<PaginateBuilder>,
    ) -> PaginateResult<Tip> {
        self.with_tournament().with_tip_strategy();
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.paginate(page).await
    }

    pub async fn list_by_tournament_id(
        &mut self,
        tournament_id: ArcUuid7,
    ) -> anyhow::Result<Vec<Tip>> {
        self.with_tournament().with_tip_strategy();
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.get().await
    }

    pub async fn by_tournament_and_id(
        &mut self,
        tournament_id: ArcUuid7,
        id: ArcUuid7,
    ) -> anyhow::Result<Option<Tip>> {
        self.with_tournament().with_tip_strategy();
        self.builder
            .is_eq(Self::col_tournament_id(), tournament_id)
            .is_eq(Self::col_id(), id);
        self.one().await
    }

    pub async fn by_tournament_tip_strategy(
        &mut self,
        tournament_id: ArcUuid7,
        tip_strategy_id: ArcUuid7,
    ) -> anyhow::Result<Vec<Tip>> {
        self.with_tournament().with_tip_strategy();
        self.builder
            .is_eq(Self::col_tournament_id(), tournament_id)
            .is_eq(Self::col_tip_strategy_id(), tip_strategy_id);
        self.get().await
    }

    pub async fn paginate_by_tournament_tip_strategy(
        &mut self,
        tournament_id: ArcUuid7,
        tip_strategy_id: ArcUuid7,
        page: Option<PaginateBuilder>,
    ) -> PaginateResult<Tip> {
        self.with_tournament().with_tip_strategy();
        self.builder
            .is_eq(Self::col_tournament_id(), tournament_id)
            .is_eq(Self::col_tip_strategy_id(), tip_strategy_id);
        self.paginate(page).await
    }

    pub async fn one_by_tournament_strategy_and_user(
        &mut self,
        tournament_id: ArcUuid7,
        strategy_id: ArcUuid7,
        user_id: ArcUuid7,
    ) -> anyhow::Result<Option<Tip>> {
        self.with_tournament().with_tip_strategy();
        self.builder
            .is_eq(Self::col_tournament_id(), tournament_id)
            .is_eq(Self::col_tip_strategy_id(), strategy_id)
            .is_eq(Self::col_user_id(), user_id);
        return self.one().await;
    }

    pub async fn all_by_tournament_and_user(
        &mut self,
        tournament_id: ArcUuid7,
        user_id: ArcUuid7,
    ) -> anyhow::Result<Vec<Tip>> {
        self.with_tournament().with_tip_strategy();
        self.builder
            .is_eq(Self::col_tournament_id(), tournament_id)
            .is_eq(Self::col_user_id(), user_id);
        self.get().await
    }

    pub async fn all_by_tip_strategy_id(
        &mut self,
        tip_strategy_id: ArcUuid7,
    ) -> anyhow::Result<Vec<Tip>> {
        self.with_tournament().with_tip_strategy();
        self.builder
            .is_eq(Self::col_tip_strategy_id(), tip_strategy_id);
        self.get().await
    }
}
