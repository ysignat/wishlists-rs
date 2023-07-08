default:
  @just --list

run *ARGS:
  #!/usr/bin/env bash
  set -euo pipefail
  IFS=$'\n\t'

  export DATABASE_URL="postgres://postgres:postgres@localhost:5432"
  podman-compose up -d
  cargo run -- {{ ARGS }}
