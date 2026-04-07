# ── Stage 1: Build Rust binaries ──────────────────────────────────────────────
FROM rust:1.93-bookworm AS builder

WORKDIR /build

# System dependencies (mlua vendored Lua 5.1 build)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy workspace manifest + lock (dependency cache layer)
COPY Cargo.toml Cargo.lock ./

# Copy crate manifests only
COPY crates/ko-core/Cargo.toml crates/ko-core/Cargo.toml
COPY crates/ko-protocol/Cargo.toml crates/ko-protocol/Cargo.toml
COPY crates/ko-db/Cargo.toml crates/ko-db/Cargo.toml
COPY crates/ko-game/Cargo.toml crates/ko-game/Cargo.toml
COPY crates/ko-login-server/Cargo.toml crates/ko-login-server/Cargo.toml
COPY crates/ko-game-server/Cargo.toml crates/ko-game-server/Cargo.toml
COPY crates/ko-tbl-import/Cargo.toml crates/ko-tbl-import/Cargo.toml
COPY crates/ko-quest-audit/Cargo.toml crates/ko-quest-audit/Cargo.toml
COPY crates/ko-quest-gen/Cargo.toml crates/ko-quest-gen/Cargo.toml

# Dummy sources for dependency pre-build (cache unless Cargo.toml/lock change)
RUN mkdir -p crates/ko-core/src && echo "" > crates/ko-core/src/lib.rs && \
    mkdir -p crates/ko-protocol/src && echo "" > crates/ko-protocol/src/lib.rs && \
    mkdir -p crates/ko-db/src && echo "" > crates/ko-db/src/lib.rs && \
    mkdir -p crates/ko-game/src && echo "" > crates/ko-game/src/lib.rs && \
    mkdir -p crates/ko-login-server/src && echo "fn main(){}" > crates/ko-login-server/src/main.rs && \
    mkdir -p crates/ko-game-server/src && echo "fn main(){}" > crates/ko-game-server/src/main.rs && \
    mkdir -p crates/ko-tbl-import/src && echo "fn main(){}" > crates/ko-tbl-import/src/main.rs && \
    mkdir -p crates/ko-quest-audit/src && echo "" > crates/ko-quest-audit/src/lib.rs && \
    mkdir -p crates/ko-quest-gen/src && echo "fn main(){}" > crates/ko-quest-gen/src/main.rs

# Limit parallel jobs to reduce memory usage on NAS
ENV CARGO_BUILD_JOBS=1
RUN cargo build --release -p ko-login-server -p ko-game-server 2>/dev/null || true

# Copy real source
COPY crates/ crates/

# Invalidate dummy builds, compile real code
RUN find crates -name "*.rs" -exec touch {} + && \
    cargo build --release -p ko-login-server -p ko-game-server

# ── Stage 2: Minimal runtime ─────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --shell /bin/bash koserver
WORKDIR /opt/koserver

# Binaries
COPY --from=builder /build/target/release/ko-login-server ./
COPY --from=builder /build/target/release/ko-game-server ./

# Migrations (sqlx runtime migrate)
COPY crates/ko-db/migrations/ ./migrations/

# Game data (from git repo)
COPY Quests/ ./Quests/
COPY Map/ ./Map/

RUN chown -R koserver:koserver /opt/koserver
USER koserver

ENV RUST_LOG=info \
    DATABASE_URL=postgresql://$DB_USER:$DB_PASSWORD@db:5432/ko_server \
    BIND_ADDR=0.0.0.0:15001 \
    BIND_IP=0.0.0.0 \
    BASE_PORT=15100 \
    GAME_SERVER_IP=0.0.0.0 \
    GAME_SERVER_PORT=15001 \
    MAP_DIR=./Map

EXPOSE 15001 15100-15109
