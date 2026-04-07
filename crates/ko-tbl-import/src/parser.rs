//! .tbl binary table parser — reads column schema and row data from decrypted bytes.

use anyhow::{bail, Context};
use std::io::{Cursor, Read};

/// Column types matching C# ColumnTypes constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnType {
    SignedByte,    // 1
    UnsignedByte,  // 2
    SignedShort,   // 3
    UnsignedShort, // 4
    SignedInt,     // 5
    UnsignedInt,   // 6
    String,        // 7
    Float,         // 8
    Double,        // 9
    SignedLong,    // 10
    UnsignedLong,  // 11
}

impl ColumnType {
    fn from_i32(v: i32) -> anyhow::Result<Self> {
        match v {
            1 => Ok(Self::SignedByte),
            2 => Ok(Self::UnsignedByte),
            3 => Ok(Self::SignedShort),
            4 => Ok(Self::UnsignedShort),
            5 => Ok(Self::SignedInt),
            6 => Ok(Self::UnsignedInt),
            7 => Ok(Self::String),
            8 => Ok(Self::Float),
            9 => Ok(Self::Double),
            10 => Ok(Self::SignedLong),
            11 => Ok(Self::UnsignedLong),
            _ => bail!("Unknown column type: {}", v),
        }
    }

    /// PostgreSQL type name for this column type.
    pub fn pg_type(&self) -> &'static str {
        match self {
            Self::SignedByte => "SMALLINT",
            Self::UnsignedByte => "SMALLINT",
            Self::SignedShort => "SMALLINT",
            Self::UnsignedShort => "INTEGER",
            Self::SignedInt => "INTEGER",
            Self::UnsignedInt => "BIGINT",
            Self::String => "TEXT",
            Self::Float => "REAL",
            Self::Double => "DOUBLE PRECISION",
            Self::SignedLong => "BIGINT",
            Self::UnsignedLong => "NUMERIC(20,0)",
        }
    }
}

/// A single cell value.
#[derive(Debug, Clone)]
pub enum CellValue {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    Str(String),
    F32(f32),
    F64(f64),
    I64(i64),
    U64(u64),
}

impl CellValue {
    /// Format as SQL literal for INSERT.
    pub fn to_sql_literal(&self) -> String {
        match self {
            Self::I8(v) => v.to_string(),
            Self::U8(v) => v.to_string(),
            Self::I16(v) => v.to_string(),
            Self::U16(v) => v.to_string(),
            Self::I32(v) => v.to_string(),
            Self::U32(v) => v.to_string(),
            Self::Str(s) => format!("'{}'", s.replace('\'', "''")),
            Self::F32(v) => {
                if v.is_nan() || v.is_infinite() {
                    "0".to_string()
                } else {
                    v.to_string()
                }
            }
            Self::F64(v) => {
                if v.is_nan() || v.is_infinite() {
                    "0".to_string()
                } else {
                    v.to_string()
                }
            }
            Self::I64(v) => v.to_string(),
            Self::U64(v) => v.to_string(),
        }
    }
}

/// Parsed .tbl table.
pub struct TblTable {
    pub columns: Vec<ColumnType>,
    pub rows: Vec<Vec<CellValue>>,
}

/// Read a little-endian i32 from cursor.
fn read_i32(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<i32> {
    let mut buf = [0u8; 4];
    cursor.read_exact(&mut buf).context("read i32")?;
    Ok(i32::from_le_bytes(buf))
}

fn read_i16(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<i16> {
    let mut buf = [0u8; 2];
    cursor.read_exact(&mut buf).context("read i16")?;
    Ok(i16::from_le_bytes(buf))
}

fn read_u16(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<u16> {
    let mut buf = [0u8; 2];
    cursor.read_exact(&mut buf).context("read u16")?;
    Ok(u16::from_le_bytes(buf))
}

fn read_u32(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<u32> {
    let mut buf = [0u8; 4];
    cursor.read_exact(&mut buf).context("read u32")?;
    Ok(u32::from_le_bytes(buf))
}

fn read_i64(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<i64> {
    let mut buf = [0u8; 8];
    cursor.read_exact(&mut buf).context("read i64")?;
    Ok(i64::from_le_bytes(buf))
}

fn read_u64(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<u64> {
    let mut buf = [0u8; 8];
    cursor.read_exact(&mut buf).context("read u64")?;
    Ok(u64::from_le_bytes(buf))
}

fn read_f32(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<f32> {
    let mut buf = [0u8; 4];
    cursor.read_exact(&mut buf).context("read f32")?;
    Ok(f32::from_le_bytes(buf))
}

fn read_f64(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<f64> {
    let mut buf = [0u8; 8];
    cursor.read_exact(&mut buf).context("read f64")?;
    Ok(f64::from_le_bytes(buf))
}

fn read_i8(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<i8> {
    let mut buf = [0u8; 1];
    cursor.read_exact(&mut buf).context("read i8")?;
    Ok(buf[0] as i8)
}

fn read_u8(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<u8> {
    let mut buf = [0u8; 1];
    cursor.read_exact(&mut buf).context("read u8")?;
    Ok(buf[0])
}

/// Serialize a TblTable back into decrypted .tbl binary format.
///
/// `new_structure` — if true, prepend the 5-byte header used by ChaosExpansion tables.
pub fn serialize_tbl(table: &TblTable, new_structure: bool) -> Vec<u8> {
    let mut buf = Vec::new();

    // New-structure header (5 bytes)
    if new_structure {
        // UnknownInteger (4 bytes LE) — use 1 as placeholder
        buf.extend_from_slice(&1i32.to_le_bytes());
        // UnknownByte (1 byte)
        buf.push(0);
    }

    // Column count
    let col_count = table.columns.len() as i32;
    buf.extend_from_slice(&col_count.to_le_bytes());

    // Column types
    for col in &table.columns {
        let type_id: i32 = match col {
            ColumnType::SignedByte => 1,
            ColumnType::UnsignedByte => 2,
            ColumnType::SignedShort => 3,
            ColumnType::UnsignedShort => 4,
            ColumnType::SignedInt => 5,
            ColumnType::UnsignedInt => 6,
            ColumnType::String => 7,
            ColumnType::Float => 8,
            ColumnType::Double => 9,
            ColumnType::SignedLong => 10,
            ColumnType::UnsignedLong => 11,
        };
        buf.extend_from_slice(&type_id.to_le_bytes());
    }

    // Row count
    let row_count = table.rows.len() as i32;
    buf.extend_from_slice(&row_count.to_le_bytes());

    // Row data
    for row in &table.rows {
        for cell in row {
            match cell {
                CellValue::I8(v) => buf.push(*v as u8),
                CellValue::U8(v) => buf.push(*v),
                CellValue::I16(v) => buf.extend_from_slice(&v.to_le_bytes()),
                CellValue::U16(v) => buf.extend_from_slice(&v.to_le_bytes()),
                CellValue::I32(v) => buf.extend_from_slice(&v.to_le_bytes()),
                CellValue::U32(v) => buf.extend_from_slice(&v.to_le_bytes()),
                CellValue::Str(s) => {
                    // Encode to EUC-KR
                    let (encoded, _, _) = encoding_rs::EUC_KR.encode(s);
                    let bytes = encoded.as_ref();
                    let str_len = bytes.len() as i32;
                    buf.extend_from_slice(&str_len.to_le_bytes());
                    buf.extend_from_slice(bytes);
                }
                CellValue::F32(v) => buf.extend_from_slice(&v.to_le_bytes()),
                CellValue::F64(v) => buf.extend_from_slice(&v.to_le_bytes()),
                CellValue::I64(v) => buf.extend_from_slice(&v.to_le_bytes()),
                CellValue::U64(v) => buf.extend_from_slice(&v.to_le_bytes()),
            }
        }
    }

    buf
}

/// Parse decrypted .tbl bytes into a TblTable.
///
/// `new_structure` — if true, skip the 5-byte header (UnknownInteger + UnknownByte)
/// used by ChaosExpansion tables.
pub fn parse_tbl(data: &[u8], new_structure: bool) -> anyhow::Result<TblTable> {
    let mut cursor = Cursor::new(data);

    // Skip new-structure header
    if new_structure {
        let _unknown_int = read_i32(&mut cursor)?;
        let mut _unknown_byte = [0u8; 1];
        cursor.read_exact(&mut _unknown_byte)?;
    }

    // Read column schema
    let column_count = read_i32(&mut cursor)? as usize;
    if column_count == 0 || column_count > 1000 {
        bail!(
            "Invalid column count: {} (likely wrong encryption or corrupt file)",
            column_count
        );
    }

    let mut columns = Vec::with_capacity(column_count);
    for i in 0..column_count {
        let ct = read_i32(&mut cursor)?;
        columns.push(ColumnType::from_i32(ct).with_context(|| format!("column {}", i))?);
    }

    // Read rows
    let row_count = read_i32(&mut cursor)? as usize;
    if row_count > 10_000_000 {
        bail!("Suspicious row count: {} (likely corrupt)", row_count);
    }

    let mut rows = Vec::with_capacity(row_count);
    for r in 0..row_count {
        let mut row = Vec::with_capacity(column_count);
        for (c, col_type) in columns.iter().enumerate() {
            let cell = match col_type {
                ColumnType::SignedByte => CellValue::I8(
                    read_i8(&mut cursor).with_context(|| format!("row {}, col {} (i8)", r, c))?,
                ),
                ColumnType::UnsignedByte => CellValue::U8(
                    read_u8(&mut cursor).with_context(|| format!("row {}, col {} (u8)", r, c))?,
                ),
                ColumnType::SignedShort => CellValue::I16(
                    read_i16(&mut cursor).with_context(|| format!("row {}, col {} (i16)", r, c))?,
                ),
                ColumnType::UnsignedShort => CellValue::U16(
                    read_u16(&mut cursor).with_context(|| format!("row {}, col {} (u16)", r, c))?,
                ),
                ColumnType::SignedInt => CellValue::I32(
                    read_i32(&mut cursor).with_context(|| format!("row {}, col {} (i32)", r, c))?,
                ),
                ColumnType::UnsignedInt => CellValue::U32(
                    read_u32(&mut cursor).with_context(|| format!("row {}, col {} (u32)", r, c))?,
                ),
                ColumnType::String => {
                    let str_len = read_i32(&mut cursor)
                        .with_context(|| format!("row {}, col {} (string len)", r, c))?;
                    if str_len <= 0 {
                        CellValue::Str(String::new())
                    } else if str_len > 65536 {
                        bail!("String too long at row {}, col {}: {} bytes", r, c, str_len);
                    } else {
                        let mut buf = vec![0u8; str_len as usize];
                        cursor
                            .read_exact(&mut buf)
                            .with_context(|| format!("row {}, col {} (string data)", r, c))?;
                        // Try UTF-8 first; if invalid, try Windows-1254 (Turkish);
                        // fall back to EUC-KR (Korean).
                        let s = if let Ok(s) = std::str::from_utf8(&buf) {
                            s.to_string()
                        } else {
                            let (decoded_tr, _, had_errors_tr) =
                                encoding_rs::WINDOWS_1254.decode(&buf);
                            if !had_errors_tr {
                                decoded_tr.into_owned()
                            } else {
                                let (decoded_kr, _, _) = encoding_rs::EUC_KR.decode(&buf);
                                decoded_kr.into_owned()
                            }
                        };
                        CellValue::Str(s)
                    }
                }
                ColumnType::Float => CellValue::F32(
                    read_f32(&mut cursor).with_context(|| format!("row {}, col {} (f32)", r, c))?,
                ),
                ColumnType::Double => CellValue::F64(
                    read_f64(&mut cursor).with_context(|| format!("row {}, col {} (f64)", r, c))?,
                ),
                ColumnType::SignedLong => CellValue::I64(
                    read_i64(&mut cursor).with_context(|| format!("row {}, col {} (i64)", r, c))?,
                ),
                ColumnType::UnsignedLong => CellValue::U64(
                    read_u64(&mut cursor).with_context(|| format!("row {}, col {} (u64)", r, c))?,
                ),
            };
            row.push(cell);
        }
        rows.push(row);
    }

    Ok(TblTable { columns, rows })
}
