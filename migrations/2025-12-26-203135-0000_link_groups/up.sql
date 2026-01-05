create table if not exists link_groups (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null,
    title varchar(255) not null default 'New Group',
    created_at timestamptz default now(),
    updated_at timestamptz default now(),
    constraint link_groups_user_fk foreign key (user_id) references users(id) on delete cascade,
    constraint link_groups_user_title_unique unique (user_id, title)
);

SELECT diesel_manage_updated_at('link_groups');