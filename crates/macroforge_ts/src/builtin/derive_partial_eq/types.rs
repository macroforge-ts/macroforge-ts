/// Contains field information needed for equality comparison generation.
///
/// Each field that participates in equality checking is represented by this struct,
/// which captures both the field name (for access) and its TypeScript type
/// (to select the appropriate comparison strategy).
pub struct EqField {
    /// The field name as it appears in the source TypeScript class.
    /// Used to generate property access expressions like `this.name`.
    pub name: String,

    /// The TypeScript type annotation for this field.
    /// Used to determine which comparison strategy to apply
    /// (e.g., strict equality for primitives, recursive equals for objects).
    pub ts_type: String,
}
