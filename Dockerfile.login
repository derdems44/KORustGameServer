# ── Stage 1: Build ────────────────────────────────────────────────────────────
FROM rust:1.93-bookworm AS builder
WORKDIR /build
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
COPY crates/ko-core/Cargo.toml crates/ko-core/Cargo.toml
COPY crates/ko-protocol/Cargo.toml crates/ko-protocol/Cargo.toml
COPY crates/ko-db/Cargo.toml crates/ko-db/Cargo.toml
COPY crates/ko-game/Cargo.toml crates/ko-game/Cargo.toml
COPY crates/ko-login-server/Cargo.toml crates/ko-login-server/Cargo.toml
COPY crates/ko-game-server/Cargo.toml crates/ko-game-server/Cargo.toml
COPY crates/ko-tbl-import/Cargo.toml crates/ko-tbl-import/Cargo.toml
COPY crates/ko-quest-audit/Cargo.toml crates/ko-quest-audit/Cargo.toml
COPY crates/ko-quest-gen/Cargo.toml crates/ko-quest-gen/Cargo.toml
RUN mkdir -p crates/ko-core/src && echo "" > crates/ko-core/src/lib.rs && \
    mkdir -p crates/ko-protocol/src && echo "" > crates/ko-protocol/src/lib.rs && \
    mkdir -p crates/ko-db/src && echo "" > crates/ko-db/src/lib.rs && \
    mkdir -p crates/ko-game/src && echo "" > crates/ko-game/src/lib.rs && \
    mkdir -p crates/ko-login-server/src && echo "fn main(){}" > crates/ko-login-server/src/main.rs && \
    mkdir -p crates/ko-game-server/src && echo "fn main(){}" > crates/ko-game-server/src/main.rs && \
    mkdir -p crates/ko-tbl-import/src && echo "fn main(){}" > crates/ko-tbl-import/src/main.rs && \
    mkdir -p crates/ko-quest-audit/src && echo "" > crates/ko-quest-audit/src/lib.rs && \
    mkdir -p crates/ko-quest-gen/src && echo "fn main(){}" > crates/ko-quest-gen/src/main.rs
ENV CARGO_BUILD_JOBS=1
RUN cargo build --release -p ko-login-server 2>/dev/null || true
COPY crates/ crates/
RUN find crates -name "*.rs" -exec touch {} + && cargo build --release -p ko-login-server

# ── Stage 2: Runtime ──────────────────────────────────────────────────────────
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
RUN useradd --create-home --shell /bin/bash koserver
WORKDIR /opt/koserver
COPY --from=builder /build/target/release/ko-login-server ./
COPY crates/ko-db/migrations/ ./migrations/
RUN chown -R koserver:koserver /opt/koserver
USER koserver
EXPOSE 15100-15109
CMD ["./ko-login-server"]
