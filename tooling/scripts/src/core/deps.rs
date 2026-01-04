//! Dependency graph management

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

/// Structure matching deps.toml with sections
#[derive(Debug, Deserialize, Default)]
struct DepsToml {
    #[serde(default)]
    crates: HashMap<String, Vec<String>>,
    #[serde(default)]
    packages: HashMap<String, Vec<String>>,
    #[serde(default)]
    other: HashMap<String, Vec<String>>,
}

/// Load dependencies from deps.toml
pub fn load_deps(root: &Path) -> Result<HashMap<String, Vec<String>>> {
    let deps_path = root.join("tooling/deps.toml");

    if !deps_path.exists() {
        return Ok(HashMap::new());
    }

    let content = std::fs::read_to_string(&deps_path).context("Failed to read deps.toml")?;

    // Try parsing as sectioned format first
    if let Ok(sectioned) = toml::from_str::<DepsToml>(&content) {
        let mut deps = HashMap::new();
        deps.extend(sectioned.crates);
        deps.extend(sectioned.packages);
        deps.extend(sectioned.other);

        // Only use sectioned result if it has content
        if !deps.is_empty() {
            return Ok(deps);
        }
    }

    // Fall back to flat format for backwards compatibility
    let deps: HashMap<String, Vec<String>> =
        toml::from_str(&content).context("Failed to parse deps.toml")?;

    Ok(deps)
}

/// Get topologically sorted order for building
/// Uses Kahn's algorithm with priority queue to prefer packages with fewer dependencies
pub fn topo_order(deps: &HashMap<String, Vec<String>>) -> Result<Vec<String>> {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashSet};

    // Build in-degree map and adjacency list
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    let mut dependents: HashMap<String, Vec<String>> = HashMap::new();

    // Initialize all nodes with 0 in-degree
    for name in deps.keys() {
        in_degree.insert(name.clone(), 0);
        dependents.insert(name.clone(), Vec::new());
    }

    // Calculate in-degrees and build dependents map
    for (name, dependencies) in deps {
        for dep in dependencies {
            // Only count edges to nodes that exist in our graph
            if deps.contains_key(dep) {
                *in_degree.get_mut(name).unwrap() += 1;
                dependents.get_mut(dep).unwrap().push(name.clone());
            }
        }
    }

    // Calculate transitive dependency count for priority ordering
    // Packages with fewer transitive deps should be processed first
    let trans_dep_count = calculate_transitive_deps(deps);

    // Priority queue: (Reverse(transitive_deps), name)
    // Reverse because BinaryHeap is a max-heap
    let mut ready: BinaryHeap<(Reverse<usize>, String)> = BinaryHeap::new();

    // Add all nodes with 0 in-degree to ready queue
    for (name, &deg) in &in_degree {
        if deg == 0 {
            let trans_count = trans_dep_count.get(name).copied().unwrap_or(0);
            ready.push((Reverse(trans_count), name.clone()));
        }
    }

    let mut result = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();

    while let Some((_, name)) = ready.pop() {
        if visited.contains(&name) {
            continue;
        }
        visited.insert(name.clone());
        result.push(name.clone());

        // Process dependents
        if let Some(deps_list) = dependents.get(&name) {
            for dependent in deps_list {
                if let Some(deg) = in_degree.get_mut(dependent) {
                    *deg = deg.saturating_sub(1);
                    if *deg == 0 && !visited.contains(dependent) {
                        let trans_count = trans_dep_count.get(dependent).copied().unwrap_or(0);
                        ready.push((Reverse(trans_count), dependent.clone()));
                    }
                }
            }
        }
    }

    // Check for cycles
    if result.len() != deps.len() {
        let remaining: Vec<_> = deps.keys().filter(|k| !visited.contains(*k)).collect();
        anyhow::bail!("Dependency cycle detected involving: {:?}", remaining);
    }

    Ok(result)
}

/// Calculate transitive dependency count for each package
fn calculate_transitive_deps(deps: &HashMap<String, Vec<String>>) -> HashMap<String, usize> {
    use std::collections::HashSet;

    fn get_all_transitive(
        name: &str,
        deps: &HashMap<String, Vec<String>>,
        cache: &mut HashMap<String, HashSet<String>>,
        visiting: &mut HashSet<String>,
    ) -> HashSet<String> {
        if let Some(cached) = cache.get(name) {
            return cached.clone();
        }

        // Detect cycles
        if visiting.contains(name) {
            return HashSet::new();
        }
        visiting.insert(name.to_string());

        let direct_deps = deps.get(name).cloned().unwrap_or_default();
        let mut all_deps: HashSet<String> = HashSet::new();

        for dep in &direct_deps {
            if deps.contains_key(dep) {
                all_deps.insert(dep.clone());
                // Add all transitive deps from this dependency
                let transitive = get_all_transitive(dep, deps, cache, visiting);
                all_deps.extend(transitive);
            }
        }

        visiting.remove(name);
        cache.insert(name.to_string(), all_deps.clone());
        all_deps
    }

    let mut cache: HashMap<String, HashSet<String>> = HashMap::new();
    let mut result: HashMap<String, usize> = HashMap::new();

    for name in deps.keys() {
        let mut visiting = HashSet::new();
        let all_deps = get_all_transitive(name, deps, &mut cache, &mut visiting);
        result.insert(name.clone(), all_deps.len());
    }
    result
}

/// Get all dependents of a package (packages that depend on it)
pub fn get_dependents(deps: &HashMap<String, Vec<String>>, package: &str) -> Vec<String> {
    deps.iter()
        .filter(|(_, dependencies)| dependencies.contains(&package.to_string()))
        .map(|(name, _)| name.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_topo_order_empty() {
        let deps: HashMap<String, Vec<String>> = HashMap::new();
        let result = topo_order(&deps).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_topo_order_single_package() {
        let mut deps = HashMap::new();
        deps.insert("package-a".to_string(), vec![]);

        let result = topo_order(&deps).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "package-a");
    }

    #[test]
    fn test_topo_order_simple_dependency() {
        let mut deps = HashMap::new();
        deps.insert("package-a".to_string(), vec![]);
        deps.insert("package-b".to_string(), vec!["package-a".to_string()]);

        let result = topo_order(&deps).unwrap();
        assert_eq!(result.len(), 2);

        // package-a should come before package-b
        let a_pos = result.iter().position(|x| x == "package-a").unwrap();
        let b_pos = result.iter().position(|x| x == "package-b").unwrap();
        assert!(a_pos < b_pos);
    }

    #[test]
    fn test_topo_order_complex_dependencies() {
        let mut deps = HashMap::new();
        deps.insert("package-a".to_string(), vec![]);
        deps.insert("package-b".to_string(), vec![]);
        deps.insert(
            "package-c".to_string(),
            vec!["package-a".to_string(), "package-b".to_string()],
        );
        deps.insert("package-d".to_string(), vec!["package-c".to_string()]);

        let result = topo_order(&deps).unwrap();
        assert_eq!(result.len(), 4);

        // Verify ordering constraints
        let a_pos = result.iter().position(|x| x == "package-a").unwrap();
        let b_pos = result.iter().position(|x| x == "package-b").unwrap();
        let c_pos = result.iter().position(|x| x == "package-c").unwrap();
        let d_pos = result.iter().position(|x| x == "package-d").unwrap();

        // Both a and b must come before c
        assert!(a_pos < c_pos);
        assert!(b_pos < c_pos);
        // c must come before d
        assert!(c_pos < d_pos);
    }

    #[test]
    fn test_topo_order_diamond_dependency() {
        // Diamond dependency pattern:
        //     a
        //    / \
        //   b   c
        //    \ /
        //     d
        let mut deps = HashMap::new();
        deps.insert("package-a".to_string(), vec![]);
        deps.insert("package-b".to_string(), vec!["package-a".to_string()]);
        deps.insert("package-c".to_string(), vec!["package-a".to_string()]);
        deps.insert(
            "package-d".to_string(),
            vec!["package-b".to_string(), "package-c".to_string()],
        );

        let result = topo_order(&deps).unwrap();
        assert_eq!(result.len(), 4);

        let a_pos = result.iter().position(|x| x == "package-a").unwrap();
        let b_pos = result.iter().position(|x| x == "package-b").unwrap();
        let c_pos = result.iter().position(|x| x == "package-c").unwrap();
        let d_pos = result.iter().position(|x| x == "package-d").unwrap();

        // a must come before both b and c
        assert!(a_pos < b_pos);
        assert!(a_pos < c_pos);
        // Both b and c must come before d
        assert!(b_pos < d_pos);
        assert!(c_pos < d_pos);
    }

    #[test]
    fn test_topo_order_cycle_detection_self_reference() {
        let mut deps = HashMap::new();
        deps.insert("package-a".to_string(), vec!["package-a".to_string()]);

        let result = topo_order(&deps);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cycle"));
    }

    #[test]
    fn test_topo_order_cycle_detection_two_packages() {
        let mut deps = HashMap::new();
        deps.insert("package-a".to_string(), vec!["package-b".to_string()]);
        deps.insert("package-b".to_string(), vec!["package-a".to_string()]);

        let result = topo_order(&deps);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cycle"));
    }

    #[test]
    fn test_topo_order_cycle_detection_three_packages() {
        let mut deps = HashMap::new();
        deps.insert("package-a".to_string(), vec!["package-b".to_string()]);
        deps.insert("package-b".to_string(), vec!["package-c".to_string()]);
        deps.insert("package-c".to_string(), vec!["package-a".to_string()]);

        let result = topo_order(&deps);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cycle"));
    }

    #[test]
    fn test_topo_order_cycle_detection_complex() {
        // Valid DAG with additional cycle
        let mut deps = HashMap::new();
        deps.insert("package-a".to_string(), vec![]);
        deps.insert("package-b".to_string(), vec!["package-a".to_string()]);
        deps.insert("package-c".to_string(), vec!["package-b".to_string()]);
        deps.insert("package-d".to_string(), vec!["package-c".to_string()]);
        // Add cycle: d -> a creates a cycle
        deps.insert("package-a".to_string(), vec!["package-d".to_string()]);

        let result = topo_order(&deps);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cycle"));
    }

    #[test]
    fn test_get_dependents_empty() {
        let deps: HashMap<String, Vec<String>> = HashMap::new();
        let dependents = get_dependents(&deps, "package-a");
        assert_eq!(dependents.len(), 0);
    }

    #[test]
    fn test_get_dependents_no_dependents() {
        let mut deps = HashMap::new();
        deps.insert("package-a".to_string(), vec![]);
        deps.insert("package-b".to_string(), vec![]);

        let dependents = get_dependents(&deps, "package-a");
        assert_eq!(dependents.len(), 0);
    }

    #[test]
    fn test_get_dependents_single_dependent() {
        let mut deps = HashMap::new();
        deps.insert("package-a".to_string(), vec![]);
        deps.insert("package-b".to_string(), vec!["package-a".to_string()]);

        let dependents = get_dependents(&deps, "package-a");
        assert_eq!(dependents.len(), 1);
        assert!(dependents.contains(&"package-b".to_string()));
    }

    #[test]
    fn test_get_dependents_multiple_dependents() {
        let mut deps = HashMap::new();
        deps.insert("package-a".to_string(), vec![]);
        deps.insert("package-b".to_string(), vec!["package-a".to_string()]);
        deps.insert("package-c".to_string(), vec!["package-a".to_string()]);
        deps.insert("package-d".to_string(), vec!["package-a".to_string()]);

        let mut dependents = get_dependents(&deps, "package-a");
        dependents.sort();
        assert_eq!(dependents.len(), 3);
        assert_eq!(dependents, vec!["package-b", "package-c", "package-d"]);
    }

    #[test]
    fn test_get_dependents_transitive_not_included() {
        // Only direct dependents should be returned, not transitive
        let mut deps = HashMap::new();
        deps.insert("package-a".to_string(), vec![]);
        deps.insert("package-b".to_string(), vec!["package-a".to_string()]);
        deps.insert("package-c".to_string(), vec!["package-b".to_string()]);

        let dependents = get_dependents(&deps, "package-a");
        assert_eq!(dependents.len(), 1);
        assert!(dependents.contains(&"package-b".to_string()));
        assert!(!dependents.contains(&"package-c".to_string()));
    }

    #[test]
    fn test_get_dependents_mixed_dependencies() {
        let mut deps = HashMap::new();
        deps.insert("package-a".to_string(), vec![]);
        deps.insert("package-b".to_string(), vec![]);
        deps.insert(
            "package-c".to_string(),
            vec!["package-a".to_string(), "package-b".to_string()],
        );
        deps.insert("package-d".to_string(), vec!["package-b".to_string()]);

        let mut dependents_a = get_dependents(&deps, "package-a");
        dependents_a.sort();
        assert_eq!(dependents_a, vec!["package-c"]);

        let mut dependents_b = get_dependents(&deps, "package-b");
        dependents_b.sort();
        assert_eq!(dependents_b, vec!["package-c", "package-d"]);
    }

    #[test]
    fn test_get_dependents_nonexistent_package() {
        let mut deps = HashMap::new();
        deps.insert("package-a".to_string(), vec![]);
        deps.insert("package-b".to_string(), vec!["package-a".to_string()]);

        let dependents = get_dependents(&deps, "package-nonexistent");
        assert_eq!(dependents.len(), 0);
    }

    #[test]
    fn test_load_deps_valid_toml() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        let toml_content = r#"
package-a = []
package-b = ["package-a"]
package-c = ["package-a", "package-b"]
"#;
        temp_file.write_all(toml_content.as_bytes())?;
        temp_file.flush()?;

        // Create a temporary directory structure
        let temp_dir = tempfile::tempdir()?;
        let tooling_dir = temp_dir.path().join("tooling");
        std::fs::create_dir(&tooling_dir)?;

        let deps_path = tooling_dir.join("deps.toml");
        std::fs::write(&deps_path, toml_content)?;

        let result = load_deps(temp_dir.path())?;

        assert_eq!(result.len(), 3);
        assert_eq!(result.get("package-a").unwrap().len(), 0);
        assert_eq!(
            result.get("package-b").unwrap(),
            &vec!["package-a".to_string()]
        );
        assert_eq!(
            result.get("package-c").unwrap(),
            &vec!["package-a".to_string(), "package-b".to_string()]
        );

        Ok(())
    }

    #[test]
    fn test_load_deps_empty_toml() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let tooling_dir = temp_dir.path().join("tooling");
        std::fs::create_dir(&tooling_dir)?;

        let deps_path = tooling_dir.join("deps.toml");
        std::fs::write(&deps_path, "")?;

        let result = load_deps(temp_dir.path())?;
        assert_eq!(result.len(), 0);

        Ok(())
    }

    #[test]
    fn test_load_deps_missing_file() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;

        // Don't create tooling/deps.toml - test that it returns empty HashMap
        let result = load_deps(temp_dir.path())?;
        assert_eq!(result.len(), 0);

        Ok(())
    }

    #[test]
    fn test_load_deps_invalid_toml() {
        let temp_dir = tempfile::tempdir().unwrap();
        let tooling_dir = temp_dir.path().join("tooling");
        std::fs::create_dir(&tooling_dir).unwrap();

        let deps_path = tooling_dir.join("deps.toml");
        std::fs::write(&deps_path, "invalid toml content [[[").unwrap();

        let result = load_deps(temp_dir.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to parse"));
    }

    #[test]
    fn test_load_deps_complex_structure() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let tooling_dir = temp_dir.path().join("tooling");
        std::fs::create_dir(&tooling_dir)?;

        let toml_content = r#"
core = []
utils = ["core"]
ui-components = ["core", "utils"]
app-frontend = ["ui-components", "utils"]
app-backend = ["core"]
app-full = ["app-frontend", "app-backend"]
"#;
        let deps_path = tooling_dir.join("deps.toml");
        std::fs::write(&deps_path, toml_content)?;

        let result = load_deps(temp_dir.path())?;

        assert_eq!(result.len(), 6);
        assert_eq!(result.get("core").unwrap().len(), 0);
        assert_eq!(result.get("utils").unwrap(), &vec!["core".to_string()]);
        assert_eq!(
            result.get("ui-components").unwrap(),
            &vec!["core".to_string(), "utils".to_string()]
        );
        assert_eq!(
            result.get("app-frontend").unwrap(),
            &vec!["ui-components".to_string(), "utils".to_string()]
        );
        assert_eq!(
            result.get("app-backend").unwrap(),
            &vec!["core".to_string()]
        );
        assert_eq!(
            result.get("app-full").unwrap(),
            &vec!["app-frontend".to_string(), "app-backend".to_string()]
        );

        Ok(())
    }

    #[test]
    fn test_integration_load_and_topo_sort() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let tooling_dir = temp_dir.path().join("tooling");
        std::fs::create_dir(&tooling_dir)?;

        let toml_content = r#"
package-a = []
package-b = ["package-a"]
package-c = ["package-b"]
"#;
        let deps_path = tooling_dir.join("deps.toml");
        std::fs::write(&deps_path, toml_content)?;

        let deps = load_deps(temp_dir.path())?;
        let order = topo_order(&deps)?;

        // Verify correct topological order
        let a_pos = order.iter().position(|x| x == "package-a").unwrap();
        let b_pos = order.iter().position(|x| x == "package-b").unwrap();
        let c_pos = order.iter().position(|x| x == "package-c").unwrap();

        assert!(a_pos < b_pos);
        assert!(b_pos < c_pos);

        Ok(())
    }

    #[test]
    fn test_integration_load_and_get_dependents() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let tooling_dir = temp_dir.path().join("tooling");
        std::fs::create_dir(&tooling_dir)?;

        let toml_content = r#"
package-a = []
package-b = ["package-a"]
package-c = ["package-a"]
package-d = ["package-b"]
"#;
        let deps_path = tooling_dir.join("deps.toml");
        std::fs::write(&deps_path, toml_content)?;

        let deps = load_deps(temp_dir.path())?;

        let mut dependents_a = get_dependents(&deps, "package-a");
        dependents_a.sort();
        assert_eq!(dependents_a, vec!["package-b", "package-c"]);

        let dependents_b = get_dependents(&deps, "package-b");
        assert_eq!(dependents_b, vec!["package-d"]);

        Ok(())
    }

    #[test]
    fn test_load_deps_sectioned_format() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let tooling_dir = temp_dir.path().join("tooling");
        std::fs::create_dir(&tooling_dir)?;

        // Test the sectioned format used in the actual deps.toml
        let toml_content = r#"
[crates]
syn = []
template = []
macros = ["syn", "template"]

[packages]
shared = ["core"]
vite-plugin = ["core", "shared"]

[other]
website = ["core"]
"#;
        let deps_path = tooling_dir.join("deps.toml");
        std::fs::write(&deps_path, toml_content)?;

        let result = load_deps(temp_dir.path())?;

        // Should have all packages from all sections
        assert_eq!(result.len(), 6);

        // Check crates section
        assert_eq!(result.get("syn").unwrap().len(), 0);
        assert_eq!(result.get("template").unwrap().len(), 0);
        assert_eq!(
            result.get("macros").unwrap(),
            &vec!["syn".to_string(), "template".to_string()]
        );

        // Check packages section
        assert_eq!(result.get("shared").unwrap(), &vec!["core".to_string()]);
        assert_eq!(
            result.get("vite-plugin").unwrap(),
            &vec!["core".to_string(), "shared".to_string()]
        );

        // Check other section
        assert_eq!(result.get("website").unwrap(), &vec!["core".to_string()]);

        Ok(())
    }

    #[test]
    fn test_topo_order_priority_by_dependency_count() {
        // Packages with fewer deps should come first when both are ready
        let mut deps = HashMap::new();
        deps.insert("core".to_string(), vec![]);
        deps.insert("mcp-server".to_string(), vec!["core".to_string()]); // 1 dep
        deps.insert("shared".to_string(), vec!["core".to_string()]); // 1 dep
        deps.insert(
            "vite-plugin".to_string(),
            vec!["core".to_string(), "shared".to_string()],
        ); // 2 deps
        deps.insert(
            "tooling".to_string(),
            vec!["core".to_string(), "vite-plugin".to_string()],
        ); // 2 deps (transitive: 3)

        let result = topo_order(&deps).unwrap();

        // core must be first (no deps)
        assert_eq!(result[0], "core");

        // mcp-server and shared should come before vite-plugin and tooling
        let mcp_pos = result.iter().position(|x| x == "mcp-server").unwrap();
        let shared_pos = result.iter().position(|x| x == "shared").unwrap();
        let vite_pos = result.iter().position(|x| x == "vite-plugin").unwrap();
        let tooling_pos = result.iter().position(|x| x == "tooling").unwrap();

        // shared must come before vite-plugin (dependency)
        assert!(
            shared_pos < vite_pos,
            "shared should come before vite-plugin"
        );

        // vite-plugin must come before tooling (dependency)
        assert!(
            vite_pos < tooling_pos,
            "vite-plugin should come before tooling"
        );

        // mcp-server should come before tooling (mcp-server has 1 transitive dep, tooling has 3)
        assert!(
            mcp_pos < tooling_pos,
            "mcp-server should come before tooling due to fewer transitive deps"
        );
    }
}
