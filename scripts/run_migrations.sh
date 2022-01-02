#! /bin/sh

echo $DATABASE_URL

echo "running migrations"

sqlx migrate run

echo "Everything OK."