pub mod handlers;
pub mod queries;
pub mod types;

use axum::{routing::get, Router};

use crate::state::AppState;
use handlers::{
    get_cabinet_handler, invite_user_handler, list_cabinet_users_handler, update_cabinet_handler,
    update_user_handler,
};

/// Build the cabinet sub-router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/cabinet",
            get(get_cabinet_handler).put(update_cabinet_handler),
        )
        .route("/cabinet/users", get(list_cabinet_users_handler))
        .route("/cabinet/users/invite", axum::routing::post(invite_user_handler))
        .route(
            "/cabinet/users/:id",
            axum::routing::put(update_user_handler),
        )
}
