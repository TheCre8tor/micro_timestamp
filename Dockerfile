FROM rust:1.56.0 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/micro_timestamp micro_timestamp
COPY configuration configuration
ENV APP_ENVIRONMENT=production

ENTRYPOINT ["./micro_timestamp"]
