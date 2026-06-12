#!/usr/bin/env sh

set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
MIGRATIONS_DIR="$SCRIPT_DIR/server/migrations"
SEEDS_DIR="$SCRIPT_DIR/server/seeds"

if ! command -v docker >/dev/null 2>&1; then
  echo "Error: docker is not installed or not available in PATH." >&2
  echo "Install Docker Desktop or Docker Engine, then rerun this script." >&2
  exit 1
fi

if [ ! -f "$SCRIPT_DIR/.env" ] && [ -f "$SCRIPT_DIR/.env.example" ]; then
  cp "$SCRIPT_DIR/.env.example" "$SCRIPT_DIR/.env"
  echo "Created .env from .env.example."
fi

if [ ! -f "$SCRIPT_DIR/server/.env" ] && [ -f "$SCRIPT_DIR/server/.env.example" ]; then
  cp "$SCRIPT_DIR/server/.env.example" "$SCRIPT_DIR/server/.env"
  echo "Created server/.env from server/.env.example."
fi

if [ -f "$SCRIPT_DIR/.env" ]; then
  set -a
  . "$SCRIPT_DIR/.env"
  set +a
fi

POSTGRES_DB=${POSTGRES_DB:-nonewhite_site}
POSTGRES_USER=${POSTGRES_USER:-nonewhite_user}

cd "$SCRIPT_DIR"
docker compose up -d postgres

echo "Waiting for PostgreSQL to become ready..."
i=0
until docker compose exec -T postgres pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB" >/dev/null 2>&1; do
  i=$((i + 1))
  if [ "$i" -ge 30 ]; then
    echo "Error: PostgreSQL did not become ready in time." >&2
    exit 1
  fi
  sleep 2
done

for migration_file in "$MIGRATIONS_DIR"/*.sql; do
  [ -f "$migration_file" ] || continue
  docker compose exec -T postgres psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" < "$migration_file"
  echo "Database migration applied: $migration_file"
done

for seed_file in "$SEEDS_DIR"/dev_*.sql; do
  [ -f "$seed_file" ] || continue
  docker compose exec -T postgres psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" < "$seed_file"
  echo "Development seed applied: $seed_file"
done
