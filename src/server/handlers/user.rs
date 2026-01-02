use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use diesel::prelude::*;
use serde_json::json;

use validator::Validate;

use crate::{
    models::{Link, NewLink, NewUser, SafeUser},
    schema::{
        links::dsl::{links as links_table, user_slug as link_user_slug},
        users::dsl::*,
    },
    structs::{CreateOrUpdateLinkPayload, CreateOrUpdateUserPayload, ProgramState},
};

pub async fn get_users(
    State(state): State<ProgramState>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut c = state
        .pool
        .get()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong"))?;

    let all_users = users
        .select(SafeUser::as_select())
        .load::<SafeUser>(&mut c)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong"))?;

    Ok(Json(json!({
        "users": all_users
    })))
}

pub async fn get_user(
    State(state): State<ProgramState>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut c = state
        .pool
        .get()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong"))?;

    let user = users
        .filter(slug.eq(username))
        .load::<SafeUser>(&mut c)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong"))?;

    Ok(Json(json!({
        "user": user
    })))
}

pub async fn update_user(
    State(state): State<ProgramState>,
    Path(username): Path<String>,
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

    let user = diesel::update(users)
        .filter(slug.eq(username))
        .set(new_user)
        .get_result::<SafeUser>(&mut c)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error"))?;

    Ok(Json(json!({
        "user": user
    })))
}

pub async fn delete_user(
    State(state): State<ProgramState>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut c = state
        .pool
        .get()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error"))?;

    let user = diesel::delete(users)
        .filter(slug.eq(username))
        .get_result::<SafeUser>(&mut c)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error"))?;

    Ok(Json(json!({
        "user": user
    })))
}

pub async fn get_user_links(
    State(state): State<ProgramState>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut c = state
        .pool
        .get()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error"))?;

    let links = links_table
        .filter(link_user_slug.eq(username))
        .load::<Link>(&mut c)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error"))?;

    Ok(Json(json!({
        "links": links
    })))
}

pub async fn create_user_link(
    State(state): State<ProgramState>,
    Path(username): Path<String>,
    Json(payload): Json<CreateOrUpdateLinkPayload>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut c = state
        .pool
        .get()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error"))?;

    payload
        .validate()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid payload"))?;

    let is_active = payload
        .is_active
        .as_ref()
        .map(|v| *v == true)
        .ok_or((StatusCode::BAD_REQUEST, "Invalid value for is_active"))?;
    let max_clicks = payload.max_clicks.unwrap_or(0);
    let order_index = payload.order_index.unwrap_or(0);

    let new_link = NewLink {
        url: payload.url.clone(),
        user_slug: username.clone(),
        group_id: payload.group_id.clone(),
        label: payload.label.clone(),
        icon: payload.icon.clone(),
        active_until: payload.active_until.clone(),
        current_clicks: Some(0),
        is_active: Some(is_active),
        max_clicks: Some(max_clicks),
        order_index: Some(order_index),
    };

    let link = diesel::insert_into(links_table)
        .values(new_link)
        .get_result::<Link>(&mut c)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error"))?;

    Ok(Json(json!({ "link": link })))
}
