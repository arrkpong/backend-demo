# Civil Park Backend v2

Backend API built with Actix Web and SeaORM, designed to store user credentials securely using Argon2 hashing.

## Features

- HTTP endpoints for health check, user registration, and login (`/`, `/auth/register`, `/auth/login`)
- SeaORM models for the `users` table and PostgreSQL support via the `DATABASE_URL` environment variable
- Argon2 password hashing helpers that salt and verify credentials before persisting or validating

## Getting started

1. Copy `.env.example` to `.env` and update `DATABASE_URL` with your PostgreSQL connection.
2. Run `cargo install sqlx-cli --no-default-features --features postgres` if you need migration tooling.
3. Start the server with `cargo run`.

## Testing

```bash
cargo test
```

There are currently no unit tests, but the command ensures the project compiles.

## Notes

- Passwords are hashed via Argon2 before storage and verified against the stored hash during login.
- Ensure any existing plaintext passwords are rehashed to avoid authentication issues.
