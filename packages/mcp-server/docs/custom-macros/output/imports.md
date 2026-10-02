## Imports

Generated code that calls a helper needs an import for it. The import methods on `TsStream` register
the import, and the host writes every requested import at the top of the file when the expansion is
done. A request is skipped when the file already imports, or another macro already requested, the
same local name, so asking twice is harmless.

### Plain Imports

Rust

```
// import { validate } from "my-validation-lib";
output.add_import("validate", "my-validation-lib");

// import type { Result } from "my-validation-lib";
output.add_type_import("Result", "my-validation-lib");

// import { validate as runValidate } from "my-validation-lib";
output.add_import("validate as runValidate", "my-validation-lib");
```

### Aliased Imports

A helper imported under its own name can clash with a name the user's file already uses. An alias
avoids that:

Rust

```
// import { resultOk as __mf_resultOk } from "@my/runtime";
output.add_aliased_import("resultOk", "@my/runtime");
output.add_aliased_type_import("Options", "@my/runtime"); // __mf_Options

// Any alias you choose
output.add_import_as("resultOk", "myResultOk", "@my/runtime");
output.add_type_import_as("Options", "MyOptions", "@my/runtime");
```

Generated code then refers to the alias, as in `__mf_resultOk(value)`. A macro with a fixed set of
runtime imports can declare them once:

Rust

```
use macroforge_ts::ts_syn::ImportConfig;

const RUNTIME_IMPORTS: &[ImportConfig] = &[
    ImportConfig::value("resultOk", "__mf_resultOk", "@my/runtime"),
    ImportConfig::type_only("Options", "__mf_Options", "@my/runtime"),
];

output.add_imports(RUNTIME_IMPORTS);
```

### Imports Resolved From a Type

A helper generated beside a type, such as `userValidate` next to `User`, lives in the type's module.
These methods find that module through the project's
[type registry](../../docs/custom-macros/type-aware) and the file's own imports, and do nothing when
the type is in the same file or unknown:

Rust

```
// The module the current file imports \`User\` from, if any
let module: Option<String> = output.module_specifier_for("User");

// import { userValidate } from "<the module of User>";
let added: bool = output.add_import_for("userValidate", "User");
output.add_type_import_for("UserErrors", "User");

// Several helpers from one module, resolving it once: (name, type-only?)
output.add_helpers_for("User", &[("userValidate", false), ("UserErrors", true)]);
```

### Cross-Module Suffixes

When a macro generates calls to helpers that other macros generate for other types, it can name the
helpers' suffix instead of resolving each one. With the suffix `GetFields` registered, a generated
call to `companyNameGetFields()` gets `import { companyNameGetFields }` from wherever the file
imports `CompanyName`:

Rust

```
// {camelCaseType}GetFields(...) calls are imported from the type's module
output.add_cross_module_suffix("GetFields");

// {PascalCaseType}Errors type references get an \`import type\`
output.add_cross_module_type_suffix("Errors");
```
