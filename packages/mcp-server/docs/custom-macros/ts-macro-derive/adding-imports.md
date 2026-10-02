## Adding Imports

If your macro generates code that requires imports, use the `add_import` method on `TsStream`:

Rust

```
// Add an import to be inserted at the top of the file
let mut output = ts_template!(Within {
    validate(): ValidationResult {
        return validateFields(this);
    }
});

// Adds: import { validateFields } from "my-validation-lib";
//       import type { ValidationResult } from "my-validation-lib";
output.add_import("validateFields", "my-validation-lib");
output.add_type_import("ValidationResult", "my-validation-lib");

Ok(output)
```

An import the file already has is not added again. Aliased imports, imports resolved from a type's
module and the rest are in [Output and Imports](../../docs/custom-macros/output#imports).
