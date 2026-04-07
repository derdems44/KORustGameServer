//! Core error types shared across all server crates.

use thiserror::Error;

/// Top-level error type for the ko-core crate.
#[derive(Debug, Error)]
pub enum CoreError {
    /// A generic internal error with a descriptive message.
    #[error("internal error: {0}")]
    Internal(String),
}
