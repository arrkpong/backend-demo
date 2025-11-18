# AGENTS.md

## Project overview
- Backend API built with [`actix-web`](https://github.com/actix/actix-web) and [`SeaORM`](https://www.sea-ql.org/SeaORM/), focused on secure credential storage, JWT auth, and modular layering (handlers → services → SeaORM).
- PostgreSQL is the production database and is accessed through the connection helpers in `src/db/connection.rs`.

## Setup commands
1. Copy `.env.example` to `.env` and provide values for `DATABASE_URL`, `JWT_SECRET`, and optional `BIND_ADDRESS`.
2. Install project Rust toolchain (>= 1.71) and ensure `cargo` is on your `PATH`.
3. (Optional) if you need to generate SQLx migrations, install `sqlx-cli` via `cargo install sqlx-cli --no-default-features --features postgres`.
4. Run `cargo build` to verify dependencies.

## Development workflow
- Start the dev server with `cargo run`.
- Use Postman/curl to hit endpoints defined under `src/routes/user_routes.rs`.
- JWT middleware lives in `src/middleware/auth_middleware.rs`, and shared state (DB, revoked tokens) sits in `src/state.rs`.
- If you add a new module, register it alongside `src/services/`, `src/routes/`, or `src/handlers/` as appropriate.

## Testing and verification
- Execute `cargo test`. This command currently just builds the binary because no unit tests exist yet.
- Keep an eye on `target/debug/` artifacts and clean them with `cargo clean` if the build gets stale.

## Code style & conventions
- Follow Rust 2021 idioms: use `clippy` hints; keep modules focused.
- Keep crypto utilities in `src/utils/` (Argon2 helpers in `auth_utils.rs`, JWT helpers in `jwt.rs`).
- Prefer explicit naming of handlers/services so `main.rs` wiring remains clear.

## Operations notes
- Compose stack with `docker compose up --build` when you also need PostgreSQL; service listens on `localhost:8080`.
- `.env` is loaded locally, but the Compose file overrides `DATABASE_URL` for the `db` service.
- `.dockerignore`, `.gitignore`, and `Cargo.lock` are in place to keep builds reproducible and caches clean.

## PR & commit guidance
- Always run `cargo fmt` before a PR and ensure `cargo test` passes in CI.
- Keep README/docs in sync with code changes (handlers, routes, config keys).

## Reference files
- `README.md` – high-level overview and basic instructions.
- `migration/` – SQL files describing schema changes; keep them atomic and versioned.
- `Cargo.toml` / `Cargo.lock` – dependency manifest; update via `cargo add` or `cargo update` when needed.
