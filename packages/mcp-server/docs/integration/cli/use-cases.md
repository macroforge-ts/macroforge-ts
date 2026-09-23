## Use Cases

### CI/CD Type Checking

Use `macroforge tsc` in your CI pipeline to type-check with macro expansion:

JSON

```
# package.json
{
  "scripts": {
    "typecheck": "macroforge tsc"
  }
}
```

### Debugging Macro Output

Use `macroforge expand` to inspect what code your macros generate:

Bash

```
macroforge expand src/models/user.ts --print | less
```

### Build Pipeline

Generate expanded files as part of a custom build:

Bash

```
#!/bin/bash
for file in src/**/*.ts; do
  outfile="dist/$(basename "$file" .ts).js"
  macroforge expand "$file" --out "$outfile"
done
```
