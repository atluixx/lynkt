create table if not exists config (
    user_id uuid primary key,
    background jsonb not null default '{}' :: jsonb,
    created_at timestamptz default now(),
    updated_at timestamptz default now(),
    constraint config_user_fk foreign key (user_id) references users(id) on delete cascade
);

SELECT diesel_manage_updated_at('config');
