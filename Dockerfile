FROM rust:1.91.1 as planner
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

FROM rust:1.91.1 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/backend .
CMD ["./backend"]
