use dirtybase_app::{
    db::{
        field_values::FieldValue,
        types::{
            ArcStrField, ArcUuid7, CreatedAtField, DeletedAtField, LabelField, NameField,
            UpdatedAtField,
        },
    },
    db_macro::DirtyTable,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub enum TournamentStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    #[default]
    Pending,
    #[serde(rename = "completed")]
    Completed,
}

impl From<TournamentStatus> for FieldValue {
    fn from(value: TournamentStatus) -> Self {
        FieldValue::String(
            serde_json::to_string(&value).expect("could not serialise tournament status"),
        )
    }
}

impl From<FieldValue> for TournamentStatus {
    fn from(value: FieldValue) -> Self {
        serde_json::from_str::<TournamentStatus>(&value.to_string())
            .expect("could not deserialise strategy")
    }
}

#[derive(Debug, Default, Clone, Serialize, DirtyTable, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
#[dirty(soft_deletable, timestamp, id_not_auto)]
pub struct Tournament {
    #[ts(type = "string")]
    pub(crate) id: Option<ArcUuid7>,
    #[ts(type = "string")]
    pub(crate) name: NameField,
    #[ts(type = "string")]
    pub(crate) label: LabelField,
    pub(crate) description: ArcStrField,
    pub(crate) status: TournamentStatus,
    #[ts(type = "Date | null")]
    pub(crate) created_at: CreatedAtField,
    #[ts(type = "Date | null")]
    pub(crate) updated_at: UpdatedAtField,
    #[ts(type = "Date | null")]
    pub(crate) deleted_at: DeletedAtField,
}
