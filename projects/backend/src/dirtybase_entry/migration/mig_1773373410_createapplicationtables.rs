use dirtybase_app::db::TableModel;
use dirtybase_contract::anyhow;
use dirtybase_contract::auth_contract::Actor;
use dirtybase_contract::db_contract::base::manager::Manager;
use dirtybase_contract::db_contract::migration::Migration;
use dirtybase_contract::prelude::Context;

use crate::dirtybase_entry::model::chat_message::ChatMessage;
use crate::dirtybase_entry::model::chat_room::ChatRoom;
use crate::dirtybase_entry::model::chat_room_user::ChatRoomUser;
use crate::dirtybase_entry::model::country::Country;
use crate::dirtybase_entry::model::game::Game;
use crate::dirtybase_entry::model::group::{CountryGroup, Group};
use crate::dirtybase_entry::model::tip::Tip;
use crate::dirtybase_entry::model::tip_strategy::TipStrategy;
use crate::dirtybase_entry::model::tournament::Tournament;
use crate::dirtybase_entry::model::user::User;

pub struct Mig1773373410CreateApplicationTables;

#[dirtybase_contract::async_trait]
impl Migration for Mig1773373410CreateApplicationTables {
    async fn up(&self, manager: &Manager, _: &Context) -> Result<(), anyhow::Error> {
        // Tournament
        manager
            .create_table_schema(Tournament::table_name(), |bp| {
                bp.uuid_as_id(None);
                bp.string(Tournament::col_name_for_name())
                    .set_is_nullable(false)
                    .set_is_unique(true);
                bp.string(Tournament::col_name_for_status())
                    .set_is_nullable(false);
                bp.string(Tournament::col_name_for_label())
                    .set_is_nullable(false);
                bp.text(Tournament::col_name_for_description())
                    .set_is_nullable(true);
                bp.timestamps();
                bp.soft_deletable();
            })
            .await?;

        // Country
        manager
            .create_table_schema(Country::table_name(), |bp| {
                bp.uuid_as_id(None);
                bp.string(Country::col_name_for_name());
                bp.sized_string(Country::col_name_for_alpha2(), 2);
                bp.sized_string(Country::col_name_for_alpha3(), 3);
                bp.timestamps();
                bp.soft_deletable();
            })
            .await?;

        // Group
        manager
            .create_table_schema(Group::table_name(), |bp| {
                bp.uuid_as_id(None);
                bp.string(Group::col_name_for_name()).set_is_unique(true);
                bp.timestamps();
                bp.soft_deletable();
            })
            .await?;

        // Country Group
        manager
            .create_table_schema(CountryGroup::table_name(), |bp| {
                bp.uuid_as_id(None);
                bp.uuid_table_fk::<Tournament>(true);
                bp.uuid_table_fk::<Group>(true);
                bp.uuid_table_fk::<Country>(true);
                bp.boolean(CountryGroup::col_name_for_is_out())
                    .default_is_false();
                bp.integer(CountryGroup::col_name_for_points())
                    .default_is_zero();
                bp.timestamps();
                bp.soft_deletable();

                bp.unique_index(&[
                    CountryGroup::col_name_for_tournament_id(),
                    CountryGroup::col_name_for_group_id(),
                    CountryGroup::col_name_for_country_id(),
                ]);
            })
            .await?;

        // Tip Strategy
        manager
            .create_table_schema(TipStrategy::table_name(), |bp| {
                bp.uuid_as_id(None);
                bp.string(TipStrategy::col_name_for_label())
                    .set_is_nullable(true);
                bp.text(TipStrategy::col_name_for_description())
                    .set_is_nullable(true);
                bp.uuid_table_fk::<Game>(true)
                    .set_is_nullable(true)
                    .set_is_unique(true);
                bp.uuid_table_fk::<Tournament>(true);
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

        // Tip
        manager
            .create_table_schema(Tip::table_name(), |bp| {
                bp.uuid_as_id(None);
                bp.uuid_table_fk::<Tournament>(true);
                bp.uuid_table_fk::<User>(true);
                bp.uuid_table_fk::<TipStrategy>(true);
                bp.json(Tip::col_name_for_strategies())
                    .default_is_empty_object();
                bp.integer(Tip::col_name_for_points()).default_is_zero();
                bp.timestamps();
                bp.soft_deletable();

                bp.unique_index(&[
                    Tip::col_name_for_tournament_id(),
                    Tip::col_name_for_tip_strategy_id(),
                ]);
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
        // Game
        manager
            .create_table_schema(Game::table_name(), |bp| {
                bp.uuid_as_id(None);
                bp.string(Game::col_name_for_label());
                bp.uuid_table_fk::<Tournament>(true);
                bp.integer(Game::col_name_for_count());
                bp.uuid_fk_as(
                    Country::table_name(),
                    Game::col_name_for_country_a_id(),
                    true,
                    Some(Country::id_column()),
                );
                bp.uuid_fk_as(
                    Country::table_name(),
                    Game::col_name_for_country_b_id(),
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
                    Game::col_name_for_winner_id(),
                    true,
                    Some(Country::id_column()),
                )
                .set_is_nullable(true);
                bp.timestamps();
                bp.soft_deletable();

                bp.unique_index(&[
                    Game::col_name_for_tournament_id(),
                    Game::col_name_for_country_a_id(),
                    Game::col_name_for_country_b_id(),
                ]);
            })
            .await?;

        // chat room
        manager
            .create_table_schema(ChatRoom::table_name(), |bp| {
                bp.uuid_as_id(Some(ChatRoom::id_column()));
                bp.string(ChatRoom::col_name_for_name())
                    .set_is_nullable(true);
                bp.string(ChatRoom::col_name_for_room_type());
                bp.timestamps();
                bp.soft_deletable();
            })
            .await?;
        // chat room user
        manager
            .create_table_schema(ChatRoomUser::table_name(), |bp| {
                bp.uuid_as_id(Some(ChatRoomUser::id_column()));
                bp.uuid_fk_as(
                    ChatRoom::table_name(),
                    ChatRoomUser::col_name_for_chat_room_id(),
                    true,
                    Some(ChatRoom::id_column()),
                );
                bp.uuid_fk_as(
                    User::table_name(),
                    ChatRoomUser::col_name_for_user_id(),
                    true,
                    Some(User::id_column()),
                );
                bp.boolean(ChatRoomUser::col_name_for_is_admin())
                    .default_is_false();
                bp.timestamps();
                bp.soft_deletable();

                bp.unique_index(&[
                    ChatRoomUser::col_name_for_chat_room_id(),
                    ChatRoomUser::col_name_for_user_id(),
                ]);
            })
            .await?;

        // chat message
        manager
            .create_table_schema(ChatMessage::table_name(), |bp| {
                bp.uuid_as_id(Some(ChatMessage::id_column()));
                bp.uuid_fk_as(
                    ChatRoom::table_name(),
                    ChatMessage::col_name_for_chat_room_id(),
                    true,
                    Some(ChatRoom::id_column()),
                );
                bp.uuid_fk_as(
                    User::table_name(),
                    ChatMessage::col_name_for_user_id(),
                    true,
                    Some(User::id_column()),
                );
                bp.text(ChatMessage::col_name_for_content());
                bp.timestamps();
                bp.soft_deletable();
            })
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &Manager, _: &Context) -> Result<(), anyhow::Error> {
        manager.drop_table(Tip::table_name()).await?;
        manager.drop_table(TipStrategy::table_name()).await?;
        manager.drop_table(Game::table_name()).await?;
        manager.drop_table(ChatMessage::table_name()).await?;
        manager.drop_table(ChatRoomUser::table_name()).await?;
        manager.drop_table(ChatRoom::table_name()).await?;
        manager.drop_table(User::table_name()).await?;
        manager.drop_table(CountryGroup::table_name()).await?;
        manager.drop_table(Group::table_name()).await?;
        manager.drop_table(Country::table_name()).await?;
        manager.drop_table(Tournament::table_name()).await?;
        Ok(())
    }
}
