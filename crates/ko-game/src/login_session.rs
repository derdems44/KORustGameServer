//! Per-client Login Server session with packet framing and encryption.
//! Simpler than the Game Server `ClientSession` — handles only the
//! launcher authentication flow (version, crypto, login, server list, news).

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpStream;
use tracing::{info, warn};

use ko_db::DbPool;
use ko_protocol::{AesCryption, JvCryption, Packet};

use crate::login_handler;
use crate::login_server::LoginServerConfig;
use crate::packet_io;

/// Per-client login session wrapping a TCP stream and encryption state.
pub struct LoginSession {
    stream: TcpStream,
    addr: SocketAddr,
    crypto: JvCryption,
    aes: AesCryption,
    sequence: u32,
    /// JvCryption S2C sequence counter (incremented per outbound packet).
    #[allow(dead_code)]
    jv_send_seq: u32,
    pool: DbPool,
    config: Arc<LoginServerConfig>,
    // Account state (set after successful 0xF3 login)
    account_id: Option<String>,
    // Server selection state (set by 0xF5 handler)
    server_index: u16,
    // Re-key state: true after server sends 0xF2 re-key (before 0xA1)
    rekeyed: bool,
    // OTP state
    otp_key: Option<String>,
    otp_trials: u8,
}

impl LoginSession {
    /// Create a new login session for an accepted TCP connection.
    pub fn new(
        stream: TcpStream,
        addr: SocketAddr,
        pool: DbPool,
        config: Arc<LoginServerConfig>,
    ) -> Self {
        let _ = stream.set_nodelay(true);
        Self {
            stream,
            addr,
            crypto: JvCryption::new(),
            aes: AesCryption::new(),
            sequence: 0,
            jv_send_seq: 0,
            pool,
            config,
            account_id: None,
            server_index: 0,
            rekeyed: false,
            otp_key: None,
            otp_trials: 0,
        }
    }

    /// Main session loop: read packets and dispatch to login handlers.
    pub async fn run(&mut self) -> anyhow::Result<()> {
        info!("[{}] Login client connected", self.addr);

        loop {
            match self.read_packet().await {
                Ok(packet) => {
                    if let Err(e) = login_handler::dispatch(self, packet).await {
                        warn!("[{}] Login handler error: {}", self.addr, e);
                        break;
                    }
                }
                Err(e) => {
                    info!("[{}] Login client disconnected: {}", self.addr, e);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Read a complete packet from the TCP stream.
    ///
    /// After 0xF2 key exchange: AES-only (matches original server, no JvCryption).
    /// Before that: plaintext.
    async fn read_packet(&mut self) -> anyhow::Result<Packet> {
        if self.aes.is_enabled() {
            // AES-only mode (v2603 sniffer verified).
            // Login server C2S has NO xor_seq counter — just [opcode][data].
            packet_io::read_packet_aes_no_seq(&mut self.stream, &self.aes).await
        } else {
            packet_io::read_packet(&mut self.stream, &self.crypto, &mut self.sequence).await
        }
    }

    /// Send a packet to the client.
    ///
    /// After 0xF2: AES-only (matches original server, no JvCryption).
    pub async fn send_packet(&mut self, packet: &Packet) -> anyhow::Result<()> {
        if self.aes.is_enabled() {
            // AES-only (original server behavior, PCAP verified).
            packet_io::send_packet_aes(&mut self.stream, &self.aes, packet).await
        } else {
            packet_io::send_packet(&mut self.stream, &self.crypto, self.sequence, packet).await
        }
    }

    /// Send a packet as PLAINTEXT, bypassing AES encryption.
    ///
    /// PCAP verified: Original server sends 0xF2 RE-KEY as plaintext
    /// even when AES is active. Client detects by payload[0] != 0x01.
    pub async fn send_packet_plaintext(&mut self, packet: &Packet) -> anyhow::Result<()> {
        packet_io::send_packet_plaintext(&mut self.stream, packet).await
    }

    /// Remote client address.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Mutable reference to the JvCryption state (legacy).
    pub fn crypto_mut(&mut self) -> &mut JvCryption {
        &mut self.crypto
    }

    /// Check if AES encryption is enabled for this session.
    pub fn aes_enabled(&self) -> bool {
        self.aes.is_enabled()
    }

    /// Mutable reference to the AES encryption state.
    pub fn aes_mut(&mut self) -> &mut AesCryption {
        &mut self.aes
    }

    /// Database connection pool.
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    /// Server configuration.
    pub fn config(&self) -> &LoginServerConfig {
        &self.config
    }

    /// OTP key stored from DB during login (if OTP required).
    pub fn account_id(&self) -> Option<&str> {
        self.account_id.as_deref()
    }

    pub fn set_account_id(&mut self, id: String) {
        self.account_id = Some(id);
    }

    /// Server index from 0xF5 request (stored for 0xA1 response).
    pub fn server_index(&self) -> u16 {
        self.server_index
    }

    pub fn set_server_index(&mut self, idx: u16) {
        self.server_index = idx;
    }

    /// Whether this session has completed the re-key exchange.
    pub fn is_rekeyed(&self) -> bool {
        self.rekeyed
    }

    pub fn set_rekeyed(&mut self, val: bool) {
        self.rekeyed = val;
    }

    pub fn otp_key(&self) -> Option<&str> {
        self.otp_key.as_deref()
    }

    /// Set OTP key after DB lookup.
    pub fn set_otp_key(&mut self, key: String) {
        self.otp_key = Some(key);
    }

    /// Number of failed OTP attempts.
    pub fn otp_trials(&self) -> u8 {
        self.otp_trials
    }

    /// Increment OTP trial counter.
    pub fn increment_otp_trials(&mut self) {
        self.otp_trials += 1;
    }
}
