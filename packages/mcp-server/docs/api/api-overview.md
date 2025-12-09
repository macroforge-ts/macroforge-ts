# API Reference

*Macroforge provides a programmatic API for expanding macros in TypeScript code.*

## Overview

```typescript
import {
  expandSync,
  transformSync,
  checkSyntax,
  parseImportSources,
  NativePlugin,
  PositionMapper
} from "macroforge";
```

## Core Functions

| [`expandSync()`]({base}/docs/api/expand-sync) 
| Expand macros synchronously 

| [`transformSync()`]({base}/docs/api/transform-sync) 
| Transform code with additional metadata 

| `checkSyntax()` 
| Validate TypeScript syntax 

| `parseImportSources()` 
| Extract import information

## Classes

| [`NativePlugin`]({base}/docs/api/native-plugin) 
| Stateful plugin with caching 

| [`PositionMapper`]({base}/docs/api/position-mapper) 
| Maps positions between original and expanded code

## Quick Example

```typescript
import { expandSync } from "macroforge";

const sourceCode = \`
/** @derive(Debug) */
class User {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}
\`;

const result = expandSync(sourceCode, "user.ts", {
  keepDecorators: false
});

console.log(result.code);
// Output: class with toString() method generated

if (result.diagnostics.length > 0) {
  console.error("Errors:", result.diagnostics);
}
```

## Detailed Reference

- [`expandSync()`]({base}/docs/api/expand-sync) - Full options and return types

- [`transformSync()`]({base}/docs/api/transform-sync) - Transform with source maps

- [`NativePlugin`]({base}/docs/api/native-plugin) - Caching for language servers

- [`PositionMapper`]({base}/docs/api/position-mapper) - Position mapping utilities