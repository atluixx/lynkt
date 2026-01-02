pub mod auth;
pub mod user;

use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use axum_extra::extract::CookieJar;

use crate::structs::ProgramState;

pub async fn middleware_function(
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
            Err(_e) => Err((StatusCode::UNAUTHORIZED, "Token is not valid")),
        }
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Cookie with name 'token' is not present",
        ));
    }
}
