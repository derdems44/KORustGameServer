//! Knight Online .tbl Table Importer
//!
//! Decrypts and imports .tbl files into PostgreSQL tables.
//! Table name in DB matches the .tbl filename (without extension, lowercased).
//!
//! ## Usage
//!
//! ```sh
//! # Import a single file:
//! ko-tbl-import --database-url postgresql://user:pass@host/db --file Data/Item_Ext_0.tbl
//!
//! # Import all .tbl files from a directory:
//! ko-tbl-import --database-url postgresql://user:pass@host/db --dir Data/
//!
//! # Dry-run (decrypt + parse only, no DB write):
//! ko-tbl-import --file Data/Item_Ext_0.tbl --dry-run
//!
//! # Export as SQL file instead of direct import:
//! ko-tbl-import --file Data/Item_Ext_0.tbl --sql-out /tmp/item_ext_0.sql
//! ```

mod decrypt;
mod parser;

use std::path::{Path, PathBuf};

use clap::Parser;
use sqlx::PgPool;
use tracing::{error, info};

use crate::parser::{CellValue, ColumnType, TblTable};

#[derive(Parser)]
#[command(name = "ko-tbl-import", about = "Import KO .tbl files into PostgreSQL")]
struct Cli {
    /// PostgreSQL connection string.
    #[arg(long, env = "DATABASE_URL")]
    database_url: Option<String>,

    /// Single .tbl file to import.
    #[arg(long)]
    file: Option<PathBuf>,

    /// Directory containing .tbl files to import (all *.tbl files).
    #[arg(long)]
    dir: Option<PathBuf>,

    /// Dry-run: decrypt and parse only, print schema — no DB writes.
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// Export SQL to file instead of direct DB import.
    #[arg(long)]
    sql_out: Option<PathBuf>,

    /// Drop existing table before import (default: skip if exists).
    #[arg(long, default_value_t = false)]
    drop_existing: bool,

    /// Schema prefix for table names (e.g. "tbl" → "tbl.item_ext_0").
    #[arg(long)]
    schema: Option<String>,

    /// Append entries from a TSV file (ID\tTEXT per line) to a .tbl file.
    /// Requires --file and --output.
    #[arg(long)]
    append: Option<PathBuf>,

    /// Output .tbl file path (used with --append).
    #[arg(long)]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    // Handle --append mode (patch a TBL file with new entries)
    if let Some(ref append_file) = cli.append {
        return handle_append(&cli, append_file);
    }

    // Collect files to process
    let files = collect_files(&cli)?;
    if files.is_empty() {
        anyhow::bail!("No .tbl files specified. Use --file or --dir.");
    }
    info!("Found {} .tbl file(s) to process", files.len());

    // Connect to DB if needed
    let pool = if !cli.dry_run && cli.sql_out.is_none() {
        let url = cli
            .database_url
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("--database-url or DATABASE_URL required for import"))?;
        let p = PgPool::connect(url).await?;
        info!("Connected to database");
        Some(p)
    } else {
        None
    };

    // SQL output file
    let mut sql_writer: Option<std::fs::File> = if let Some(ref path) = cli.sql_out {
        Some(std::fs::File::create(path)?)
    } else {
        None
    };

    let mut success = 0u32;
    let mut failed = 0u32;

    for file_path in &files {
        let file_name = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        let table_name = sanitize_table_name(file_name, cli.schema.as_deref());

        match process_file(file_path, &table_name, &cli, pool.as_ref(), &mut sql_writer).await {
            Ok(row_count) => {
                info!(
                    "[OK] {} -> {} ({} rows, {} cols)",
                    file_name, table_name, row_count.0, row_count.1
                );
                success += 1;
            }
            Err(e) => {
                error!("[FAIL] {} -> {}: {:#}", file_name, table_name, e);
                failed += 1;
            }
        }
    }

    info!(
        "Done: {} success, {} failed out of {} total",
        success,
        failed,
        files.len()
    );

    if let Some(pool) = pool {
        pool.close().await;
    }

    Ok(())
}

/// Append entries from a TSV file to an existing .tbl file.
///
/// TSV format: `ID\tTEXT` per line (lines starting with `#` are comments).
/// The output .tbl is re-encrypted with ChaosExpansion.
fn handle_append(cli: &Cli, append_file: &Path) -> anyhow::Result<()> {
    let tbl_path = cli
        .file
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("--file is required with --append"))?;
    let output_path = cli
        .output
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("--output is required with --append"))?;

    // Read and decrypt existing TBL
    let raw_data = std::fs::read(tbl_path)?;
    let (decrypted, new_structure) = decrypt::decrypt_tbl(&raw_data)?;
    let mut table = parser::parse_tbl(&decrypted, new_structure)?;

    info!(
        "Loaded {} with {} rows, {} columns",
        tbl_path.display(),
        table.rows.len(),
        table.columns.len()
    );

    // Collect existing IDs
    let mut existing_ids: std::collections::HashSet<u32> = std::collections::HashSet::new();
    for row in &table.rows {
        if let Some(CellValue::U32(id)) = row.first() {
            existing_ids.insert(*id);
        }
    }

    // Read TSV entries
    let tsv_content = std::fs::read_to_string(append_file)?;
    let mut added = 0u32;
    let mut skipped = 0u32;

    for line in tsv_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, '\t').collect();
        if parts.len() != 2 {
            error!("Invalid line (expected ID<TAB>TEXT): {}", line);
            continue;
        }

        let id: u32 = parts[0]
            .trim()
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid ID '{}': {}", parts[0], e))?;
        let text = parts[1].trim().to_string();

        if existing_ids.contains(&id) {
            skipped += 1;
            continue;
        }

        table
            .rows
            .push(vec![CellValue::U32(id), CellValue::Str(text)]);
        existing_ids.insert(id);
        added += 1;
    }

    info!(
        "Added {} entries, skipped {} (already exist)",
        added, skipped
    );

    // Sort rows by ID
    table.rows.sort_by_key(|row| {
        if let Some(CellValue::U32(id)) = row.first() {
            *id
        } else {
            0
        }
    });

    // Serialize and encrypt
    let plaintext = parser::serialize_tbl(&table, new_structure);
    let encrypted = decrypt::encrypt_tbl(&plaintext);

    std::fs::write(output_path, &encrypted)?;
    info!(
        "Wrote {} ({} rows, {} bytes)",
        output_path.display(),
        table.rows.len(),
        encrypted.len()
    );

    // Verify by re-reading
    let verify_raw = std::fs::read(output_path)?;
    let (verify_dec, verify_ns) = decrypt::decrypt_tbl(&verify_raw)?;
    let verify_table = parser::parse_tbl(&verify_dec, verify_ns)?;
    info!(
        "Verification: {} rows, {} columns — OK",
        verify_table.rows.len(),
        verify_table.columns.len()
    );

    Ok(())
}

fn collect_files(cli: &Cli) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if let Some(ref path) = cli.file {
        if !path.exists() {
            anyhow::bail!("File not found: {}", path.display());
        }
        files.push(path.clone());
    }

    if let Some(ref dir) = cli.dir {
        if !dir.is_dir() {
            anyhow::bail!("Not a directory: {}", dir.display());
        }
        let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e.eq_ignore_ascii_case("tbl"))
                    .unwrap_or(false)
            })
            .collect();
        entries.sort();
        files.extend(entries);
    }

    Ok(files)
}

fn sanitize_table_name(name: &str, schema: Option<&str>) -> String {
    // Lowercase, replace spaces/dashes with underscore
    let clean: String = name
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();

    if let Some(s) = schema {
        format!("{}.{}", s, clean)
    } else {
        clean
    }
}

async fn process_file(
    file_path: &Path,
    table_name: &str,
    cli: &Cli,
    pool: Option<&PgPool>,
    sql_writer: &mut Option<std::fs::File>,
) -> anyhow::Result<(usize, usize)> {
    // Read raw file
    let raw_data = std::fs::read(file_path)?;
    let enc_type = decrypt::detect_encryption(&raw_data);
    info!(
        "  Encryption: {:?}, size: {} bytes",
        enc_type,
        raw_data.len()
    );

    // Decrypt
    let (decrypted, new_structure) = decrypt::decrypt_tbl(&raw_data)?;

    // Parse
    let table = parser::parse_tbl(&decrypted, new_structure)?;
    let col_count = table.columns.len();
    let row_count = table.rows.len();

    info!("  Parsed: {} columns, {} rows", col_count, row_count);

    if cli.dry_run {
        // Print schema info
        for (i, col) in table.columns.iter().enumerate() {
            println!("  col[{}]: {:?} -> {}", i, col, col.pg_type());
        }
        if !table.rows.is_empty() {
            println!(
                "  First row: {:?}",
                table.rows[0]
                    .iter()
                    .map(|c| c.to_sql_literal())
                    .collect::<Vec<_>>()
            );
        }
        return Ok((row_count, col_count));
    }

    // Generate SQL
    let create_sql = generate_create_table(table_name, &table.columns, cli.drop_existing);
    let insert_sql = generate_inserts(table_name, &table);

    if let Some(ref mut writer) = sql_writer {
        use std::io::Write;
        writeln!(writer, "-- Table: {}", table_name)?;
        writeln!(writer, "{}", create_sql)?;
        for stmt in &insert_sql {
            writeln!(writer, "{}", stmt)?;
        }
        writeln!(writer)?;
        return Ok((row_count, col_count));
    }

    // Execute on DB
    if let Some(pool) = pool {
        // Drop + create
        if cli.drop_existing {
            sqlx::query(&format!("DROP TABLE IF EXISTS \"{}\"", table_name))
                .execute(pool)
                .await?;
        }
        sqlx::query(&create_sql).execute(pool).await?;

        // Batch insert (chunks of 100 rows to avoid query size limits)
        for stmt in &insert_sql {
            sqlx::query(stmt).execute(pool).await?;
        }
    }

    Ok((row_count, col_count))
}

fn generate_create_table(table_name: &str, columns: &[ColumnType], _drop_existing: bool) -> String {
    let mut sql = String::new();
    sql.push_str(&format!(
        "CREATE TABLE IF NOT EXISTS \"{}\" (\n",
        table_name
    ));
    for (i, col) in columns.iter().enumerate() {
        if i > 0 {
            sql.push_str(",\n");
        }
        sql.push_str(&format!("    \"col_{}\" {}", i, col.pg_type()));
    }
    sql.push_str("\n);");
    sql
}

fn generate_inserts(table_name: &str, table: &TblTable) -> Vec<String> {
    let mut stmts = Vec::new();
    let col_list: String = (0..table.columns.len())
        .map(|i| format!("\"col_{}\"", i))
        .collect::<Vec<_>>()
        .join(", ");

    // Batch 100 rows per INSERT
    for chunk in table.rows.chunks(100) {
        let mut sql = format!("INSERT INTO \"{}\" ({}) VALUES\n", table_name, col_list);
        for (ri, row) in chunk.iter().enumerate() {
            if ri > 0 {
                sql.push_str(",\n");
            }
            sql.push('(');
            for (ci, cell) in row.iter().enumerate() {
                if ci > 0 {
                    sql.push_str(", ");
                }
                sql.push_str(&cell.to_sql_literal());
            }
            sql.push(')');
        }
        sql.push(';');
        stmts.push(sql);
    }

    stmts
}
