.PHONY: dev build run docker
dev:
	cargo run -p {{WRAPPER_NAME_KEBAB}}-service
build:
	cargo build -p {{WRAPPER_NAME_KEBAB}}-service --release
run:
	PORT=8080 cargo run -p {{WRAPPER_NAME_KEBAB}}-service
docker:
	docker build -t {{WRAPPER_NAME_KEBAB}}:dev .
	docker run -it --rm -p 8080:8080 {{WRAPPER_NAME_KEBAB}}:dev
