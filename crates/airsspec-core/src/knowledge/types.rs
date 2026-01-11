//! Knowledge types for document storage, embeddings, and search results.
//!
//! This module defines the data structures used for knowledge management,
//! including documents, embeddings, and search results.

// Layer 1: Standard library imports
use std::fmt;

// Layer 2: Third-party crate imports
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::{Value, json};
// Layer 3: Internal module imports (none - this is a leaf module)

#[allow(unused_imports)]
/// A document stored in the knowledge base.
///
/// Documents are the fundamental units of stored knowledge, containing
/// content and optional metadata for indexing and retrieval.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique identifier for this document.
    pub id: String,

    /// The document content (text, markdown, etc.).
    pub content: String,

    /// Additional metadata for this document.
    ///
    /// This can include:
    /// - Source information (file path, URL, etc.)
    /// - Tags or categories
    /// - Timestamps
    /// - Custom fields for filtering
    pub metadata: Value,
}

impl Document {
    /// Creates a new document with empty metadata.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the document
    /// * `content` - The document content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::knowledge::types::Document;
    ///
    /// let doc = Document::new(
    ///     "doc-123".to_string(),
    ///     "Document content".to_string()
    /// );
    /// ```
    #[must_use]
    pub fn new(id: String, content: String) -> Self {
        Self {
            id,
            content,
            metadata: Value::Object(serde_json::Map::new()),
        }
    }

    /// Creates a new document with metadata.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the document
    /// * `content` - The document content
    /// * `metadata` - JSON metadata for the document
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::knowledge::types::Document;
    /// use serde_json::json;
    ///
    /// let metadata = json!({
    ///     "source": "README.md",
    ///     "category": "documentation"
    /// });
    ///
    /// let doc = Document::with_metadata(
    ///     "doc-123".to_string(),
    ///     "Document content".to_string(),
    ///     metadata
    /// );
    /// ```
    #[must_use]
    pub const fn with_metadata(id: String, content: String, metadata: Value) -> Self {
        Self {
            id,
            content,
            metadata,
        }
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Document(id={}, content_len={})",
            self.id,
            self.content.len()
        )
    }
}

/// A vector embedding for semantic search.
///
/// Embeddings represent text as high-dimensional vectors, enabling
/// semantic similarity comparisons. This is the foundation for RAG
/// (Retrieval-Augmented Generation) patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    /// The vector values.
    ///
    /// This is a list of floating-point numbers representing the embedding.
    /// The dimension (length of this vector) depends on the embedding model.
    pub vector: Vec<f32>,

    /// The number of dimensions in the embedding.
    pub dimensions: usize,
}

impl Embedding {
    /// Creates a new embedding from a vector.
    ///
    /// # Arguments
    ///
    /// * `vector` - The embedding vector
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::knowledge::types::Embedding;
    ///
    /// let vector = vec![0.1, 0.2, 0.3, 0.4];
    /// let embedding = Embedding::new(vector);
    /// assert_eq!(embedding.dimensions, 4);
    /// ```
    #[must_use]
    pub fn new(vector: Vec<f32>) -> Self {
        let dimensions = vector.len();
        Self { vector, dimensions }
    }

    /// Creates a new embedding with explicit dimension count.
    ///
    /// # Arguments
    ///
    /// * `vector` - The embedding vector
    /// * `dimensions` - The dimension count
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::knowledge::types::Embedding;
    ///
    /// let vector = vec![0.1, 0.2, 0.3, 0.4];
    /// let embedding = Embedding::with_dimensions(vector, 4);
    /// ```
    #[must_use]
    pub const fn with_dimensions(vector: Vec<f32>, dimensions: usize) -> Self {
        Self { vector, dimensions }
    }
}

/// A search result from semantic search.
///
/// Search results are returned from knowledge queries, ordered by
/// similarity score (highest first).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// The ID of the matching document.
    pub document_id: String,

    /// Similarity score (0.0 to 1.0).
    ///
    /// Higher values indicate more relevant results.
    pub score: f32,

    /// A snippet of the matching content.
    ///
    /// This provides a preview of the relevant content without
    /// needing to fetch the full document.
    pub snippet: String,
}

impl SearchResult {
    /// Creates a new search result.
    ///
    /// # Arguments
    ///
    /// * `document_id` - The ID of the matching document
    /// * `score` - Similarity score (0.0 to 1.0)
    /// * `snippet` - A preview of the matching content
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::knowledge::types::SearchResult;
    ///
    /// let result = SearchResult::new(
    ///     "doc-123".to_string(),
    ///     0.95,
    ///     "Relevant text snippet...".to_string()
    /// );
    /// ```
    #[must_use]
    pub const fn new(document_id: String, score: f32, snippet: String) -> Self {
        Self {
            document_id,
            score,
            snippet,
        }
    }
}

impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SearchResult({}, score={:.2}, snippet=\"{}...\")",
            self.document_id,
            self.score,
            self.snippet.chars().take(50).collect::<String>()
        )
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::float_cmp,
    clippy::uninlined_format_args,
    unused_imports
)]
mod tests {
    use super::*;

    #[test]
    fn test_document_new() {
        let doc = Document::new("doc-123".to_string(), "Test content".to_string());

        assert_eq!(doc.id, "doc-123");
        assert_eq!(doc.content, "Test content");
        assert_eq!(doc.metadata, Value::Object(serde_json::Map::new()));
    }

    #[test]
    fn test_document_with_metadata() {
        let metadata = json!({
            "source": "README.md",
            "category": "documentation",
            "tags": ["rust", "ai"]
        });

        let doc =
            Document::with_metadata("doc-123".to_string(), "Test content".to_string(), metadata);

        assert_eq!(doc.id, "doc-123");
        assert_eq!(doc.metadata["source"], "README.md");
        assert_eq!(doc.metadata["category"], "documentation");
        assert_eq!(doc.metadata["tags"][0], "rust");
    }

    #[test]
    fn test_document_display() {
        let doc = Document::new("doc-123".to_string(), "Test content".to_string());

        let display = doc.to_string();
        assert!(display.contains("doc-123"));
        assert!(display.contains("12")); // content length
    }

    #[test]
    fn test_embedding_new() {
        let vector = vec![0.1, 0.2, 0.3];
        let embedding = Embedding::new(vector.clone());

        assert_eq!(embedding.vector, vector);
        assert_eq!(embedding.dimensions, 3);
    }

    #[test]
    fn test_embedding_with_dimensions() {
        let vector = vec![0.1, 0.2, 0.3];
        let embedding = Embedding::with_dimensions(vector.clone(), 3);

        assert_eq!(embedding.vector, vector);
        assert_eq!(embedding.dimensions, 3);
    }

    #[test]
    fn test_search_result_new() {
        let result = SearchResult::new("doc-123".to_string(), 0.95, "Relevant snippet".to_string());

        assert_eq!(result.document_id, "doc-123");
        assert_eq!(result.score, 0.95);
        assert_eq!(result.snippet, "Relevant snippet");
    }

    #[test]
    fn test_search_result_display() {
        let result = SearchResult::new("doc-123".to_string(), 0.95, "Relevant snippet".to_string());

        let display = result.to_string();
        assert!(display.contains("doc-123"));
        assert!(display.contains("0.95"));
    }

    #[test]
    fn test_serialization_document() {
        let doc = Document::new("doc-123".to_string(), "Test content".to_string());

        let json = serde_json::to_string(&doc).unwrap();
        assert!(json.contains("doc-123"));

        let deserialized: Document = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, doc.id);
    }

    #[test]
    fn test_serialization_embedding() {
        let embedding = Embedding::new(vec![0.1, 0.2, 0.3]);

        let json = serde_json::to_string(&embedding).unwrap();
        assert!(json.contains("0.1"));

        let deserialized: Embedding = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.vector, embedding.vector);
    }

    #[test]
    fn test_serialization_search_result() {
        let result = SearchResult::new("doc-123".to_string(), 0.95, "Snippet".to_string());

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("doc-123"));

        let deserialized: SearchResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.document_id, result.document_id);
    }

    #[test]
    fn test_clone_document() {
        let doc = Document::new("doc-123".to_string(), "Test content".to_string());

        let cloned = doc.clone();
        assert_eq!(cloned.id, doc.id);
        assert_eq!(cloned.content, doc.content);
    }

    #[test]
    fn test_clone_embedding() {
        let embedding = Embedding::new(vec![0.1, 0.2, 0.3]);

        let cloned = embedding.clone();
        assert_eq!(cloned.vector, embedding.vector);
        assert_eq!(cloned.dimensions, embedding.dimensions);
    }

    #[test]
    fn test_clone_search_result() {
        let result = SearchResult::new("doc-123".to_string(), 0.95, "Snippet".to_string());

        let cloned = result.clone();
        assert_eq!(cloned.document_id, result.document_id);
        assert_eq!(cloned.score, result.score);
        assert_eq!(cloned.snippet, result.snippet);
    }
}
