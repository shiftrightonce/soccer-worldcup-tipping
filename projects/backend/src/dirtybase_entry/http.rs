mod api;
mod general;
mod open_api;

use dirtybase_contract::prelude::{Request, RouterManager};
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub(crate) fn register_routes(manager: &mut RouterManager) {
    // Prefix all routes with the crate's name

    // v1 routes
    api::v1::register_routes(manager);

    // open api routes
    open_api::register_routes(manager);

    // general routes
    manager.general(None, |router| {
        router.get_x("/", |request: Request| async {
            let server = ServeDir::new("./spa").append_index_html_on_directories(true);
            server.oneshot(request).await
        });

        router.get_x("/{*path}", |request: Request| async {
            let server = ServeDir::new("./spa").append_index_html_on_directories(true);
            server.oneshot(request).await
        });
    });
}
