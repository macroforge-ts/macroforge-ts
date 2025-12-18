## Adding Imports

If your macro generates code that requires imports, use the `add_import` method on `TsStream`:

Rust

```
// Add an import to be inserted at the top of the file
let mut output = body! {
    validate(): ValidationResult {
        return validateFields(this);
    }
};

// This will add: import { validateFields, ValidationResult } from "my-validation-lib";
output.add_import("validateFields", "my-validation-lib");
output.add_import("ValidationResult", "my-validation-lib");

Ok(output)
```

Note

Imports are automatically deduplicated. If the same import already exists in the file, it won't be added again.