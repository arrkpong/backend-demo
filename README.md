# backend-actix-web

Backend API built with `actix-web` and `SeaORM`, focused on secure credential storage, JWT authentication, and a clean modular layout.

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

- Handlers for registration, login, logout, a home index (`/`), and an authenticated profile endpoint (`/me`).
- Secure Argon2 hashing with per-user salt stored in `utils::auth_utils`.
- JWT encode/decode helpers that can plug into middleware (and a token blacklist for logout).
- Clean layering: handlers call services -> services call SeaORM -> utils provide crypto helpers.

## Tech stack

- **Web framework**: `actix-web` 4.x  
- **ORM**: `SeaORM` with PostgreSQL  
- **Hashing**: `argon2` + `rand_core::OsRng` for salts  
- **Tokens**: `jsonwebtoken` with the `rust_crypto` feature  
- **Env**: `dotenvy` for reading `.env`

## Requirements

- Rust (1.71+) toolchain  
- PostgreSQL database reachable via `DATABASE_URL`  
- `cargo` (bundled with Rust)

## Configuration

Copy `.env.example` to `.env` and define:

- `DATABASE_URL` – database connection string  
- `JWT_SECRET` – secret used to sign JWTs  
- `BIND_ADDRESS` *(optional)* – defaults to `127.0.0.1:8080`

## Development setup

1. Run `cargo install sqlx-cli --no-default-features --features postgres` only if you need SQLx migrations.
2. Start the backend with `cargo run`.
3. Consume the exposed HTTP endpoints (see below) with curl/Postman.

## Docker

1. Launch the stack with `docker compose up --build`.
2. The Compose file brings up Postgres plus the backend; Postgres listens on `localhost:5432` and the server on `localhost:8080`.
3. `.env` is read for `JWT_SECRET`/`BIND_ADDRESS`, while `DATABASE_URL` is overridden to point at the `db` service inside the network.

Shut down with `docker compose down`; the named volume `db-data` persists Postgres data.

## API endpoints

- `GET /` – home/index welcome message.  
- `POST /auth/register` – create a new user (returns token + filtered user data).  
- `POST /auth/login` – authenticate and receive a JWT.  
- `POST /auth/logout` – revoke the current bearer token (requires `Authorization: Bearer <token>`).  
- `GET /me` – read profile info (requires valid, non-revoked bearer token).

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

This command simply builds the binary as no dedicated unit tests exist yet.

## License

Licensed under the terms described in the [`LICENSE`](LICENSE) file.
