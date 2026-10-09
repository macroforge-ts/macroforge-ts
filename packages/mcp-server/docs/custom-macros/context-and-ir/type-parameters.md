## Type parameters

Every generic declaration lists its type parameters as declared. `declaration()` renders one as a
generic function would declare it, and `declare_all` and `apply_all` render a whole list as
`<T extends Shape>` and `<T>`. Variance annotations (`in`, `out`) are not kept: TypeScript rejects
them on a function.

Rust

```
pub struct TypeParamIR {
    pub name: String,
    pub constraint: Option<String>,  // the type after \`extends\`
    pub default: Option<String>,     // the type after \`=\`
    pub is_const: bool,
}
```
