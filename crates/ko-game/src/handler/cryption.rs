//! WIZ_CRYPTION (0x2C) handler — DEPRECATED.
//!
//! The original server uses AES-128-CBC delivered via the 0x2B response.
//! No separate 0x2C key exchange is performed. JvCryption is disabled.
//!
//! If the client somehow sends 0x2C, log a warning and ignore it.

use ko_protocol::Packet;

use crate::session::ClientSession;

/// Handle WIZ_CRYPTION from the client — no-op (AES replaces JvCryption).
pub async fn handle(session: &mut ClientSession, _pkt: Packet) -> anyhow::Result<()> {
    tracing::warn!(
        "[{}] Received WIZ_CRYPTION (0x2C) — ignored, AES-128-CBC is active",
        session.addr()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use ko_protocol::Opcode;

    #[test]
    fn test_cryption_opcode_value() {
        assert_eq!(Opcode::WizCryption as u8, 0x2C);
    }
}
