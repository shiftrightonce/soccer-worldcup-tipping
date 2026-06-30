mod country_controller;
mod country_group_controller;
mod game_controller;
mod group_controller;
mod server_send_controller;
mod tip_controller;
mod tip_strategy_controller;
mod tournament_controller;
mod user_controller;

use dirtybase_contract::prelude::RouterManager;

pub fn register_routes(manager: &mut RouterManager) {
    manager.api(Some("/v1"), |router| {
        // Tournament
        router.nest("/tournaments", |router| {
            router
                .get_x("/", tournament_controller::list_handler)
                .get_x("/{tournament_id}", tournament_controller::get_by_id_handler)
                .post_x_with_middleware(
                    "/",
                    tournament_controller::create_handler,
                    ["can:tournaments:create"],
                )
                .put_x_with_middleware(
                    "/{tournament_id}",
                    tournament_controller::update_handler,
                    ["can:tournaments:update"],
                );
        });

        // countries
        router.nest("/countries", |router| {
            router
                .get_x("/", country_controller::list_handler)
                .get_x("/all", country_controller::all_handler)
                .get_x("/{id}", country_controller::get_handler);
        });

        // groups
        router.nest("/groups", |router| {
            router.get_x("/", group_controller::all_handler);
        });

        // country groups
        router.nest("/country-groups/{tournament_id}", |router| {
            router
                .get_x("/", country_group_controller::list_handler)
                .get_x("/all", country_group_controller::all_handler)
                .post_x_with_middleware(
                    "/",
                    country_group_controller::create_handler,
                    ["can:country-group:create"],
                )
                .get_x(
                    "/by-group/{group_id}",
                    country_group_controller::by_group_handler,
                )
                .get_x("/{id}", country_group_controller::get_handler)
                .put_x_with_middleware(
                    "/{id}",
                    country_group_controller::update_handler,
                    ["can:country-group:update"],
                );
        });

        // Tip Strategies
        router.nest("/tip-strategies/{tournament_id}", |router| {
            router
                .get_x("/", tip_strategy_controller::list_handler)
                .get_x("/all", tip_strategy_controller::all_handler)
                .get_x("/open", tip_strategy_controller::all_open_handler)
                .get_x("/closed", tip_strategy_controller::all_closed_handler)
                .post_x_with_middleware(
                    "/",
                    tip_strategy_controller::create_handler,
                    ["can:strategy:create"],
                )
                .post_x_with_middleware(
                    "/results/{id}",
                    tip_strategy_controller::create_result_handler,
                    ["can:strategy-result:create"],
                )
                .get_x("/{id}", tip_strategy_controller::get_handler)
                .put_x_with_middleware(
                    "/{id}",
                    tip_strategy_controller::update_handler,
                    ["can:strategy:update"],
                )
                .delete_x_with_middleware(
                    "/{id}",
                    tip_strategy_controller::delete_handler,
                    ["can:strategy:delete"],
                );
        });

        // games
        router.nest("/games/{tournament_id}", |router| {
            router
                .get_x("/", game_controller::list_handler)
                .get_x("/all", game_controller::all_handler)
                .get_x("/no-strategy", game_controller::list_for_strategy_handler)
                .get_x("/by-status/{status}", game_controller::by_status_handler)
                .get_x("/{id}", game_controller::get_handler)
                .post_x_with_middleware("/", game_controller::create_handler, ["can:games:create"])
                .put_x_with_middleware(
                    "/{id}",
                    game_controller::update_handler,
                    ["can:games:update"],
                );
        });

        router.nest("/tips/{tournament_id}", |router| {
            router
                .get_x_with_middleware("/", tip_controller::list_handler, ["can:tips:view-all"])
                .get_x("/leader-board", tip_controller::leader_board_handler)
                .get_x("/my-points", tip_controller::my_points_handler)
                .get_x("/my-tips", tip_controller::my_tips_handler)
                .post_x("/my-tips", tip_controller::create_handler)
                .put_x("/my-tips/{id}", tip_controller::update_handler);
        });

        // Users
        router.nest("/users", |router| {
            router
                .get_x_with_middleware("/", user_controller::list_handler, ["can:users:view-all"])
                .get_x("/me", user_controller::get_me_handler)
                .put_x("/update/me", user_controller::update_credential_handler)
                .get_x("/player/{id}", user_controller::view_player_handler)
                .get_x_with_middleware(
                    "/{user_id}",
                    user_controller::get_user_handler,
                    &["can:users:view"],
                );
        });

        // sse
        router.nest("/live-sse", |router| {
            router.get_x("/", server_send_controller::sse_handler);
        });
    });
}
