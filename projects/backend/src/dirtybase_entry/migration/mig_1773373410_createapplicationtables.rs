use dirtybase_app::db::TableModel;
use dirtybase_contract::anyhow;
use dirtybase_contract::auth_contract::Actor;
use dirtybase_contract::db_contract::base::manager::Manager;
use dirtybase_contract::db_contract::migration::Migration;

use crate::dirtybase_entry::model::country::Country;
use crate::dirtybase_entry::model::game::Game;
use crate::dirtybase_entry::model::tip::Tip;
use crate::dirtybase_entry::model::tip_strategy::TipStrategy;
use crate::dirtybase_entry::model::user::User;

pub struct Mig1773373410CreateApplicationTables;

#[dirtybase_contract::async_trait]
impl Migration for Mig1773373410CreateApplicationTables {
    async fn up(&self, manager: &Manager) -> Result<(), anyhow::Error> {
        // Tip
        manager
            .create_table_schema(Tip::table_name(), |bp| {
                bp.uuid_as_id(None);
                bp.uuid_table_fk::<User>(true);
                bp.json(Tip::col_name_for_strategies())
                    .default_is_empty_object();
                bp.integer(Tip::col_name_for_points()).default_is_zero();
                bp.timestamps();
                bp.soft_deletable();
            })
            .await?;

        // Tip Strategy
        manager
            .create_table_schema(TipStrategy::table_name(), |bp| {
                bp.uuid_as_id(None);
                bp.uuid_table_fk::<Game>(true)
                    .set_is_nullable(true)
                    .set_is_unique(true);
                bp.timestamp(TipStrategy::col_name_for_opens_at());
                bp.timestamp(TipStrategy::col_name_for_ends_at());
                bp.timestamp(TipStrategy::col_name_for_calculate_points_on());
                bp.boolean(TipStrategy::col_name_for_completed());
                bp.json(TipStrategy::col_name_for_strategy_types())
                    .default_is_empty_array();
                bp.timestamps();
                bp.soft_deletable();
            })
            .await?;
        // Users
        manager
            .create_table_schema(User::table_name(), |bp| {
                bp.uuid_as_id(None);
                bp.string(User::col_name_for_email());
                bp.json(User::col_name_for_data()).nullable();
                bp.uuid_table_fk::<Actor>(true)
                    .set_is_nullable(true)
                    .set_is_unique(true);
                bp.timestamps();
                bp.soft_deletable();
            })
            .await?;
        // Country
        manager
            .create_table_schema(Country::table_name(), |bp| {
                bp.uuid_as_id(None);
                bp.sized_string(Country::col_name_for_game_code(), 2);
                bp.string(Country::col_name_for_name());
                bp.sized_string(Country::col_name_for_short(), 4);
                bp.string(Country::col_name_for_image());
                bp.timestamps();
                bp.soft_deletable();
            })
            .await?;
        // Game
        manager
            .create_table_schema(Game::table_name(), |bp| {
                bp.uuid_as_id(None);
                bp.string(Game::col_name_for_label());
                bp.integer(Game::col_name_for_year());
                bp.uuid_fk_as(
                    Country::table_name(),
                    Game::col_name_for_country_a(),
                    true,
                    Some(Country::id_column()),
                );
                bp.uuid_fk_as(
                    Country::table_name(),
                    Game::col_name_for_country_b(),
                    true,
                    Some(Country::id_column()),
                );
                bp.boolean(Game::col_name_for_penalty()).default_is_false();
                bp.integer(Game::col_name_for_country_a_goals())
                    .default_is_zero();
                bp.integer(Game::col_name_for_country_b_goals())
                    .default_is_zero();
                bp.integer(Game::col_name_for_country_a_penalty_goals())
                    .default_is_zero();
                bp.integer(Game::col_name_for_country_b_penalty_goals())
                    .default_is_zero();
                bp.uuid_fk_as(
                    Country::table_name(),
                    Game::col_name_for_winner(),
                    true,
                    Some(Country::id_column()),
                )
                .set_is_nullable(true);
                bp.timestamps();
                bp.soft_deletable();
            })
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &Manager) -> Result<(), anyhow::Error> {
        manager.drop_table(Tip::table_name()).await?;
        manager.drop_table(TipStrategy::table_name()).await?;
        manager.drop_table(Game::table_name()).await?;
        manager.drop_table(Country::table_name()).await?;
        manager.drop_table(User::table_name()).await?;
        Ok(())
    }
}
