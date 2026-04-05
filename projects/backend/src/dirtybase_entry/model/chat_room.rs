use dirtybase_app::{
    db::{
        field_values::FieldValue,
        types::{ArcUuid7, DateTimeField, StringField},
    },
    db_macro::DirtyTable,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, DirtyTable)]
pub struct ChatRoom {
    id: Option<ArcUuid7>,
    name: StringField,
    room_type: RoomType,
    created_at: Option<DateTimeField>,
    updated_at: Option<DateTimeField>,
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
