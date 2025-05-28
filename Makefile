export TEST_DATABASE_URL=postgres://postgres:postgres@localhost/pitza_test

test: reset-test-db
	cargo test

clean:
	cargo clean

lint:
	cargo clippy -- -D warnings

fmt:
	cargo fmt

migrate:
	diesel migration run --database-url=$(TEST_DATABASE_URL)

setup-db:
	psql -h localhost -U postgres -c "CREATE DATABASE pitza_test;"

delete-db:
	psql -h localhost -U postgres -c "DROP DATABASE pitza_test;"

check-db:
	psql -h localhost -U postgres -d pitza_test -c "\dt"

import-dump:
	psql -h localhost -U postgres -d pitza_test -f migrations/hexlet.sql


fix-sequences:
	psql -h localhost -U postgres -d pitza_test -c "SELECT setval('users_id_seq', (SELECT MAX(id) FROM users));"

reset-test-db: delete-db setup-db import-dump fix-sequences

all: fmt lint test