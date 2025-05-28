export TEST_DATABASE_URL=postgres://postgres:postgres@localhost/pitza_test
export DATABASE_URL=postgres://postgres:postgres@localhost/pitza
run:
	cargo run

build:
	cargo build --release

test: 
	cargo test

test-coverage:
	cargo tarpaulin --out Html --output-dir target/coverage

clean:
	cargo clean

lint:
	cargo clippy -- -D warnings

lint-fix:
	cargo clippy --fix --allow-dirty --allow-staged

fmt:
	cargo fmt

migrate-test:
	diesel migration run --database-url=$(TEST_DATABASE_URL)

migrate-dev:
	diesel migration run --database-url=$(DATABASE_URL)

all: fmt lint test

docker-compose:
	docker-compose -f docker-compose.dev.yml up -d

docker-compose-down:
	docker-compose down