#![allow(dead_code)]

pub mod auth;
pub mod user;

use crate::{server::errors::ApiError, structs::ProgramState};
use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use axum_extra::extract::CookieJar;
use std::sync::OnceLock;

static FRONTEND_SECRET: OnceLock<String> = OnceLock::new();

fn get_frontend_secret() -> &'static str {
    FRONTEND_SECRET
        .get_or_init(|| std::env::var("FRONTEND_SECRET").expect("FRONTEND_SECRET missing from env"))
        .as_str()
}

pub async fn auth_middleware(
    State(state): State<ProgramState>,
    jar: CookieJar,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, (StatusCode, &'static str)> {
    let cookie = jar.get("token").map(|c| c.value().to_owned());

    if let Some(cookie) = cookie {
        match state.auth.decode(&cookie) {
            Ok(c) => {
                req.extensions_mut().insert(c);
                Ok(next.run(req).await)
            }
            Err(_) => Err((StatusCode::UNAUTHORIZED, "Token is not valid")),
        }
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            "Cookie with name 'token' is not present",
        ))
    }
}

pub async fn server_middleware(
    State(_state): State<ProgramState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, ApiError> {
    let header = req
        .headers()
        .get("x-frontend-secret")
        .and_then(|h| h.to_str().ok());

    let Some(header) = header else {
        return Err(ApiError::Unauthorized(
            "Header named `x-frontend-secret` missing".into(),
        ));
    };

    println!("Frontend Secret: {}", get_frontend_secret());
    println!("Header Secret: {}", header);

    if header != get_frontend_secret() {
        return Err(ApiError::Unauthorized(
            "Header named `x-frontend-secret` invalid".into(),
        ));
    }

    Ok(next.run(req).await)
}
