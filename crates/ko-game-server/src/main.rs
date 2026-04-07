//! Knight Online Game Server
//!
//! Main game server that handles all in-game packet processing (WIZ_* protocol).
//! Listens on port 15001. The launcher first connects to the Login Server (port 15100),
//! then the game client connects here after authentication.
//!
//! ## Usage
//!
//! ```sh
//! DATABASE_URL=postgresql://user:pass@localhost:5432/ko_server \
//! BIND_ADDR=0.0.0.0:15001 \
//! MAP_DIR=./Map \
//! RUST_LOG=info \
//! cargo run -p ko-game-server
//! ```

use std::path::PathBuf;

use tracing::info;

use ko_game::server::{GameServer, ServerConfig};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    // Debug build: info + warn + error (all logs visible)
    // Release build: warn + error only (critical info only)
    // Override at runtime: RUST_LOG=debug cargo run -p ko-game-server
    let default_filter = if cfg!(debug_assertions) {
        "info"
    } else {
        "warn"
    };

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(default_filter)),
        )
        .init();

    println!("╔══════════════════════════════════════════╗");
    println!("║   Knight Online Game Server  v0.4.50     ║");
    println!("╚══════════════════════════════════════════╝");
    info!("[1/5] Initializing...");

    // Configuration from environment variables
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is required");
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:15001".to_string());
    let map_dir = PathBuf::from(std::env::var("MAP_DIR").unwrap_or_else(|_| "./Map".to_string()));

    info!(
        "[1/5] Config: bind={}, map_dir={}",
        bind_addr,
        map_dir.display()
    );

    // Database connection pool
    info!("[2/5] Connecting to database...");
    let pool = ko_db::create_pool(&database_url).await?;
    info!("[2/5] Database connected");

    // Run pending migrations
    info!("[3/5] Running migrations...");
    ko_db::run_migrations(&pool).await?;
    info!("[3/5] Migrations applied");

    // Start game server (loads zones, NPCs, items, magic tables)
    info!("[4/5] Loading world data (zones, NPCs, items, magic)...");
    let config = ServerConfig { bind_addr, map_dir };
    let server = GameServer::new(config, pool).await?;
    info!("[5/5] Starting background tasks and accepting connections...");
    server.run().await
}
