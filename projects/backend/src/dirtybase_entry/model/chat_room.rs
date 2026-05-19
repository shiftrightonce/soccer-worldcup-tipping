use dirtybase_app::{
    db::{
        field_values::FieldValue,
        types::{ArcUuid7, DateTimeField, StringField},
    },
    db_macro::DirtyTable,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub enum RoomType {
    #[default]
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

impl From<FieldValue> for RoomType {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::String(s) => {
                if s == "private" {
                    RoomType::Private
                } else {
                    RoomType::Public
                }
            }
            _ => RoomType::default(),
        }
    }
}

impl From<RoomType> for FieldValue {
    fn from(value: RoomType) -> Self {
        match value {
            RoomType::Private => FieldValue::String("private".to_string()),
            RoomType::Public => FieldValue::String("public".to_string()),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, DirtyTable, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
#[dirty(timestampable, id_not_auto, soft_deletable)]
pub struct ChatRoom {
    #[ts(type = "string")]
    id: Option<ArcUuid7>,
    #[ts(type = "string")]
    name: StringField,
    #[ts(type = "string")]
    room_type: RoomType,
    #[ts(type = "Date | null")]
    created_at: Option<DateTimeField>,
    #[ts(type = "Date | null")]
    updated_at: Option<DateTimeField>,
    #[ts(type = "Date | null")]
    deleted_at: Option<DateTimeField>,
}

impl ChatRoom {
    pub fn new(name: String, room_type: RoomType) -> Self {
        Self {
            name: StringField::from(name),
            room_type,
            ..Default::default()
        }
    }
}
