export TEST_DATABASE_URL=postgres://postgres:postgres@localhost/pitza_test

test:
	cargo test --test test_api

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

all: fmt lint test