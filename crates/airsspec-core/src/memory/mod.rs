//! Memory tier traits and types.
//!
//! This module defines the abstraction layer for the three-tier memory system:
//! - **Hot memory**: Recent working context with high token limits
//! - **Warm memory**: Compressed summaries of completed conversations
//! - **Cold memory**: Vector embeddings for long-term knowledge retrieval

pub mod traits;
pub mod types;

// No type re-exports per project-standard.md §4.3
// Callers import types directly from the types module:
// use airsspec_core::memory::types::{MemoryFragment, CompressionConfig};
