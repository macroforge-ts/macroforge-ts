//! Version cache management

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Version information for a package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// Local version in manifest files
    pub local: String,
    /// Published version on registry
    pub registry: String,
}

/// Version cache stored in versions.json
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VersionsCache {
    #[serde(flatten)]
    pub versions: HashMap<String, Option<VersionInfo>>,
}

impl VersionsCache {
    /// Load versions from versions.json
    pub fn load(root: &Path) -> Result<Self> {
        let versions_path = root.join("tooling/versions.json");

        if !versions_path.exists() {
            return Ok(Self::default());
        }

        let content =
            std::fs::read_to_string(&versions_path).context("Failed to read versions.json")?;

        let cache: Self =
            serde_json::from_str(&content).context("Failed to parse versions.json")?;

        Ok(cache)
    }

    /// Save versions to versions.json
    pub fn save(&self, root: &Path) -> Result<()> {
        let versions_path = root.join("tooling/versions.json");

        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut serializer = serde_json::Serializer::with_formatter(buf, formatter);
        self.serialize(&mut serializer)
            .context("Failed to serialize versions")?;
        let mut content =
            String::from_utf8(serializer.into_inner()).context("Failed to convert to UTF-8")?;
        content.push('\n');

        std::fs::write(&versions_path, content).context("Failed to write versions.json")?;

        Ok(())
    }

    /// Get local version for a package
    pub fn get_local(&self, name: &str) -> Option<&str> {
        self.versions
            .get(name)
            .and_then(|v| v.as_ref())
            .map(|v| v.local.as_str())
    }

    /// Get registry version for a package
    pub fn get_registry(&self, name: &str) -> Option<&str> {
        self.versions
            .get(name)
            .and_then(|v| v.as_ref())
            .map(|v| v.registry.as_str())
    }

    /// Set local version for a package
    pub fn set_local(&mut self, name: &str, version: &str) {
        if let Some(Some(info)) = self.versions.get_mut(name) {
            info.local = version.to_string();
        } else {
            self.versions.insert(
                name.to_string(),
                Some(VersionInfo {
                    local: version.to_string(),
                    registry: String::new(),
                }),
            );
        }
    }

    /// Set registry version for a package
    pub fn set_registry(&mut self, name: &str, version: &str) {
        if let Some(Some(info)) = self.versions.get_mut(name) {
            info.registry = version.to_string();
        } else {
            self.versions.insert(
                name.to_string(),
                Some(VersionInfo {
                    local: String::new(),
                    registry: version.to_string(),
                }),
            );
        }
    }
}

/// Parse a semver version and increment the patch number
pub fn increment_patch(version: &str) -> Result<String> {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        anyhow::bail!("Invalid semver version: {}", version);
    }

    let major = parts[0];
    let minor = parts[1];
    let patch: u32 = parts[2].parse().context("Invalid patch version number")?;

    Ok(format!("{}.{}.{}", major, minor, patch + 1))
}

/// Compare two semver versions, returns true if a > b
pub fn version_gt(a: &str, b: &str) -> bool {
    let parse = |v: &str| -> (u32, u32, u32) {
        let parts: Vec<u32> = v.split('.').filter_map(|p| p.parse().ok()).collect();
        (
            parts.first().copied().unwrap_or(0),
            parts.get(1).copied().unwrap_or(0),
            parts.get(2).copied().unwrap_or(0),
        )
    };

    parse(a) > parse(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_increment_patch() {
        assert_eq!(increment_patch("1.2.3").unwrap(), "1.2.4");
        assert_eq!(increment_patch("0.0.0").unwrap(), "0.0.1");
        assert_eq!(increment_patch("10.20.30").unwrap(), "10.20.31");
    }

    #[test]
    fn test_increment_patch_invalid() {
        assert!(increment_patch("1.2").is_err());
        assert!(increment_patch("1.2.3.4").is_err());
        assert!(increment_patch("invalid").is_err());
    }

    #[test]
    fn test_version_gt() {
        assert!(version_gt("1.0.0", "0.9.9"));
        assert!(version_gt("1.1.0", "1.0.9"));
        assert!(version_gt("1.0.1", "1.0.0"));
        assert!(!version_gt("1.0.0", "1.0.0"));
        assert!(!version_gt("0.9.9", "1.0.0"));
    }

    #[test]
    fn test_version_gt_edge_cases() {
        assert!(version_gt("2.0.0", "1.99.99"));
        assert!(version_gt("0.0.1", "0.0.0"));
        assert!(!version_gt("0.0.0", "0.0.1"));
    }

    #[test]
    fn test_versions_cache_get_set_local() {
        let mut cache = VersionsCache::default();

        assert!(cache.get_local("test").is_none());

        cache.set_local("test", "1.0.0");
        assert_eq!(cache.get_local("test"), Some("1.0.0"));

        cache.set_local("test", "2.0.0");
        assert_eq!(cache.get_local("test"), Some("2.0.0"));
    }

    #[test]
    fn test_versions_cache_get_set_registry() {
        let mut cache = VersionsCache::default();

        assert!(cache.get_registry("test").is_none());

        cache.set_registry("test", "1.0.0");
        assert_eq!(cache.get_registry("test"), Some("1.0.0"));

        cache.set_registry("test", "2.0.0");
        assert_eq!(cache.get_registry("test"), Some("2.0.0"));
    }

    #[test]
    fn test_versions_cache_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let tooling_dir = temp_dir.path().join("tooling");
        std::fs::create_dir_all(&tooling_dir).unwrap();

        let mut cache = VersionsCache::default();
        cache.set_local("core", "1.2.3");
        cache.set_registry("core", "1.2.2");
        cache.set_local("shared", "0.5.0");

        cache.save(temp_dir.path()).unwrap();

        let loaded = VersionsCache::load(temp_dir.path()).unwrap();
        assert_eq!(loaded.get_local("core"), Some("1.2.3"));
        assert_eq!(loaded.get_registry("core"), Some("1.2.2"));
        assert_eq!(loaded.get_local("shared"), Some("0.5.0"));
    }

    #[test]
    fn test_versions_cache_load_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let cache = VersionsCache::load(temp_dir.path()).unwrap();
        assert!(cache.versions.is_empty());
    }

    #[test]
    fn test_versions_cache_multiple_packages() {
        let mut cache = VersionsCache::default();

        cache.set_local("pkg1", "1.0.0");
        cache.set_local("pkg2", "2.0.0");
        cache.set_local("pkg3", "3.0.0");

        assert_eq!(cache.get_local("pkg1"), Some("1.0.0"));
        assert_eq!(cache.get_local("pkg2"), Some("2.0.0"));
        assert_eq!(cache.get_local("pkg3"), Some("3.0.0"));
    }
}
