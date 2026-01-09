# Rust Rig Integration Guidelines

**Date**: 2026-01-02
**Context**: Definitive Integration Guide for AirsSpec
**Status**: Consolidated & Authoritative

---

## 1. Philosophy & Architecture

`rig` is the **Cognitive Engine** for AirsSpec. It forces a separation of concerns that aligns with our architecture:
*   **The Brain**: `CompletionModel` (Stateless).
*   **The Actor**: `Agent` (Stateful Wrapper).
*   **The Hands**: `Tool` (Side-effects).
*   **The Eyes**: `Extractor` (Structured Parsing).

## 2. Global Provider Management (The "Registry" Pattern)

This is the core of our multi-provider strategy. We do not hardcode providers. We load them dynamically from configuration into a memory-safe Registry.

### 2.1 Configuration Schema
The user configures providers in `config.toml` (or environment variables).

```toml
[providers.primary]
service = "openai"
model = "gpt-4-turbo"
api_key = "env:OPENAI_API_KEY"

[providers.fast]
service = "anthropic"
model = "claude-3-haiku"
api_key = "env:ANTHROPIC_API_KEY"

[providers.local]
service = "ollama"
model = "llama3"
base_url = "http://localhost:11434"
```

### 2.2 The Registry Implementation
We use a `HashMap` of factory functions to instantiate specialized drivers while returning a boxed trait object `Box<dyn CompletionModel>`.

```rust
use std::collections::HashMap;
use rig::completion::CompletionModel;
use rig::providers::{openai, anthropic, azure, ollama};

/// A type-erased Model that can be used by any Agent
pub type DynModel = Box<dyn CompletionModel<Response = String>>;

pub struct ProviderRegistry {
    configs: HashMap<String, ProviderConfig>,
}

impl ProviderRegistry {
    /// Instantiates a provider by name
    pub async fn get_model(&self, provider_id: &str) -> Result<DynModel, ProviderError> {
        let config = self.configs.get(provider_id)
            .ok_or(ProviderError::NotFound(provider_id.to_string()))?;

        // Resolve API Key (Env Var vs Static)
        let token = resolve_key(&config.api_key)?;

        match config.service.as_str() {
            "openai" => {
                let client = openai::Client::new(&token);
                Ok(Box::new(client.model(&config.model)))
            },
            "anthropic" => {
                let client = anthropic::Client::new(&token);
                Ok(Box::new(client.model(&config.model)))
            },
             "ollama" => {
                let url = config.base_url.as_deref().unwrap_or("http://localhost:11434");
                let client = ollama::Client::from_url(url);
                Ok(Box::new(client.model(&config.model)))
            },
            "azure" => {
                // Azure requires both Key + Endpoint
                let endpoint = config.endpoint.as_ref()
                    .ok_or(ProviderError::ConfigMissing("endpoint".into()))?;
                let client = azure::Client::from_url(endpoint, &token);
                Ok(Box::new(client.model(&config.model)))
            },
            _ => Err(ProviderError::UnsupportedService(config.service.clone())),
        }
    }
}
```

## 3. The Extraction Engine ("Spec Parser")

The most critical feature for AirsSpec is the strict `Extractor`. We avoid regex parsing in favor of Type-Driven Semantic Parsing.

**Implementation**:
Rig uses `schemars` to generate JSON Schemas from Rust structs. The LLM is constrained to output JSON validation against this schema.

```rust
use rig::completion::Prompt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// 1. Define the Strict Schema
// This struct effectively becomes the "Prompt" for the LLM
#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct Requirement {
    pub id: String,         // e.g. "REQ-001"
    pub title: String,      // Short summary
    pub description: String,// Detailed user story
    pub critical: bool,     // Priority flag
    pub tags: Vec<String>,  // Taxonomy
}

// 2. The Extraction Call
async fn extract_requirements(agent: &Agent<impl CompletionModel>, text: &str) -> Result<Vec<Requirement>> {
    let output = agent.extractor::<Vec<Requirement>>()
        .extract(text)
        .await?;
    Ok(output)
}
```

## 4. Robust Tooling Implementation

We use the `rig-derive` crate to eliminate boilerplate, but we wrapping it in our own logic for safety (Confirmation & Security).

### 4.1 The Secure Pattern
Do NOT just implement `Tool` directly on unsafe code. Wrap it.

```rust
use rig_derive::Tool;

#[derive(Deserialize, JsonSchema)]
struct WriteFileArgs {
    path: String,
    content: String,
}

#[derive(Tool)]
#[tool(name = "write_file", description = "Safely writes content to a file. Fails if outside sandbox.")]
struct WriteFileTool {
    project_root: PathBuf,
}

impl Tool for WriteFileTool {
    type Args = WriteFileArgs;
    type Output = String;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition::from_args::<Self::Args>(self.name(), self.description())
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // 1. Security Check (Sandbox)
        let target_path = self.project_root.join(&args.path);
        let canonical_target = target_path.canonicalize().unwrap_or(target_path.clone());
        if !canonical_target.starts_with(&self.project_root) {
             return Err(ToolError::SecurityViolation("Path traversal attempt detected".into()));
        }

        // 2. Atomic Write
        tokio::fs::write(&target_path, &args.content).await
            .map_err(|e| ToolError::Io(e))?;
            
        Ok(format!("Successfully wrote {} bytes to {}", args.content.len(), args.path))
    }
}
```

## 5. RAG Pipeline (`rig-lancedb`) integration

We use `rig-lancedb` to persist knowledge embeddings. This runs in the background or during `airsspec learn`.

```rust
use rig_lancedb::{LanceDbVectorStore, SearchParams};

pub async fn build_rag_agent(
    model: DynModel, 
    embedding_model: Box<dyn EmbeddingModel>,
    db_path: PathBuf
) -> Result<Agent<DynModel>> {
    
    // 1. Connect to LanceDB (Local File)
    let db = rig_lancedb::connect(db_path.to_str().unwrap()).await?;
    
    // 2. Initialize Store with Embedding Model
    // This allows .add_documents() to auto-embed content
    let store = LanceDbVectorStore::new(
        db.open_table("knowledge_base").await?, 
        embedding_model
    );
    
    // 3. Create Retrieval Index
    // "top_k: 5" means we fetch the 5 most relevant snippets
    let index = store.index(SearchParams::default().top_k(5));

    // 4. Inject into Agent
    let agent = Agent::builder(model)
        .preamble("You are an expert architect. Use the context provided.")
        .dynamic_context(index) // <--- The magic happens here
        .build();
        
    Ok(agent)
}
```

## 6. Authentication (OAuth)

For providers requiring OAuth (Azure AD, Google Vertex), AirsSpec is responsible for the "Token Dance". Rig blindly accepts the token as a string.

1.  **Airsspec**: Calls `azure_identity::get_token()`.
2.  **Rig Integration**: 
    The `resolve_key` helper function in the Registry detects if the key starts with `azure-cli:` prefix.
    ```rust
    fn resolve_key(key_config: &str) -> Result<String> {
        if key_config.starts_with("env:") {
            std::env::var(key_config.strip_prefix("env:").unwrap())
                .map_err(|_| ProviderError::EnvVarMissing)
        } else if key_config == "auth:azure-cli" {
             // Call internal helper to run `az account get-access-token`
             internal::azure::get_cli_token().await
        } else {
            Ok(key_config.to_string()) // Raw key
        }
    }
    ```
