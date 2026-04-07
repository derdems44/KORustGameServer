//! Login Server — TCP listener and connection accept loop.
//! The C++ server listens on 10 consecutive ports (base_port .. base_port+9).
//! Launchers pick one at random, so we must bind all 10.
//! Separate from the Game Server (port 15001).

use std::sync::Arc;

use tokio::net::TcpListener;
use tracing::{error, info};

use ko_db::DbPool;

use crate::login_session::LoginSession;

/// Number of listener ports (mirrors `for (int i = 0; i < 10; i++)`).
const LISTENER_COUNT: u16 = 10;

/// Login server configuration.
pub struct LoginServerConfig {
    /// IP address to bind (e.g. `0.0.0.0`).
    pub bind_ip: String,
    /// Base port — server listens on base_port .. base_port + 9 (default 15100).
    pub base_port: u16,
    /// Game server public IP (sent in server list).
    pub game_server_ip: String,
    /// Game server port (default 15001).
    pub game_server_port: u16,
    /// Protocol version to report to clients.
    pub version: u16,
    /// FTP URL for patch downloads (sent in LS_DOWNLOADINFO_REQ response).
    pub ftp_url: String,
    /// FTP path for patch downloads (sent in LS_DOWNLOADINFO_REQ response).
    pub ftp_path: String,
    /// News title displayed on login screen.
    pub news_title: String,
    /// News content displayed on login screen.
    pub news_content: String,
}

/// Login server — accepts launcher TCP connections and spawns login sessions.
pub struct LoginServer {
    config: Arc<LoginServerConfig>,
    pool: DbPool,
}

impl LoginServer {
    /// Create a new login server with the given config and database pool.
    pub fn new(config: LoginServerConfig, pool: DbPool) -> Self {
        Self {
            config: Arc::new(config),
            pool,
        }
    }

    /// Run the server — binds 10 consecutive ports and accepts connections on all.
    pub async fn run(&self) -> anyhow::Result<()> {
        let base = self.config.base_port;
        let ip = &self.config.bind_ip;

        // Bind all 10 ports (same as C++ LoginServer::Startup)
        let mut listeners = Vec::with_capacity(LISTENER_COUNT as usize);
        for i in 0..LISTENER_COUNT {
            let addr = format!("{}:{}", ip, base + i);
            let listener = TcpListener::bind(&addr).await?;
            info!("Login Server listening on {}", addr);
            listeners.push(listener);
        }

        info!("====================================");
        info!("  Login Server READY — {} ports active", LISTENER_COUNT);
        info!("  Ports: {}:{}-{}", ip, base, base + LISTENER_COUNT - 1);
        info!("====================================");

        // Spawn an accept loop for each listener
        let mut handles = Vec::with_capacity(listeners.len());
        for listener in listeners {
            let pool = self.pool.clone();
            let config = Arc::clone(&self.config);
            handles.push(tokio::spawn(async move {
                Self::accept_loop(listener, pool, config).await;
            }));
        }

        // Wait for shutdown signal (Ctrl+C / SIGTERM)
        let shutdown = async {
            let ctrl_c = tokio::signal::ctrl_c();
            #[cfg(unix)]
            {
                let mut sigterm =
                    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                        .expect("failed to register SIGTERM handler");
                tokio::select! {
                    _ = ctrl_c => {}
                    _ = sigterm.recv() => {}
                }
            }
            #[cfg(not(unix))]
            {
                ctrl_c.await.ok();
            }
        };
        shutdown.await;
        info!(
            "Login Server shutdown signal received — stopping {} listeners...",
            handles.len()
        );
        for h in &handles {
            h.abort();
        }
        info!("Login Server shut down.");

        Ok(())
    }

    /// Accept loop for a single listener port.
    async fn accept_loop(listener: TcpListener, pool: DbPool, config: Arc<LoginServerConfig>) {
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let pool = pool.clone();
                    let config = Arc::clone(&config);
                    tokio::spawn(async move {
                        let handle = tokio::spawn(async move {
                            let mut session = LoginSession::new(stream, addr, pool, config);
                            session.run().await
                        });
                        match handle.await {
                            Ok(Ok(())) => {}
                            Ok(Err(e)) => error!("[{}] Login session error: {}", addr, e),
                            Err(join_err) => {
                                error!("[{}] Login session panicked: {}", addr, join_err)
                            }
                        }
                    });
                }
                Err(e) => {
                    error!("Login accept error: {}", e);
                }
            }
        }
    }
}
