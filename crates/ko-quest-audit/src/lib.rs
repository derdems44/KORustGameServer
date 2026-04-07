//! Quest audit library — TBL data loading and Lua script parsing.
//!
//! Re-exports `tbl_loader` and `lua_parser` for use by other crates
//! (e.g., `ko-quest-gen`).

// Tool crate — many fields are kept for future use and debugging output.
#![allow(dead_code)]

pub mod lua_parser;
pub mod tbl_loader;
