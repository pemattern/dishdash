include .env
export

.PHONY: dev dev-setup prod db-connect db-reset

dev:
	npm run dev

dev-setup:
	npm install && cargo install sqlx-cli --no-default-features --features native-tls,postgres && cd web && npm install

prod:
	npm run prod

db-connect:
	psql "$$DATABASE_URL"

db-reset:
	psql "$$DATABASE_URL" -v ON_ERROR_STOP=1 -c "DROP SCHEMA IF EXISTS public CASCADE; CREATE SCHEMA public;"
	sqlx migrate run --source server/migrations

sort-csv:
	python tools/sort_csv.py server/seed_data/ingredients.csv
	python tools/sort_csv.py server/seed_data/ingredient_aliases.csv
	python tools/sort_csv.py server/seed_data/ingredient_categories.csv
	python tools/sort_csv.py server/seed_data/ingredient_translations.csv
	python tools/sort_csv.py server/seed_data/locales.csv
