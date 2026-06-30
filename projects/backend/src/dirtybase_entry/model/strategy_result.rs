use std::collections::{HashMap, HashSet};

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
    tip::{StrategyPoint, TipRepo},
    tip_strategy::{Strategy, StrategyType, TipStrategyRepo},
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

    async fn do_calculation(id: ArcUuid7, m: &Manager) {
        let manager = m.clone();
        tokio::spawn(async move {
            let mut strategy_result_repo = StrategyResultRepo::new(&manager);
            if let Ok(Some(tip_strategy_result)) = strategy_result_repo.by_id(id).await {
                let mut tip_repo = TipRepo::new(&manager);

                // let mut tips_page = tip_repo
                //     .paginate_by_tip_strategy_id(tip_strategy_result.tip_strategy_id.clone(), None)
                //     .await;
                // tracing::error!("{:#?}", tips_page.data_ref());

                let tip_strategy_id = tip_strategy_result.tip_strategy_id.clone();
                let strategies = HashMap::<StrategyType, Strategy>::from_iter(
                    tip_strategy_result
                        .strategy_results
                        .iter()
                        .map(|s| (s.strategy_type(), s.clone())),
                );

                let tips = match tip_repo.all_by_tip_strategy_id(tip_strategy_id).await {
                    Ok(list) => list,
                    Err(e) => {
                        tracing::error!("could not get tips: {}", e);
                        Vec::new()
                    }
                };

                // loop {
                //     let tips = tips_page.data_ref().as_ref().unwrap_or(&vec![]).clone();
                //     if tips.is_empty() {
                //         break;
                //     }
                // tracing::debug!("Calculating points for {} tips", tips.len());
                // tracing::error!("paginator: {:#?}", tips_page.next_ref());

                for mut tip in tips {
                    tracing::debug!("calculating: {:#?}, user: {}", &tip.id, &tip.user_id);
                    tip.points = 0;
                    for a_strategy in tip.strategies.iter() {
                        if let Some(b_strategy) = strategies.get(&a_strategy.strategy_type()) {
                            let strategy_point = StrategyPoint {
                                strategy: a_strategy.strategy_type(),
                                points: a_strategy.compare_and_score(b_strategy),
                            };
                            tip.points += strategy_point.points;
                            tip.tip_strategy_pts.insert(strategy_point);
                        }
                    }
                    if let Err(e) = tip_repo.update(tip).await {
                        tracing::error!("Failed to update tip with new points: {}", e);
                    }
                }

                //     tips_page = tip_repo
                //         .paginate_by_tip_strategy_id(
                //             tip_strategy_result.tip_strategy_id.clone(),
                //             tips_page.next_ref().cloned(),
                //         )
                //         .await;
                // }
            }
        });
    }
}
