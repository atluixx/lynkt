use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};

use crate::{
    server::handlers::{
        auth::{login, me, register},
        middleware_function,
        user::{create_user_link, delete_user, get_user, get_user_links, get_users, update_user},
    },
    structs::ProgramState,
};

// -> /users
pub fn get_users_router() -> Router<ProgramState> {
    Router::new()
        .route("/", get(get_users))
        .route("/{slug}/", get(get_user))
        .route("/{slug}/", patch(update_user))
        .route("/{slug}/", delete(delete_user))
        .route("/{slug}/links/", get(get_user_links))
        .route("/{slug}/links/", post(create_user_link))
}

// -> /auth
pub fn get_auth_router(state: ProgramState) -> Router<ProgramState> {
    Router::new()
        .route("/register/", post(register))
        .route("/login/", post(login))
        .route(
            "/me/",
            post(me).layer(middleware::from_fn_with_state(
                state.clone(),
                middleware_function,
            )),
        )
}
