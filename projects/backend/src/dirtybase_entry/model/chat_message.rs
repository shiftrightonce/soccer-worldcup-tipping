use dirtybase_app::{
    db::types::{ArcUuid7, DateTimeField, StringField},
    db_macro::DirtyTable,
};
use serde::Serialize;

use crate::dirtybase_entry::model::{chat_room::ChatRoom, user::User};

#[derive(Debug, Clone, Default, Serialize, DirtyTable, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
#[dirty(id_not_auto, timestampable, soft_deletable)]
pub struct ChatMessage {
    #[ts(type = "string")]
    id: Option<ArcUuid7>,
    #[dirty(rel(kind = "belongs_to"))]
    room: Option<ChatRoom>,
    #[dirty(rel(kind = "belongs_to"))]
    user: Option<User>,
    content: StringField,
    #[ts(type = "string")]
    chat_room_id: ArcUuid7,
    #[ts(type = "string")]
    user_id: ArcUuid7,
    #[ts(type = "Date | null")]
    created_at: Option<DateTimeField>,
    #[ts(type = "Date | null")]
    updated_at: Option<DateTimeField>,
    #[ts(type = "Date | null")]
    deleted_at: Option<DateTimeField>,
}
