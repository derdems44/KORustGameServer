//! Cross-reference TBL, Lua, and quest maps to find gaps and issues.

use std::collections::{HashMap, HashSet};

use crate::lua_parser::LuaFileData;
use crate::quest_map::NpcQuestMap;
use crate::tbl_loader::TblData;

/// A single audit finding.
#[derive(Debug, Clone)]
pub struct Finding {
    pub severity: Severity,
    pub category: Category,
    pub npc_id: i32,
    pub lua_file: String,
    pub detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "ERROR"),
            Self::Warning => write!(f, "WARN"),
            Self::Info => write!(f, "INFO"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    MissingLuaFile,
    MissingEventHandler,
    BrokenButton,
    MissingExchange,
    MissingQuestTalk,
    MissingQuestMenu,
    OrphanLuaFile,
    DeadNpc,
    MissingClientNpc,
    PortraitMismatch,
    MissingMonsterExchange,
    GiveNoSlotCheck,
    ExchangeIdMismatch,
    LuaPortraitWrong,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingLuaFile => write!(f, "MISSING_LUA"),
            Self::MissingEventHandler => write!(f, "MISSING_EVENT"),
            Self::BrokenButton => write!(f, "BROKEN_BUTTON"),
            Self::MissingExchange => write!(f, "MISSING_EXCHANGE"),
            Self::MissingQuestTalk => write!(f, "MISSING_TALK"),
            Self::MissingQuestMenu => write!(f, "MISSING_MENU"),
            Self::OrphanLuaFile => write!(f, "ORPHAN_LUA"),
            Self::DeadNpc => write!(f, "DEAD_NPC"),
            Self::MissingClientNpc => write!(f, "NO_CLIENT_NPC"),
            Self::PortraitMismatch => write!(f, "PORTRAIT_MISMATCH"),
            Self::MissingMonsterExchange => write!(f, "MISSING_MONSTER_EX"),
            Self::GiveNoSlotCheck => write!(f, "GIVE_NO_SLOT_CHK"),
            Self::ExchangeIdMismatch => write!(f, "EXCHANGE_MISMATCH"),
            Self::LuaPortraitWrong => write!(f, "LUA_PORTRAIT"),
        }
    }
}

/// Full audit report.
pub struct AuditReport {
    pub findings: Vec<Finding>,
    pub summary: AuditSummary,
}

/// Summary counts.
pub struct AuditSummary {
    pub total_npcs: usize,
    pub total_quests: usize,
    pub total_lua_files: usize,
    pub total_findings: usize,
    pub by_category: HashMap<Category, usize>,
    pub by_severity: HashMap<Severity, usize>,
}

/// Run a full audit.
pub fn run_audit(
    tbl: &TblData,
    lua_files: &HashMap<String, LuaFileData>,
    quest_maps: &[NpcQuestMap],
) -> AuditReport {
    let mut findings = Vec::new();

    // 1. Missing Lua files: quest_helper references a lua file that doesn't exist
    check_missing_lua_files(tbl, lua_files, &mut findings);

    // 2. Missing EVENT handlers: buttons point to events not in Lua
    check_missing_event_handlers(quest_maps, &mut findings);

    // 3. Broken buttons: buttons with event -1 or pointing to non-existent events
    check_broken_buttons(quest_maps, &mut findings);

    // 4. Missing item exchanges: RunQuestExchange references IDs not in TBL
    check_missing_exchanges(quest_maps, tbl, &mut findings);

    // 5. Missing quest_talk entries: dialog text IDs not in TBL
    check_missing_quest_talks(quest_maps, tbl, &mut findings);

    // 6. Missing quest_menu entries: button text IDs not in TBL
    check_missing_quest_menus(quest_maps, tbl, &mut findings);

    // 7. Orphan Lua files: files on disk but not referenced by quest_helper
    check_orphan_lua_files(tbl, lua_files, &mut findings);

    // 8. Dead NPCs: in quest_helper but not in client quest_npc_desc
    check_dead_npcs(quest_maps, &mut findings);

    // 9. Missing quest_helper but in client NPC desc
    check_missing_client_npcs(tbl, lua_files, &mut findings);

    // 10. Portrait mismatches
    check_portrait_mismatches(quest_maps, tbl, &mut findings);

    // 11. Missing monster exchange data
    check_missing_monster_exchanges(tbl, &mut findings);

    // 12. GiveItem without slot check
    check_give_no_slot_check(lua_files, &mut findings);

    // 13. Exchange ID mismatch (quest_helper vs Lua)
    check_exchange_id_mismatch(quest_maps, tbl, lua_files, &mut findings);

    // 14. Lua portrait mismatch (SelectMsg header vs quest_talk extra_1)
    check_lua_portrait_wrong(quest_maps, tbl, lua_files, &mut findings);

    // Sort by severity then NPC ID
    findings.sort_by(|a, b| {
        a.severity
            .cmp(&b.severity)
            .then(a.category.cmp(&b.category))
            .then(a.npc_id.cmp(&b.npc_id))
    });

    // Build summary
    let mut by_category: HashMap<Category, usize> = HashMap::new();
    let mut by_severity: HashMap<Severity, usize> = HashMap::new();
    for f in &findings {
        *by_category.entry(f.category).or_default() += 1;
        *by_severity.entry(f.severity).or_default() += 1;
    }

    let total_quests: usize = quest_maps.iter().map(|m| m.quest_ids.len()).sum();

    let summary = AuditSummary {
        total_npcs: quest_maps.len(),
        total_quests,
        total_lua_files: lua_files.len(),
        total_findings: findings.len(),
        by_category,
        by_severity,
    };

    AuditReport { findings, summary }
}

fn check_missing_lua_files(
    tbl: &TblData,
    lua_files: &HashMap<String, LuaFileData>,
    findings: &mut Vec<Finding>,
) {
    let mut seen = HashSet::new();
    for helper in &tbl.quest_helpers {
        let lua_key = helper.lua_filename.to_lowercase();
        if lua_key.is_empty() || seen.contains(&lua_key) {
            continue;
        }
        seen.insert(lua_key.clone());
        if !lua_files.contains_key(&lua_key) {
            findings.push(Finding {
                severity: Severity::Error,
                category: Category::MissingLuaFile,
                npc_id: helper.npc_id,
                lua_file: helper.lua_filename.clone(),
                detail: format!(
                    "quest_helper references '{}' but file not found (npc={}, quest={})",
                    helper.lua_filename, helper.npc_id, helper.event_data_index
                ),
            });
        }
    }
}

fn check_missing_event_handlers(quest_maps: &[NpcQuestMap], findings: &mut Vec<Finding>) {
    let mut seen = HashSet::new();
    for map in quest_maps {
        for helper in &map.helper_entries {
            // EVENT 1000 is SearchQuest dispatch — not a regular handler
            // Check event_trigger exists in Lua
            if helper.event_trigger > 0 && helper.event_trigger != 1000 {
                let key = (map.npc_id, helper.event_trigger, true);
                if seen.insert(key) {
                    if let Some(step) = map.steps.get(&helper.event_trigger) {
                        if !step.has_lua_handler {
                            findings.push(Finding {
                                severity: Severity::Error,
                                category: Category::MissingEventHandler,
                                npc_id: map.npc_id,
                                lua_file: map.lua_filename.clone(),
                                detail: format!(
                                    "EVENT {} (trigger for quest {}) has no Lua handler",
                                    helper.event_trigger, helper.event_data_index
                                ),
                            });
                        }
                    }
                }
            }
            // Check event_complete exists in Lua
            if helper.event_complete > 0 && helper.event_complete != 1000 {
                let key = (map.npc_id, helper.event_complete, false);
                if seen.insert(key) {
                    if let Some(step) = map.steps.get(&helper.event_complete) {
                        if !step.has_lua_handler {
                            findings.push(Finding {
                                severity: Severity::Warning,
                                category: Category::MissingEventHandler,
                                npc_id: map.npc_id,
                                lua_file: map.lua_filename.clone(),
                                detail: format!(
                                    "EVENT {} (complete for quest {}) has no Lua handler",
                                    helper.event_complete, helper.event_data_index
                                ),
                            });
                        }
                    }
                }
            }
        }
    }
}

fn check_broken_buttons(quest_maps: &[NpcQuestMap], findings: &mut Vec<Finding>) {
    for map in quest_maps {
        for step in map.steps.values() {
            for btn in &step.buttons {
                if btn.next_event <= 0 {
                    continue; // -1 = close dialog, valid
                }
                // Check if target event has a handler
                if !map.steps.contains_key(&btn.next_event) {
                    findings.push(Finding {
                        severity: Severity::Error,
                        category: Category::BrokenButton,
                        npc_id: map.npc_id,
                        lua_file: map.lua_filename.clone(),
                        detail: format!(
                            "Button '{}' (text_id={}) in EVENT {} points to EVENT {} which has no handler",
                            btn.text.as_deref().unwrap_or("?"),
                            btn.text_id,
                            step.event_id,
                            btn.next_event
                        ),
                    });
                }
            }
        }
    }
}

fn check_missing_exchanges(quest_maps: &[NpcQuestMap], tbl: &TblData, findings: &mut Vec<Finding>) {
    for map in quest_maps {
        for step in map.steps.values() {
            if let Some(ex_id) = step.exchange_id {
                if ex_id > 0 && !tbl.item_exchanges.contains_key(&ex_id) {
                    findings.push(Finding {
                        severity: Severity::Error,
                        category: Category::MissingExchange,
                        npc_id: map.npc_id,
                        lua_file: map.lua_filename.clone(),
                        detail: format!(
                            "EVENT {} references exchange_id={} which is not in Item_Exchange.tbl",
                            step.event_id, ex_id
                        ),
                    });
                }
            }
        }
    }
}

fn check_missing_quest_talks(
    quest_maps: &[NpcQuestMap],
    tbl: &TblData,
    findings: &mut Vec<Finding>,
) {
    let mut seen = HashSet::new();
    for map in quest_maps {
        for step in map.steps.values() {
            if step.dialog_text_id > 0 && !tbl.quest_talks.contains_key(&step.dialog_text_id) {
                let key = (map.npc_id, step.dialog_text_id);
                if seen.insert(key) {
                    findings.push(Finding {
                        severity: Severity::Warning,
                        category: Category::MissingQuestTalk,
                        npc_id: map.npc_id,
                        lua_file: map.lua_filename.clone(),
                        detail: format!(
                            "EVENT {} uses text_id={} not found in Quest_Talk",
                            step.event_id, step.dialog_text_id
                        ),
                    });
                }
            }
        }
    }
}

fn check_missing_quest_menus(
    quest_maps: &[NpcQuestMap],
    tbl: &TblData,
    findings: &mut Vec<Finding>,
) {
    let mut seen = HashSet::new();
    for map in quest_maps {
        for step in map.steps.values() {
            for btn in &step.buttons {
                if btn.text_id > 0
                    && btn.text.is_none()
                    && !tbl.quest_menus.contains_key(&btn.text_id)
                {
                    let key = (map.npc_id, btn.text_id);
                    if seen.insert(key) {
                        findings.push(Finding {
                            severity: Severity::Warning,
                            category: Category::MissingQuestMenu,
                            npc_id: map.npc_id,
                            lua_file: map.lua_filename.clone(),
                            detail: format!(
                                "Button text_id={} in EVENT {} not found in Quest_Menu",
                                btn.text_id, step.event_id
                            ),
                        });
                    }
                }
            }
        }
    }
}

fn check_orphan_lua_files(
    tbl: &TblData,
    lua_files: &HashMap<String, LuaFileData>,
    findings: &mut Vec<Finding>,
) {
    let referenced: HashSet<String> = tbl
        .quest_helpers
        .iter()
        .map(|h| h.lua_filename.to_lowercase())
        .collect();

    for (key, data) in lua_files {
        if !referenced.contains(key) {
            findings.push(Finding {
                severity: Severity::Info,
                category: Category::OrphanLuaFile,
                npc_id: data.npc_id,
                lua_file: data.filename.clone(),
                detail: format!(
                    "Lua file '{}' exists but is not referenced by any quest_helper entry",
                    data.filename
                ),
            });
        }
    }
}

fn check_dead_npcs(quest_maps: &[NpcQuestMap], findings: &mut Vec<Finding>) {
    for map in quest_maps {
        if !map.in_client_npc_desc {
            findings.push(Finding {
                severity: Severity::Info,
                category: Category::DeadNpc,
                npc_id: map.npc_id,
                lua_file: map.lua_filename.clone(),
                detail: format!(
                    "NPC {} ({}) is in quest_helper but NOT in client quest_npc_desc",
                    map.npc_id, map.npc_name
                ),
            });
        }
    }
}

fn check_missing_client_npcs(
    tbl: &TblData,
    lua_files: &HashMap<String, LuaFileData>,
    findings: &mut Vec<Finding>,
) {
    // NPCs in client quest_npc_desc but not in quest_helper TBL
    for npc_id in tbl.npc_descs.keys() {
        if !tbl.helpers_by_npc.contains_key(npc_id) {
            // Try to find a lua file for this NPC
            let lua_file = lua_files
                .values()
                .find(|d| d.npc_id == *npc_id)
                .map(|d| d.filename.clone())
                .unwrap_or_default();
            findings.push(Finding {
                severity: Severity::Info,
                category: Category::MissingClientNpc,
                npc_id: *npc_id,
                lua_file,
                detail: format!(
                    "NPC {} is in client quest_npc_desc but has no quest_helper entries",
                    npc_id
                ),
            });
        }
    }
}

fn check_portrait_mismatches(
    quest_maps: &[NpcQuestMap],
    tbl: &TblData,
    findings: &mut Vec<Finding>,
) {
    for map in quest_maps {
        if let Some(portrait_id) = map.client_portrait_text_id {
            // Check if any quest_talk entries for this NPC have mismatched tbl_extra_1
            for helper in &map.helper_entries {
                if helper.event_talk > 0 {
                    if let Some(talk) = tbl.quest_talks.get(&helper.event_talk) {
                        if talk.extra_1 > 0 && talk.extra_1 != portrait_id {
                            findings.push(Finding {
                                severity: Severity::Warning,
                                category: Category::PortraitMismatch,
                                npc_id: map.npc_id,
                                lua_file: map.lua_filename.clone(),
                                detail: format!(
                                    "quest_talk text_id={} has extra_1={} but npc_desc col_8={} (portrait mismatch)",
                                    talk.text_id, talk.extra_1, portrait_id
                                ),
                            });
                        }
                    }
                }
            }
        }
    }
}

fn check_missing_monster_exchanges(tbl: &TblData, findings: &mut Vec<Finding>) {
    // quest_helper entries with quest_type that implies monster kill but no monster_exchange
    for helper in &tbl.quest_helpers {
        // quest_type 2 or 3 typically means kill quest
        if (helper.quest_type == 2 || helper.quest_type == 3)
            && helper.event_data_index > 0
            && !tbl.monster_exchanges.contains_key(&helper.event_data_index)
        {
            findings.push(Finding {
                severity: Severity::Warning,
                category: Category::MissingMonsterExchange,
                npc_id: helper.npc_id,
                lua_file: helper.lua_filename.clone(),
                detail: format!(
                    "quest {} (type={}) has no monster_exchange entry",
                    helper.event_data_index, helper.quest_type
                ),
            });
        }
    }
}

fn check_give_no_slot_check(lua_files: &HashMap<String, LuaFileData>, findings: &mut Vec<Finding>) {
    for data in lua_files.values() {
        for handler in data.event_handlers.values() {
            // If handler has GiveItem but no CheckGiveSlot/isRoomForItem
            if !handler.give_items.is_empty() && !handler.has_slot_check {
                let items: Vec<String> = handler
                    .give_items
                    .iter()
                    .map(|gi| gi.item_id.to_string())
                    .collect();
                findings.push(Finding {
                    severity: Severity::Warning,
                    category: Category::GiveNoSlotCheck,
                    npc_id: data.npc_id,
                    lua_file: data.filename.clone(),
                    detail: format!(
                        "EVENT {} has GiveItem({}) but no CheckGiveSlot/isRoomForItem",
                        handler.event_id,
                        items.join(", ")
                    ),
                });
            }
        }
    }
}

fn check_exchange_id_mismatch(
    quest_maps: &[NpcQuestMap],
    _tbl: &TblData,
    lua_files: &HashMap<String, LuaFileData>,
    findings: &mut Vec<Finding>,
) {
    let mut seen = HashSet::new();
    for map in quest_maps {
        let lua_key = map.lua_filename.to_lowercase();
        let lua_data = match lua_files.get(&lua_key) {
            Some(d) => d,
            None => continue,
        };

        for helper in &map.helper_entries {
            if helper.exchange_index <= 0 {
                continue;
            }
            // Check if the event_trigger handler calls RunQuestExchange with a DIFFERENT ID
            if let Some(handler) = lua_data.event_handlers.get(&helper.event_trigger) {
                for ex in &handler.exchanges {
                    if ex.exchange_id != helper.exchange_index {
                        let key = (map.npc_id, helper.event_trigger, ex.exchange_id);
                        if seen.insert(key) {
                            findings.push(Finding {
                                severity: Severity::Error,
                                category: Category::ExchangeIdMismatch,
                                npc_id: map.npc_id,
                                lua_file: map.lua_filename.clone(),
                                detail: format!(
                                    "EVENT {} calls exchange {} but quest_helper expects exchange {} (quest={})",
                                    helper.event_trigger, ex.exchange_id, helper.exchange_index,
                                    helper.event_data_index
                                ),
                            });
                        }
                    }
                }
            }
        }
    }
}

fn check_lua_portrait_wrong(
    quest_maps: &[NpcQuestMap],
    tbl: &TblData,
    lua_files: &HashMap<String, LuaFileData>,
    findings: &mut Vec<Finding>,
) {
    let mut seen = HashSet::new();
    for map in quest_maps {
        // Get expected portrait from client npc_desc
        let expected_portrait = match map.client_portrait_text_id {
            Some(id) if id > 0 => id,
            _ => continue,
        };

        let lua_key = map.lua_filename.to_lowercase();
        let lua_data = match lua_files.get(&lua_key) {
            Some(d) => d,
            None => continue,
        };

        // Check all SelectMsg calls — vals[3] is the header text_id,
        // and quest_talk.extra_1 for that text_id should match portrait
        for handler in lua_data.event_handlers.values() {
            for sm in &handler.select_msgs {
                if sm.header_text_id <= 0 {
                    continue;
                }
                if let Some(talk) = tbl.quest_talks.get(&sm.header_text_id) {
                    if talk.extra_1 > 0 && talk.extra_1 != expected_portrait {
                        let key = (map.npc_id, sm.header_text_id);
                        if seen.insert(key) {
                            findings.push(Finding {
                                severity: Severity::Warning,
                                category: Category::LuaPortraitWrong,
                                npc_id: map.npc_id,
                                lua_file: map.lua_filename.clone(),
                                detail: format!(
                                    "SelectMsg text_id={} in EVENT {} has portrait={} but NPC expects {} (line {})",
                                    sm.header_text_id, handler.event_id,
                                    talk.extra_1, expected_portrait, sm.line_num
                                ),
                            });
                        }
                    }
                }
            }
        }
    }
}

/// Print the audit report to stdout.
pub fn print_report(report: &AuditReport, verbose: bool) {
    println!("\n{}", "=".repeat(80));
    println!("  QUEST AUDIT REPORT");
    println!("{}\n", "=".repeat(80));

    // Summary
    println!("SUMMARY:");
    println!("  NPCs audited:     {}", report.summary.total_npcs);
    println!("  Total quests:     {}", report.summary.total_quests);
    println!("  Lua files:        {}", report.summary.total_lua_files);
    println!("  Total findings:   {}", report.summary.total_findings);
    println!();

    // By severity
    println!("BY SEVERITY:");
    for sev in &[Severity::Error, Severity::Warning, Severity::Info] {
        let count = report.summary.by_severity.get(sev).copied().unwrap_or(0);
        if count > 0 {
            println!("  {:<8} {}", format!("{}:", sev), count);
        }
    }
    println!();

    // By category
    println!("BY CATEGORY:");
    let mut cats: Vec<_> = report.summary.by_category.iter().collect();
    cats.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
    for (cat, count) in &cats {
        println!("  {:<22} {}", format!("{}:", cat), count);
    }
    println!();

    // Findings
    if verbose {
        println!("{}", "-".repeat(80));
        println!("DETAILED FINDINGS:");
        println!("{}", "-".repeat(80));
        let mut current_cat = None;
        for f in &report.findings {
            if current_cat != Some(f.category) {
                current_cat = Some(f.category);
                println!("\n--- {} ---", f.category);
            }
            println!(
                "  [{:>5}] NPC {:>5} | {} | {}",
                f.severity, f.npc_id, f.lua_file, f.detail
            );
        }
    } else {
        // Compact: only errors and warnings
        let errors: Vec<_> = report
            .findings
            .iter()
            .filter(|f| f.severity == Severity::Error)
            .collect();
        let warnings: Vec<_> = report
            .findings
            .iter()
            .filter(|f| f.severity == Severity::Warning)
            .collect();

        if !errors.is_empty() {
            println!("{}", "-".repeat(80));
            println!("ERRORS ({}):", errors.len());
            println!("{}", "-".repeat(80));
            for f in &errors {
                println!(
                    "  NPC {:>5} | {:<30} | {} | {}",
                    f.npc_id, f.lua_file, f.category, f.detail
                );
            }
        }

        if !warnings.is_empty() {
            println!("\n{}", "-".repeat(80));
            println!("WARNINGS ({}):", warnings.len());
            println!("{}", "-".repeat(80));
            for f in &warnings {
                println!(
                    "  NPC {:>5} | {:<30} | {} | {}",
                    f.npc_id, f.lua_file, f.category, f.detail
                );
            }
        }

        let info_count = report
            .findings
            .iter()
            .filter(|f| f.severity == Severity::Info)
            .count();
        if info_count > 0 {
            println!(
                "\n  ({} INFO findings hidden, use --verbose to show)",
                info_count
            );
        }
    }

    println!();
}

/// Print the quest flow map for a specific NPC.
pub fn print_npc_quest_map(map: &NpcQuestMap) {
    println!("\n{}", "=".repeat(80));
    println!(
        "  NPC {} — {} ({})",
        map.npc_id, map.npc_name, map.lua_filename
    );
    println!(
        "  Client NPC desc: {} | Portrait: {}",
        if map.in_client_npc_desc { "YES" } else { "NO" },
        map.client_portrait_text_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "N/A".to_string()),
    );
    println!("  Quests: {:?}", map.quest_ids);
    println!("{}", "=".repeat(80));

    // Print quest_helper entries
    println!("\nQUEST_HELPER ENTRIES:");
    for h in &map.helper_entries {
        println!(
            "  index={} quest={} trigger={} complete={} exchange={} talk={} menu={} type={} status={}",
            h.index, h.event_data_index, h.event_trigger, h.event_complete,
            h.exchange_index, h.event_talk, h.quest_menu, h.quest_type, h.event_status
        );
    }

    // Print dialog flow
    println!("\nDIALOG FLOW:");
    let mut event_ids: Vec<i32> = map.steps.keys().copied().collect();
    event_ids.sort();

    for event_id in &event_ids {
        let step = &map.steps[event_id];
        let lua_marker = if step.has_lua_handler { "LUA" } else { "---" };
        println!("\n  EVENT {} [{}]:", event_id, lua_marker);

        if let Some(ref text) = step.dialog_text {
            let preview: String = text.chars().take(80).collect();
            println!("    Dialog (text_id={}): {}", step.dialog_text_id, preview);
        }

        for btn in &step.buttons {
            let text = btn.text.as_deref().unwrap_or("?");
            let target = if btn.next_event > 0 {
                format!("→ EVENT {}", btn.next_event)
            } else {
                "→ CLOSE".to_string()
            };
            println!("    [Button] text_id={} '{}' {}", btn.text_id, text, target);
        }

        if let Some(ref ex) = step.exchange_detail {
            println!(
                "    [Exchange] id={} random={}",
                ex.exchange_id, ex.random_flag
            );
            if !ex.required_items.is_empty() {
                let items: Vec<String> = ex
                    .required_items
                    .iter()
                    .map(|(id, c)| format!("{}x{}", id, c))
                    .collect();
                println!("      Required: {}", items.join(", "));
            }
            if !ex.reward_items.is_empty() {
                let items: Vec<String> = ex
                    .reward_items
                    .iter()
                    .map(|(id, c, t)| {
                        if *t > 0 {
                            format!("{}x{} ({}s)", id, c, t)
                        } else {
                            format!("{}x{}", id, c)
                        }
                    })
                    .collect();
                println!("      Rewards:  {}", items.join(", "));
            }
        } else if let Some(ex_id) = step.exchange_id {
            println!("    [Exchange] id={} NOT FOUND IN TBL!", ex_id);
        }

        if !step.save_events.is_empty() {
            println!("    [SaveEvent] {:?}", step.save_events);
        }
        if !step.give_items.is_empty() {
            let items: Vec<String> = step
                .give_items
                .iter()
                .map(|(id, c)| format!("{}x{}", id, c))
                .collect();
            println!("    [GiveItem] {}", items.join(", "));
        }
        if !step.rob_items.is_empty() {
            let items: Vec<String> = step
                .rob_items
                .iter()
                .map(|(id, c)| format!("{}x{}", id, c))
                .collect();
            println!("    [RobItem] {}", items.join(", "));
        }
    }

    // Monster requirements
    if !map.monster_requirements.is_empty() {
        println!("\nMONSTER KILL REQUIREMENTS:");
        for (qid, groups) in &map.monster_requirements {
            println!("  Quest {}:", qid);
            for (i, (mobs, count)) in groups.iter().enumerate() {
                let mob_strs: Vec<String> = mobs.iter().map(|m| m.to_string()).collect();
                println!(
                    "    Group {}: kill {}x [{}]",
                    i + 1,
                    count,
                    mob_strs.join("/")
                );
            }
        }
    }

    println!();
}
