mod event;
mod event_handler;
mod http;
mod migration;
mod model;
mod permission;
mod seeder;

use dirtybase_app::db::base::manager::Manager;
use dirtybase_contract::cli_contract::prelude::ArgMatches;
use dirtybase_contract::prelude::*;

use crate::dirtybase_entry::model::country::CountryRepo;
use crate::dirtybase_entry::model::group::{CountryGroupRepo, GroupRepo};
use crate::dirtybase_entry::model::tip_strategy::TipStrategyRepo;
use crate::dirtybase_entry::model::tournament::TournamentRepo;
use crate::dirtybase_entry::model::user::UserRepo;

pub const ADMIN_ROLE: &'static str = "administrator";
pub const PLAYER_ROLE: &'static str = "player";

#[derive(Default)]
pub struct Extension;

#[dirtybase_contract::async_trait]
impl dirtybase_contract::ExtensionSetup for Extension {
    async fn setup(&mut self, _global_context: &Context) {
        event_handler::setup().await;
        ContextResourceManager::scoped("tournament_repo", |ctx| async move {
            let manager = ctx.get::<Manager>().await?;
            Ok(TournamentRepo::new(&manager))
        })
        .await;
        ContextResourceManager::scoped("tip_strategy_repo", |ctx| async move {
            let manager = ctx.get::<Manager>().await?;
            Ok(TipStrategyRepo::new(&manager))
        })
        .await;
        ContextResourceManager::scoped("user_repo", |ctx| async move {
            let manager = ctx.get::<Manager>().await?;
            Ok(UserRepo::new(&manager))
        })
        .await;
        ContextResourceManager::scoped("country_repo", |ctx| async move {
            let manager = ctx.get::<Manager>().await?;
            Ok(CountryRepo::new(&manager))
        })
        .await;
        ContextResourceManager::scoped("group_repo", |ctx| async move {
            let manager = ctx.get::<Manager>().await?;
            Ok(GroupRepo::new(&manager))
        })
        .await;
        ContextResourceManager::scoped("country_group_repo", |ctx| async move {
            let manager = ctx.get::<Manager>().await?;
            Ok(CountryGroupRepo::new(&manager))
        })
        .await;
    }

    async fn migrations(
        &self,
        _global_context: &Context,
    ) -> Option<dirtybase_contract::ExtensionMigrations> {
        migration::setup()
    }

    fn register_routes(&self, manager: &mut RouterManager) {
        http::register_routes(manager);
    }

    async fn on_cli_command(
        &self,
        cmd: &str,
        matches: ArgMatches,
        _context: Context,
    ) -> ArgMatches {
        if cmd == "seed" {
            seeder::register_seeders().await;
        }

        matches
    }
}
