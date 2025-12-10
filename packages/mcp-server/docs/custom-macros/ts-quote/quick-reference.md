## Quick Reference

| `@&#123;expr&#125;` 
| Interpolate a Rust expression (adds space after) 

| `&#123;| content |&#125;` 
| Ident block: concatenates without spaces (e.g., `&#123;|get@&#123;name&#125;|&#125;` → `getUser`) 

| `&#123;> comment <&#125;` 
| Block comment: outputs `/* comment */` 

| `&#123;>> doc <<&#125;` 
| Doc comment: outputs `/** doc */` (for JSDoc) 

| `@@&#123;` 
| Escape for literal `@&#123;` (e.g., `"@@&#123;foo&#125;"` → `@&#123;foo&#125;`) 

| `"text @&#123;expr&#125;"` 
| String interpolation (auto-detected) 

| `"'^template $&#123;js&#125;^'"` 
| JS backtick template literal (outputs ``template $&#123;js&#125;``) 

| `&#123;#if cond&#125;...&#123;/if&#125;` 
| Conditional block 

| `&#123;#if cond&#125;...&#123;:else&#125;...&#123;/if&#125;` 
| Conditional with else 

| `&#123;#if a&#125;...&#123;:else if b&#125;...&#123;:else&#125;...&#123;/if&#125;` 
| Full if/else-if/else chain 

| `&#123;#if let pattern = expr&#125;...&#123;/if&#125;` 
| Pattern matching if-let 

| `&#123;#match expr&#125;&#123;:case pattern&#125;...&#123;/match&#125;` 
| Match expression with case arms 

| `&#123;#for item in list&#125;...&#123;/for&#125;` 
| Iterate over a collection 

| `&#123;#while cond&#125;...&#123;/while&#125;` 
| While loop 

| `&#123;#while let pattern = expr&#125;...&#123;/while&#125;` 
| While-let pattern matching loop 

| `&#123;$let name = expr&#125;` 
| Define a local constant 

| `&#123;$let mut name = expr&#125;` 
| Define a mutable local variable 

| `&#123;$do expr&#125;` 
| Execute a side-effectful expression 

| `&#123;$typescript stream&#125;` 
| Inject a TsStream, preserving its source and runtime_patches (imports)

**Note:** A single `@` not followed by `&#123;` passes through unchanged (e.g., `email@domain.com` works as expected).

## Interpolation: `@&#123;expr&#125;`

Insert Rust expressions into the generated TypeScript:

```rust
let class_name = "User";
let method = "toString";

let code = ts_template! {
    @{class_name}.prototype.@{method} = function() {
        return "User instance";
    };
};
```

**Generates:**

```typescript
User.prototype.toString = function () {
  return "User instance";
};
```