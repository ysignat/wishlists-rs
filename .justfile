alias cm := create-migration

default:
  @just --list

create-migration NAME:
  sqlx migrate add -r {{ NAME }}

migrate:
  #!/usr/bin/env sh
  set -eu
  
  sqlx database reset -y
  sqlx migrate run