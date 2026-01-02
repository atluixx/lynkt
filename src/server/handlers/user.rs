#![allow(dead_code)]

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{Link, NewLink, NewUser, SafeUser},
    schema::{
        links::dsl::{
            active_until, group_id, icon, id as link_id, is_active, label, links as links_table,
            max_clicks, order_index, url as link_url, user_slug as link_user_slug,
        },
        users::dsl::*,
    },
    server::errors::ApiError,
    structs::{CreateLinkPayload, CreateOrUpdateUserPayload, ProgramState, UpdateLinkPayload},
};

pub async fn get_users(State(state): State<ProgramState>) -> Result<impl IntoResponse, ApiError> {
    let mut c = state.pool.get().map_err(|_| ApiError::PoolError)?;

    let all_users = users
        .select(SafeUser::as_select())
        .load::<SafeUser>(&mut c)?;

    Ok(Json(json!({ "users": all_users })))
}

pub async fn get_user(
    State(state): State<ProgramState>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let mut c = state.pool.get().map_err(|_| ApiError::PoolError)?;

    let user = users
        .filter(slug.eq(username))
        .get_result::<SafeUser>(&mut c)?;

    Ok(Json(json!({ "user": user })))
}

pub async fn update_user(
    State(state): State<ProgramState>,
    Path(username): Path<String>,
    Json(user): Json<CreateOrUpdateUserPayload>,
) -> Result<impl IntoResponse, ApiError> {
    user.validate()?;

    let mut c = state.pool.get().map_err(|_| ApiError::PoolError)?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(user.password.as_bytes(), &salt)
        .map_err(|_| ApiError::HashError)?
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

    let user = diesel::update(users.filter(slug.eq(username)))
        .set(new_user)
        .get_result::<SafeUser>(&mut c)?;

    Ok(Json(json!({ "user": user })))
}

pub async fn delete_user(
    State(state): State<ProgramState>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let mut c = state.pool.get().map_err(|_| ApiError::PoolError)?;

    let user = diesel::delete(users.filter(slug.eq(username))).get_result::<SafeUser>(&mut c)?;

    Ok(Json(json!({ "user": user })))
}

pub async fn get_user_link(
    State(state): State<ProgramState>,
    Path((_user_id, path_id)): Path<(String, Uuid)>,
) -> Result<impl IntoResponse, ApiError> {
    let mut c = state.pool.get().map_err(|_| ApiError::PoolError)?;

    let link = links_table
        .filter(link_id.eq(path_id))
        .get_result::<Link>(&mut c)?;

    Ok(Json(json!({ "link": link })))
}

pub async fn get_user_links(
    State(state): State<ProgramState>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let mut c = state.pool.get().map_err(|_| ApiError::PoolError)?;

    let links = links_table
        .filter(link_user_slug.eq(username))
        .load::<Link>(&mut c)?;

    Ok(Json(json!({ "links": links })))
}

pub async fn create_user_link(
    State(state): State<ProgramState>,
    Path(username): Path<String>,
    Json(payload): Json<CreateLinkPayload>,
) -> Result<impl IntoResponse, ApiError> {
    let mut c = state.pool.get().map_err(|_| ApiError::PoolError)?;

    payload.validate()?;

    let is_active_value = payload
        .is_active
        .ok_or(ApiError::BadRequest("is_active is required".into()))?;

    let new_link = NewLink {
        url: payload.url.clone(),
        user_slug: username.clone(),
        group_id: payload.group_id.clone(),
        label: payload.label.clone(),
        icon: payload.icon.clone(),
        active_until: payload.active_until.clone(),
        current_clicks: Some(0),
        is_active: Some(is_active_value),
        max_clicks: Some(payload.max_clicks.unwrap_or(0)),
        order_index: Some(payload.order_index.unwrap_or(0)),
    };

    let link = diesel::insert_into(links_table)
        .values(new_link)
        .get_result::<Link>(&mut c)?;

    Ok(Json(json!({ "link": link })))
}

pub async fn update_user_link(
    State(state): State<ProgramState>,
    Path((_user_id, path_id)): Path<(String, Uuid)>,
    Json(payload): Json<UpdateLinkPayload>,
) -> Result<impl IntoResponse, ApiError> {
    let mut c = state.pool.get().map_err(|_| ApiError::PoolError)?;

    payload.validate()?;

    let updated_link = (
        payload.url.as_ref().map(|v| link_url.eq(v)),
        payload.group_id.as_ref().map(|v| group_id.eq(v)),
        payload.label.as_ref().map(|v| label.eq(v)),
        payload.icon.as_ref().map(|v| icon.eq(v)),
        payload.active_until.as_ref().map(|v| active_until.eq(v)),
        payload.is_active.map(|v| is_active.eq(v)),
        payload.max_clicks.map(|v| max_clicks.eq(v)),
        payload.order_index.map(|v| order_index.eq(v)),
    );

    let link = diesel::update(links_table.filter(link_id.eq(path_id)))
        .set(updated_link)
        .get_result::<Link>(&mut c)?;

    Ok(Json(json!({ "link": link })))
}

pub async fn delete_user_link(
    State(state): State<ProgramState>,
    Path((_user_id, path_id)): Path<(String, Uuid)>,
) -> Result<impl IntoResponse, ApiError> {
    let mut c = state.pool.get().map_err(|_| ApiError::PoolError)?;

    let link =
        diesel::delete(links_table.filter(link_id.eq(path_id))).get_result::<Link>(&mut c)?;

    Ok(Json(json!({ "link": link })))
}
