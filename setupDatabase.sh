#!/usr/bin/env sh

set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
MIGRATIONS_DIR="$SCRIPT_DIR/server/migrations"
SEEDS_DIR="$SCRIPT_DIR/server/seeds"

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
POSTGRES_HOST=${POSTGRES_HOST:-localhost}
POSTGRES_USER=${POSTGRES_USER:-nonewhite_user}
POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-nonewhite_password}
POSTGRES_IMAGE=${POSTGRES_IMAGE:-postgres:17}
POSTGRES_PORT=${POSTGRES_PORT:-5432}
POSTGRES_CONTAINER_NAME=${POSTGRES_CONTAINER_NAME:-nonewhite_postgres}
POSTGRES_VOLUME_NAME=${POSTGRES_VOLUME_NAME:-nonewhite_postgres_data}
DB_SETUP_DRIVER=${DB_SETUP_DRIVER:-auto}

cd "$SCRIPT_DIR"

if [ "$DB_SETUP_DRIVER" != "auto" ] && [ "$DB_SETUP_DRIVER" != "local" ] && [ "$DB_SETUP_DRIVER" != "docker" ]; then
  echo "Error: DB_SETUP_DRIVER must be one of: auto, local, docker." >&2
  exit 1
fi

if [ "$DB_SETUP_DRIVER" != "docker" ] && command -v psql >/dev/null 2>&1 && command -v pg_isready >/dev/null 2>&1; then
  if [ "$DB_SETUP_DRIVER" = "local" ] || pg_isready -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -U "$POSTGRES_USER" -d "$POSTGRES_DB" >/dev/null 2>&1; then
    DB_RUNNER="local"
  fi
fi

if [ "${DB_RUNNER:-}" = "" ]; then
  if ! command -v docker >/dev/null 2>&1; then
    echo "Error: docker is not installed or not available in PATH." >&2
    echo "Install Docker, or set DB_SETUP_DRIVER=local to use a local PostgreSQL service." >&2
    exit 1
  fi

  if ! docker info >/dev/null 2>&1; then
    echo "Error: Docker daemon is not running or current user cannot access it." >&2
    echo "Check daemon status: sudo systemctl status docker" >&2
    echo "Start it on Arch Linux: sudo systemctl start docker" >&2
    echo "Enable it after reboot: sudo systemctl enable docker" >&2
    echo "If docker.service is active but this command still fails, refresh your docker group membership:" >&2
    echo "  newgrp docker" >&2
    echo "or fully log out and log back in after running:" >&2
    echo "  sudo usermod -aG docker \$USER" >&2
    echo "Current groups: $(id -nG 2>/dev/null || true)" >&2
    echo "To bypass Docker and use pacman-installed PostgreSQL, set: DB_SETUP_DRIVER=local" >&2
    exit 1
  fi

  if docker compose version >/dev/null 2>&1; then
  DB_RUNNER="docker-compose-plugin"
  elif command -v docker-compose >/dev/null 2>&1; then
    DB_RUNNER="docker-compose-standalone"
  else
    DB_RUNNER="docker-run"
  fi
fi

start_postgres() {
  case "$DB_RUNNER" in
    local)
      echo "Using local PostgreSQL at $POSTGRES_HOST:$POSTGRES_PORT."
      ;;
    docker-compose-plugin)
      docker compose up -d postgres
      ;;
    docker-compose-standalone)
      docker-compose up -d postgres
      ;;
    docker-run)
      echo "Docker Compose is not available; starting PostgreSQL with docker run fallback."
      if [ "$(docker inspect -f '{{.State.Running}}' "$POSTGRES_CONTAINER_NAME" 2>/dev/null || true)" = "true" ]; then
        return 0
      fi
      if docker inspect "$POSTGRES_CONTAINER_NAME" >/dev/null 2>&1; then
        docker start "$POSTGRES_CONTAINER_NAME" >/dev/null
        return 0
      fi
      docker run -d \
        --name "$POSTGRES_CONTAINER_NAME" \
        --restart unless-stopped \
        -e POSTGRES_DB="$POSTGRES_DB" \
        -e POSTGRES_USER="$POSTGRES_USER" \
        -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
        -p "$POSTGRES_PORT:5432" \
        -v "$POSTGRES_VOLUME_NAME:/var/lib/postgresql/data" \
        "$POSTGRES_IMAGE" >/dev/null
      ;;
  esac
}

postgres_exec() {
  case "$DB_RUNNER" in
    local)
      PGPASSWORD="$POSTGRES_PASSWORD" psql -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -U "$POSTGRES_USER" -d "$POSTGRES_DB" "$@"
      ;;
    docker-compose-plugin)
      docker compose exec -T postgres "$@"
      ;;
    docker-compose-standalone)
      docker-compose exec -T postgres "$@"
      ;;
    docker-run)
      docker exec -i "$POSTGRES_CONTAINER_NAME" "$@"
      ;;
  esac
}

if ! start_postgres; then
  echo "Error: failed to start PostgreSQL container." >&2
  echo "If Docker reports a connection reset while pulling postgres:17, your network cannot reach Docker Hub reliably." >&2
  echo "Set POSTGRES_IMAGE in .env to a reachable PostgreSQL image mirror, then rerun ./setupDatabase.sh." >&2
  echo "Example .env line: POSTGRES_IMAGE=postgres:17" >&2
  exit 1
fi

echo "Waiting for PostgreSQL to become ready..."
i=0
until {
  if [ "$DB_RUNNER" = "local" ]; then
    pg_isready -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -U "$POSTGRES_USER" -d "$POSTGRES_DB"
  else
    postgres_exec pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB"
  fi
} >/dev/null 2>&1; do
  i=$((i + 1))
  if [ "$i" -ge 30 ]; then
    echo "Error: PostgreSQL did not become ready in time." >&2
    if [ "$DB_RUNNER" = "local" ]; then
      echo "Local PostgreSQL is not reachable at $POSTGRES_HOST:$POSTGRES_PORT for database '$POSTGRES_DB'." >&2
      echo "On Arch Linux, initialize and start it with:" >&2
      echo "  sudo -iu postgres initdb -D /var/lib/postgres/data" >&2
      echo "  sudo systemctl start postgresql" >&2
      echo "Then create the project role/database:" >&2
      echo "  sudo -iu postgres createuser -P $POSTGRES_USER" >&2
      echo "  sudo -iu postgres createdb -O $POSTGRES_USER $POSTGRES_DB" >&2
      echo "If the database already exists, verify it with: pg_isready -h $POSTGRES_HOST -p $POSTGRES_PORT" >&2
    fi
    exit 1
  fi
  sleep 2
done

for migration_file in "$MIGRATIONS_DIR"/*.sql; do
  [ -f "$migration_file" ] || continue
  if [ "$DB_RUNNER" = "local" ]; then
    postgres_exec < "$migration_file"
  else
    postgres_exec psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" < "$migration_file"
  fi
  echo "Database migration applied: $migration_file"
done

for seed_file in "$SEEDS_DIR"/dev_*.sql; do
  [ -f "$seed_file" ] || continue
  if [ "$DB_RUNNER" = "local" ]; then
    postgres_exec < "$seed_file"
  else
    postgres_exec psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" < "$seed_file"
  fi
  echo "Development seed applied: $seed_file"
done
