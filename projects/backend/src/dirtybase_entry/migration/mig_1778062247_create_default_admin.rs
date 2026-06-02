use dirtybase_contract::anyhow;
use dirtybase_contract::auth_contract::{
    ActorPayload, ActorRole, AuthUserStatus, PermStorageProvider, Permission, PermissionStorage,
    PersistActorPayload, PersistActorRolePayload, PersistPermissionPayload, PersistRolePayload,
    PersistRolePermission, Role, RolePermission,
};
use dirtybase_contract::db_contract::base::manager::Manager;
use dirtybase_contract::db_contract::migration::Migration;
use dirtybase_contract::prelude::Context;

use crate::dirtybase_entry::model::user::User;
use crate::dirtybase_entry::{ADMIN_ROLE, PLAYER_ROLE};

pub struct Mig1778062247CreateDefaultAdmin;

#[dirtybase_contract::async_trait]
impl Migration for Mig1778062247CreateDefaultAdmin {
    async fn up(&self, manager: &Manager, context: &Context) -> Result<(), anyhow::Error> {
        let mut user_repo = User::repo_instance(&manager);

        if user_repo.count().await.unwrap() > 0 {
            return Ok(());
        }

        let storage = context.get::<PermStorageProvider>().await?;
        let roles = [
            storage
                .save_role(PersistRolePayload::Save {
                    role: Role::new(ADMIN_ROLE, "Adminstrator"),
                })
                .await?
                .unwrap(),
            storage
                .save_role(PersistRolePayload::Save {
                    role: Role::new(PLAYER_ROLE, "Player"),
                })
                .await?
                .unwrap(),
        ];

        let admin_permission = storage
            .save_permission(PersistPermissionPayload::Save {
                perm: Permission::new("*", "Full Permission"),
            })
            .await?
            .unwrap();

        let player_permissions = [
            storage
                .save_permission(PersistPermissionPayload::Save {
                    perm: Permission::new("place-tip", "Can Place Tips"),
                })
                .await?
                .unwrap(),
            storage
                .save_permission(PersistPermissionPayload::Save {
                    perm: Permission::new("chat", "Can Chat"),
                })
                .await?
                .unwrap(),
        ];

        _ = storage
            .save_role_permission(PersistRolePermission::Save {
                record: RolePermission::new(
                    admin_permission.id().cloned().unwrap(),
                    roles[0].id().cloned(),
                    None,
                ),
            })
            .await?;

        for p in &player_permissions {
            _ = storage
                .save_role_permission(PersistRolePermission::Save {
                    record: RolePermission::new(
                        p.id().cloned().unwrap(),
                        roles[1].id().cloned(),
                        None,
                    ),
                })
                .await?;
        }

        let username = "administrator".to_string();
        let email = "administrator@example.com".to_string();
        let actor = ActorPayload {
            email: Some(email.clone()),
            username: Some(username),
            status: Some(AuthUserStatus::Active),
            password: Some("changeme".to_string()),
            ..Default::default()
        };
        let actor_payload = PersistActorPayload::Save {
            actor: actor.into(),
        };

        if let Some(actor) = storage
            .save_actor(actor_payload)
            .await
            .expect("could not create user's actor")
        {
            let actor_id = actor.id().cloned().unwrap();
            let mut user = User::new(&email, actor_id.clone());
            user.generate_avatar()
                .expect("could not generate avatar for admin");
            user_repo.insert(user).await.expect("could not create user");
            let actor_role = PersistActorRolePayload::Save {
                record: ActorRole::new(actor_id, roles[0].id().cloned().unwrap()),
            };

            storage
                .save_actor_role(actor_role)
                .await
                .expect("could not save admin's actor role");
        }

        Ok(())
    }

    async fn down(&self, _manager: &Manager, _: &Context) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
