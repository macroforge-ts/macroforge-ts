# API Reference

macroforge v0.1.42 55 exported items

Macroforge provides a programmatic API for expanding macros in TypeScript code.

## Overview

TypeScript

```
import {
  expandSync,
  transformSync,
  checkSyntax,
  parseImportSources,
  NativePlugin,
  PositionMapper
} from "macroforge";
```

## Core Functions

| Function                                        | Description                             |
| ----------------------------------------------- | --------------------------------------- |
| [`expandSync()`](../docs/api/expand-sync)       | Expand macros synchronously             |
| [`transformSync()`](../docs/api/transform-sync) | Transform code with additional metadata |
| `checkSyntax()`                                 | Validate TypeScript syntax              |
| `parseImportSources()`                          | Extract import information              |

## Classes

| Class                                           | Description                                       |
| ----------------------------------------------- | ------------------------------------------------- |
| [`NativePlugin`](../docs/api/native-plugin)     | Stateful plugin with caching                      |
| [`PositionMapper`](../docs/api/position-mapper) | Maps positions between original and expanded code |

## Quick Example

TypeScript

```
import { expandSync } from "macroforge";

const sourceCode = \`
/** @derive(Debug) */
class User {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}
\`;

const result = expandSync(sourceCode, "user.ts", {
  keepDecorators: false
});

console.log(result.code);
// Output: class with toString() method generated

if (result.diagnostics.length > 0) {
  console.error("Errors:", result.diagnostics);
}
```

## Detailed Reference

*   [`expandSync()`](../docs/api/expand-sync) - Full options and return types
*   [`transformSync()`](../docs/api/transform-sync) - Transform with source maps
*   [`NativePlugin`](../docs/api/native-plugin) - Caching for language servers
*   [`PositionMapper`](../docs/api/position-mapper) - Position mapping utilities