# Command Line Interface

*The `macroforge` CLI provides commands for expanding macros and running type checks with macro support.*

## Installation

The CLI is a Rust binary. You can install it using Cargo:

```bash
cargo install macroforge_ts
```

Or build from source:

```bash
git clone https://github.com/rymskip/macroforge-ts.git
cd macroforge-ts/crates
cargo build --release --bin macroforge

# The binary is at target/release/macroforge
```

## Commands

### macroforge expand

Expands macros in a TypeScript file and outputs the transformed code.

```bash
macroforge expand <input> [options]
```

#### Arguments

| `<input>` 
| Path to the TypeScript or TSX file to expand

#### Options

| `--out <path>` 
| Write the expanded JavaScript/TypeScript to a file 

| `--types-out <path>` 
| Write the generated `.d.ts` declarations to a file 

| `--print` 
| Print output to stdout even when `--out` is specified 

| `--builtin-only` 
| Use only built-in Rust macros (faster, but no external macro support)

#### Examples

Expand a file and print to stdout:

```bash
macroforge expand src/user.ts
```

Expand and write to a file:

```bash
macroforge expand src/user.ts --out dist/user.js
```

Expand with both runtime output and type declarations:

```bash
macroforge expand src/user.ts --out dist/user.js --types-out dist/user.d.ts
```

Use fast built-in macros only (no external macro support):

```bash
macroforge expand src/user.ts --builtin-only
```

>
> By default, the CLI uses Node.js for full macro support (including external macros). It must be run from your project's root directory where `macroforge` and any external macro packages are installed in `node_modules`.

### macroforge tsc

Runs TypeScript type checking with macro expansion. This wraps `tsc --noEmit` and expands macros before type checking, so your generated methods are properly type-checked.

```bash
macroforge tsc [options]
```

#### Options

| `-p, --project <path>` 
| Path to `tsconfig.json` (defaults to `tsconfig.json` in current directory)

#### Examples

Type check with default tsconfig.json:

```bash
macroforge tsc
```

Type check with a specific config:

```bash
macroforge tsc -p tsconfig.build.json
```

## Output Format

### Expanded Code

When expanding a file like this:

```typescript
/** @derive(Debug) */
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}
```

The CLI outputs the expanded code with the generated methods:

```typescript
class User {
  name: string;
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }

  [Symbol.for("nodejs.util.inspect.custom")](): string {
    return \`User { name: \${this.name}, age: \${this.age} }\`;
  }
}
```

### Diagnostics

Errors and warnings are printed to stderr in a readable format:

```text
[macroforge] error at src/user.ts:5:1: Unknown derive macro: InvalidMacro
[macroforge] warning at src/user.ts:10:3: Field 'unused' is never used
```

## Use Cases

### CI/CD Type Checking

Use `macroforge tsc` in your CI pipeline to type-check with macro expansion:

```json
# package.json
{
  "scripts": {
    "typecheck": "macroforge tsc"
  }
}
```

### Debugging Macro Output

Use `macroforge expand` to inspect what code your macros generate:

```bash
macroforge expand src/models/user.ts | less
```

### Build Pipeline

Generate expanded files as part of a custom build:

```bash
#!/bin/bash
for file in src/**/*.ts; do
  outfile="dist/$(basename "$file" .ts).js"
  macroforge expand "$file" --out "$outfile"
done
```

## Built-in vs Full Mode

By default, the CLI uses Node.js for full macro support including external macros. Use `--builtin-only` for faster expansion when you only need built-in macros:

| Built-in macros 
| Yes 
| Yes 

| External macros 
| Yes 
| No 

| Performance 
| Standard 
| Faster 

| Dependencies 
| Requires `macroforge` in node_modules 
| None