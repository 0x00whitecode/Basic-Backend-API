# -------- Build stage --------
FROM rust:1.75 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo build --release

# -------- Runtime stage --------
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/Cat_API .

EXPOSE 8080
CMD ["./Cat_API"]
