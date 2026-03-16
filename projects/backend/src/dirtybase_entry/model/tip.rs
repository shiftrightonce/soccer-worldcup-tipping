use std::collections::HashSet;

use dirtybase_app::{
    db::types::{ArcUuid7, DateTimeField, IntegerField},
    db_macro::DirtyTable,
};
use serde::{Deserialize, Serialize};

use crate::dirtybase_entry::model::tip_strategy::Strategy;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, DirtyTable)]
#[dirty(soft_deletable, timestamps)]
pub struct Tip {
    id: Option<ArcUuid7>,
    strategy_id: ArcUuid7,
    user_id: ArcUuid7,
    strategies: HashSet<Strategy>,
    points: IntegerField,
    created_at: Option<DateTimeField>,
    updated_at: Option<DateTimeField>,
    deleted_at: Option<DateTimeField>,
}
