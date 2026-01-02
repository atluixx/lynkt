use crate::authentication::JwtAuth;
use chrono::{DateTime, Utc};
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};
pub type DatabasePool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub struct ProgramState {
    pub pool: DatabasePool,
    pub auth: JwtAuth,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateOrUpdateUserPayload {
    #[validate(length(min = 4, max = 100))]
    pub name: String,

    #[validate(length(min = 20, max = 1000))]
    pub bio: Option<String>,

    #[validate(length(min = 4, max = 100))]
    pub slug: String,

    #[validate(email)]
    pub email: String,

    #[validate(custom(function = "strong_password"))]
    pub password: String,

    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email)]
    pub email: String,

    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateOrUpdateLinkPayload {
    #[validate(url)]
    pub url: String,

    pub label: String,

    pub icon: Option<String>,

    pub group_id: Option<Uuid>,

    pub order_index: Option<i32>,

    pub is_active: Option<bool>,

    pub max_clicks: Option<i32>,

    pub active_until: Option<DateTime<Utc>>,
}

fn strong_password(password: &str) -> Result<(), ValidationError> {
    let len_ok = password.len() >= 8;
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    if len_ok && has_upper && has_lower && has_digit && has_symbol {
        Ok(())
    } else {
        Err(ValidationError::new("weak_password"))
    }
}
