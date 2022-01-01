#! /bin/sh

printenv DATABASE_URL

echo "Creating database if it doesn't exist"

sqlx database create

echo "Running any pending migrations"

sqlx migrate run

echo "Everything OK."