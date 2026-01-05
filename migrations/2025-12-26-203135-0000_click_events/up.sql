create table if not exists click_events (
    id uuid primary key default gen_random_uuid(),
    link_id uuid not null,
    user_id uuid not null,
    clicked_at timestamptz not null default now(),
    region text,
    country text,
    device text,
    os text,
    browser text,
    referrer text,
    ip_hash text not null,
    created_at timestamptz default now(),
    constraint click_events_link_fk foreign key (link_id) references links(id) on delete cascade,
    constraint click_events_user_fk foreign key (user_id) references users(id) on delete cascade
);

create index if not exists click_events_link_id_idx on click_events(link_id);
create index if not exists click_events_user_id_idx on click_events(user_id);
create index if not exists click_events_clicked_at_idx on click_events(clicked_at);

SELECT diesel_manage_updated_at('click_events');
