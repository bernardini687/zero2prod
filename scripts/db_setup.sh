#!/usr/bin/env bash

# set -x # print all executed commands
set -e # abort on non-zero exit codes
# set -u # abort on undeclared variables
set -o pipefail

# allow custom values or default to what's on the right of `:=`
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
DB_NAME=${POSTGRES_DB:=newsletter}
DB_PORT=${POSTGRES_PORT:=5432}

# allow to skip docker when container is already running
if [[ -z ${SKIP_DOCKER} ]]; then
  docker run -d --rm --name pg12 \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p ${DB_PORT}:5432 \
    postgres:12 \
    postgres -N 1000 # increase max number of connections for testing purposes
fi

# ping postgres until it's available (make sure `psql` is installed)
until PGPASSWORD=${DB_PASSWORD} psql -h localhost -U ${DB_USER} -p ${DB_PORT} -d postgres -c "\q"; do
  sleep 1
done

>&2 echo "Postgres up on port ${DB_PORT}. Starting migrations..."

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Done"
