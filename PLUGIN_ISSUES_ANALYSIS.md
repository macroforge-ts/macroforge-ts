# TypeScript Language Service Plugin Issues - Analysis

## Summary
The ts-derive-plugin has two critical issues causing incorrect error reporting:

1. **Malformed Generated Code** - Methods are jammed on same line as closing braces
2. **Position Mapping Failure** - No source maps between original and expanded code

## Problem Details

### 1. Malformed Code Generation

**What's happening:**
When the Rust macro expands `@Derive` decorators, it generates type definitions where methods are concatenated onto the same line as the constructor closing brace/semicolon.

**Example:**
```typescript
// Original (line 15)
constructor(id: number, name: string, email: string, authToken: string) {

// Expanded becomes (line 12-14)
constructor(id: number, name: string, email: string, authToken: string);
  toString(): string;
toJSON(): Record<string, unknown>;;}
```

**Root cause:**
`crates/ts_macro_host/src/patch_applicator.rs:184-199`
- The `emit_node()` function uses SWC's emitter to serialize AST nodes
- When inserting at `body_span.end - 1` (before closing brace), it doesn't add proper indentation/newlines
- SWC emitter output is directly concatenated without formatting

### 2. Line Number Misalignment

**What's happening:**
- Original file: 24 lines
- Expanded file: 17 lines (-7 lines from removing decorators and method bodies)
- TypeScript reports errors on line 10 of expanded code as line 10 of original code
- But line 10 in original is `email: string;` while line 10 in expanded is `authToken: string;`

**Impact:**
Errors appear on completely wrong lines in the editor, making them confusing and hard to fix.

### 3. Missing Methods Errors (TS2339)

Errors like:
```
playground/vanilla/src/main.ts(10,28): error TS2339: Property 'toJSON' does not exist on type 'User'.
```

**Root cause:**
The plugin sometimes provides the original snapshot instead of the expanded one. Possible causes:
- Cache invalidation issues
- Race conditions between snapshot updates
- Plugin not being invoked for all file reads

### 4. Unrelated Errors

Most TS2307 errors in the logs are legitimate missing dependencies:
- `'runed'` - Missing npm package
- `'json-schema'` - Missing npm package
- `'$lib/utils/nested-values.util.js'` - Missing source files
- `'tailwind-merge'` - Missing npm package

These are NOT plugin issues, just missing dependencies in the Svelte playground.

## Diagnostic Test Results

Running `expandSync()` directly on `user.ts`:
- ✓ Expansion succeeds
- ✓ Types are generated
- ✓ No diagnostics reported
- ✗ Output has malformed formatting (methods on same line)
- ✗ Line count mismatch (24 → 17 lines)

## Recommendations

### Immediate Fixes

1. **Fix Code Formatting** (High Priority)
   - Modify `crates/ts_macro_host/src/patch_applicator.rs`
   - Add proper indentation and newlines when inserting ClassMembers
   - Insert before closing brace with leading newline and proper indentation

2. **Add Source Mapping** (High Priority)
   - Track position mappings between original and expanded code
   - Store mappings in the plugin cache alongside expanded output
   - Remap diagnostic positions back to original positions

3. **Filter Generated Code Diagnostics** (Medium Priority)
   - Identify which diagnostics reference generated code positions
   - Either filter them out or remap to original positions
   - Consider adding special comments to mark generated regions

4. **Improve Caching** (Medium Priority)
   - Add logging to track when snapshots are vs aren't expanded
   - Verify cache key includes all relevant file state
   - Ensure expansion happens before TypeScript semantic analysis

### Long-term Solutions

1. **Source Map Generation**
   - Generate proper TypeScript source maps
   - Use `//# sourceMappingURL=` comments
   - Let TypeScript handle position mapping automatically

2. **Alternative Architecture**
   - Instead of replacing entire file snapshot, consider:
     - Virtual file system with separate `.d.ts` files
     - Type-only augmentation via ambient declarations
     - Declaration merging with module augmentation

3. **Better IDE Integration**
   - Provide "go to macro definition" support
   - Show both original and expanded code in hover tooltips
   - Add quick fixes for macro-related errors

## Test Case for Validation

```typescript
import { Derive, Debug } from "@ts-macros/macros";

@Derive("Debug", "JSON")
class User {
  @Debug({ rename: "identifier" })
  id: number;
  name: string;

  constructor(id: number, name: string) {
    this.id = id;
    this.name = name;
  }
}

const user = new User(1, "John");
user.toString(); // Should have no error
user.toJSON();   // Should have no error
```

Expected behavior:
- ✓ No TS2339 errors on `toString()` or `toJSON()`
- ✓ Correct line numbers if errors occur elsewhere
- ✓ Readable expanded type definition
- ✓ Consistent behavior across all TS operations

## Files to Modify

1. `crates/ts_macro_host/src/patch_applicator.rs` - Fix formatting
2. `crates/swc-napi-macros/src/macro_host.rs` - Track line mappings
3. `packages/ts-derive-plugin/src/index.ts` - Add source mapping and diagnostic filtering
