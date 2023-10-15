alias cm := create-migration

default:
  @just --list

create-migration NAME:
  sea-orm-cli \
    migrate \
    generate \
    --universal-time \
    -d crates/migrations \
    {{ NAME }}

migrate:
  #!/usr/bin/env sh
  set -eu
  
  export DATABASE_URL="postgres://${PGUSER}:${PGPASSWORD}@${PGHOST}:${PGPORT}/${PGDATABASE}"

  sea-orm-cli \
    migrate \
    fresh \
    -d crates/migrations
  
  sea-orm-cli \
    generate \
    entity \
    -o crates/entities/src \
    -l
