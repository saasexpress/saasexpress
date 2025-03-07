.PHONY: build run run-dev test lint clean docker docker-run

build:
	cargo build --release

run:
	cargo run --release

run-dev:
	cargo watch -x run

test:
	cargo test

lint:
	cargo clippy -- -D warnings
	cargo fmt --check

clean:
	cargo clean

docker:
	docker build -t admin-api-rs .

docker-run:
	docker run -p 8081:8081 admin-api-rs