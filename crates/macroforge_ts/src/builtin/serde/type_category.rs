//! TypeScript type categorization for serialization strategy selection.

use super::foreign_types::{ForeignTypeMatch, register_foreign_type_namespaces};
use super::helpers::{find_top_level_comma, split_top_level_union};
use crate::host::ForeignTypeConfig;
use crate::host::import_registry::with_registry;

/// Determines the serialization strategy for a TypeScript type
#[derive(Debug, Clone, PartialEq)]
pub enum TypeCategory {
    Primitive,
    Array(String),
    Optional(String),
    Nullable(String),
    Date,
    Map(String, String),
    Set(String),
    Record(String, String),
    /// Wrapper types like Partial<T>, Required<T>, Readonly<T>, Pick<T, K>, Omit<T, K>, NonNullable<T>
    /// These don't change the runtime value structure, so we serialize based on the inner type.
    /// Contains the inner type name (e.g., "User" for Partial<User>)
    Wrapper(String),
    Serializable(String),
    Unknown,
}

impl TypeCategory {
    pub fn from_ts_type(ts_type: &str) -> Self {
        let trimmed = ts_type.trim();

        // Handle string literal types (e.g., "Zoned", 'foo') - these are primitive-like
        if (trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        {
            return Self::Primitive;
        }

        // Handle primitives
        match trimmed {
            "string" | "number" | "boolean" | "null" | "undefined" | "bigint" => {
                return Self::Primitive;
            }
            "Date" => return Self::Date,
            _ => {}
        }

        // Handle Array<T> or T[]
        if trimmed.starts_with("Array<") && trimmed.ends_with('>') {
            let inner = &trimmed[6..trimmed.len() - 1];
            return Self::Array(inner.to_string());
        }
        if let Some(inner) = trimmed.strip_suffix("[]") {
            return Self::Array(inner.to_string());
        }

        // Handle Map<K, V>
        if trimmed.starts_with("Map<") && trimmed.ends_with('>') {
            let inner = &trimmed[4..trimmed.len() - 1];
            if let Some(comma_pos) = find_top_level_comma(inner) {
                let key = inner[..comma_pos].trim().to_string();
                let value = inner[comma_pos + 1..].trim().to_string();
                return Self::Map(key, value);
            }
        }

        // Handle Set<T>
        if trimmed.starts_with("Set<") && trimmed.ends_with('>') {
            let inner = &trimmed[4..trimmed.len() - 1];
            return Self::Set(inner.to_string());
        }

        // Handle Record<K, V>
        if trimmed.starts_with("Record<") && trimmed.ends_with('>') {
            let inner = &trimmed[7..trimmed.len() - 1];
            if let Some(comma_pos) = find_top_level_comma(inner) {
                let key = inner[..comma_pos].trim().to_string();
                let value = inner[comma_pos + 1..].trim().to_string();
                return Self::Record(key, value);
            }
        }

        // Handle union types (T | undefined, T | null)
        // Only split on top-level `|` — pipes inside <>, (), [], or string
        // literals (e.g. Pick<User, 'a' | 'b'>) are ignored.
        if let Some(parts) = split_top_level_union(trimmed) {
            if parts.contains(&"undefined") {
                let non_undefined: Vec<&str> = parts
                    .iter()
                    .filter(|p| *p != &"undefined")
                    .copied()
                    .collect();
                return Self::Optional(non_undefined.join(" | "));
            }
            if parts.contains(&"null") {
                let non_null: Vec<&str> = parts.iter().filter(|p| *p != &"null").copied().collect();
                return Self::Nullable(non_null.join(" | "));
            }
            // Remaining unions (e.g., Utc | number) are not a single serializable type
            return Self::Unknown;
        }

        // Check if it looks like a class/interface name (starts with uppercase)
        // Exclude built-in types and utility types
        if let Some(first_char) = trimmed.chars().next()
            && first_char.is_uppercase()
            && !matches!(
                trimmed,
                "String"
                    | "Number"
                    | "Boolean"
                    | "Object"
                    | "Function"
                    | "Symbol"
                    | "URL"
                    | "URLSearchParams"
                    | "RegExp"
                    | "Error"
                    | "EvalError"
                    | "RangeError"
                    | "ReferenceError"
                    | "SyntaxError"
                    | "TypeError"
                    | "URIError"
                    | "Int8Array"
                    | "Uint8Array"
                    | "Uint8ClampedArray"
                    | "Int16Array"
                    | "Uint16Array"
                    | "Int32Array"
                    | "Uint32Array"
                    | "Float32Array"
                    | "Float64Array"
                    | "BigInt64Array"
                    | "BigUint64Array"
                    | "ArrayBuffer"
                    | "DataView"
            )
        {
            // Extract base type name (handle generics like Foo<T>)
            let base_type = if let Some(idx) = trimmed.find('<') {
                &trimmed[..idx]
            } else {
                trimmed
            };

            // Handle TypeScript wrapper utility types that preserve the inner type's structure
            // These don't change runtime values, so we serialize based on the inner type
            if matches!(
                base_type,
                "Partial" | "Required" | "Readonly" | "NonNullable"
            ) {
                // Single type argument: Partial<T>, Required<T>, Readonly<T>, NonNullable<T>
                if let Some(start) = trimmed.find('<') {
                    let inner = &trimmed[start + 1..trimmed.len() - 1];
                    return Self::Wrapper(inner.to_string());
                }
            }

            if matches!(base_type, "Pick" | "Omit") {
                // Two type arguments: Pick<T, K>, Omit<T, K> - we care about T
                if let Some(start) = trimmed.find('<') {
                    let inner = &trimmed[start + 1..trimmed.len() - 1];
                    if let Some(comma_pos) = find_top_level_comma(inner) {
                        let first_arg = inner[..comma_pos].trim();
                        return Self::Wrapper(first_arg.to_string());
                    }
                }
            }

            // These utility types don't have a meaningful serialization strategy
            // (they operate on function types, unions, or async types)
            if matches!(
                base_type,
                "Exclude"
                    | "Extract"
                    | "ReturnType"
                    | "Parameters"
                    | "InstanceType"
                    | "ThisType"
                    | "Awaited"
                    | "Promise"
            ) {
                return Self::Unknown;
            }

            let base = if let Some(idx) = trimmed.find('<') {
                &trimmed[..idx]
            } else {
                trimmed
            };

            // Dot-qualified types like DateTime.Utc are foreign types handled
            // via match_foreign_type at the field level. Classifying them as
            // Serializable would produce garbled function names (e.g.,
            // "dateTime.utcSerializeWithContext") because to_case(Camel)
            // doesn't handle dots correctly.
            if base.contains('.') {
                return Self::Unknown;
            }

            return Self::Serializable(base.to_string());
        }

        Self::Unknown
    }

    /// Check if a type matches a configured foreign type.
    ///
    /// Attempts to match the type name against the configured foreign types,
    /// checking both exact name matches and imports from configured sources.
    ///
    /// # Arguments
    ///
    /// * `ts_type` - The TypeScript type string (e.g., "DateTime", "ZonedDateTime")
    /// * `foreign_types` - List of configured foreign type handlers
    ///
    /// # Returns
    ///
    /// The matching foreign type configuration, if found.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let foreign_types = vec![ForeignTypeConfig {
    ///     name: "DateTime".to_string(),
    ///     from: vec!["effect".to_string()],
    ///     ..Default::default()
    /// }];
    ///
    /// // Matches by exact name
    /// assert!(TypeCategory::match_foreign_type("DateTime", &foreign_types).is_some());
    ///
    /// // Doesn't match other types
    /// assert!(TypeCategory::match_foreign_type("Date", &foreign_types).is_none());
    /// ```
    /// Match a TypeScript type against configured foreign types with import source validation.
    ///
    /// # Arguments
    ///
    /// * `ts_type` - The TypeScript type string (e.g., "DateTime.DateTime", "Duration")
    /// * `foreign_types` - List of configured foreign type handlers
    ///
    /// # Returns
    ///
    /// A `ForeignTypeMatch` containing the matched config (if any) and potential warnings
    /// about near-matches that failed due to import source validation.
    pub fn match_foreign_type<'a>(
        ts_type: &str,
        foreign_types: &'a [ForeignTypeConfig],
    ) -> ForeignTypeMatch<'a> {
        let trimmed = ts_type.trim();
        let import_sources = with_registry(|r| r.source_modules());
        let import_aliases = with_registry(|r| r.aliases());

        // Skip empty types
        if trimmed.is_empty() {
            return ForeignTypeMatch::none();
        }

        // Extract the base type name (handle generics like Foo<T>)
        let base_type = if let Some(idx) = trimmed.find('<') {
            &trimmed[..idx]
        } else {
            trimmed
        };

        // For qualified types like DateTime.DateTime, extract parts
        let (namespace_part, type_name) = if let Some(dot_idx) = base_type.rfind('.') {
            (Some(&base_type[..dot_idx]), &base_type[dot_idx + 1..])
        } else {
            (None, base_type)
        };

        // Extract the first part of the namespace for import lookup
        // For A.B.C, the import name is A (the first part before any dot)
        let first_namespace_part = namespace_part.map(|ns| {
            if let Some(dot_idx) = ns.find('.') {
                &ns[..dot_idx]
            } else {
                ns
            }
        });

        // Resolve the import name - use first namespace part if qualified, otherwise type_name
        let import_name = first_namespace_part.unwrap_or(type_name);

        // For unqualified types (no namespace), the type_name itself might be an alias
        // e.g., EffectOption -> Option. For qualified types, only the namespace is aliased.
        let resolved_type_name = if namespace_part.is_none() {
            import_aliases
                .get(type_name)
                .map(String::as_str)
                .unwrap_or(type_name)
        } else {
            // For qualified types, the type_name (last part) is not aliased
            type_name
        };

        // For unqualified aliased types, resolve the base_type
        let resolved_base_type = if namespace_part.is_none() {
            import_aliases
                .get(base_type)
                .map(String::as_str)
                .unwrap_or(base_type)
        } else {
            base_type
        };

        // For qualified types, resolve the namespace alias
        // e.g., EffectDateTime.DateTime with import { DateTime as EffectDateTime }
        // should resolve namespace EffectDateTime -> DateTime
        // For A.B.C where A is aliased to X, resolve to X.B.C
        let resolved_namespace: Option<String> = namespace_part.map(|ns| {
            let parts: Vec<&str> = ns.split('.').collect();
            if let Some(first_part) = parts.first() {
                if let Some(resolved_first) = import_aliases.get(*first_part) {
                    // Replace the first part with the resolved alias and rejoin
                    let mut resolved_parts: Vec<&str> = vec![resolved_first.as_str()];
                    resolved_parts.extend(&parts[1..]);
                    resolved_parts.join(".")
                } else {
                    ns.to_string()
                }
            } else {
                ns.to_string()
            }
        });

        let mut near_match: Option<(&ForeignTypeConfig, String)> = None;

        for ft in foreign_types {
            let ft_type_name = ft.get_type_name();
            let ft_namespace = ft.get_namespace();
            let ft_qualified = ft.get_qualified_name();

            // Check if the type name matches (using resolved name for aliased imports)
            let name_matches = type_name == ft_type_name
                || base_type == ft.name
                || base_type == ft_qualified
                || resolved_type_name == ft_type_name
                || resolved_base_type == ft.name
                || resolved_base_type == ft_qualified;

            // Also check namespace matches if both have namespaces
            // Use resolved namespace for aliased imports (e.g., EffectDateTime -> DateTime)
            let namespace_matches = match (namespace_part, ft_namespace) {
                (Some(ns), Some(ft_ns)) => {
                    // Match either original namespace or resolved (aliased) namespace
                    let resolved_ns = resolved_namespace.as_deref().unwrap_or(ns);
                    ns == ft_ns || resolved_ns == ft_ns
                }
                (None, None) => true,
                // If config has namespace but type doesn't, it's not a match
                (None, Some(_)) => false,
                // If type has namespace but config doesn't, require exact base_type match
                (Some(_), None) => base_type == ft.name || resolved_base_type == ft.name,
            };

            if name_matches && namespace_matches {
                // Global types (empty from list) don't need import validation
                if ft.from.is_empty() {
                    return ForeignTypeMatch::matched(ft);
                }
                // Now validate import source
                if let Some(actual_source) = import_sources.get(import_name) {
                    // Check if the actual import source matches any configured source
                    let source_matches = ft.from.iter().any(|configured_source| {
                        actual_source == configured_source
                            || actual_source.ends_with(configured_source)
                            || configured_source.ends_with(actual_source)
                    });

                    if source_matches {
                        // Register required namespace imports for this foreign type
                        register_foreign_type_namespaces(ft, actual_source);
                        return ForeignTypeMatch::matched(ft);
                    }
                    // Type is imported from a different source - don't match
                    // We can't know if that library's type has the right methods
                    // Let it fall through to generic handling; TypeScript will catch issues
                } else {
                    // No import found for this type - likely a local type or re-exported
                    // Don't match foreign type config for types we can't verify the source of
                }
            } else if (type_name == ft_type_name || resolved_type_name == ft_type_name)
                && !name_matches
            {
                // Type name matches but qualified form doesn't - helpful hint
                let warning = format!(
                    "Type '{}' has the same name as foreign type '{}' but uses different qualification. \
                     Expected '{}' or configure with namespace: '{}'.",
                    base_type,
                    ft.name,
                    ft_qualified,
                    namespace_part.unwrap_or(type_name)
                );
                if near_match.is_none() {
                    near_match = Some((ft, warning));
                }
            }

            // Check aliases for this foreign type
            for alias in &ft.aliases {
                // Parse alias name for potential namespace
                let (alias_namespace, alias_type_name) =
                    if let Some(dot_idx) = alias.name.rfind('.') {
                        (Some(&alias.name[..dot_idx]), &alias.name[dot_idx + 1..])
                    } else {
                        (None, alias.name.as_str())
                    };

                // Check if alias name matches the type
                let alias_name_matches = type_name == alias_type_name || base_type == alias.name;

                // Check namespace matches for alias
                let alias_namespace_matches = match (namespace_part, alias_namespace) {
                    (Some(ns), Some(alias_ns)) => ns == alias_ns,
                    (None, None) => true,
                    (None, Some(_)) => false,
                    (Some(_), None) => base_type == alias.name,
                };

                if alias_name_matches && alias_namespace_matches {
                    // Global types (empty from list) don't need import validation
                    if ft.from.is_empty() {
                        return ForeignTypeMatch::matched(ft);
                    }
                    // Validate import source against alias's from
                    let import_name = namespace_part.unwrap_or(type_name);

                    if let Some(actual_source) = import_sources.get(import_name) {
                        // Check if import source matches the alias's from
                        if actual_source == &alias.from
                            || actual_source.ends_with(&alias.from)
                            || alias.from.ends_with(actual_source)
                        {
                            // Register required namespace imports for this foreign type
                            register_foreign_type_namespaces(ft, actual_source);
                            return ForeignTypeMatch::matched(ft);
                        }
                    }
                }
            }
        }

        if let Some((ft, warning)) = near_match {
            ForeignTypeMatch::near_match(ft, warning)
        } else {
            ForeignTypeMatch::none()
        }
    }
}
