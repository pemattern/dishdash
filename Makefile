include .env
export

.PHONY: dev db-connect db-reset

dev:
	npm run dev

db-connect:
	psql "$$DATABASE_URL"

db-reset:
	psql "$$DATABASE_URL" -v ON_ERROR_STOP=1 -c "DROP SCHEMA IF EXISTS public CASCADE; CREATE SCHEMA public;"
	sqlx migrate run --source server/migrations
