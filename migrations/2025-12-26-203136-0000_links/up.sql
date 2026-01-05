create table if not exists links (
    id uuid primary key default gen_random_uuid(),
    url text not null,
    user_slug varchar(50) not null,
    group_id uuid,
    label varchar(255) not null,
    icon text,
    order_index int default 0,
    is_active boolean default true,
    max_clicks int check (max_clicks >= 0),
    current_clicks int default 0 check (current_clicks >= 0),
    active_until timestamptz,
    created_at timestamptz default now(),
    updated_at timestamptz default now(),
    deleted_at timestamptz,
    constraint user_link_id foreign key (user_slug) references users(slug) on delete cascade,
    constraint group_link_id foreign key (group_id) references link_groups(id) on delete cascade
);

create index if not exists links_user_name_idx on links(user_slug);
create index if not exists links_group_id_idx on links(group_id);

SELECT diesel_manage_updated_at('links');