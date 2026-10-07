FROM rust:1.82 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY services/api/Cargo.toml services/api/Cargo.toml
RUN mkdir -p services/api/src && echo "fn main() {}" > services/api/src/main.rs
RUN cargo build -p field-service-api --release || true
COPY . .
RUN cargo build -p field-service-api --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/field-service-api /usr/local/bin/field-service-api
COPY services/api/migrations /app/migrations
ENV API_BIND_ADDR=0.0.0.0:3000
EXPOSE 3000
CMD ["field-service-api"]
