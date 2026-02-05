//! Project configuration types.
//!
//! Configuration is stored in `.airsspec/config.toml`.

use serde::{Deserialize, Serialize};

use crate::spec::Category;

/// Project configuration stored in `.airsspec/config.toml`.
///
/// # Examples
///
/// ```
/// use airsspec_core::workspace::ProjectConfig;
///
/// let config = ProjectConfig::new("My Project", "A sample project");
/// assert_eq!(config.name(), "My Project");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectConfig {
    project: ProjectInfo,
    #[serde(default)]
    defaults: SpecDefaults,
}

impl ProjectConfig {
    /// Creates a new project configuration.
    #[must_use]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            project: ProjectInfo {
                name: name.into(),
                description: description.into(),
            },
            defaults: SpecDefaults::default(),
        }
    }

    /// Returns the project name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.project.name
    }

    /// Returns the project description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.project.description
    }

    /// Returns the default category for new specs.
    #[must_use]
    pub fn default_category(&self) -> Category {
        self.defaults.category
    }

    /// Sets the default category for new specs.
    pub fn set_default_category(&mut self, category: Category) {
        self.defaults.category = category;
    }
}

/// Basic project information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectInfo {
    name: String,
    description: String,
}

impl ProjectInfo {
    /// Returns the project name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the project description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// Default values for new specs.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecDefaults {
    #[serde(default)]
    category: Category,
}

impl SpecDefaults {
    /// Returns the default category.
    #[must_use]
    pub fn category(&self) -> Category {
        self.category
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = ProjectConfig::new("Test Project", "A test project");
        assert_eq!(config.name(), "Test Project");
        assert_eq!(config.description(), "A test project");
    }

    #[test]
    fn test_config_getters() {
        let config = ProjectConfig::new("My App", "An awesome app");
        assert_eq!(config.name(), "My App");
        assert_eq!(config.description(), "An awesome app");
        assert_eq!(config.default_category(), Category::Feature);
    }

    #[test]
    fn test_config_default_category() {
        let config = ProjectConfig::new("Test", "Test");
        assert_eq!(config.default_category(), Category::Feature);
    }

    #[test]
    fn test_config_set_default_category() {
        let mut config = ProjectConfig::new("Test", "Test");
        config.set_default_category(Category::BugFix);
        assert_eq!(config.default_category(), Category::BugFix);
    }

    #[test]
    fn test_config_serde_roundtrip_toml() {
        let config = ProjectConfig::new("Test Project", "A test project");
        let toml_str = toml::to_string(&config).expect("should serialize to TOML");
        let parsed: ProjectConfig = toml::from_str(&toml_str).expect("should parse TOML");
        assert_eq!(config, parsed);
    }

    #[test]
    fn test_config_serde_roundtrip_json() {
        let config = ProjectConfig::new("Test Project", "A test project");
        let json_str = serde_json::to_string(&config).expect("should serialize to JSON");
        let parsed: ProjectConfig = serde_json::from_str(&json_str).expect("should parse JSON");
        assert_eq!(config, parsed);
    }

    #[test]
    fn test_config_clone() {
        let config = ProjectConfig::new("Test", "Test");
        let cloned = config.clone();
        assert_eq!(config, cloned);
    }

    #[test]
    fn test_config_eq() {
        let config1 = ProjectConfig::new("Test", "Test");
        let config2 = ProjectConfig::new("Test", "Test");
        let config3 = ProjectConfig::new("Other", "Other");
        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    #[test]
    fn test_project_info_getters() {
        let config = ProjectConfig::new("My Project", "Description");
        assert_eq!(config.name(), "My Project");
        assert_eq!(config.description(), "Description");
    }

    #[test]
    fn test_spec_defaults_category() {
        let defaults = SpecDefaults::default();
        assert_eq!(defaults.category(), Category::Feature);
    }

    #[test]
    fn test_config_with_non_default_category() {
        let mut config = ProjectConfig::new("Test", "Test");
        config.set_default_category(Category::Enhancement);

        let toml_str = toml::to_string(&config).expect("should serialize to TOML");
        let parsed: ProjectConfig = toml::from_str(&toml_str).expect("should parse TOML");
        assert_eq!(parsed.default_category(), Category::Enhancement);
    }
}
