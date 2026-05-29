use dirtybase_app::{
    db::types::{ArcUuid7, CreatedAtField, DeletedAtField, StringField, UpdatedAtField},
    db_macro::DirtyTable,
};
use dirtybase_common::anyhow;
use dirtybase_contract::auth_contract::{Actor, ActorRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, DirtyTable, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
#[dirty(id_not_auto, id = "push_subscription")]
pub struct UserData {
    pub push_subscription: Option<StringField>,
}

impl UserData {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, Clone, DirtyTable, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
#[dirty(id_not_auto, timestamp, soft_deletable)]
pub struct User {
    #[ts(type = "string")]
    pub(crate) id: Option<ArcUuid7>,
    email: StringField,
    #[dirty(embedded)]
    pub(crate) data: UserData,
    #[dirty(rel(kind = "belongs_to"))]
    #[ts(type = "object")]
    pub(crate) actor: Option<Actor>,
    #[dirty(rel(
        kind = "has_many_through",
        soft_deletable,
        pivot = Actor,
        pivot_through_col = "id",
        through_col="auth_actor_id",
        local_col = "auth_actor_id",
        foreign_col = "id"))
    ]
    #[ts(type = "object")]
    pub(crate) actor_roles: Option<Vec<ActorRole>>,
    #[ts(type = "string")]
    pub(crate) auth_actor_id: Option<ArcUuid7>,
    #[ts(type = "Date | null")]
    pub(crate) created_at: CreatedAtField,
    #[ts(type = "Date | null")]
    pub(crate) updated_at: UpdatedAtField,
    #[ts(type = "Date | null")]
    pub(crate) deleted_at: DeletedAtField,
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

impl UserRepo {
    pub async fn by_actor_id(&mut self, actor_id: ArcUuid7) -> Result<Option<User>, anyhow::Error> {
        self.filter(|q| {
            q.is_eq(Self::col_auth_actor_id(), actor_id);
        });

        self.one().await
    }
}
