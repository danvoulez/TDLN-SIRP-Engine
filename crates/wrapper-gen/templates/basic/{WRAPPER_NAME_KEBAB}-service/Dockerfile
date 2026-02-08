FROM rust:1.75 as build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
COPY --from=build /app/target/release/{{WRAPPER_NAME_KEBAB}}-service /usr/local/bin/svc
COPY static /app/static
ENV PORT=8080
EXPOSE 8080
CMD ["/usr/local/bin/svc"]
