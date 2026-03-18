use dirtybase_contract::app_contract::Context;
use dirtybase_contract::auth_contract::storage::{PermStorageProvider, PermissionStorage};
use dirtybase_contract::auth_contract::{ActorPayload, AuthUserStatus, PersistActorPayload};
use dirtybase_contract::db_contract::base::manager::Manager;
use dirtybase_contract::prelude::*;

use crate::dirtybase_entry::model::country::Country;
use crate::dirtybase_entry::model::user::User;

pub(crate) async fn seed(manager: Manager, context: Context) {
    seed_users(&manager, &context).await;
    seed_countries(&manager).await
}

async fn seed_users(manager: &Manager, context: &Context) {
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
            let user = User::new(&email, actor.id().cloned().unwrap());
            user_repo.insert(user).await.expect("could not create user");
        }
    }
}

async fn seed_countries(manager: &Manager) {
    let group_total = 12;
    let mut group = 'A';

    for _ in 1..=group_total {
        let mut country_repo = Country::repo_instance(&manager);
        for group_member in 1..=4 {
            let game_code = format!("{}{}", group, group_member);
            let name = dirtybase_helper::random::random_string(8);
            let short = dirtybase_helper::random::random_string(3).to_uppercase();
            let image = dirtybase_helper::random::random_string(10);

            let country = Country::new(&game_code, &name, &short, &image);
            _ = country_repo
                .insert(country)
                .await
                .expect("could not create mock country data");
        }

        group = std::char::from_u32(group as u32 + 1).unwrap();
    }
}
