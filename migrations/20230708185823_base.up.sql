-- Add up migration script here
create table users (
  id uuid primary key,
  first_name text constraint first_name_length check (coalesce(length(first_name) > 0, true)),
  second_name text constraint second_name_length check (coalesce(length(second_name) > 0, true)),
  nickname text unique not null constraint nickname_length check (length(nickname) > 0),
  created_at timestamptz not null,
  updated_at timestamptz not null
);

create table wishlists (
  id uuid primary key,
  name text not null constraint wishlist_name_length check (length(name) > 0),
  user_id uuid not null references users (id),
  created_at timestamptz not null,
  updated_at timestamptz not null
);