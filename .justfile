alias cm := create-migration

default:
  @just --list

export DATABASE_URL := "postgres://postgres:postgres@localhost:5432"

run *ARGS:
  podman-compose down
  podman-compose up -d
  sleep 3
  sqlx migrate run
  cargo run -- {{ ARGS }}

create-migration NAME:
  sqlx migrate add -r {{ NAME }}

migrate:
  podman-compose down
  podman-compose up -d
  sqlx migrate run

psql:
  podman exec -it postgres psql $DATABASE_URL