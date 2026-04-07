//! LS_SERVERLIST (0xF5) handler — returns the game server list.
//!
//! ## v2599 Client Behavior
//!
//! Our v2599 client requires the F5 echo response with server list data
//! to populate the server selection UI and advance the login flow.
//! Without it, the client stalls after FD and never sends F6.
//!
//! Note: The v2602 original server client doesn't need this response
//! (server list comes from launcher/config), but our v2599 client does.
//!
//! ## Request (Client → Server)
//!
//! | Offset | Type  | Description         |
//! |--------|-------|---------------------|
//! | 0      | u16le | Server index (echo) |
//!
//! ## Response (Server → Client) — opcode 0xF5
//!
//! | Offset | Type   | Description                    |
//! |--------|--------|--------------------------------|
//! | 0      | u16le  | Server index (echo)            |
//! | 2      | u8     | Server count                   |
//! | 3+     | ...    | Per-server entries (C++ format) |

use ko_db::repositories::server_list::ServerListRepository;
use ko_protocol::{LoginOpcode, Packet, PacketReader};

use crate::login_session::LoginSession;

/// Handle LS_SERVERLIST (0xF5) — echo with server list from DB.
pub async fn handle(session: &mut LoginSession, pkt: Packet) -> anyhow::Result<()> {
    let mut reader = PacketReader::new(&pkt.data);
    let server_index = reader.read_u16().unwrap_or(0);

    session.set_server_index(server_index);

    // Echo pattern: respond with same opcode (0xF5)
    let mut response = Packet::new(LoginOpcode::LsServerList as u8);
    response.write_u16(server_index); // echo

    // Load server list from DB
    let repo = ServerListRepository::new(session.pool());
    let servers = repo.load_all().await.unwrap_or_default();

    response.write_u8(servers.len() as u8);

    for srv in &servers {
        response.write_string(&srv.lan_ip);
        response.write_string(&srv.server_ip);
        response.write_string(&srv.server_name);

        if srv.concurrent_users >= srv.player_cap as i32 {
            response.write_i16(-1);
        } else {
            response.write_i16(srv.concurrent_users as i16);
        }

        response.write_i16(srv.server_id);
        response.write_i16(srv.group_id);
        response.write_i16(srv.player_cap);
        response.write_i16(srv.free_player_cap);
        response.write_u8(0);
        response.write_u8(srv.screen_type as u8);
        response.write_string(&srv.karus_king);
        response.write_string(&srv.karus_notice);
        response.write_string(&srv.elmorad_king);
        response.write_string(&srv.elmorad_notice);
    }

    session.send_packet(&response).await?;

    tracing::info!(
        "[{}] 0xF5 → sent server list ({} servers), server_index={}",
        session.addr(),
        servers.len(),
        server_index,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use ko_protocol::{LoginOpcode, Packet, PacketReader};

    #[test]
    fn test_server_list_opcode() {
        assert_eq!(LoginOpcode::LsServerList as u8, 0xF5);
    }

    #[test]
    fn test_server_list_c2s_format() {
        let mut pkt = Packet::new(LoginOpcode::LsServerList as u8);
        pkt.write_u16(42);
        assert_eq!(pkt.data.len(), 2);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u16(), Some(42));
    }
}
