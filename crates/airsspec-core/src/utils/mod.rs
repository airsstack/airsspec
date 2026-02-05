//! # Utility Functions Module
//!
//! Pure utility functions used across the `AirsSpec` crate.
//!
//! ## Submodules
//!
//! - [`slug`] - URL-safe slug generation (ADR-003 compliant)
//! - [`id`] - Spec ID generation helpers
//!
//! ## Example
//!
//! ```
//! use airsspec_core::utils::{slug, id};
//!
//! // Generate a slug
//! let s = slug::generate("My Feature Title", 50);
//! assert_eq!(s, "my-feature-title");
//!
//! // Generate a spec ID
//! let spec_id = id::generate_spec_id("My Feature");
//! assert!(!spec_id.slug().is_empty());
//! ```

pub mod id;
pub mod slug;
