//! Three-tier memory traits for cognitive memory management.
//!
//! This module defines the core traits for the `AirsSpec` memory system:
//! - **`HotMemory`**: Fast in-memory storage for recent working context
//! - **`WarmMemory`**: Compressed summaries of completed conversations
//! - **`ColdMemory`**: Vector embeddings for long-term knowledge retrieval
//!
//! These traits enable swapping between different memory implementations
//! while maintaining a consistent interface.

// Layer 1: Standard library imports (none needed)

// Layer 2: Third-party crate imports
use async_trait::async_trait;

// Layer 3: Internal module imports
use crate::error::MemoryError;
use crate::knowledge::types::SearchResult;
use crate::memory::types::MemoryFragment;

/// Hot memory trait for recent working context.
///
/// Hot memory stores the most recent fragments of a conversation for immediate
/// access. It operates in memory and provides fast push/retrieval operations.
/// When the token count exceeds the configured threshold, fragments should be
/// compressed and moved to warm memory.
///
/// # Examples
///
/// Implementing hot memory with a simple `Vec`:
///
/// ```ignore
/// use async_trait::async_trait;
/// use airsspec_core::memory::{HotMemory, types::MemoryFragment};
/// use std::sync::{Arc, Mutex};
///
/// pub struct SimpleHotMemory {
///     fragments: Arc<Mutex<Vec<MemoryFragment>>>,
/// }
///
/// #[async_trait]
/// impl HotMemory for SimpleHotMemory {
///     async fn push(&mut self, fragment: MemoryFragment) {
///         let mut fragments = self.fragments.lock().unwrap();
///         fragments.push(fragment);
///     }
///
///     async fn get_window(&self, limit: usize) -> Vec<MemoryFragment> {
///         let fragments = self.fragments.lock().unwrap();
///         fragments.iter()
///             .rev()
///             .take(limit)
///             .cloned()
///             .collect()
///     }
///
///     async fn token_count(&self) -> u32 {
///         let fragments = self.fragments.lock().unwrap();
///         fragments.iter().map(|f| f.token_count).sum()
///     }
///
///     async fn clear(&mut self) {
///         let mut fragments = self.fragments.lock().unwrap();
///         fragments.clear();
///     }
/// }
/// ```
#[async_trait]
pub trait HotMemory: Send + Sync {
    /// Push a new fragment into hot memory.
    ///
    /// This method adds a new fragment to the hot memory store. Implementations
    /// should consider token budget management and trigger compression when
    /// thresholds are exceeded.
    ///
    /// # Arguments
    ///
    /// * `fragment` - The memory fragment to add
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use airsspec_core::memory::{HotMemory, types::MemoryFragment};
    ///
    /// # async fn example(mut hot: impl HotMemory) {
    /// let fragment = MemoryFragment::new(
    ///     "frag-1".to_string(),
    ///     "User input".to_string(),
    ///     10
    /// );
    /// hot.push(fragment).await;
    /// # }
    /// ```
    async fn push(&mut self, fragment: MemoryFragment);

    /// Get a window of the most recent fragments.
    ///
    /// This retrieves the most recent fragments up to the specified limit,
    /// ordered from newest to oldest. This is useful for constructing context
    /// windows for LLM requests.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of fragments to return
    ///
    /// # Returns
    ///
    /// A vector of fragments, ordered from newest to oldest.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use airsspec_core::memory::HotMemory;
    ///
    /// # async fn example(hot: &impl HotMemory) {
    /// let window = hot.get_window(10).await;
    /// for fragment in window {
    ///     println!("{}: {}", fragment.id, fragment.content);
    /// }
    /// # }
    /// ```
    async fn get_window(&self, limit: usize) -> Vec<MemoryFragment>;

    /// Get the total token count in hot memory.
    ///
    /// This returns the sum of all fragment token counts, useful for budget
    /// management and determining when to compress.
    ///
    /// # Returns
    ///
    /// Total number of tokens stored in hot memory.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use airsspec_core::memory::HotMemory;
    ///
    /// # async fn example(hot: &impl HotMemory) {
    /// let count = hot.token_count().await;
    /// if count > 4000 {
    ///     println!("Time to compress!");
    /// }
    /// # }
    /// ```
    async fn token_count(&self) -> u32;

    /// Clear all fragments from hot memory.
    ///
    /// This removes all fragments from hot memory. Useful for starting a new
    /// conversation or clearing context.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use airsspec_core::memory::HotMemory;
    ///
    /// # async fn example(mut hot: impl HotMemory) {
    /// hot.clear().await;
    /// # }
    /// ```
    async fn clear(&mut self);
}

/// Warm memory trait for compressed conversation summaries.
///
/// Warm memory stores compressed summaries of completed conversations or
/// compressed fragments from hot memory. These summaries can be retrieved
/// and injected into the context when needed.
///
/// # Examples
///
/// Implementing warm memory with a simple `HashMap`:
///
/// ```ignore
/// use async_trait::async_trait;
/// use airsspec_core::memory::{WarmMemory, error::MemoryError};
/// use std::collections::HashMap;
/// use std::sync::{Arc, Mutex};
///
/// pub struct SimpleWarmMemory {
///     summaries: Arc<Mutex<HashMap<String, String>>>,
/// }
///
/// #[async_trait]
/// impl WarmMemory for SimpleWarmMemory {
///     async fn store(&mut self, summary: String) -> Result<String, MemoryError> {
///         let id = format!("summary-{}", uuid::Uuid::new_v4());
///         let mut summaries = self.summaries.lock().unwrap();
///         summaries.insert(id.clone(), summary);
///         Ok(id)
///     }
///
///     async fn retrieve(&self, id: &str) -> Result<String, MemoryError> {
///         let summaries = self.summaries.lock().unwrap();
///         summaries.get(id)
///             .cloned()
///             .ok_or_else(|| MemoryError::Warm(format!("Summary not found: {id}")))
///     }
/// }
/// ```
#[async_trait]
pub trait WarmMemory: Send + Sync {
    /// Store a compressed summary.
    ///
    /// This method stores a summary of compressed content, typically generated
    /// from hot memory fragments. The implementation should return a unique
    /// identifier that can be used to retrieve the summary later.
    ///
    /// # Arguments
    ///
    /// * `summary` - The compressed summary content
    ///
    /// # Returns
    ///
    /// A unique identifier for the stored summary, or a `MemoryError` on failure.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use airsspec_core::memory::{WarmMemory, error::MemoryError};
    ///
    /// # async fn example(mut warm: impl WarmMemory) -> Result<(), MemoryError> {
    /// let summary = "The user asked about X and we discussed Y.".to_string();
    /// let id = warm.store(summary).await?;
    /// println!("Stored summary with id: {}", id);
    /// # Ok(())
    /// # }
    /// ```
    async fn store(&mut self, summary: String) -> Result<String, MemoryError>;

    /// Retrieve a stored summary by ID.
    ///
    /// This method retrieves a previously stored summary by its identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the summary to retrieve
    ///
    /// # Returns
    ///
    /// The summary content, or a `MemoryError` if the ID is not found.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use airsspec_core::memory::{WarmMemory, error::MemoryError};
    ///
    /// # async fn example(warm: &impl WarmMemory) -> Result<(), MemoryError> {
    /// let id = "summary-123";
    /// let summary = warm.retrieve(id).await?;
    /// println!("Retrieved: {}", summary);
    /// # Ok(())
    /// # }
    /// ```
    async fn retrieve(&self, id: &str) -> Result<String, MemoryError>;
}

/// Cold memory trait for vector embeddings and semantic search.
///
/// Cold memory stores content as vector embeddings for long-term knowledge
/// retrieval. Content is indexed and can be searched using semantic similarity.
/// This is the foundation for Retrieval-Augmented Generation (RAG) patterns.
///
/// # Examples
///
/// Implementing cold memory with a vector store:
///
/// ```ignore
/// use async_trait::async_trait;
/// use airsspec_core::memory::{ColdMemory, error::MemoryError};
/// use airsspec_core::knowledge::types::SearchResult;
///
/// pub struct SimpleColdMemory {
///     // Vector store implementation
/// }
///
/// #[async_trait]
/// impl ColdMemory for SimpleColdMemory {
///     async fn index(&mut self, content: &str) -> Result<(), MemoryError> {
///         // Generate embedding and store
///         let embedding = generate_embedding(content)?;
///         self.vector_store.insert(embedding)?;
///         Ok(())
///     }
///
///     async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, MemoryError> {
///         // Generate query embedding and search
///         let embedding = generate_embedding(query)?;
///         let results = self.vector_store.similarity_search(embedding, limit)?;
///         Ok(results)
///     }
/// }
/// ```
#[async_trait]
pub trait ColdMemory: Send + Sync {
    /// Index content for semantic search.
    ///
    /// This method processes content and adds it to the cold memory index,
    /// typically by generating vector embeddings. The indexed content can then
    /// be retrieved via semantic search.
    ///
    /// # Arguments
    ///
    /// * `content` - The content to index
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or a `MemoryError` on failure.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use airsspec_core::memory::{ColdMemory, error::MemoryError};
    ///
    /// # async fn example(mut cold: impl ColdMemory) -> Result<(), MemoryError> {
    /// let content = "The project uses Rust and implements the AI-DLC pattern.";
    /// cold.index(content).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn index(&mut self, content: &str) -> Result<(), MemoryError>;

    /// Search indexed content by semantic similarity.
    ///
    /// This method performs a semantic search across indexed content, returning
    /// results ordered by similarity score. This enables retrieval of relevant
    /// context for RAG patterns.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query text
    /// * `limit` - Maximum number of results to return
    ///
    /// # Returns
    ///
    /// A vector of search results ordered by similarity score (highest first),
    /// or a `MemoryError` on failure.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use airsspec_core::memory::{ColdMemory, error::MemoryError};
    ///
    /// # async fn example(cold: &impl ColdMemory) -> Result<(), MemoryError> {
    /// let query = "What programming language does AirsSpec use?";
    /// let results = cold.search(query, 5).await?;
    /// for result in results {
    ///     println!("{} (score: {:.2}): {}", result.document_id, result.score, result.snippet);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, MemoryError>;
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::significant_drop_tightening,
    clippy::unused_async,
    clippy::uninlined_format_args,
    clippy::len_zero,
    clippy::type_complexity
)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // Mock HotMemory implementation for testing
    struct MockHotMemory {
        fragments: Arc<Mutex<Vec<MemoryFragment>>>,
    }

    impl MockHotMemory {
        fn new() -> Self {
            Self {
                fragments: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    #[async_trait]
    impl HotMemory for MockHotMemory {
        async fn push(&mut self, fragment: MemoryFragment) {
            let mut fragments = self.fragments.lock().unwrap();
            fragments.push(fragment);
        }

        async fn get_window(&self, limit: usize) -> Vec<MemoryFragment> {
            let fragments = self.fragments.lock().unwrap();
            fragments.iter().rev().take(limit).cloned().collect()
        }

        async fn token_count(&self) -> u32 {
            let fragments = self.fragments.lock().unwrap();
            fragments.iter().map(|f| f.token_count).sum()
        }

        async fn clear(&mut self) {
            let mut fragments = self.fragments.lock().unwrap();
            fragments.clear();
        }
    }

    // Mock WarmMemory implementation for testing
    struct MockWarmMemory {
        summaries: Arc<Mutex<std::collections::HashMap<String, String>>>,
        counter: Arc<Mutex<u32>>,
    }

    impl MockWarmMemory {
        fn new() -> Self {
            Self {
                summaries: Arc::new(Mutex::new(std::collections::HashMap::new())),
                counter: Arc::new(Mutex::new(0)),
            }
        }
    }

    #[async_trait]
    impl WarmMemory for MockWarmMemory {
        async fn store(&mut self, summary: String) -> Result<String, MemoryError> {
            let mut counter = self.counter.lock().unwrap();
            let id = format!("summary-{}", *counter);
            *counter += 1;
            let mut summaries = self.summaries.lock().unwrap();
            summaries.insert(id.clone(), summary);
            Ok(id)
        }

        async fn retrieve(&self, id: &str) -> Result<String, MemoryError> {
            let summaries = self.summaries.lock().unwrap();
            summaries
                .get(id)
                .cloned()
                .ok_or_else(|| MemoryError::Warm(format!("Summary not found: {id}")))
        }
    }

    // Mock ColdMemory implementation for testing
    struct MockColdMemory {
        documents: Arc<Mutex<Vec<(String, Vec<f32>)>>>,
    }

    impl MockColdMemory {
        fn new() -> Self {
            Self {
                documents: Arc::new(Mutex::new(Vec::new())),
            }
        }

        async fn mock_search_impl(
            &self,
            query: &str,
            limit: usize,
        ) -> Result<Vec<SearchResult>, MemoryError> {
            let documents = self.documents.lock().unwrap();
            let mut results: Vec<SearchResult> = documents
                .iter()
                .filter(|(_, embedding)| {
                    // Simple similarity check based on dimension size
                    !embedding.is_empty()
                })
                .take(limit)
                .map(|(id, _)| SearchResult {
                    document_id: id.clone(),
                    score: 0.95,
                    snippet: format!("Mock snippet for {id} (query: {query})"),
                })
                .collect();

            results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            Ok(results)
        }
    }

    #[async_trait]
    impl ColdMemory for MockColdMemory {
        async fn index(&mut self, content: &str) -> Result<(), MemoryError> {
            let id = format!("doc-{}", content.len());
            let embedding = vec![0.5; 10]; // Mock embedding
            let mut documents = self.documents.lock().unwrap();
            documents.push((id, embedding));
            Ok(())
        }

        async fn search(
            &self,
            query: &str,
            limit: usize,
        ) -> Result<Vec<SearchResult>, MemoryError> {
            self.mock_search_impl(query, limit).await
        }
    }

    #[tokio::test]
    async fn test_hot_memory_push() {
        let mut hot = MockHotMemory::new();
        let fragment = MemoryFragment::new("test-id".to_string(), "test content".to_string(), 10);

        hot.push(fragment).await;
        assert_eq!(hot.token_count().await, 10);
    }

    #[tokio::test]
    async fn test_hot_memory_get_window() {
        let mut hot = MockHotMemory::new();

        for i in 0..5 {
            let fragment = MemoryFragment::new(format!("frag-{}", i), format!("content-{}", i), 10);
            hot.push(fragment).await;
        }

        let window = hot.get_window(3).await;
        assert_eq!(window.len(), 3);
        assert_eq!(window[0].id, "frag-4"); // Most recent first
        assert_eq!(window[1].id, "frag-3");
        assert_eq!(window[2].id, "frag-2");
    }

    #[tokio::test]
    async fn test_hot_memory_token_count() {
        let mut hot = MockHotMemory::new();

        assert_eq!(hot.token_count().await, 0);

        hot.push(MemoryFragment::new("1".to_string(), "a".to_string(), 10))
            .await;
        hot.push(MemoryFragment::new("2".to_string(), "b".to_string(), 15))
            .await;
        hot.push(MemoryFragment::new("3".to_string(), "c".to_string(), 20))
            .await;

        assert_eq!(hot.token_count().await, 45);
    }

    #[tokio::test]
    async fn test_hot_memory_clear() {
        let mut hot = MockHotMemory::new();

        hot.push(MemoryFragment::new("1".to_string(), "a".to_string(), 10))
            .await;
        hot.push(MemoryFragment::new("2".to_string(), "b".to_string(), 15))
            .await;

        assert_eq!(hot.token_count().await, 25);

        hot.clear().await;

        assert_eq!(hot.token_count().await, 0);
        assert_eq!(hot.get_window(10).await.len(), 0);
    }

    #[tokio::test]
    async fn test_warm_memory_store() {
        let mut warm = MockWarmMemory::new();
        let summary = "Test summary content".to_string();

        let result = warm.store(summary.clone()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "summary-0");
    }

    #[tokio::test]
    async fn test_warm_memory_retrieve() {
        let mut warm = MockWarmMemory::new();
        let summary = "Test summary content".to_string();

        let id = warm.store(summary.clone()).await.unwrap();
        let retrieved = warm.retrieve(&id).await;

        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap(), "Test summary content");
    }

    #[tokio::test]
    async fn test_warm_memory_retrieve_not_found() {
        let warm = MockWarmMemory::new();

        let result = warm.retrieve("non-existent-id").await;
        assert!(result.is_err());

        match result {
            Err(MemoryError::Warm(msg)) => {
                assert!(msg.contains("Summary not found"));
            }
            Err(e) => panic!("Expected Warm error, got: {:?}", e),
            Ok(_) => panic!("Expected error"),
        }
    }

    #[tokio::test]
    async fn test_cold_memory_index() {
        let mut cold = MockColdMemory::new();
        let content = "Test content for indexing".to_string();

        let result = cold.index(&content).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cold_memory_search() {
        let mut cold = MockColdMemory::new();

        // Index some content
        cold.index("First document about Rust").await.unwrap();
        cold.index("Second document about AirsSpec").await.unwrap();

        // Search
        let results = cold.search("Rust", 5).await.unwrap();
        assert!(results.len() > 0);

        // Check result structure
        for result in results {
            assert!(!result.document_id.is_empty());
            assert!(result.score > 0.0);
            assert!(result.snippet.contains("Rust"));
        }
    }

    #[tokio::test]
    async fn test_cold_memory_search_limit() {
        let mut cold = MockColdMemory::new();

        // Index multiple documents
        for i in 0..10 {
            cold.index(&format!("Document {}", i)).await.unwrap();
        }

        // Search with limit
        let results = cold.search("test", 3).await.unwrap();
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_all_memory_traits_send_sync() {
        // This test verifies that the trait objects can be used with Send + Sync bounds
        fn assert_send_sync<T: Send + Sync>(_t: &T) {}

        let hot = MockHotMemory::new();
        let warm = MockWarmMemory::new();
        let cold = MockColdMemory::new();

        assert_send_sync(&hot);
        assert_send_sync(&warm);
        assert_send_sync(&cold);
    }
}
