# Build stage
FROM rust:latest as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    build-essential \
    lld \
    clang \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/basi_api /app/

EXPOSE 8080

ENV PORT=8080

CMD ["./basi_api"]