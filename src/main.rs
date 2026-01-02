use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::{database::connect, structs::ProgramState};

mod authentication;
mod database;
mod models;
mod schema;
mod server;
mod structs;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(vec![
            "http://127.0.0.1:5500".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    let database_url = env::var("DATABASE_URL").expect("failed to read .env");
    let secret = env::var("JWT_SECRET").expect("failed to read .env");
    let expiration_time = env::var("TOKEN_EXPIRATION_TIME")
        .expect("failed to read .env")
        .parse::<i64>()
        .unwrap();

    let pool = connect(&database_url);
    let auth_config = authentication::JwtConfig::new(secret, expiration_time);
    let auth = authentication::JwtAuth::new(auth_config);
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    let state = ProgramState { auth, pool };

    let make_service = Router::new()
        .nest("/users/", server::routers::get_users_router())
        .nest("/auth/", server::routers::get_auth_router(state.clone()))
        .with_state(state)
        .layer(cors)
        .into_make_service();

    println!(":: server is running : 0.0.0.0:3000 ::");
    axum::serve(listener, make_service).await
}
