use dirtybase_app::{
    db::types::{ArcUuid7, DateTimeField},
    db_macro::DirtyTable,
};
use serde::{Deserialize, Serialize};

use crate::dirtybase_entry::model::{chat_room::ChatRoom, user::User};

#[derive(Debug, Clone, Default, DirtyTable, Serialize, Deserialize)]
pub struct ChatRoomUser {
    id: Option<ArcUuid7>,
    #[dirty(rel(kind = "belongs_to"))]
    room: Option<ChatRoom>,
    #[dirty(rel(kind = "belongs_to"))]
    user: Option<User>,
    chat_room_id: ArcUuid7,
    user_id: ArcUuid7,
    is_admin: bool,
    created_at: Option<DateTimeField>,
    updated_at: Option<DateTimeField>,
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
