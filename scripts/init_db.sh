#!/usr/bin/env bash

set -x
set -eo pipefail

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=mgrd4f}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

#Launch postgres
if [[ -z "${SKIP_DOCKER}" ]]
then
    docker run \
       -e POSTGRES_USER=${DB_USER} \
       -e POSTGRES_PASSWORD=${DB_PASSWORD} \
       -e POSTGRES_DB=${DB_NAME} \
       -p "${DB_PORT}":5432 \
       -d postgres \
       postgres -N 1000
    # ^ increased max number of connections for testing purposes
fi

export PGPASSWORD="${DB_PASSWORD}"

# -c is flag for sub-command to follow. '\q' is to quit.
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping for 5 seconds"
    sleep 5
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
