# transformSync()

*A lower-level transform function that returns additional metadata alongside the transformed code.*

## Signature

```typescript
function transformSync(
  code: string,
  filepath: string
): TransformResult
```

## Parameters

| `code` 
| `string` 
| TypeScript source code to transform 

| `filepath` 
| `string` 
| File path (used for error reporting)

## TransformResult

```typescript
interface TransformResult {
  // Transformed TypeScript code
  code: string;

  // Source map (JSON string, not yet implemented)
  map?: string;

  // Generated type declarations
  types?: string;

  // Macro expansion metadata
  metadata?: string;
}
```

## Comparison with expandSync()

| Options 
| Yes 
| No 

| Diagnostics 
| Yes 
| No 

| Source Mapping 
| Yes 
| Limited 

| Use Case 
| General purpose 
| Build tools

## Example

```typescript
import { transformSync } from "macroforge";

const sourceCode = \`
/** @derive(Debug) */
class User {
  name: string;
}
\`;

const result = transformSync(sourceCode, "user.ts");

console.log(result.code);

if (result.types) {
  // Write to .d.ts file
  fs.writeFileSync("user.d.ts", result.types);
}

if (result.metadata) {
  // Parse and use metadata
  const meta = JSON.parse(result.metadata);
  console.log("Macros expanded:", meta);
}
```

## When to Use

Use `transformSync` when:

- Building custom integrations

- You need raw output without diagnostics

- You're implementing a build tool plugin

Use `expandSync` for most other use cases, as it provides better error handling.