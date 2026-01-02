use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = users)]
pub struct SafeUser {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password_hash: String,

    pub bio: Option<String>,
    pub country: String,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub slug: String,
    pub email: String,
    pub password_hash: String,
    pub bio: String,
    pub country: String,
}

#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, Associations)]
#[diesel(table_name = links)]
#[diesel(belongs_to(SafeUser, foreign_key = user_slug))]
pub struct Link {
    pub id: Uuid,
    pub url: String,
    pub user_slug: String,
    pub group_id: Option<Uuid>,
    pub label: String,
    pub icon: Option<String>,
    pub order_index: Option<i32>,
    pub is_active: Option<bool>,
    pub max_clicks: Option<i32>,
    pub current_clicks: Option<i32>,
    pub active_until: Option<DateTime<Utc>>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = links)]
pub struct NewLink {
    pub url: String,
    pub user_slug: String,
    pub group_id: Option<Uuid>,
    pub label: String,
    pub icon: Option<String>,
    pub order_index: Option<i32>,
    pub is_active: Option<bool>,
    pub max_clicks: Option<i32>,
    pub current_clicks: Option<i32>,
    pub active_until: Option<DateTime<Utc>>,
}
