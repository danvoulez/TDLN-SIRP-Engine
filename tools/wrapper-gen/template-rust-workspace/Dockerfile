FROM rust:1.75 as build
WORKDIR /app
COPY . .
RUN cargo build --release -p service
FROM debian:stable-slim
WORKDIR /app
COPY --from=build /app/target/release/service /usr/local/bin/service
EXPOSE 8080
ENV SERVICE_PORT=8080
CMD ["/usr/local/bin/service"]
