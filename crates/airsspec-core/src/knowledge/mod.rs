//! Knowledge store traits and types.
//!
//! This module defines the abstraction layer for knowledge management,
//! including document storage, embeddings, and semantic search.

pub mod traits;
pub mod types;

// No type re-exports per project-standard.md §4.3
// Callers import types directly from the types module:
// use airsspec_core::knowledge::types::{Document, Embedding, SearchResult};
