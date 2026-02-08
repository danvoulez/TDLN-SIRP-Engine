    .PHONY: build run test docker
    build:
	cargo build --workspace
    run:
	cargo run -p service
    test:
	cargo test --workspace
    docker:
	docker build -t wrapper-svc .
