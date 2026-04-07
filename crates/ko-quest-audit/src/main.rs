//! Quest audit tool — maps quest flows from TBL files and cross-references with Lua/DB.
//!
//! Usage:
//!   ko-quest-audit --data-dir <client_data> --quest-dir <lua_dir> [--npc <id>] [--verbose]
//!
//! Examples:
//!   # Full audit of all quests
//!   ko-quest-audit -d /path/to/Data -q /path/to/Quests
//!
//!   # Show quest map for specific NPC
//!   ko-quest-audit -d /path/to/Data -q /path/to/Quests --npc 18005
//!
//!   # Verbose audit (show INFO findings too)
//!   ko-quest-audit -d /path/to/Data -q /path/to/Quests --verbose

// Tool crate — many fields are kept for future use and debugging output.
#![allow(dead_code)]

mod audit;
mod lua_parser;
mod quest_map;
mod tbl_loader;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "ko-quest-audit", about = "Quest flow mapper and audit tool")]
struct Args {
    /// Client Data directory containing TBL files.
    #[arg(short = 'd', long, env = "KO_DATA_DIR")]
    data_dir: PathBuf,

    /// Quest Lua scripts directory.
    #[arg(short = 'q', long, env = "KO_QUEST_DIR")]
    quest_dir: PathBuf,

    /// Audit a specific NPC (show quest map). Can be repeated.
    #[arg(long)]
    npc: Vec<i32>,

    /// Show full quest map for all NPCs (not just audit).
    #[arg(long)]
    map_all: bool,

    /// Show detailed findings (including INFO level).
    #[arg(short, long)]
    verbose: bool,

    /// Only show specific categories (comma-separated).
    /// e.g., --category MISSING_LUA,BROKEN_BUTTON
    #[arg(long)]
    category: Option<String>,
}

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ko_quest_audit=info,ko_tbl_import=warn".parse().unwrap()),
        )
        .with_target(false)
        .init();

    let args = Args::parse();

    // Validate directories
    if !args.data_dir.is_dir() {
        anyhow::bail!("Data directory not found: {}", args.data_dir.display());
    }
    if !args.quest_dir.is_dir() {
        anyhow::bail!("Quest directory not found: {}", args.quest_dir.display());
    }

    // 1. Load TBL data
    tracing::info!("Loading TBL files from {}...", args.data_dir.display());
    let tbl = tbl_loader::TblData::load(&args.data_dir)?;

    // 2. Parse Lua files
    tracing::info!("Parsing Lua files from {}...", args.quest_dir.display());
    let lua_files = lua_parser::load_all_lua(&args.quest_dir)?;

    // 3. Build quest maps
    tracing::info!("Building quest flow maps...");
    let quest_maps = if args.npc.is_empty() {
        quest_map::build_all_quest_maps(&tbl, &lua_files)
    } else {
        let mut maps = Vec::new();
        for npc_id in &args.npc {
            if let Some(map) = quest_map::build_npc_quest_map(*npc_id, &tbl, &lua_files) {
                maps.push(map);
            } else {
                tracing::warn!(npc_id, "NPC not found in quest_helper");
            }
        }
        maps
    };

    // 4. If specific NPCs requested, show their quest maps
    if !args.npc.is_empty() {
        for map in &quest_maps {
            audit::print_npc_quest_map(map);
        }
    }

    // 5. If --map-all, show all quest maps
    if args.map_all {
        for map in &quest_maps {
            audit::print_npc_quest_map(map);
        }
    }

    // 6. Run audit
    tracing::info!("Running audit...");
    let mut report = audit::run_audit(&tbl, &lua_files, &quest_maps);

    // Filter by category if requested
    if let Some(ref cats) = args.category {
        let allowed: Vec<String> = cats.split(',').map(|s| s.trim().to_uppercase()).collect();
        report
            .findings
            .retain(|f| allowed.contains(&format!("{}", f.category)));
        // Recompute summary
        report.summary.total_findings = report.findings.len();
        report.summary.by_category.clear();
        report.summary.by_severity.clear();
        for f in &report.findings {
            *report.summary.by_category.entry(f.category).or_default() += 1;
            *report.summary.by_severity.entry(f.severity).or_default() += 1;
        }
    }

    // 7. Print report
    audit::print_report(&report, args.verbose);

    Ok(())
}
