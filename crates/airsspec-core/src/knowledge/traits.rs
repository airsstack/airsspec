//! Knowledge store, compressor, and vector store traits.
//!
//! This module defines core traits for knowledge management:
//! - **`KnowledgeStore`**: Document ingestion and query
//! - **`Compressor`**: Memory fragment compression
//! - **`VectorStore`**: Embedding storage and similarity search

// Layer 1: Standard library imports (none needed)

// Layer 2: Third-party crate imports
use async_trait::async_trait;

// Layer 3: Internal module imports
use crate::error::{KnowledgeError, MemoryError};
use crate::knowledge::types::{Document, Embedding, SearchResult};
use crate::memory::types::MemoryFragment;

/// Knowledge store trait for document management.
///
/// The knowledge store manages documents, supporting ingestion of new content
/// and query-based retrieval using semantic search. This is the primary interface
/// for Retrieval-Augmented Generation (RAG) patterns.
///
/// # Examples
///
/// Implementing a simple knowledge store:
///
/// ```ignore
/// use async_trait::async_trait;
/// use airsspec_core::knowledge::{KnowledgeStore, types::{Document, SearchResult}};
/// use airsspec_core::error::KnowledgeError;
/// use std::collections::HashMap;
/// use std::sync::{Arc, Mutex};
///
/// pub struct SimpleKnowledgeStore {
///     documents: Arc<Mutex<HashMap<String, Document>>>,
/// }
///
/// #[async_trait]
/// impl KnowledgeStore for SimpleKnowledgeStore {
///     async fn ingest(&mut self, doc: Document) -> Result<(), KnowledgeError> {
///         let mut documents = self.documents.lock().unwrap();
///         documents.insert(doc.id.clone(), doc);
///         Ok(())
///     }
///
///     async fn query(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, KnowledgeError> {
///         let documents = self.documents.lock().unwrap();
///         // Simple keyword matching (real implementation would use vector search)
///         let mut results: Vec<SearchResult> = documents
///             .values()
///             .filter(|doc| doc.content.contains(query))
///             .map(|doc| SearchResult {
///                 document_id: doc.id.clone(),
///                 score: 0.8,
///                 snippet: doc.content.chars().take(100).collect(),
///             })
///             .take(limit)
///             .collect();
///         results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
///         Ok(results)
///     }
/// }
/// ```
#[async_trait]
pub trait KnowledgeStore: Send + Sync {
    /// Ingest a document into the knowledge store.
    ///
    /// This method processes and stores a document, typically generating
    /// embeddings for vector search. The document becomes available for
    /// subsequent queries.
    ///
    /// # Arguments
    ///
    /// * `doc` - The document to ingest
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or a `KnowledgeError` on failure.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Document validation fails
    /// - Embedding generation fails
    /// - Storage operation fails
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use airsspec_core::knowledge::{KnowledgeStore, types::Document};
    ///
    /// # async fn example(mut store: impl KnowledgeStore) -> Result<(), KnowledgeError> {
    /// let doc = Document::new(
    ///     "doc-123".to_string(),
    ///     "AirsSpec is an AI-native development framework.".to_string()
    /// );
    /// store.ingest(doc).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn ingest(&mut self, doc: Document) -> Result<(), KnowledgeError>;

    /// Query the knowledge store for relevant documents.
    ///
    /// This method performs semantic search across stored documents,
    /// returning results ordered by relevance score. This is the core
    /// operation for Retrieval-Augmented Generation (RAG).
    ///
    /// # Arguments
    ///
    /// * `query` - The search query text
    /// * `limit` - Maximum number of results to return
    ///
    /// # Returns
    ///
    /// A vector of search results ordered by relevance score (highest first),
    /// or a `KnowledgeError` on failure.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Query parsing fails
    /// - Embedding generation fails
    /// - Search operation fails
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use airsspec_core::knowledge::{KnowledgeStore, error::KnowledgeError};
    ///
    /// # async fn example(store: &impl KnowledgeStore) -> Result<(), KnowledgeError> {
    /// let query = "What is AirsSpec?";
    /// let results = store.query(query, 5).await?;
    ///
    /// for result in results {
    ///     println!("{} (score: {:.2}): {}", result.document_id, result.score, result.snippet);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn query(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, KnowledgeError>;
}

/// Compressor trait for memory fragment compression.
///
/// The compressor takes a collection of memory fragments and produces a
/// compressed summary. This is used to manage token budgets by reducing
/// the size of conversation history stored in warm memory.
///
/// # Examples
///
/// Implementing a simple compressor:
///
/// ```ignore
/// use async_trait::async_trait;
/// use airsspec_core::knowledge::Compressor;
/// use airsspec_core::memory::{types::MemoryFragment, error::MemoryError};
///
/// pub struct SimpleCompressor;
///
/// #[async_trait]
/// impl Compressor for SimpleCompressor {
///     async fn compress(&self, fragments: Vec<MemoryFragment>) -> Result<String, MemoryError> {
///         if fragments.is_empty() {
///             return Ok(String::new());
///         }
///
///         // Simple concatenation (real implementation would use LLM summarization)
///         let summary = fragments.iter()
///             .map(|f| f.content.as_str())
///             .collect::<Vec<&str>>()
///             .join(" ");
///
///         Ok(summary)
///     }
/// }
/// ```
#[async_trait]
pub trait Compressor: Send + Sync {
    /// Compress a collection of memory fragments into a summary.
    ///
    /// This method takes fragments from hot memory and produces a compressed
    /// summary suitable for storage in warm memory. The compression ratio
    /// should follow the configured target ratio.
    ///
    /// # Arguments
    ///
    /// * `fragments` - The fragments to compress
    ///
    /// # Returns
    ///
    /// A compressed summary string, or a `MemoryError` on failure.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Compression operation fails
    /// - Token counting fails
    /// - Summary generation fails
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use airsspec_core::knowledge::Compressor;
    /// use airsspec_core::memory::types::MemoryFragment;
    ///
    /// # async fn example(compressor: &impl Compressor) -> Result<(), MemoryError> {
    /// let fragments = vec![
    ///     MemoryFragment::new("1".to_string(), "User said hello".to_string(), 5),
    ///     MemoryFragment::new("2".to_string(), "Assistant replied".to_string(), 5),
    /// ];
    ///
    /// let summary = compressor.compress(fragments).await?;
    /// println!("Compressed: {}", summary);
    /// # Ok(())
    /// # }
    /// ```
    async fn compress(&self, fragments: Vec<MemoryFragment>) -> Result<String, MemoryError>;
}

/// Vector store trait for embedding storage and similarity search.
///
/// The vector store manages embeddings for fast similarity search, typically
/// using vector databases or approximate nearest neighbor (ANN) indexes.
/// This is the underlying storage for semantic search operations.
///
/// # Examples
///
/// Implementing a simple in-memory vector store:
///
/// ```ignore
/// use async_trait::async_trait;
/// use airsspec_core::knowledge::{VectorStore, types::Embedding};
/// use airsspec_core::error::KnowledgeError;
/// use std::collections::HashMap;
/// use std::sync::{Arc, Mutex};
///
/// pub struct SimpleVectorStore {
///     embeddings: Arc<Mutex<HashMap<String, Embedding>>>,
/// }
///
/// #[async_trait]
/// impl VectorStore for SimpleVectorStore {
///     async fn upsert(&mut self, id: &str, embedding: Embedding) -> Result<(), KnowledgeError> {
///         let mut embeddings = self.embeddings.lock().unwrap();
///         embeddings.insert(id.to_string(), embedding);
///         Ok(())
///     }
///
///     async fn search(&self, query_embedding: Embedding, limit: usize)
///         -> Result<Vec<(String, f32)>, KnowledgeError>
///     {
///         let embeddings = self.embeddings.lock().unwrap();
///         let mut results: Vec<(String, f32)> = embeddings
///             .iter()
///             .map(|(id, emb)| {
///                 let score = query_embedding.cosine_similarity(emb);
///                 (id.clone(), score)
///             })
///             .filter(|(_, score)| *score > 0.0)
///             .collect();
///
///         results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
///         results.truncate(limit);
///         Ok(results)
///     }
/// }
/// ```
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// Upsert an embedding into the vector store.
    ///
    /// This method inserts or updates an embedding with the given ID.
    /// If an embedding with the same ID exists, it is replaced.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the embedding
    /// * `embedding` - The embedding to store
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or a `KnowledgeError` on failure.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Storage operation fails
    /// - Embedding validation fails
    /// - Index update fails
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use airsspec_core::knowledge::{VectorStore, types::Embedding};
    ///
    /// # async fn example(mut store: impl VectorStore) -> Result<(), KnowledgeError> {
    /// let embedding = Embedding::new(vec![0.1, 0.2, 0.3]);
    /// store.upsert("doc-123", embedding).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn upsert(&mut self, id: &str, embedding: Embedding) -> Result<(), KnowledgeError>;

    /// Search for similar embeddings.
    ///
    /// This method finds the most similar embeddings to the query embedding,
    /// returning IDs and similarity scores ordered by relevance (highest first).
    ///
    /// # Arguments
    ///
    /// * `query_embedding` - The query embedding to search with
    /// * `limit` - Maximum number of results to return
    ///
    /// # Returns
    ///
    /// A vector of (ID, score) tuples ordered by relevance (highest score first),
    /// or a `KnowledgeError` on failure.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Search operation fails
    /// - Similarity computation fails
    /// - Index access fails
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use airsspec_core::knowledge::{VectorStore, types::Embedding};
    ///
    /// # async fn example(store: &impl VectorStore) -> Result<(), KnowledgeError> {
    /// let query = Embedding::new(vec![0.1, 0.2, 0.3]);
    /// let results = store.search(query, 5).await?;
    ///
    /// for (id, score) in results {
    ///     println!("{} (score: {:.2})", id, score);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn search(
        &self,
        query_embedding: Embedding,
        limit: usize,
    ) -> Result<Vec<(String, f32)>, KnowledgeError>;
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::significant_drop_tightening,
    clippy::unused_async,
    clippy::cast_precision_loss,
    clippy::uninlined_format_args,
    clippy::float_cmp,
    clippy::type_complexity
)]
mod tests {
    use super::*;
    use crate::memory::types::MemoryFragment;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    // Mock KnowledgeStore implementation for testing
    struct MockKnowledgeStore {
        documents: Arc<Mutex<HashMap<String, Document>>>,
    }

    impl MockKnowledgeStore {
        fn new() -> Self {
            Self {
                documents: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    #[async_trait]
    impl KnowledgeStore for MockKnowledgeStore {
        async fn ingest(&mut self, doc: Document) -> Result<(), KnowledgeError> {
            let mut documents = self.documents.lock().unwrap();
            documents.insert(doc.id.clone(), doc);
            Ok(())
        }

        async fn query(
            &self,
            query: &str,
            limit: usize,
        ) -> Result<Vec<SearchResult>, KnowledgeError> {
            let documents = self.documents.lock().unwrap();
            let mut results: Vec<SearchResult> = documents
                .values()
                .filter(|doc| doc.content.contains(query))
                .map(|doc| SearchResult {
                    document_id: doc.id.clone(),
                    score: 0.9,
                    snippet: doc.content.chars().take(50).collect(),
                })
                .take(limit)
                .collect();

            results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            Ok(results)
        }
    }

    // Mock Compressor implementation for testing
    struct MockCompressor;

    #[async_trait]
    impl Compressor for MockCompressor {
        async fn compress(&self, fragments: Vec<MemoryFragment>) -> Result<String, MemoryError> {
            if fragments.is_empty() {
                return Ok(String::new());
            }

            // Simple concatenation for testing
            let summary = fragments
                .iter()
                .map(|f| f.content.as_str())
                .collect::<Vec<&str>>()
                .join(" | ");

            Ok(summary)
        }
    }

    // Mock VectorStore implementation for testing
    struct MockVectorStore {
        embeddings: Arc<Mutex<HashMap<String, Embedding>>>,
    }

    impl MockVectorStore {
        fn new() -> Self {
            Self {
                embeddings: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    #[async_trait]
    impl VectorStore for MockVectorStore {
        async fn upsert(&mut self, id: &str, embedding: Embedding) -> Result<(), KnowledgeError> {
            let mut embeddings = self.embeddings.lock().unwrap();
            embeddings.insert(id.to_string(), embedding);
            Ok(())
        }

        async fn search(
            &self,
            query_embedding: Embedding,
            limit: usize,
        ) -> Result<Vec<(String, f32)>, KnowledgeError> {
            let embeddings = self.embeddings.lock().unwrap();
            let mut results: Vec<(String, f32)> = embeddings
                .iter()
                .map(|(id, emb)| {
                    // Simple mock similarity: dot product (sufficient for tests)
                    let score = query_embedding
                        .vector
                        .iter()
                        .zip(emb.vector.iter())
                        .map(|(a, b)| a * b)
                        .sum::<f32>();
                    (id.clone(), score)
                })
                .filter(|(_, score)| *score > 0.0)
                .collect();

            results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            results.truncate(limit);
            Ok(results)
        }
    }

    #[tokio::test]
    async fn test_knowledge_store_ingest() {
        let mut store = MockKnowledgeStore::new();
        let doc = Document::new("doc-123".to_string(), "Test document content".to_string());

        let result = store.ingest(doc).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_knowledge_store_query() {
        let mut store = MockKnowledgeStore::new();

        store
            .ingest(Document::new(
                "doc-1".to_string(),
                "Rust is a systems programming language".to_string(),
            ))
            .await
            .unwrap();

        store
            .ingest(Document::new(
                "doc-2".to_string(),
                "Python is great for data science".to_string(),
            ))
            .await
            .unwrap();

        let results = store.query("Rust", 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].document_id, "doc-1");
    }

    #[tokio::test]
    async fn test_knowledge_store_query_limit() {
        let mut store = MockKnowledgeStore::new();

        for i in 0..10 {
            store
                .ingest(Document::new(
                    format!("doc-{}", i),
                    format!("Document {} with test content", i),
                ))
                .await
                .unwrap();
        }

        let results = store.query("test", 3).await.unwrap();
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_knowledge_store_query_empty() {
        let store = MockKnowledgeStore::new();
        let results = store.query("nonexistent", 10).await.unwrap();
        assert_eq!(results.len(), 0);
    }

    #[tokio::test]
    async fn test_compressor_compress() {
        let compressor = MockCompressor;

        let fragments = vec![
            MemoryFragment::new("1".to_string(), "User said hello".to_string(), 5),
            MemoryFragment::new("2".to_string(), "Assistant replied".to_string(), 5),
        ];

        let result = compressor.compress(fragments).await.unwrap();
        assert!(result.contains("User said hello"));
        assert!(result.contains("Assistant replied"));
    }

    #[tokio::test]
    async fn test_compressor_compress_empty() {
        let compressor = MockCompressor;
        let fragments = vec![];

        let result = compressor.compress(fragments).await.unwrap();
        assert_eq!(result, "");
    }

    #[tokio::test]
    async fn test_compressor_compress_single() {
        let compressor = MockCompressor;

        let fragments = vec![MemoryFragment::new(
            "1".to_string(),
            "Single fragment".to_string(),
            5,
        )];

        let result = compressor.compress(fragments).await.unwrap();
        assert_eq!(result, "Single fragment");
    }

    #[tokio::test]
    async fn test_vector_store_upsert() {
        let mut store = MockVectorStore::new();
        let embedding = Embedding::new(vec![0.1, 0.2, 0.3]);

        let result = store.upsert("doc-123", embedding).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_vector_store_search() {
        let mut store = MockVectorStore::new();

        let embedding1 = Embedding::new(vec![1.0, 0.0, 0.0]);
        let embedding2 = Embedding::new(vec![0.5, 0.5, 0.0]);

        store.upsert("doc-1", embedding1).await.unwrap();
        store.upsert("doc-2", embedding2).await.unwrap();

        let query = Embedding::new(vec![1.0, 0.0, 0.0]);
        let results = store.search(query, 10).await.unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "doc-1"); // Most similar
        assert!(results[0].1 > results[1].1);
    }

    #[tokio::test]
    async fn test_vector_store_search_limit() {
        let mut store = MockVectorStore::new();

        for i in 0..10 {
            let embedding = Embedding::new(vec![i as f32 / 10.0, 0.0, 0.0]);
            store
                .upsert(&format!("doc-{}", i), embedding)
                .await
                .unwrap();
        }

        let query = Embedding::new(vec![1.0, 0.0, 0.0]);
        let results = store.search(query, 3).await.unwrap();

        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_vector_store_search_empty() {
        let store = MockVectorStore::new();
        let query = Embedding::new(vec![1.0, 0.0, 0.0]);

        let results = store.search(query, 10).await.unwrap();
        assert_eq!(results.len(), 0);
    }

    #[tokio::test]
    async fn test_vector_store_upsert_overwrite() {
        let mut store = MockVectorStore::new();

        let embedding1 = Embedding::new(vec![1.0, 0.0, 0.0]);
        let embedding2 = Embedding::new(vec![0.0, 1.0, 0.0]);

        store.upsert("doc-1", embedding1).await.unwrap();
        store.upsert("doc-1", embedding2).await.unwrap();

        let query = Embedding::new(vec![0.0, 1.0, 0.0]);
        let results = store.search(query, 10).await.unwrap();

        assert_eq!(results.len(), 1);
        assert!((results[0].1 - 1.0).abs() < 0.001); // Updated embedding matches
    }

    #[tokio::test]
    async fn test_all_knowledge_traits_send_sync() {
        // This test verifies that the trait objects can be used with Send + Sync bounds
        fn assert_send_sync<T: Send + Sync>(_t: &T) {}

        let store = MockKnowledgeStore::new();
        let compressor = MockCompressor;
        let vec_store = MockVectorStore::new();

        assert_send_sync(&store);
        assert_send_sync(&compressor);
        assert_send_sync(&vec_store);
    }
}
