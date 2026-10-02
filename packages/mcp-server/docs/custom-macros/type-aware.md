# Type-Aware Macros

Before expanding, Macroforge scans the project and records every class, interface, enum and type
alias it finds. A macro can look any of them up, so it can generate code that depends on what a
field's type really is, not just on its name.

## The Type Registry

The context's `type_registry` holds the scan. Each entry is a type's IR, where it is declared and
the imports of its file:

Rust

```
pub struct TypeRegistryEntry {
    pub name: String,                     // "User"
    pub file_path: String,                // where it is declared
    pub is_exported: bool,
    pub definition: TypeDefinitionIR,     // Class, Interface, Enum or TypeAlias IR
    pub file_imports: Vec<FileImportEntry>, // that file's imports
}
```

### Looking Types Up

| Method                                 | Finds                                                                                                                             |
| -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `resolve_in_file(name, file, imports)` | The type `name` means in `file`: what it imports under that name, or what it declares. Use this one for a name read from a field. |
| `resolve(name, imports)`               | The same, without the same-file check                                                                                             |
| `get(name)`                            | The type by name alone; `None` when several files declare that name                                                               |
| `get_all(name)`                        | Every declaration with that name                                                                                                  |
| `get_qualified("src/user.ts::User")`   | A declaration in a given file                                                                                                     |
| `alias_cycle(name)`                    | The loop, if following `name`'s aliases leads back to it                                                                          |

Rust

```
use macroforge_ts::ts_syn::TypeDefinitionIR;

let ctx = &input.context;
let imports = ctx.import_registry.file_import_entries();
let resolved_fields = ctx.resolved_fields.as_ref();

for field in class.fields() {
    // The field's type with arrays and generics stripped: "User" for "User[]"
    let Some(resolved) = resolved_fields.and_then(|fields| fields.get(&field.name)) else {
        continue;
    };
    let Some(entry) =
        ctx.type_registry.resolve_in_file(&resolved.base_type_name, &ctx.file_name, &imports)
    else {
        continue; // a primitive, a generic parameter, or a type outside the project
    };
    if let TypeDefinitionIR::Enum(enum_ir) = &entry.definition {
        // the field holds one of enum_ir.variants
    }
}
```

Note

Macroforge records every lookup a macro makes, so when a looked-up type changes, the files whose
expansion depended on it are expanded again. Look types up through the registry rather than reading
other files yourself, or a change will not reach your macro's output.

### Resolved Field Types

For a derive on a class or interface, `resolved_fields` has every field's type already followed,
keyed by field name:

Rust

```
pub struct ResolvedTypeRef {
    pub raw_type: String,           // "User[]"
    pub base_type_name: String,     // "User"
    pub registry_key: Option<String>, // the registry entry, when it is a project type
    pub is_collection: bool,        // an array, Set, Map or the like
    pub is_optional: bool,          // | null or | undefined
    pub type_args: Vec<ResolvedTypeRef>, // Map<string, User> → [string, User]
}
```

Rust

```
use macroforge_ts::builtin::derive_common::resolved_type_has_derive;

if let Some(resolved) = input.context.resolved_fields.as_ref().and_then(|f| f.get(&field.name)) {
    // Does the field's type, or its element type, also derive Clone?
    if resolved_type_has_derive(&input.context.type_registry, resolved, "Clone") {
        // call the type's generated clone helper instead of copying
    }
}
```

### Generic Aliases

`resolve_generic_aliases` expands every generic alias in a type string, so a macro sees the shape
behind it. With `type Link<T> = string | T`, `Link<User>[]` becomes `(string | User)[]`:

Rust

```
use macroforge_ts::ts_syn::resolve_generic_aliases;

let shape = resolve_generic_aliases(
    &field.ts_type,
    &ctx.type_registry,
    &ctx.file_name,
    &ctx.import_registry.file_import_entries(),
);
```

### Helpers

`macroforge_ts::builtin::derive_common` holds the helpers the built-in macros use:

| Helper                                                                        | Use                                                          |
| ----------------------------------------------------------------------------- | ------------------------------------------------------------ |
| `type_has_derive(registry, name, "Clone")`                                    | Whether a project type derives a macro                       |
| `resolved_type_has_derive(registry, resolved, "Clone")`                       | The same, for a field's resolved type                        |
| `collection_element_type(resolved)`                                           | The element of `User[]`, a `Set`'s value, a `Map`'s value    |
| `standalone_fn_name("User", "Clone")`                                         | The name of a generated helper: `userClone`                  |
| `get_effective_fields(alias, registry)`                                       | The fields of an object alias, or of an intersection of them |
| `is_primitive_type`, `is_numeric_type`, `is_nullable_type`, `is_generic_type` | Classify a type string                                       |
| `get_type_default`, `get_type_default_with_registry`                          | A default value expression for a type                        |

## Importing a Type's Helpers

Code generated for one type often calls helpers generated beside another, in another module.
`ctx.import_specifier_for("User")` gives the module the current file should import from: the one it
already imports `User` from, or else the relative path to `User`'s file. The stream methods in
[Output and Imports](../../docs/custom-macros/output#imports) build on it.

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

[Foreign types](../../docs/serde/foreign-types) are the usual reason to read it: a macro that
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
    // foreign.serialize_expr, deserialize_expr, default_expr, has_shape_expr
    // hold the configured expressions, as source text
}
```

## Next Steps

- [Context and IR](../../docs/custom-macros/context-and-ir)
- [Testing and Debugging](../../docs/custom-macros/testing-and-debugging)
