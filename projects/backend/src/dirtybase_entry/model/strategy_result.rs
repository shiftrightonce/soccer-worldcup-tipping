use std::collections::HashSet;

use dirtybase_app::{
    db::{
        base::manager::Manager,
        types::{ArcUuid7, CreatedAtField, DeletedAtField, UpdatedAtField},
    },
    db_macro::DirtyTable,
};
use dirtybase_common::anyhow;
use serde::Serialize;

use crate::dirtybase_entry::model::{
    tip::TipRepo,
    tip_strategy::{Strategy, TipStrategyRepo},
};

#[derive(Debug, Clone, Default, Serialize, DirtyTable, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
#[dirty(soft_deletable, timestamps, id_not_auto)]
pub struct StrategyResult {
    #[ts(type = "string")]
    pub(crate) id: Option<ArcUuid7>,
    #[ts(type = "string")]
    pub(crate) tip_strategy_id: ArcUuid7,
    pub(crate) strategy_results: HashSet<Strategy>,
    #[ts(type = "Date | null")]
    pub(crate) created_at: CreatedAtField,
    #[ts(type = "Date | null")]
    pub(crate) updated_at: UpdatedAtField,
    #[ts(type = "Date | null")]
    pub(crate) deleted_at: DeletedAtField,
}

impl StrategyResultRepo {
    pub async fn find_by_tip_strategy_id(
        &mut self,
        tip_strategy_id: ArcUuid7,
    ) -> Result<Option<StrategyResult>, anyhow::Error> {
        self.find_by(|query| {
            query.is_eq(Self::col_tip_strategy_id(), tip_strategy_id);
        })
        .await
    }

    pub async fn save(
        &mut self,
        mut strategy_result: StrategyResult,
    ) -> Result<StrategyResult, anyhow::Error> {
        let tip_strategy_id = strategy_result.tip_strategy_id.clone();
        match self
            .find_by_tip_strategy_id(strategy_result.tip_strategy_id.clone())
            .await
        {
            Ok(Some(mut existing)) => {
                existing.strategy_results = strategy_result.strategy_results;
                let id = existing.id.clone().unwrap_or_default();
                let result = self.update(existing).await;
                if result.is_ok() {
                    self.mark_tip_strategy_as_completed(tip_strategy_id).await?;
                }
                Self::do_calculation(id, &self.manager).await;
                result
            }
            Ok(None) => {
                strategy_result.id = Some(ArcUuid7::default());
                let id = strategy_result.id.clone().unwrap_or_default();
                let result = self.insert(strategy_result).await;

                if result.is_ok() {
                    self.mark_tip_strategy_as_completed(tip_strategy_id).await?;
                }
                Self::do_calculation(id, &self.manager).await;
                result
            }
            Err(e) => Err(e),
        }
    }

    async fn mark_tip_strategy_as_completed(
        &mut self,
        tip_strategy_id: ArcUuid7,
    ) -> Result<(), anyhow::Error> {
        let mut tip_strategy_repo = TipStrategyRepo::new(&self.manager);
        if let Some(mut tip_strategy) = tip_strategy_repo.by_id(tip_strategy_id).await? {
            tip_strategy.completed = true;
            tip_strategy_repo.update(tip_strategy).await?;
        }
        Ok(())
    }

    async fn do_calculation(id: ArcUuid7, manager: &Manager) {
        let mut strategy_result_repo = StrategyResultRepo::new(manager);
        if let Ok(Some(tip_strategy_result)) = strategy_result_repo.by_id(id).await {
            let mut tip_repo = TipRepo::new(manager);
            let tips = tip_repo
                .all_by_tip_strategy_id(tip_strategy_result.tip_strategy_id.clone())
                .await
                .unwrap_or_default();
            tracing::error!(
                "total tips to process for tip strategy id {}: {}",
                tip_strategy_result.tip_strategy_id,
                tips.len()
            );
            for tip in tips {
                // Here you would calculate the points for each tip based on the strategy result and update the tip accordingly
                tracing::info!(
                    "Calculating points for tip id: {} based on strategy result id: {}",
                    tip.id.clone().unwrap_or_default(),
                    tip_strategy_result.id.clone().unwrap_or_default()
                );
            }
        }
    }
}
