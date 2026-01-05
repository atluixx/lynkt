use crate::{models::SafeUser, schema::users::dsl::*, structs::ProgramState};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use diesel::prelude::*;
use serde_json::json;

pub async fn check(
    State(state): State<ProgramState>,
    Path(slug_path): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut c = state
        .pool
        .get()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error"))?;

    let slug_available = !users
        .filter(slug.eq(slug_path))
        .first::<SafeUser>(&mut c)
        .is_ok();

    Ok(Json(json!({
        "available": slug_available
    })))
}
