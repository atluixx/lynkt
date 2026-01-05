create extension if not exists "pgcrypto";

create table if not exists users (
  id uuid primary key default gen_random_uuid(),
  name varchar(100) not null,
  slug varchar(50) unique not null,
  email text unique not null,
  password_hash text not null,
  bio text default 'Hey! I am using Lynkt!',
  country varchar(100) not null,
  created_at timestamptz default now(),
  updated_at timestamptz default now(),
  deleted_at timestamptz
);

SELECT diesel_manage_updated_at('users');