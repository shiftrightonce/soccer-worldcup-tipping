use dirtybase_app::{
    db::{
        base::helper::generate_ulid,
        types::{ArcUuid7, CreatedAtField, DeletedAtField, StringField, UpdatedAtField},
    },
    db_macro::DirtyTable,
};
use dirtybase_common::anyhow;
use dirtybase_contract::{
    auth_contract::{Actor, ActorRole},
    prelude::{Context, Observable},
};
use identicon_rs::Identicon;
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
    pub(crate) email: StringField,
    avatar: Option<StringField>,
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

    pub fn generate_avatar(&mut self) -> anyhow::Result<String> {
        let mut image = Identicon::new(&self.email);
        let avater_dir = "public/avatar";
        let name = format!("{}.png", generate_ulid());
        let avatar_path = format!("{}/{}", &avater_dir, name);

        if let Err(e) = std::fs::create_dir_all(avater_dir) {
            anyhow::bail!("Failed to create avatar directory: {e}");
        }

        image.set_border(10);

        if let Err(e) = image.set_scale(100) {
            anyhow::bail!("could not set image scale: {}", e);
        }

        match image.export_png_data() {
            Ok(bytes) => {
                if let Err(e) = std::fs::write(&avatar_path, bytes) {
                    anyhow::bail!("Failed to save avatar: {e}");
                }
            }
            Err(e) => anyhow::bail!("Failed to generate avatar: {e}"),
        }

        if let Err(e) = image.save_image(&avatar_path) {
            anyhow::bail!("Failed to save avatar: {e}");
        }

        self.avatar = Some(name.clone().into());
        Ok(avatar_path)
    }
}

impl UserRepo {
    pub async fn by_actor_id(&mut self, actor_id: ArcUuid7) -> Result<Option<User>, anyhow::Error> {
        self.filter(|q| {
            q.is_eq(Self::col_auth_actor_id(), actor_id);
        });

        self.one().await
    }

    pub async fn register(&mut self, user: User, context: Context) -> Result<User, anyhow::Error> {
        let u = self.insert(user).await?;
        //
        let mut repo = self.clone();
        let mut user = u.clone();

        tokio::spawn(async move {
            // Generate avater
            if let Err(e) = user.generate_avatar() {
                return tracing::error!("Failed to save avatar: {e}");
            } else {
                if let Err(e) = repo.update(user.clone()).await {
                    return tracing::error!("Failed to update user with avatar: {e}");
                }
                if let Ok(Some(user)) = repo
                    .with_actor()
                    .with_actor_roles()
                    .by_id(user.id.unwrap())
                    .await
                {
                    UserCreated { user }.notify(&context).await;
                }
            }
        });

        Ok(u)
    }
}

pub struct UserCreated {
    user: User,
}

impl UserCreated {
    pub fn user_ref(&self) -> &User {
        &self.user
    }
}

impl Observable for UserCreated {}
