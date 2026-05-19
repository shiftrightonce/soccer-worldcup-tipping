use dirtybase_app::{
    db::types::{ArcUuid7, DateTimeField},
    db_macro::DirtyTable,
};
use serde::Serialize;

use crate::dirtybase_entry::model::{chat_room::ChatRoom, user::User};

#[derive(Debug, Clone, Default, DirtyTable, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
#[dirty(id_not_auto, timestable, soft_deletable)]
pub struct ChatRoomUser {
    #[ts(type = "string")]
    id: Option<ArcUuid7>,
    #[dirty(rel(kind = "belongs_to"))]
    room: Option<ChatRoom>,
    #[dirty(rel(kind = "belongs_to"))]
    user: Option<User>,
    #[ts(type = "string")]
    chat_room_id: ArcUuid7,
    #[ts(type = "string")]
    user_id: ArcUuid7,
    is_admin: bool,
    #[ts(type = "Date | null")]
    created_at: Option<DateTimeField>,
    #[ts(type = "Date | null")]
    updated_at: Option<DateTimeField>,
    #[ts(type = "Date | null")]
    deleted_at: Option<DateTimeField>,
}

impl ChatRoomUser {
    pub fn new(chat_room_id: ArcUuid7, user_id: ArcUuid7, is_admin: bool) -> Self {
        Self {
            chat_room_id,
            user_id,
            is_admin,
            ..Default::default()
        }
    }
}
