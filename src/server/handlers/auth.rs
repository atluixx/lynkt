use crate::{
    authentication::Claims,
    models::{NewUser, SafeUser},
    schema::users::dsl::*,
    structs::{CreateOrUpdateUserPayload, LoginPayload, ProgramState},
};

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use diesel::prelude::*;

pub async fn register(
    State(state): State<ProgramState>,
    Json(user): Json<CreateOrUpdateUserPayload>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    user.validate()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid payload"))?;

    let mut c = state
        .pool
        .get()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error"))?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(user.password.as_bytes(), &salt)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Hashing failed"))?
        .to_string();

    let new_user = NewUser {
        name: user.name.clone(),
        slug: user.slug.clone(),
        email: user.email.clone(),
        password_hash: hash,
        bio: user
            .bio
            .clone()
            .unwrap_or_else(|| "Hey! I am using Lynkt!".into()),
        country: user.country.clone(),
    };

    let user = diesel::insert_into(users)
        .values(new_user)
        .get_result::<SafeUser>(&mut c)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error"))?;

    Ok(Json(json!({
        "user": user
    })))
}

pub async fn login(
    State(state): State<ProgramState>,
    jar: CookieJar,
    Json(user): Json<LoginPayload>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    user.validate()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid payload"))?;

    let mut c = state
        .pool
        .get()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error"))?;

    let user_result = users
        .filter(email.eq(&user.email))
        .first::<SafeUser>(&mut c)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid credentials"))?;

    let parsed_hash = argon2::PasswordHash::new(&user_result.password_hash)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Hash parsing failed"))?;

    let argon2 = Argon2::default();
    argon2
        .verify_password(user.password.as_bytes(), &parsed_hash)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid credentials"))?;

    let token = state
        .auth
        .encode(&user_result.id.to_string())
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create authentication token",
            )
        })?;

    let cookie_jar = jar.add(
        Cookie::build(("token", token))
            .path("/")
            .http_only(true)
            .build(),
    );

    Ok((
        cookie_jar,
        Json(json!({
            "user": user_result
        })),
    ))
}

pub async fn me(
    State(state): State<ProgramState>,
    req: Request,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let c = req.extensions().get::<Claims>();
    if let Some(c) = c {
        let user_id = c.sub.clone().parse::<Uuid>().map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "Unable to get your data from token",
            )
        })?;

        let mut c = state
            .pool
            .get()
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error"))?;

        let user = users
            .filter(id.eq(user_id))
            .first::<SafeUser>(&mut c)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error"))?;

        return Ok(Json(json!({
            "user": user
        })));
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Unable to get your data from token",
        ));
    }
}
