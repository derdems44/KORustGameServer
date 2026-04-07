//! Parses MSSQL pipe-delimited dump files for quest_helper and item_exchange.

use std::path::Path;

/// A quest_helper row from the MSSQL dump.
/// 19 pipe-delimited fields per line.
#[derive(Debug, Clone)]
pub struct MssqlQuestHelper {
    pub n_index: i32,
    pub b_message_type: i32,
    pub b_level: i32,
    pub n_exp: i32,
    pub b_class: i32,
    pub b_nation: i32,
    pub b_quest_type: i32,
    pub b_zone: i32,
    pub s_npc_id: i32,
    pub s_event_data_index: i32, // quest ID
    pub b_event_status: i32,
    pub n_event_trigger_index: i32,
    pub n_event_complete_index: i32,
    pub n_exchange_index: i32,
    pub n_event_talk_index: i32,
    pub str_lua_filename: String,
    pub s_quest_menu: i32,
    pub s_npc_main: i32,
    pub s_quest_solo: i32,
}

/// An item_exchange row from the MSSQL dump.
/// 27 pipe-delimited fields per line.
#[derive(Debug, Clone)]
pub struct MssqlItemExchange {
    pub index: i32,
    pub random_flag: i32,
    pub origin_items: Vec<(i32, i32)>,   // (item_id, count) × 5
    pub exchange_items: Vec<(i32, i32)>, // (item_id, count) × 5
}

/// Parse the MSSQL quest_helper dump file.
pub fn parse_quest_helper_dump(path: &Path) -> anyhow::Result<Vec<MssqlQuestHelper>> {
    let content = std::fs::read_to_string(path)?;
    let mut result = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() < 19 {
            tracing::warn!(
                line = line_num + 1,
                fields = fields.len(),
                "Skipping short line"
            );
            continue;
        }
        let row = MssqlQuestHelper {
            n_index: fields[0].parse().unwrap_or(0),
            b_message_type: fields[1].parse().unwrap_or(0),
            b_level: fields[2].parse().unwrap_or(0),
            n_exp: fields[3].parse().unwrap_or(0),
            b_class: fields[4].parse().unwrap_or(0),
            b_nation: fields[5].parse().unwrap_or(0),
            b_quest_type: fields[6].parse().unwrap_or(0),
            b_zone: fields[7].parse().unwrap_or(0),
            s_npc_id: fields[8].parse().unwrap_or(0),
            s_event_data_index: fields[9].parse().unwrap_or(0),
            s_quest_menu: fields[16].parse().unwrap_or(-1),
            s_npc_main: fields[17].parse().unwrap_or(0),
            s_quest_solo: fields[18].parse().unwrap_or(0),
            b_event_status: fields[10].parse().unwrap_or(0),
            n_event_trigger_index: fields[11].parse().unwrap_or(0),
            n_event_complete_index: fields[12].parse().unwrap_or(0),
            n_exchange_index: fields[13].parse().unwrap_or(0),
            n_event_talk_index: fields[14].parse().unwrap_or(0),
            str_lua_filename: fields[15].to_string(),
        };
        result.push(row);
    }

    tracing::info!(rows = result.len(), "MSSQL quest_helper parsed");
    Ok(result)
}

/// Parse the MSSQL item_exchange dump file.
pub fn parse_item_exchange_dump(
    path: &Path,
) -> anyhow::Result<std::collections::HashMap<i32, MssqlItemExchange>> {
    let content = std::fs::read_to_string(path)?;
    let mut result = std::collections::HashMap::new();

    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() < 27 {
            tracing::warn!(
                line = line_num + 1,
                fields = fields.len(),
                "Skipping short exchange line"
            );
            continue;
        }
        let index: i32 = fields[0].parse().unwrap_or(0);
        let random_flag: i32 = fields[1].parse().unwrap_or(0);

        let mut origin_items = Vec::new();
        for i in 0..5 {
            let item_id: i32 = fields[2 + i].parse().unwrap_or(0);
            let count: i32 = fields[7 + i].parse().unwrap_or(0);
            if item_id != 0 {
                origin_items.push((item_id, count));
            }
        }

        let mut exchange_items = Vec::new();
        for i in 0..5 {
            let item_id: i32 = fields[12 + i].parse().unwrap_or(0);
            let count: i32 = fields[17 + i].parse().unwrap_or(0);
            if item_id != 0 {
                exchange_items.push((item_id, count));
            }
        }

        result.insert(
            index,
            MssqlItemExchange {
                index,
                random_flag,
                origin_items,
                exchange_items,
            },
        );
    }

    tracing::info!(rows = result.len(), "MSSQL item_exchange parsed");
    Ok(result)
}
