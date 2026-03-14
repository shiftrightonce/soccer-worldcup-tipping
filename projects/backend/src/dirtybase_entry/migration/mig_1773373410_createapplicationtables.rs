use dirtybase_app::db::TableModel;
use dirtybase_contract::anyhow;
use dirtybase_contract::db_contract::base::manager::Manager;
use dirtybase_contract::db_contract::migration::Migration;

use crate::dirtybase_entry::model::country::Country;
use crate::dirtybase_entry::model::user::User;

pub struct Mig1773373410CreateApplicationTables;

#[dirtybase_contract::async_trait]
impl Migration for Mig1773373410CreateApplicationTables {
    async fn up(&self, manager: &Manager) -> Result<(), anyhow::Error> {
        // Users
        manager
            .create_table_schema(User::table_name(), |bp| {
                bp.uuid_as_id(None);
                bp.string(User::col_name_for_email());
                bp.json(User::col_name_for_data()).nullable();
                bp.integer(User::col_name_for_points());
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
        Ok(())
    }

    async fn down(&self, manager: &Manager) -> Result<(), anyhow::Error> {
        manager.drop_table(Country::table_name()).await?;
        manager.drop_table(User::table_name()).await?;
        Ok(())
    }
}
