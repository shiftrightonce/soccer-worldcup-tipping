use std::collections::HashMap;

use dirtybase_contract::app_contract::Context;
use dirtybase_contract::auth_contract::storage::{PermStorageProvider, PermissionStorage};
use dirtybase_contract::auth_contract::{
    ActorPayload, ActorRole, AuthUserStatus, FetchRolePayload, PersistActorPayload,
    PersistActorRolePayload, Role,
};
use dirtybase_contract::db_contract::base::manager::Manager;

use crate::dirtybase_entry::model::user::User;
use crate::dirtybase_entry::{ADMIN_ROLE, PLAYER_ROLE};

pub(crate) async fn seed(manager: Manager, context: Context) {
    seed_users(&manager, &context).await;
}

async fn seed_users(manager: &Manager, context: &Context) {
    let roles = seed_roles(context).await;
    let mut user_repo = User::repo_instance(&manager);
    let auth_storage = context
        .get::<PermStorageProvider>()
        .await
        .expect("could not get auth storage");

    for u in 1..200 {
        let username = format!("tip_user{}", u);
        let email = format!("tip_user{}@example.com", u);
        let actor = ActorPayload {
            email: Some(email.clone()),
            username: Some(username),
            status: Some(AuthUserStatus::Active),
            password: Some("password".to_string()),
            ..Default::default()
        };

        let actor_payload = PersistActorPayload::Save {
            actor: actor.into(),
        };
        if let Some(actor) = auth_storage
            .save_actor(actor_payload)
            .await
            .expect("could not create user's actor")
        {
            let actor_id = actor.id().cloned().unwrap();
            let user = User::new(&email, actor_id.clone());
            user_repo.insert(user).await.expect("could not create user");
            let actor_role = PersistActorRolePayload::Save {
                record: ActorRole::new(
                    actor_id,
                    roles
                        .get(if u == 1 { ADMIN_ROLE } else { PLAYER_ROLE })
                        .cloned()
                        .unwrap()
                        .id()
                        .cloned()
                        .unwrap(),
                ),
            };
            auth_storage
                .save_actor_role(actor_role)
                .await
                .expect("could not save actor role");
        }
    }
}

async fn seed_roles(context: &Context) -> HashMap<String, Role> {
    let auth_storage = context
        .get::<PermStorageProvider>()
        .await
        .expect("could not get auth storage");

    let mut roles = HashMap::new();
    if let Ok(Some(r)) = auth_storage
        .fetch_role(
            FetchRolePayload::ByName {
                name: ADMIN_ROLE.to_string().into(),
            },
            None,
        )
        .await
    {
        roles.insert(r.name().to_string(), r);
    }

    if let Ok(Some(r)) = auth_storage
        .fetch_role(
            FetchRolePayload::ByName {
                name: PLAYER_ROLE.to_string().into(),
            },
            None,
        )
        .await
    {
        roles.insert(r.name().to_string(), r);
    }

    roles
}
