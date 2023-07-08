-- Add up migration script here
create table users (
  id uuid primary key,
  name text not null,
  surname text not null,
  nickname text not null,
  created_at timestamptz not null,
  updated_at timestamptz not null
)