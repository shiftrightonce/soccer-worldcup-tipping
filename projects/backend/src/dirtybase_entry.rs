mod config;
pub mod email;
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

use crate::dirtybase_entry::email::{EmailSender, SmtpWrapper};
use crate::dirtybase_entry::model::country::CountryRepo;
use crate::dirtybase_entry::model::game::GameRepo;
use crate::dirtybase_entry::model::group::{CountryGroupRepo, GroupRepo};
use crate::dirtybase_entry::model::strategy_result::StrategyResultRepo;
use crate::dirtybase_entry::model::tip::TipRepo;
use crate::dirtybase_entry::model::tip_strategy::TipStrategyRepo;
use crate::dirtybase_entry::model::tournament::TournamentRepo;
use crate::dirtybase_entry::model::user::UserRepo;
use crate::dirtybase_entry::model::user_validation::UserValidationRepo;

pub use config::*;

pub const ADMIN_ROLE: &'static str = "administrator";
pub const PLAYER_ROLE: &'static str = "player";

#[derive(Default)]
pub struct Extension;

#[dirtybase_contract::async_trait]
impl dirtybase_contract::ExtensionSetup for Extension {
    async fn setup(&mut self, global_context: &Context) {
        event_handler::setup().await;

        if let Err(e) = global_context
            .get_config_once::<TipConfig>("tip_config")
            .await
        {
            panic!("could not load application config: {}", e);
        }

        global_context
            .container_ref()
            .resolver(|_| async {
                let smtp_wrapper = SmtpWrapper;
                EmailSender::new(smtp_wrapper)
            })
            .await;

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
        ContextResourceManager::scoped("tip_repo", |ctx| async move {
            let manager = ctx.get::<Manager>().await?;
            Ok(TipRepo::new(&manager))
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
        ContextResourceManager::scoped("game_repo", |ctx| async move {
            let manager = ctx.get::<Manager>().await?;
            Ok(GameRepo::new(&manager))
        })
        .await;
        ContextResourceManager::scoped("country_group_repo", |ctx| async move {
            let manager = ctx.get::<Manager>().await?;
            Ok(CountryGroupRepo::new(&manager))
        })
        .await;
        ContextResourceManager::scoped("tip_result_repo", |ctx| async move {
            let manager = ctx.get::<Manager>().await?;
            Ok(StrategyResultRepo::new(&manager))
        })
        .await;
        ContextResourceManager::scoped("user_validation_repo", |ctx| async move {
            let manager = ctx.get::<Manager>().await?;
            Ok(UserValidationRepo::new(&manager))
        })
        .await;

        // FIXME: cron and queue crate are messed up. Fix both before going further

        // dirtybase_cron::CronJobRegisterer::register("calculate_points", |job| {
        //     Box::pin(async move {
        //         tracing::info!(
        //             "checking tip results that need to be calculated: {}",
        //             job.id()
        //         );
        //     })
        // })
        // .await;

        // // FIXME: This should be remove as the cron crate suppose to handle this based on the env vars, but for some reason it is not working, need to investigate
        // let config = JobConfig::new("calculate_points", "every 6 seconds", true, None, None);
        // let r = CronJobRegisterer::new(Context::make_global().await, config);
        // if let Ok(j) = r.get_handler().await {
        //     _ = j.schedule().await;
        // }
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
