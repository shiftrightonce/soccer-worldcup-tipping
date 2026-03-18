use dirtybase_app::{
    db::types::{ArcUuid7, CreatedAtField, DeletedAtField, StringField, UpdatedAtField},
    db_macro::DirtyTable,
};
use dirtybase_contract::auth_contract::Actor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, DirtyTable, Serialize, Deserialize)]
#[dirty(id_not_auto, id = "push_subscription")]
pub struct UserData {
    pub push_subscription: Option<StringField>,
}

impl UserData {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, Clone, DirtyTable, Serialize, Deserialize)]
#[dirty(id_not_auto, timestamp, soft_deletable)]
pub struct User {
    id: Option<ArcUuid7>,
    email: StringField,
    #[dirty(embedded)]
    data: UserData,
    #[dirty(rel(kind = "belongs_to"))]
    actor: Option<Actor>,
    auth_actor_id: Option<ArcUuid7>,
    created_at: CreatedAtField,
    updated_at: UpdatedAtField,
    deleted_at: DeletedAtField,
}

impl User {
    pub fn new(email: &str, actor_id: ArcUuid7) -> Self {
        Self {
            id: Some(ArcUuid7::default()),
            email: email.to_string().into(),
            auth_actor_id: Some(actor_id),
            ..Default::default()
        }
    }
}
