create table if not exists analytics (
    id uuid primary key default gen_random_uuid(),
    link_id uuid not null,
    metric_date date not null,
    total_clicks int not null default 0 check (total_clicks >= 0),
    unique_visitors int not null default 0 check (unique_visitors >= 0),
    created_at timestamptz default now(),
    constraint analytics_link_fk foreign key (link_id) references links(id) on delete cascade,
    constraint analytics_link_day_unique unique (link_id, metric_date)
);

create index analytics_link_id_idx on analytics(link_id);
create index analytics_metric_date_idx on analytics(metric_date);

SELECT diesel_manage_updated_at('analytics');