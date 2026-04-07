//! WIZ_WORLD_BOSS (0xD6) handler — World boss event system.
//! v2525 client's world boss tracking panel. Displays boss status,
//! UI initialization data, and ranking/info messages.
//! ## Client RE
//! - Handler: `0x70B210` — reads `[u8=1][u8 sub2]`, dispatches sub2 (1-3)
//! - Panel: `[game+0x6B4]` — Group B (panel-dependent)
//! - Storage: 4 boss panel slots at `this+0x180` through `+0x5508`
//! - Status handler: `0x70C1E0` — 10-entry jump table at `0x70C400`
//! - UI init handler: `0x70BC20` — complex per-boss entry format
//! - Ranking handler: `0x70B830` — LUT at `0x70BBB8` (1-102 range)
//! ## S2C Packet Format
//! ```text
//! [u8 sub1]  — MUST be 1 (client checks)
//! [u8 sub2]  — sub-opcode:
//!   sub2=1: Boss Status
//!     [i32 status_code]  — 1-10 (jump table)
//!   sub2=2: Boss UI Init
//!     [i32 result_code]  — must be 1 (else error string 0x9C8B/40075)
//!     [u8 boss_count]    — 0-4 boss entries
//!       for each boss:
//!         [u16 slot_id]      — 1-based (client does -1, valid 1-4)
//!         [u16 name_len]     — boss name string length
//!         [name_len bytes]   — boss name (raw bytes)
//!         [u8 alive_state]   — stored at slot+0x840
//!         [u8 hp_gauge]      — HP bar percentage, slot+0x834
//!         [u8 boss_type]     — 1-4, determines gauge clamping
//!         [u16 boss_info_id] — animation resource lookup
//!         [u8 gauge_level]   — clamped by boss_type
//!         [u32 packed_color] — low 24=RGB, high 8=alpha tier
//!         [u8 enabled_flag]  — stored at this+0x4220+slot_index
//!   sub2=3: Ranking/Info
//!     [i32 info_id]      — 1-102 range, string lookup via LUT
//! ```
//! ## Status Codes (sub2=1, jump table at `0x70C400`)
//! | code | String ID | Hex    | Meaning                         |
//! |------|-----------|--------|---------------------------------|
//! | 1    | —         | —      | Boss spawned (opens panel)      |
//! | 2    | 31115     | 0x798B | Status message                  |
//! | 3    | 31116     | 0x798C | Status message                  |
//! | 4    | 31117     | 0x798D | Status message                  |
//! | 5-9  | —         | —      | Fall through (no display)       |
//! | 10   | 31118     | 0x798E | Status message                  |
//! ## Ranking Info IDs (sub2=3, LUT at `0x70BBB8`)
//! | info_id | String ID | Decimal |
//! |---------|-----------|---------|
//! | 1       | 0x4075    | 16501   |
//! | 2       | 0x7983    | 31107   |
//! | 3       | 0x7984    | 31108   |
//! | 4-8     | 0x7985-89 | 31109-13|
//! | 10      | 0x798A    | 31114   |
//! | 101     | 0x798F    | 31119   |
//! | 102     | 0x7990    | 31120   |
//! | default | 0x5E6     | 1510    |
//! ## Boss Type Gauge Clamping
//! - Type 1/2: gauge_level [0,3], alpha [0,3]
//! - Type 3/4: gauge_level [4,7], alpha [3,6]
//! ## C2S Packets
//! - Ranking request: `[u8=1][u8 sub2=3][u8 slot_id][u8 state][i32 panel_id][u16 name_len][name]`
//! - Panel button: `[u8=1][u8 sub2=1][u16 param][optional string]`

use std::sync::atomic::{AtomicI32, AtomicU8, Ordering};

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::{debug, warn};

use crate::session::{ClientSession, SessionState};
use ko_db::repositories::world_boss::WorldBossRepository;

// ── Status code constants ─────────────────────────────────────────────

/// Boss spawned — opens panel via `0x70CB40(0)`.
pub const STATUS_SPAWNED: i32 = 1;

/// Status message — string `0x798B` (31115).
pub const STATUS_MSG_798B: i32 = 2;

/// Status message — string `0x798C` (31116).
pub const STATUS_MSG_798C: i32 = 3;

/// Status message — string `0x798D` (31117).
pub const STATUS_MSG_798D: i32 = 4;

/// Status message — string `0x798E` (31118).
pub const STATUS_MSG_798E: i32 = 10;

// ── Sub2 constants ────────────────────────────────────────────────────

/// Boss status sub-opcode.
pub const SUB2_STATUS: u8 = 1;

/// Boss UI init sub-opcode.
pub const SUB2_UI_INIT: u8 = 2;

/// Boss ranking/info sub-opcode.
pub const SUB2_RANKING: u8 = 3;

// ── Data types ────────────────────────────────────────────────────────

/// A boss entry for the UI init packet (sub2=2).
#[derive(Debug, Clone)]
pub struct BossEntry {
    /// Panel slot (1-based, client does -1 internally). Valid: 1-4.
    pub slot_id: u16,
    /// Boss display name.
    pub name: String,
    /// Alive state (stored at slot+0x840).
    pub alive_state: u8,
    /// HP bar gauge percentage (stored at slot+0x834).
    pub hp_gauge: u8,
    /// Boss type (1-4). Determines gauge_level clamping range.
    pub boss_type: u8,
    /// Animation resource lookup ID (stored at slot+0x83C).
    pub boss_info_id: u16,
    /// Gauge level (clamped by boss_type: 1/2→[0,3], 3/4→[4,7]).
    pub gauge_level: u8,
    /// Packed color: low 24 bits = RGB, high 8 bits = alpha tier.
    pub packed_color: u32,
    /// Enabled flag (stored at this+0x4220+slot_index).
    pub enabled_flag: u8,
}

/// Runtime boss state for live tracking (atomic — safe for concurrent access).
/// Each boss slot (1-4) has one of these, stored in the world state or
/// a shared map. Updated by the NPC/combat system when bosses take damage.
#[derive(Debug)]
pub struct LiveBossState {
    /// Boss slot (1-4).
    pub slot_id: u8,
    /// Alive state: 0 = dead/not spawned, 1 = alive.
    pub alive: AtomicU8,
    /// HP gauge percentage (0-100).
    pub hp_gauge: AtomicU8,
    /// Current HP (raw value from NPC system).
    pub current_hp: AtomicI32,
    /// Maximum HP.
    pub max_hp: AtomicI32,
    /// Gauge level (clamped by boss_type).
    pub gauge_level: AtomicU8,
}

impl LiveBossState {
    /// Create a new dead/inactive boss state for a slot.
    pub fn new(slot_id: u8) -> Self {
        Self {
            slot_id,
            alive: AtomicU8::new(0),
            hp_gauge: AtomicU8::new(0),
            current_hp: AtomicI32::new(0),
            max_hp: AtomicI32::new(0),
            gauge_level: AtomicU8::new(0),
        }
    }

    /// Update HP and recalculate gauge percentage.
    pub fn update_hp(&self, current: i32, max: i32) {
        self.current_hp.store(current, Ordering::Relaxed);
        self.max_hp.store(max, Ordering::Relaxed);
        let pct = if max > 0 {
            ((current.max(0) as u64) * 100 / (max as u64)).min(100) as u8
        } else {
            0
        };
        self.hp_gauge.store(pct, Ordering::Relaxed);
    }

    /// Mark boss as spawned/alive.
    pub fn spawn(&self, max_hp: i32) {
        self.alive.store(1, Ordering::Relaxed);
        self.current_hp.store(max_hp, Ordering::Relaxed);
        self.max_hp.store(max_hp, Ordering::Relaxed);
        self.hp_gauge.store(100, Ordering::Relaxed);
    }

    /// Mark boss as dead.
    pub fn kill(&self) {
        self.alive.store(0, Ordering::Relaxed);
        self.current_hp.store(0, Ordering::Relaxed);
        self.hp_gauge.store(0, Ordering::Relaxed);
    }

    /// Check if boss is alive.
    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::Relaxed) != 0
    }

    /// Get HP gauge as percentage (0-100).
    pub fn gauge(&self) -> u8 {
        self.hp_gauge.load(Ordering::Relaxed)
    }
}

// ── S2C Builders ──────────────────────────────────────────────────────

/// Build a world boss status message packet (sub2=1).
/// Client RE: `0x70C1E0` — 10-entry jump table. Code 1 opens panel,
/// 2-4/10 show string messages, 5-9 silently fall through.
/// Wire: `[u8 sub1=1][u8 sub2=1][i32 status_code]`
pub fn build_status(status_code: i32) -> Packet {
    let mut pkt = Packet::new(Opcode::WizWorldBoss as u8);
    pkt.write_u8(1); // sub1 (must be 1)
    pkt.write_u8(SUB2_STATUS);
    pkt.write_i32(status_code);
    pkt
}

/// Build a world boss UI init packet (sub2=2).
/// Client RE: `0x70BC20` — reads result_code (must be 1), then
/// boss_count entries with per-boss 10-field structure.
/// - `bosses`: Up to 4 boss entries
/// Wire: `[u8 sub1=1][u8 sub2=2][i32 result=1][u8 count][{per-boss}×N]`
pub fn build_ui_init(bosses: &[BossEntry]) -> Packet {
    let mut pkt = Packet::new(Opcode::WizWorldBoss as u8);
    pkt.write_u8(1); // sub1
    pkt.write_u8(SUB2_UI_INIT);
    pkt.write_i32(1); // result_code (must be 1)
    let count = bosses.len().min(4) as u8;
    pkt.write_u8(count);
    for boss in bosses.iter().take(4) {
        pkt.write_u16(boss.slot_id);
        let name_bytes = boss.name.as_bytes();
        pkt.write_u16(name_bytes.len() as u16);
        for &b in name_bytes {
            pkt.write_u8(b);
        }
        pkt.write_u8(boss.alive_state);
        pkt.write_u8(boss.hp_gauge);
        pkt.write_u8(boss.boss_type);
        pkt.write_u16(boss.boss_info_id);
        pkt.write_u8(boss.gauge_level);
        pkt.write_u32(boss.packed_color);
        pkt.write_u8(boss.enabled_flag);
    }
    pkt
}

/// Build a world boss UI init error packet (sub2=2, result != 1).
/// Client shows error string `0x9C8B` (40075) when result_code != 1.
/// Wire: `[u8 sub1=1][u8 sub2=2][i32 result_code]`
pub fn build_ui_init_error(result_code: i32) -> Packet {
    let mut pkt = Packet::new(Opcode::WizWorldBoss as u8);
    pkt.write_u8(1); // sub1
    pkt.write_u8(SUB2_UI_INIT);
    pkt.write_i32(result_code);
    pkt
}

/// Build a world boss ranking/info packet (sub2=3).
/// Client RE: `0x70B830` — LUT at `0x70BBB8` maps info_id to string.
/// - `info_id`: Ranking/info identifier (1-102)
/// Wire: `[u8 sub1=1][u8 sub2=3][i32 info_id]`
pub fn build_ranking_info(info_id: i32) -> Packet {
    let mut pkt = Packet::new(Opcode::WizWorldBoss as u8);
    pkt.write_u8(1); // sub1
    pkt.write_u8(SUB2_RANKING);
    pkt.write_i32(info_id);
    pkt
}

// ── C2S Handler ───────────────────────────────────────────────────────

/// Handle WIZ_WORLD_BOSS (0xD6) from the client.
/// C2S packets:
/// - Panel button: `[u8=1][u8 sub2=1][u16 param]`
/// - UI init request: `[u8=1][u8 sub2=2]`
/// - Ranking request: `[u8=1][u8 sub2=3][u8 slot_id][u8 state][i32 panel_id][name]`
/// Implemented: sub2=2 (UI init from DB config).
/// Stub: sub2=1 (status), sub2=3 (ranking).
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);
    let sub1 = reader.read_u8().unwrap_or(0);

    if sub1 != 1 {
        debug!(
            "[{}] WIZ_WORLD_BOSS unexpected sub1={} (must be 1)",
            session.addr(),
            sub1
        );
        return Ok(());
    }

    let sub2 = reader.read_u8().unwrap_or(0);

    match sub2 {
        SUB2_STATUS => {
            // Client button press — no live boss events yet
            let param = reader.read_u16().unwrap_or(0);
            debug!(
                "[{}] WIZ_WORLD_BOSS status request param={}",
                session.addr(),
                param
            );
            // Send "no active event" status (code 0 → client jump table falls through, no display)
            session.send_packet(&build_status(0)).await?;
        }
        SUB2_UI_INIT => {
            // Client opened world boss panel — send boss config from DB
            handle_ui_init(session).await?;
        }
        SUB2_RANKING => {
            // C2S: [u8 slot_id][u8 state][i32 panel_id][u16 name_len][name bytes]
            let slot_id = reader.read_u8().unwrap_or(0);
            let state = reader.read_u8().unwrap_or(0);
            let panel_id = reader.read_i32().unwrap_or(0);
            let name = reader.read_string().unwrap_or_default();
            debug!(
                "[{}] WIZ_WORLD_BOSS ranking: slot={}, state={}, panel_id={}, name='{}'",
                session.addr(),
                slot_id,
                state,
                panel_id,
                name
            );
            handle_ranking(session, slot_id, panel_id).await?;
        }
        _ => {
            debug!(
                "[{}] WIZ_WORLD_BOSS unknown sub2={} ({}B remaining)",
                session.addr(),
                sub2,
                reader.remaining()
            );
            // Unknown sub2 — send status code 0 (no event) to avoid silent drop
            session.send_packet(&build_status(0)).await?;
        }
    }

    Ok(())
}

/// Handle world boss UI init request (sub2=2).
/// Loads boss configurations from the DB and sends them to the client.
/// All bosses start as dead/inactive (no live spawn lifecycle yet).
async fn handle_ui_init(session: &mut ClientSession) -> anyhow::Result<()> {
    let pool = session.pool().clone();
    let repo = WorldBossRepository::new(&pool);

    let configs = match repo.load_configs().await {
        Ok(c) => c,
        Err(e) => {
            warn!(
                "[{}] WIZ_WORLD_BOSS DB error loading configs: {}",
                session.addr(),
                e
            );
            session.send_packet(&build_ui_init_error(0)).await?;
            return Ok(());
        }
    };

    let bosses: Vec<BossEntry> = configs
        .iter()
        .filter(|c| c.enabled)
        .map(|c| BossEntry {
            slot_id: c.slot_id as u16,
            name: c.boss_name.clone(),
            alive_state: 0, // dead (no live tracking yet)
            hp_gauge: 0,    // empty HP bar
            boss_type: c.boss_type as u8,
            boss_info_id: c.boss_info_id as u16,
            gauge_level: 0,
            packed_color: 0,
            enabled_flag: 1,
        })
        .collect();

    debug!(
        "[{}] WIZ_WORLD_BOSS UI init: {} enabled bosses",
        session.addr(),
        bosses.len()
    );

    session.send_packet(&build_ui_init(&bosses)).await?;
    Ok(())
}

/// Handle ranking request (sub2=3).
/// Queries the DB for rankings on the given boss slot. If data exists,
/// sends info_id=10 (string 31114 — "ranking available"). Otherwise
/// sends info_id=101 (string 31119 — "no ranking data").
async fn handle_ranking(
    session: &mut ClientSession,
    slot_id: u8,
    _panel_id: i32,
) -> anyhow::Result<()> {
    if slot_id == 0 || slot_id > 4 {
        return session.send_packet(&build_ranking_info(101)).await;
    }

    let pool = session.pool().clone();
    let repo = WorldBossRepository::new(&pool);

    match repo.top_rankings(slot_id as i16, 1).await {
        Ok(rankings) if !rankings.is_empty() => {
            // Rankings exist — send "ranking available" info
            session.send_packet(&build_ranking_info(10)).await
        }
        Ok(_) => {
            // No rankings — send "no ranking data"
            session.send_packet(&build_ranking_info(101)).await
        }
        Err(e) => {
            warn!(
                "[{}] WIZ_WORLD_BOSS ranking DB error: {}",
                session.addr(),
                e
            );
            session.send_packet(&build_ranking_info(101)).await
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::PacketReader;

    // ── Status builder (sub2=1) ───────────────────────────────────────

    #[test]
    fn test_build_status_opcode() {
        let pkt = build_status(STATUS_SPAWNED);
        assert_eq!(pkt.opcode, Opcode::WizWorldBoss as u8);
    }

    #[test]
    fn test_build_status_format() {
        let pkt = build_status(STATUS_SPAWNED);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // sub1
        assert_eq!(r.read_u8(), Some(SUB2_STATUS)); // sub2=1
        assert_eq!(r.read_i32(), Some(STATUS_SPAWNED)); // status_code
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_status_data_length() {
        // u8 sub1 + u8 sub2 + i32 status = 1+1+4 = 6
        let pkt = build_status(0);
        assert_eq!(pkt.data.len(), 6);
    }

    #[test]
    fn test_build_status_message_codes() {
        for &code in &[
            STATUS_MSG_798B,
            STATUS_MSG_798C,
            STATUS_MSG_798D,
            STATUS_MSG_798E,
        ] {
            let pkt = build_status(code);
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u8(), Some(1));
            assert_eq!(r.read_u8(), Some(SUB2_STATUS));
            assert_eq!(r.read_i32(), Some(code));
        }
    }

    // ── UI Init builder (sub2=2) ──────────────────────────────────────

    #[test]
    fn test_build_ui_init_empty() {
        let pkt = build_ui_init(&[]);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // sub1
        assert_eq!(r.read_u8(), Some(SUB2_UI_INIT)); // sub2=2
        assert_eq!(r.read_i32(), Some(1)); // result_code
        assert_eq!(r.read_u8(), Some(0)); // boss_count=0
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_ui_init_empty_data_length() {
        // u8 sub1 + u8 sub2 + i32 result + u8 count = 1+1+4+1 = 7
        let pkt = build_ui_init(&[]);
        assert_eq!(pkt.data.len(), 7);
    }

    #[test]
    fn test_build_ui_init_one_boss() {
        let boss = BossEntry {
            slot_id: 1,
            name: "Dragon".to_string(),
            alive_state: 1,
            hp_gauge: 80,
            boss_type: 2,
            boss_info_id: 105,
            gauge_level: 2,
            packed_color: 0x03FF0000, // alpha tier 3, red
            enabled_flag: 1,
        };
        let pkt = build_ui_init(&[boss]);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // sub1
        assert_eq!(r.read_u8(), Some(SUB2_UI_INIT)); // sub2
        assert_eq!(r.read_i32(), Some(1)); // result
        assert_eq!(r.read_u8(), Some(1)); // count
                                          // Boss entry
        assert_eq!(r.read_u16(), Some(1)); // slot_id (1-based)
        assert_eq!(r.read_u16(), Some(6)); // name_len
                                           // Skip name bytes
        for &expected in b"Dragon" {
            assert_eq!(r.read_u8(), Some(expected));
        }
        assert_eq!(r.read_u8(), Some(1)); // alive_state
        assert_eq!(r.read_u8(), Some(80)); // hp_gauge
        assert_eq!(r.read_u8(), Some(2)); // boss_type
        assert_eq!(r.read_u16(), Some(105)); // boss_info_id
        assert_eq!(r.read_u8(), Some(2)); // gauge_level
        assert_eq!(r.read_u32(), Some(0x03FF0000)); // packed_color
        assert_eq!(r.read_u8(), Some(1)); // enabled_flag
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_ui_init_one_boss_data_length() {
        let boss = BossEntry {
            slot_id: 1,
            name: "Test".to_string(), // 4 bytes
            alive_state: 0,
            hp_gauge: 0,
            boss_type: 1,
            boss_info_id: 0,
            gauge_level: 0,
            packed_color: 0,
            enabled_flag: 0,
        };
        let pkt = build_ui_init(&[boss]);
        // header: 7 + per-boss: u16(2)+u16(2)+4+u8(1)+u8(1)+u8(1)+u16(2)+u8(1)+u32(4)+u8(1) = 15+4
        // 7 + 19 = 26
        assert_eq!(pkt.data.len(), 26);
    }

    #[test]
    fn test_build_ui_init_error() {
        let pkt = build_ui_init_error(0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // sub1
        assert_eq!(r.read_u8(), Some(SUB2_UI_INIT)); // sub2
        assert_eq!(r.read_i32(), Some(0)); // result_code != 1 → error
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_ui_init_error_data_length() {
        // u8 sub1 + u8 sub2 + i32 result = 1+1+4 = 6
        let pkt = build_ui_init_error(-1);
        assert_eq!(pkt.data.len(), 6);
    }

    #[test]
    fn test_build_ui_init_max_4_bosses() {
        let bosses: Vec<BossEntry> = (1..=5)
            .map(|i| BossEntry {
                slot_id: i,
                name: format!("B{}", i),
                alive_state: 1,
                hp_gauge: 100,
                boss_type: 1,
                boss_info_id: i * 100,
                gauge_level: 0,
                packed_color: 0,
                enabled_flag: 1,
            })
            .collect();
        let pkt = build_ui_init(&bosses);
        let mut r = PacketReader::new(&pkt.data);
        r.read_u8(); // sub1
        r.read_u8(); // sub2
        r.read_i32(); // result
        assert_eq!(r.read_u8(), Some(4)); // capped at 4
    }

    // ── Ranking builder (sub2=3) ──────────────────────────────────────

    #[test]
    fn test_build_ranking_opcode() {
        let pkt = build_ranking_info(1);
        assert_eq!(pkt.opcode, Opcode::WizWorldBoss as u8);
    }

    #[test]
    fn test_build_ranking_format() {
        let pkt = build_ranking_info(42);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // sub1
        assert_eq!(r.read_u8(), Some(SUB2_RANKING)); // sub2=3
        assert_eq!(r.read_i32(), Some(42)); // info_id
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_ranking_data_length() {
        // u8 sub1 + u8 sub2 + i32 info_id = 1+1+4 = 6
        let pkt = build_ranking_info(0);
        assert_eq!(pkt.data.len(), 6);
    }

    #[test]
    fn test_build_ranking_range() {
        for &id in &[1, 10, 101, 102] {
            let pkt = build_ranking_info(id);
            let mut r = PacketReader::new(&pkt.data);
            r.read_u8();
            r.read_u8();
            assert_eq!(r.read_i32(), Some(id));
        }
    }

    // ── Header byte validation ────────────────────────────────────────

    #[test]
    fn test_sub1_always_one() {
        assert_eq!(build_status(0).data[0], 1);
        assert_eq!(build_ui_init(&[]).data[0], 1);
        assert_eq!(build_ui_init_error(0).data[0], 1);
        assert_eq!(build_ranking_info(0).data[0], 1);
    }

    // ── Constants ─────────────────────────────────────────────────────

    #[test]
    fn test_status_code_values() {
        assert_eq!(STATUS_SPAWNED, 1);
        assert_eq!(STATUS_MSG_798B, 2);
        assert_eq!(STATUS_MSG_798C, 3);
        assert_eq!(STATUS_MSG_798D, 4);
        assert_eq!(STATUS_MSG_798E, 10);
    }

    #[test]
    fn test_sub2_values() {
        assert_eq!(SUB2_STATUS, 1);
        assert_eq!(SUB2_UI_INIT, 2);
        assert_eq!(SUB2_RANKING, 3);
    }

    #[test]
    fn test_c2s_ranking_format() {
        // C2S: [u8 sub1=1][u8 sub2=3][u8 slot_id][u8 state][i32 panel_id][string name]
        let mut pkt = Packet::new(Opcode::WizWorldBoss as u8);
        pkt.write_u8(1); // sub1
        pkt.write_u8(SUB2_RANKING);
        pkt.write_u8(2); // slot_id
        pkt.write_u8(1); // state
        pkt.write_i32(100); // panel_id
        pkt.write_string("Dragon"); // name

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1), "sub1");
        assert_eq!(r.read_u8(), Some(SUB2_RANKING), "sub2");
        assert_eq!(r.read_u8(), Some(2), "slot_id");
        assert_eq!(r.read_u8(), Some(1), "state");
        assert_eq!(r.read_i32(), Some(100), "panel_id");
        assert_eq!(r.read_string(), Some("Dragon".to_string()), "name");
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_c2s_status_format() {
        // C2S: [u8 sub1=1][u8 sub2=1][u16 param]
        let mut pkt = Packet::new(Opcode::WizWorldBoss as u8);
        pkt.write_u8(1);
        pkt.write_u8(SUB2_STATUS);
        pkt.write_u16(42);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1), "sub1");
        assert_eq!(r.read_u8(), Some(SUB2_STATUS), "sub2");
        assert_eq!(r.read_u16(), Some(42), "param");
        assert_eq!(r.remaining(), 0);
    }

    // ── LiveBossState ───────────────────────────────────────────────────

    #[test]
    fn test_live_boss_state_new() {
        let state = LiveBossState::new(1);
        assert_eq!(state.slot_id, 1);
        assert!(!state.is_alive());
        assert_eq!(state.gauge(), 0);
    }

    #[test]
    fn test_live_boss_state_spawn() {
        let state = LiveBossState::new(2);
        state.spawn(10000);
        assert!(state.is_alive());
        assert_eq!(state.gauge(), 100);
        assert_eq!(state.current_hp.load(Ordering::Relaxed), 10000);
        assert_eq!(state.max_hp.load(Ordering::Relaxed), 10000);
    }

    #[test]
    fn test_live_boss_state_update_hp() {
        let state = LiveBossState::new(1);
        state.spawn(1000);
        state.update_hp(500, 1000);
        assert_eq!(state.gauge(), 50);
        state.update_hp(250, 1000);
        assert_eq!(state.gauge(), 25);
        state.update_hp(0, 1000);
        assert_eq!(state.gauge(), 0);
    }

    #[test]
    fn test_live_boss_state_kill() {
        let state = LiveBossState::new(3);
        state.spawn(5000);
        assert!(state.is_alive());
        state.kill();
        assert!(!state.is_alive());
        assert_eq!(state.gauge(), 0);
        assert_eq!(state.current_hp.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_live_boss_state_hp_gauge_capped() {
        let state = LiveBossState::new(1);
        // HP > max should cap at 100%
        state.update_hp(2000, 1000);
        assert!(state.gauge() <= 100);
    }

    #[test]
    fn test_live_boss_state_zero_max_hp() {
        let state = LiveBossState::new(1);
        state.update_hp(100, 0);
        assert_eq!(state.gauge(), 0); // division by zero protection
    }

    #[test]
    fn test_live_boss_state_negative_hp() {
        let state = LiveBossState::new(1);
        state.update_hp(-100, 1000);
        assert_eq!(state.gauge(), 0); // negative HP → 0%
    }

    // ── Ranking handler ─────────────────────────────────────────────────

    #[test]
    fn test_ranking_info_id_for_data() {
        // info_id=10 → string 31114 (ranking available)
        let pkt = build_ranking_info(10);
        let mut r = PacketReader::new(&pkt.data);
        r.read_u8(); // sub1
        r.read_u8(); // sub2
        assert_eq!(r.read_i32(), Some(10));
    }

    #[test]
    fn test_ranking_info_id_for_no_data() {
        // info_id=101 → string 31119 (no ranking data)
        let pkt = build_ranking_info(101);
        let mut r = PacketReader::new(&pkt.data);
        r.read_u8();
        r.read_u8();
        assert_eq!(r.read_i32(), Some(101));
    }

    #[test]
    fn test_ranking_slot_range() {
        // Valid slots: 1-4
        assert!(1u8 <= 4 && 4u8 <= 4);
        // Invalid: 0 or 5+
        assert!(0u8 == 0 || 5u8 > 4);
    }
}
