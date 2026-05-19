use dirtybase_app::{
    db::types::{
        ArcUuid7, CreatedAtField, DeletedAtField, IntegerField, StringField, UpdatedAtField,
    },
    db_macro::DirtyTable,
};
use serde::Serialize;

use crate::dirtybase_entry::model::{country::Country, tournament::Tournament};

#[derive(Debug, Default, Clone, DirtyTable, Serialize, ts_rs::TS)]
#[ts(export)]
#[dirty(soft_deletable, id_not_auto, timestamp, id_not_auto)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[ts(type = "string")]
    pub(crate) id: Option<ArcUuid7>,
    pub(crate) name: StringField,
    #[ts(type = "Date")]
    pub(crate) created_at: CreatedAtField,
    #[ts(type = "Date")]
    pub(crate) updated_at: UpdatedAtField,
    #[ts(type = "Date")]
    pub(crate) deleted_at: DeletedAtField,
}

#[derive(Debug, Default, Clone, DirtyTable, Serialize, ts_rs::TS)]
#[ts(export)]
#[dirty(soft_deletable, id_not_auto, timestamp, id_not_auto)]
#[serde(rename_all = "camelCase")]
pub struct CountryGroup {
    #[ts(type = "string")]
    pub(crate) id: Option<ArcUuid7>,
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) tournament: Option<Tournament>,
    #[ts(type = "string")]
    pub(crate) tournament_id: ArcUuid7,
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) group: Option<Group>,
    #[ts(type = "string")]
    pub(crate) group_id: ArcUuid7,
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) country: Option<Country>,
    #[ts(type = "string")]
    pub(crate) country_id: ArcUuid7,
    pub(crate) is_out: bool,
    pub(crate) points: IntegerField,
    #[ts(type = "Date | null")]
    pub(crate) created_at: CreatedAtField,
    #[ts(type = "Date | null")]
    pub(crate) updated_at: UpdatedAtField,
    #[ts(type = "Date | null")]
    pub(crate) deleted_at: DeletedAtField,
}
