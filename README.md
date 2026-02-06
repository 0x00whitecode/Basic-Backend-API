# basi_api — Basic Backend API

Brief, self-contained Rust + Actix Web API used for HNG task deployment examples.

**Contents**
- **Overview** — What this project is and what it exposes.
- **Prerequisites** — Tools required to build and deploy.
- **Local development** — How to run locally with Cargo.
- **Docker** — How the image is built and run.
- **Deploy to Fly.io** — Steps to deploy using `flyctl`.
- **API Endpoints** — Routes and example responses.

## Overview

`basi_api` is a small Actix Web server exposing a few JSON endpoints under the `/api` scope. It is intended to be built with Cargo and packaged in Docker for deployment to platforms like Fly.io.

## Prerequisites

- Rust toolchain (stable). Install via https://rustup.rs
- Docker (for building images locally or on CI)
- flyctl (for deployment to Fly.io) — https://fly.io/docs/hands-on/install-flyctl/
- Git to clone and push changes

## Local development

1. Clone the repo:

```bash
git clone git@github.com:0x00whitecode/Basic-Backend-API.git
cd Basic-Backend-API
```

2. Run the server locally:

```bash
cargo run
```

3. The server binds to the `PORT` environment variable (defaults to `8080`). Example:

```bash
PORT=8080 cargo run
```

4. Test endpoints:

```bash
curl http://localhost:8080/api/         # welcome message
curl http://localhost:8080/api/health   # health check
curl http://localhost:8080/api/yourname # personalized welcome
```

## Docker

This repo includes a multi-stage `Dockerfile` that builds the Rust binary in a builder image and produces a small runtime image.

Build locally:

```bash
docker build -t basi_api:latest .
```

Run the container (exposes port 8080):

```bash
docker run -p 8080:8080 -e PORT=8080 basi_api:latest
```

Notes about the Dockerfile:
- Uses a builder stage to compile the release binary.
- Copies the compiled binary into a minimal Debian-based runtime.
- Sets `PORT=8080` and exposes `8080` so platform orchestrators (like Fly) can map ports.

If your build fails with a Cargo edition error, ensure `Cargo.toml` uses a supported edition (e.g., `edition = "2021"`) and that the builder image provides a sufficiently recent Rust toolchain.

## Deploying to Fly.io

1. Install `flyctl` and authenticate:

```bash
flyctl auth login
```

2. Create or reuse an app (if you don't have `fly.toml` already):

```bash
flyctl launch --name basic-backend-api --region iad --no-deploy
```

3. Build and deploy:

```bash
flyctl deploy
```

Or to build only and push an image for later deploy or CI:

```bash
flyctl deploy --build-only
```

Fly-specific notes:
- The app reads `PORT` from the environment and binds to `0.0.0.0`, which Fly requires for public traffic.
- Check or update `fly.toml` to set `internal_port = 8080` under `[http_service]`.

## API Endpoints

- `GET /api/` — Welcome message JSON
- `GET /api/health` — Health check JSON
- `GET /api/{name}` — Returns an identity message containing `{name}`

Example response for `/api/`:

```json
[{"success":"true","message":"welcome to HNG Task 0x0001"}]
```

## Environment variables

- `PORT` — Port to bind to (defaults to `8080`)
- `DATABASE_URL` — (optional) If you add Diesel/Postgres usage, set this for DB connectivity.

## Contributing

Feel free to open issues or PRs. Keep changes small and focused; run `cargo fmt` and ensure the project builds with `cargo build --release`.

## License

This project does not include a license file. Add one if you plan to make the repo public with a specific license.
