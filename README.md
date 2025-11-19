# backend-actix-web

Backend API built with `actix-web` and `SeaORM`, focused on credential handling, JWT authentication, and a clean modular layout.

## Table of Contents

1. [Features](#features)
2. [Tech stack](#tech-stack)
3. [Requirements](#requirements)
4. [Configuration](#configuration)
5. [Development setup](#development-setup)
6. [Docker](#docker)
7. [API endpoints](#api-endpoints)
8. [Project structure](#project-structure)
9. [Testing](#testing)
10. [License](#license)

## Features

- Handlers for registration, login, logout, a home index (`/`), and a secured profile endpoint (`/me`).
- JWT encode/decode helpers that can plug into middleware (and a token blacklist for logout).
- Credential storage currently keeps passwords as provided (no hashing helpers).
- Clean layering: handlers call services -> services call SeaORM -> utils provide token helpers.

## Tech stack

- **Web framework**: `actix-web` 4.x
- **ORM**: `SeaORM` with PostgreSQL
- **Hashing**: disabled (passwords stored as provided)
- **Tokens**: `jsonwebtoken` with the `rust_crypto` feature
- **Env**: `dotenvy` for reading `.env`

## Requirements

- Rust (1.71+) toolchain
- PostgreSQL database reachable via `DATABASE_URL`
- `cargo` installed via the Rust toolchain

## Configuration

Copy `.env.example` to `.env` and define:

- `DATABASE_URL` -> database connection string
- `JWT_SECRET` -> secret used to sign JWTs
- `BIND_ADDRESS` *(optional)* -> defaults to `127.0.0.1:8080`

## Development setup

1. Run `cargo install sqlx-cli --no-default-features --features postgres` only if you need SQLx migrations.
2. Start the backend with `cargo run`.
3. Use curl/Postman to exercise the HTTP endpoints listed below.

## Docker

1. Launch the stack with `docker compose up --build`.
2. Compose starts Postgres plus the API; Postgres listens on `localhost:5432` and the server on `localhost:8080`.
3. `.env` supplies `JWT_SECRET`/`BIND_ADDRESS` while Compose overrides `DATABASE_URL` for the `db` service.

Shut down with `docker compose down`; the named volume `db-data` preserves Postgres data.

## API endpoints

- `GET /` -> home/index welcome message.
- `POST /auth/register` -> create a new user (returns token + filtered user data).
- `POST /auth/login` -> authenticate and receive a JWT.
- `POST /auth/logout` -> revoke the current bearer token (requires `Authorization: Bearer <token>`).
- `GET /me` -> read profile info (requires valid, non-revoked bearer token).

## Project structure

```
my_actix_app/
├── .cargo/
│   └── config.toml                # Cargo workspace settings (optional)
├── .idea/
│   └── workspace.xml              # JetBrains project metadata (ignore in CI)
├── migration/
│   └── *.sql                      # database schema changes
├── src/
│   ├── db/
│   │   └── connection.rs         # Postgres connection helper
│   ├── handlers/
│   │   ├── auth_handler.rs       # login/register/logout controllers
│   │   └── user_handler.rs       # `/` home and `/me` profile
│   ├── middleware/
│   │   └── auth_middleware.rs    # JWT helper storing claims into request extensions
│   ├── models/
│   │   └── user.rs               # SeaORM user entity
│   ├── routes/
│   │   └── user_routes.rs        # central router wiring handlers
│   ├── services/
│   │   └── user_service.rs       # DB logic for finding/creating users
│   ├── utils/
│   │   ├── auth_utils.rs         # Argon2 hash/verify helpers
│   │   └── jwt.rs                # encode/decode helpers plus claims
│   ├── config.rs                 # AppConfig loader
│   ├── main.rs                   # wires AppConfig, AppState, DB connection, and routes
│   └── state.rs                  # shared AppState (DB, config, revoked tokens)
├── target/
│   └── debug/                     # compiled artifacts
├── .dockerignore                  # excludes generated files from Docker contexts
├── .env                           # local configuration overrides loaded at runtime
├── .env.example                   # template for required env variables
├── .gitignore                     # files excluded from version control
├── Cargo.lock                     # locked dependency graph for reproducible builds
├── Cargo.toml                    # dependency manifest
├── docker-compose.yml           # Postgres + backend stack
├── Dockerfile                   # multi-stage build for the server
├── LICENSE
└── README.md                     # this documentation
```

## Testing

```bash
cargo test
```

This command simply builds the binary since no dedicated unit tests exist yet.

## License

Licensed under the terms described in the [`LICENSE`](LICENSE) file.
