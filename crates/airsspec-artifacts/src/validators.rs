//! Artifact validators using JSON Schema.
//!
//! This module provides JSON Schema-based validators for each artifact type.
//! Validators parse artifact frontmatter and validate it against a JSON Schema
//! definition, returning detailed error messages for any validation failures.

// Layer 1: Standard library imports
use std::path::Path;
use std::sync::Arc;

// Layer 2: Third-party crate imports
use async_trait::async_trait;
use jsonschema::Validator;
use serde_json::Value;

// Layer 3: Internal module imports
use airsspec_core::artifact::traits::{ArtifactStore, ArtifactValidator};
use airsspec_core::artifact::types::{ArtifactType, ValidationError, ValidationResult};
use airsspec_core::error::ArtifactError;

use crate::persistence::JsonlPersistence;

/// Load and compile a JSON schema from a string.
///
/// # Arguments
///
/// * `schema_str` - JSON schema as a string
///
/// # Returns
///
/// Compiled `Validator` instance
///
/// # Errors
///
/// Returns an `ArtifactError` if schema cannot be parsed or compiled.
fn load_schema(schema_str: &str) -> Result<Validator, ArtifactError> {
    let schema_json: Value = serde_json::from_str(schema_str)
        .map_err(|e| ArtifactError::Validation(format!("Failed to parse schema JSON: {e}")))?;

    jsonschema::validator_for(&schema_json)
        .map_err(|e| ArtifactError::Validation(format!("Failed to compile schema: {e}")))
}

/// Validate JSON value against a schema and convert errors.
///
/// # Arguments
///
/// * `schema` - The compiled JSON schema
/// * `instance` - The JSON value to validate
///
/// # Returns
///
/// A `ValidationResult` containing any errors found during validation.
fn validate_json(schema: &Validator, instance: &Value) -> ValidationResult {
    let errors: Vec<ValidationError> = schema
        .iter_errors(instance)
        .map(|error| {
            // Build a JSON pointer path from the instance location
            let field = error
                .instance_path
                .to_string()
                .trim_start_matches('/')
                .replace('/', ".");

            let message = format!("{:?}", error.kind);

            ValidationError::new(
                if field.is_empty() {
                    "root".to_string()
                } else {
                    field
                },
                message,
            )
        })
        .collect();

    if errors.is_empty() {
        ValidationResult::success()
    } else {
        ValidationResult::failure(errors)
    }
}

/// Validator for Requirements artifacts.
///
/// Validates the frontmatter of Requirements documents against their JSON Schema.
#[derive(Debug)]
pub struct RequirementsValidator {
    schema: Arc<Validator>,
}

impl RequirementsValidator {
    /// Creates a new `RequirementsValidator`.
    ///
    /// # Errors
    ///
    /// Returns an `ArtifactError` if the schema cannot be loaded.
    pub fn new() -> Result<Self, ArtifactError> {
        let schema = load_schema(include_str!("../schemas/requirements.schema.json"))?;
        Ok(Self {
            schema: Arc::new(schema),
        })
    }
}

#[async_trait]
impl ArtifactValidator for RequirementsValidator {
    fn artifact_type(&self) -> ArtifactType {
        ArtifactType::Requirements
    }

    async fn validate(&self, content: &str) -> ValidationResult {
        let Some(frontmatter_yaml) = JsonlPersistence::extract_frontmatter(content) else {
            return ValidationResult::failure(vec![ValidationError::new(
                "frontmatter",
                "No frontmatter found in content",
            )]);
        };

        let frontmatter_value: Value = match serde_yaml::from_str(&frontmatter_yaml) {
            Ok(val) => val,
            Err(e) => {
                return ValidationResult::failure(vec![ValidationError::new(
                    "frontmatter",
                    format!("Failed to parse frontmatter YAML: {e}"),
                )]);
            }
        };

        validate_json(&self.schema, &frontmatter_value)
    }

    async fn validate_file(&self, path: &Path) -> Result<ValidationResult, ArtifactError> {
        let store = JsonlPersistence::new();
        let content = store.read(path).await?;
        Ok(self.validate(&content).await)
    }
}

/// Validator for DAA (Domain Architecture Analysis) artifacts.
///
/// Validates the frontmatter of DAA documents against their JSON Schema.
#[derive(Debug)]
pub struct DaaValidator {
    schema: Arc<Validator>,
}

impl DaaValidator {
    /// Creates a new `DaaValidator`.
    ///
    /// # Errors
    ///
    /// Returns an `ArtifactError` if the schema cannot be loaded.
    pub fn new() -> Result<Self, ArtifactError> {
        let schema = load_schema(include_str!("../schemas/daa.schema.json"))?;
        Ok(Self {
            schema: Arc::new(schema),
        })
    }
}

#[async_trait]
impl ArtifactValidator for DaaValidator {
    fn artifact_type(&self) -> ArtifactType {
        ArtifactType::Daa
    }

    async fn validate(&self, content: &str) -> ValidationResult {
        let Some(frontmatter_yaml) = JsonlPersistence::extract_frontmatter(content) else {
            return ValidationResult::failure(vec![ValidationError::new(
                "frontmatter",
                "No frontmatter found in content",
            )]);
        };

        let frontmatter_value: Value = match serde_yaml::from_str(&frontmatter_yaml) {
            Ok(val) => val,
            Err(e) => {
                return ValidationResult::failure(vec![ValidationError::new(
                    "frontmatter",
                    format!("Failed to parse frontmatter YAML: {e}"),
                )]);
            }
        };

        validate_json(&self.schema, &frontmatter_value)
    }

    async fn validate_file(&self, path: &Path) -> Result<ValidationResult, ArtifactError> {
        let store = JsonlPersistence::new();
        let content = store.read(path).await?;
        Ok(self.validate(&content).await)
    }
}

/// Validator for ADR (Architecture Decision Record) artifacts.
///
/// Validates the frontmatter of ADR documents against their JSON Schema.
#[derive(Debug)]
pub struct AdrValidator {
    schema: Arc<Validator>,
}

impl AdrValidator {
    /// Creates a new `AdrValidator`.
    ///
    /// # Errors
    ///
    /// Returns an `ArtifactError` if the schema cannot be loaded.
    pub fn new() -> Result<Self, ArtifactError> {
        let schema = load_schema(include_str!("../schemas/adr.schema.json"))?;
        Ok(Self {
            schema: Arc::new(schema),
        })
    }
}

#[async_trait]
impl ArtifactValidator for AdrValidator {
    fn artifact_type(&self) -> ArtifactType {
        ArtifactType::Adr
    }

    async fn validate(&self, content: &str) -> ValidationResult {
        let Some(frontmatter_yaml) = JsonlPersistence::extract_frontmatter(content) else {
            return ValidationResult::failure(vec![ValidationError::new(
                "frontmatter",
                "No frontmatter found in content",
            )]);
        };

        let frontmatter_value: Value = match serde_yaml::from_str(&frontmatter_yaml) {
            Ok(val) => val,
            Err(e) => {
                return ValidationResult::failure(vec![ValidationError::new(
                    "frontmatter",
                    format!("Failed to parse frontmatter YAML: {e}"),
                )]);
            }
        };

        validate_json(&self.schema, &frontmatter_value)
    }

    async fn validate_file(&self, path: &Path) -> Result<ValidationResult, ArtifactError> {
        let store = JsonlPersistence::new();
        let content = store.read(path).await?;
        Ok(self.validate(&content).await)
    }
}

/// Validator for RFC (Request for Comments) artifacts.
///
/// Validates the frontmatter of RFC documents against their JSON Schema.
#[derive(Debug)]
pub struct RfcValidator {
    schema: Arc<Validator>,
}

impl RfcValidator {
    /// Creates a new `RfcValidator`.
    ///
    /// # Errors
    ///
    /// Returns an `ArtifactError` if the schema cannot be loaded.
    pub fn new() -> Result<Self, ArtifactError> {
        let schema = load_schema(include_str!("../schemas/rfc.schema.json"))?;
        Ok(Self {
            schema: Arc::new(schema),
        })
    }
}

#[async_trait]
impl ArtifactValidator for RfcValidator {
    fn artifact_type(&self) -> ArtifactType {
        ArtifactType::Rfc
    }

    async fn validate(&self, content: &str) -> ValidationResult {
        let Some(frontmatter_yaml) = JsonlPersistence::extract_frontmatter(content) else {
            return ValidationResult::failure(vec![ValidationError::new(
                "frontmatter",
                "No frontmatter found in content",
            )]);
        };

        let frontmatter_value: Value = match serde_yaml::from_str(&frontmatter_yaml) {
            Ok(val) => val,
            Err(e) => {
                return ValidationResult::failure(vec![ValidationError::new(
                    "frontmatter",
                    format!("Failed to parse frontmatter YAML: {e}"),
                )]);
            }
        };

        validate_json(&self.schema, &frontmatter_value)
    }

    async fn validate_file(&self, path: &Path) -> Result<ValidationResult, ArtifactError> {
        let store = JsonlPersistence::new();
        let content = store.read(path).await?;
        Ok(self.validate(&content).await)
    }
}

/// Validator for Bolt Plan artifacts.
///
/// Validates the frontmatter of Bolt Plan documents against their JSON Schema.
#[derive(Debug)]
pub struct BoltPlanValidator {
    schema: Arc<Validator>,
}

impl BoltPlanValidator {
    /// Creates a new `BoltPlanValidator`.
    ///
    /// # Errors
    ///
    /// Returns an `ArtifactError` if the schema cannot be loaded.
    pub fn new() -> Result<Self, ArtifactError> {
        let schema = load_schema(include_str!("../schemas/bolt-plan.schema.json"))?;
        Ok(Self {
            schema: Arc::new(schema),
        })
    }
}

#[async_trait]
impl ArtifactValidator for BoltPlanValidator {
    fn artifact_type(&self) -> ArtifactType {
        ArtifactType::BoltPlan
    }

    async fn validate(&self, content: &str) -> ValidationResult {
        let Some(frontmatter_yaml) = JsonlPersistence::extract_frontmatter(content) else {
            return ValidationResult::failure(vec![ValidationError::new(
                "frontmatter",
                "No frontmatter found in content",
            )]);
        };

        let frontmatter_value: Value = match serde_yaml::from_str(&frontmatter_yaml) {
            Ok(val) => val,
            Err(e) => {
                return ValidationResult::failure(vec![ValidationError::new(
                    "frontmatter",
                    format!("Failed to parse frontmatter YAML: {e}"),
                )]);
            }
        };

        validate_json(&self.schema, &frontmatter_value)
    }

    async fn validate_file(&self, path: &Path) -> Result<ValidationResult, ArtifactError> {
        let store = JsonlPersistence::new();
        let content = store.read(path).await?;
        Ok(self.validate(&content).await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a validator for each type
    #[allow(clippy::unwrap_used)]
    fn get_validators() -> Vec<Box<dyn ArtifactValidator>> {
        vec![
            Box::new(RequirementsValidator::new().unwrap()) as Box<dyn ArtifactValidator>,
            Box::new(DaaValidator::new().unwrap()) as Box<dyn ArtifactValidator>,
            Box::new(AdrValidator::new().unwrap()) as Box<dyn ArtifactValidator>,
            Box::new(RfcValidator::new().unwrap()) as Box<dyn ArtifactValidator>,
            Box::new(BoltPlanValidator::new().unwrap()) as Box<dyn ArtifactValidator>,
        ]
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_requirements_validator_valid() {
        let validator = RequirementsValidator::new().unwrap();
        let content = r#"---
id: UOW-001
title: Foundation Layer
version: "1.0"
status: draft
author: airsspec-orchestrator
created_at: 2026-01-10
priority: high
phase: Research
---

# Requirements

Test content.
"#;

        let result = validator.validate(content).await;
        assert!(
            result.valid,
            "Validation should pass for valid requirements: {result:?}"
        );
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_requirements_validator_missing_required_field() {
        let validator = RequirementsValidator::new().unwrap();
        let content = r#"---
id: UOW-001
title: Foundation Layer
version: "1.0"
---

# Requirements

Test content.
"#;

        let result = validator.validate(content).await;
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_daa_validator_valid() {
        let validator = DaaValidator::new().unwrap();
        let content = r#"---
id: DAA-UOW-001
title: Domain Architecture Analysis
version: "1.0"
status: draft
author: airsspec-architect
created_at: 2026-01-10
uow_ref: UOW-001-foundation
---

# DAA

Test content.
"#;

        let result = validator.validate(content).await;
        assert!(
            result.valid,
            "Validation should pass for valid DAA: {result:?}"
        );
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_daa_validator_missing_required_field() {
        let validator = DaaValidator::new().unwrap();
        let content = r"---
id: DAA-UOW-001
title: Domain Architecture Analysis
---

# DAA

Test content.
";

        let result = validator.validate(content).await;
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_adr_validator_valid() {
        let validator = AdrValidator::new().unwrap();
        let content = r#"---
id: ADR-001
title: Primitives Module Design
status: accepted
date: 2026-01-10
uow_ref: UOW-001-foundation
sub_phase: "1.1"
---

# ADR

Test content.
"#;

        let result = validator.validate(content).await;
        assert!(
            result.valid,
            "Validation should pass for valid ADR: {result:?}"
        );
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_adr_validator_missing_required_field() {
        let validator = AdrValidator::new().unwrap();
        let content = r"---
id: ADR-001
title: Primitives Module Design
---

# ADR

Test content.
";

        let result = validator.validate(content).await;
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_rfc_validator_valid() {
        let validator = RfcValidator::new().unwrap();
        let content = r#"---
id: RFC-UOW-001
title: Foundation Layer Implementation Plan
version: "1.0"
status: draft
author: airsspec-manager
created_at: 2026-01-10
uow_ref: UOW-001-foundation
---

# RFC

Test content.
"#;

        let result = validator.validate(content).await;
        assert!(
            result.valid,
            "Validation should pass for valid RFC: {result:?}"
        );
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_rfc_validator_missing_required_field() {
        let validator = RfcValidator::new().unwrap();
        let content = r"---
id: RFC-UOW-001
title: Foundation Layer Implementation Plan
---

# RFC

Test content.
";

        let result = validator.validate(content).await;
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_bolt_plan_validator_valid() {
        let validator = BoltPlanValidator::new().unwrap();
        let content = r"---
id: PLAN-001
title: JSONL Persistence
objective: Implement JSONL persistence
steps:
  - Implement ArtifactStore trait
  - Create JsonlPersistence struct
verification:
  - cargo build passes
  - Unit tests pass
---

# Plan

Test content.
";

        let result = validator.validate(content).await;
        assert!(
            result.valid,
            "Validation should pass for valid bolt plan: {result:?}"
        );
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_bolt_plan_validator_missing_required_field() {
        let validator = BoltPlanValidator::new().unwrap();
        let content = r"---
id: PLAN-001
title: JSONL Persistence
---

# Plan

Test content.
";

        let result = validator.validate(content).await;
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_validator_no_frontmatter() {
        let validator = RequirementsValidator::new().unwrap();
        let content = "# Just content\nNo frontmatter here.";

        let result = validator.validate(content).await;
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
        assert!(result.errors[0].field == "frontmatter");
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_validator_invalid_yaml() {
        let validator = RequirementsValidator::new().unwrap();
        let content = "---\ninvalid: yaml: content:\n---\n# Content";

        let result = validator.validate(content).await;
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_validator_wrong_type() {
        let validator = RequirementsValidator::new().unwrap();
        let content = r#"---
id: UOW-001
title: Foundation Layer
version: "1.0"
status: invalid_status
author: airsspec-orchestrator
created_at: 2026-01-10
priority: high
phase: Research
---

# Requirements
"#;

        let result = validator.validate(content).await;
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_validator_artifact_type() {
        let validators = get_validators();

        assert_eq!(validators[0].artifact_type(), ArtifactType::Requirements);
        assert_eq!(validators[1].artifact_type(), ArtifactType::Daa);
        assert_eq!(validators[2].artifact_type(), ArtifactType::Adr);
        assert_eq!(validators[3].artifact_type(), ArtifactType::Rfc);
        assert_eq!(validators[4].artifact_type(), ArtifactType::BoltPlan);
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_all_validators_create() {
        // Test RequirementsValidator creation
        let _req_validator = RequirementsValidator::new().unwrap();

        // Test DaaValidator creation
        let _daa_validator = DaaValidator::new().unwrap();

        // Test AdrValidator creation
        let _adr_validator = AdrValidator::new().unwrap();

        // Test RfcValidator creation
        let _rfc_validator = RfcValidator::new().unwrap();

        // Test BoltPlanValidator creation
        let _bolt_plan_validator = BoltPlanValidator::new().unwrap();

        // All validators created successfully
    }
}
