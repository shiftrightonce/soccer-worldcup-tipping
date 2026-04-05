use dirtybase_app::{
    db::types::{ArcUuid7, DateTimeField, StringField},
    db_macro::DirtyTable,
};
use serde::{Deserialize, Serialize};

use crate::dirtybase_entry::model::{chat_room::ChatRoom, user::User};

#[derive(Debug, Clone, Default, Serialize, Deserialize, DirtyTable)]
pub struct ChatMessage {
    id: Option<ArcUuid7>,
    #[dirty(rel(kind = "belongs_to"))]
    room: Option<ChatRoom>,
    #[dirty(rel(kind = "belongs_to"))]
    user: Option<User>,
    content: StringField,
    chat_room_id: ArcUuid7,
    user_id: ArcUuid7,
    created_at: Option<DateTimeField>,
    updated_at: Option<DateTimeField>,
    deleted_at: Option<DateTimeField>,
}
