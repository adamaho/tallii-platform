#! /bin/sh

echo $DATABASE_URL

echo "Creating database if it doesn't exist and running migrations"

sqlx database setup

echo "Everything OK."