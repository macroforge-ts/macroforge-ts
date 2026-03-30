/// Contains field information needed for hash code generation.
///
/// Each field that participates in hashing is represented by this struct,
/// which captures both the field name (for access) and its TypeScript type
/// (to select the appropriate hashing strategy).
pub struct HashField {
    /// The field name as it appears in the source TypeScript class.
    /// Used to generate property access expressions like `this.name`.
    pub name: String,

    /// The TypeScript type annotation for this field.
    /// Used to determine which hashing strategy to apply
    /// (e.g., numeric comparison, string hashing, recursive hashCode call).
    pub ts_type: String,
}
