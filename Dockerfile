# Build stage
FROM rust:1.94.1-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY templates ./templates

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/gruvbox_ports ./gruvbox_ports
COPY templates ./templates
COPY public ./public
COPY migrations ./migrations

CMD ["./gruvbox_ports"]
