//! WIZ_HACKTOOL (0x72) handler — hack tool detection.
//! The C++ server ignores this packet entirely (`break;` with no processing).
//! We do the same — accept and discard silently.

use ko_protocol::Packet;

use crate::session::ClientSession;

/// Handle WIZ_HACKTOOL from the client.
pub fn handle(_session: &mut ClientSession, _pkt: Packet) -> anyhow::Result<()> {
    // Server intentionally ignores this packet (matches C++ behavior)
    Ok(())
}

#[cfg(test)]
mod tests {
    use ko_protocol::Opcode;

    #[test]
    fn test_hacktool_opcode_value() {
        assert_eq!(Opcode::WizHacktool as u8, 0x72);
    }

    #[test]
    fn test_hacktool_is_noop() {
        // C++ server also ignores this packet (break with no processing)
        // Handler signature: fn handle(_session, _pkt) -> Ok(())
        // Both parameters are prefixed with underscore = intentionally unused
        assert_eq!(
            std::mem::size_of::<fn() -> anyhow::Result<()>>(),
            std::mem::size_of::<usize>(),
            "handler is a simple function pointer"
        );
    }
}
