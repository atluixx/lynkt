// @generated automatically by Diesel CLI.

diesel::table! {
    analytics (id) {
        id -> Uuid,
        link_id -> Uuid,
        metric_date -> Date,
        total_clicks -> Int4,
        unique_visitors -> Int4,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    config (user_id) {
        user_id -> Uuid,
        background -> Jsonb,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    link_groups (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    links (id) {
        id -> Uuid,
        url -> Text,
        #[max_length = 50]
        user_slug -> Varchar,
        group_id -> Nullable<Uuid>,
        #[max_length = 255]
        label -> Varchar,
        icon -> Nullable<Text>,
        order_index -> Nullable<Int4>,
        is_active -> Nullable<Bool>,
        max_clicks -> Nullable<Int4>,
        current_clicks -> Nullable<Int4>,
        active_until -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 50]
        slug -> Varchar,
        email -> Text,
        password_hash -> Text,
        bio -> Nullable<Text>,
        #[max_length = 100]
        country -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(analytics -> links (link_id));
diesel::joinable!(config -> users (user_id));
diesel::joinable!(link_groups -> users (user_id));
diesel::joinable!(links -> link_groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(analytics, config, link_groups, links, users,);
