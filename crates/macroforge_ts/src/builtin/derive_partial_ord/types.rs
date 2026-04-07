/// Contains field information needed for partial ordering comparison generation.
///
/// Each field that participates in ordering is represented by this struct,
/// which captures both the field name (for access) and its TypeScript type
/// (to select the appropriate comparison strategy).
pub(crate) struct OrdField {
    /// The field name as it appears in the source TypeScript class.
    /// Used to generate property access expressions like `this.name`.
    pub name: String,

    /// The TypeScript type annotation for this field.
    /// Used to determine which comparison strategy to apply
    /// (e.g., numeric comparison, string localeCompare, recursive compareTo).
    pub ts_type: String,
}
