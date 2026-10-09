## The Project's Config

`ctx.config` is the project's `macroforge.config`, when there is one:

Rust

```
pub struct MacroforgeConfig {
    pub keep_decorators: bool,
    pub generate_convenience_const: bool,
    pub foreign_types: Vec<ForeignTypeConfig>,
    pub cfg: CfgFlags,                 // features, target, debug_assertions, custom
    pub deprecated: DeprecatedConfig,
    pub must_use: MustUseConfig,
    pub non_exhaustive: NonExhaustiveConfig,
    pub buildtime: BuildtimeConfig,
    pub config_imports: HashMap<String, ImportInfo>, // the config file's own imports
}
```

[Foreign types](../../docs/endec/foreign-types) are the usual reason to read it: a macro that
handles field types itself should treat a configured foreign type, such as `DateTime.DateTime`, the
way the project configured it.

Rust

```
let foreign = input
    .context
    .config
    .as_ref()
    .and_then(|config| config.foreign_types.iter().find(|ft| ft.name == field.ts_type));

if let Some(foreign) = foreign {
    // foreign.encode_expr, decode_expr, default_expr, has_shape_expr
    // hold the configured expressions, as source text
}
```

## Next Steps

- [Context and IR](../../docs/custom-macros/context-and-ir)
- [Testing and Debugging](../../docs/custom-macros/testing-and-debugging)
