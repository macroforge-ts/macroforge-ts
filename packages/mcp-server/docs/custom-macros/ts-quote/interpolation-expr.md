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

<h2 id="ident-blocks">
    Identifier Concatenation: `&#123;| content |&#125;`
</h2>

When you need to build identifiers dynamically (like `getUser`, `setName`), use the ident block syntax. Everything inside `&#123;| |&#125;` is concatenated without spaces:

```rust
let field_name = "User";

let code = ts_template! {
    function {|get@{field_name}|}() {
        return this.@{field_name.to_lowercase()};
    }
};
```

**Generates:**

```typescript
function getUser() {
  return this.user;
}
```

Without ident blocks, `@&#123;&#125;` always adds a space after for readability. Use `&#123;| |&#125;` when you explicitly want concatenation:

```rust
let name = "Status";

// With space (default behavior)
ts_template! { namespace @{name} }  // → "namespace Status"

// Without space (ident block)
ts_template! { {|namespace@{name}|} }  // → "namespaceStatus"
```

Multiple interpolations can be combined:

```rust
let entity = "user";
let action = "create";

ts_template! { {|@{entity}_@{action}|} }  // → "user_create"
```

<h2 id="comments">
    Comments: `&#123;> "..." <&#125;` and
    `&#123;>> "..." <<&#125;`
</h2>

Since Rust's tokenizer strips whitespace before macros see them, use string literals to preserve exact spacing in comments:

### Block Comments

Use `&#123;> "comment" <&#125;` for block comments:

```rust
let code = ts_template! {
    {> "This is a block comment" <}
    const x = 42;
};
```

**Generates:**

```typescript
/* This is a block comment */
const x = 42;
```

### Doc Comments (JSDoc)

Use `&#123;>> "doc" <<&#125;` for JSDoc comments:

```rust
let code = ts_template! {
    {>> "@param {string} name - The user's name" <<}
    {>> "@returns {string} A greeting message" <<}
    function greet(name: string): string {
        return "Hello, " + name;
    }
};
```

**Generates:**

```typescript
/** @param {string} name - The user's name */
/** @returns {string} A greeting message */
function greet(name: string): string {
    return "Hello, " + name;
}
```

### Comments with Interpolation

Use `format!()` or similar to build dynamic comment strings:

```rust
let param_name = "userId";
let param_type = "number";
let comment = format!("@param {{{}}} {} - The user ID", param_type, param_name);

let code = ts_template! {
    {>> @{comment} <<}
    function getUser(userId: number) {}
};
```

**Generates:**

```typescript
/** @param {number} userId - The user ID */
function getUser(userId: number) {}
```

<h2 id="string-interpolation">
    String Interpolation: `"text @&#123;expr&#125;"`
</h2>

Interpolation works automatically inside string literals - no <code >format!()</code > needed:

```rust
let name = "World";
let count = 42;

let code = ts_template! {
    console.log("Hello @{name}!");
    console.log("Count: @{count}, doubled: @{count * 2}");
};
```

**Generates:**

```typescript
console.log("Hello World!");
console.log("Count: 42, doubled: 84");
```

This also works with method calls and complex expressions:

```rust
let field = "username";

let code = ts_template! {
    throw new Error("Invalid @{field.to_uppercase()}");
};
```

<h2 id="backtick-templates">
    Backtick Template Literals: `"'^...^'"`
</h2>

For JavaScript template literals (backtick strings), use the <code >'^...^'</code > syntax. This outputs actual backticks and passes through `${"${}"}` for JS interpolation:

```rust
let tag_name = "div";

let code = ts_template! {
    const html = "'^<@{tag_name}>\${content}</@{tag_name}>^'";
};
```

**Generates:**

<CodeBlock code={"const html = `${content}`;"} lang="typescript" />

You can mix Rust `@&#123;&#125;` interpolation (evaluated at macro expansion time) with JS `${"${}"}` interpolation (evaluated at runtime):

```rust
let class_name = "User";

let code = ts_template! {
    "'^Hello \${this.name}, you are a @{class_name}^'"
};
```

**Generates:**

<CodeBlock code={"`Hello ${this.name}, you are a User`"} lang="typescript" />

<h2 id="conditionals">
    Conditionals: `&#123;#if&#125;...&#123;/if&#125;`
</h2>

Basic conditional:

```rust
let needs_validation = true;

let code = ts_template! {
    function save() {
        {#if needs_validation}
            if (!this.isValid()) return false;
        {/if}
        return this.doSave();
    }
};
```

### If-Else

```rust
let has_default = true;

let code = ts_template! {
    {#if has_default}
        return defaultValue;
    {:else}
        throw new Error("No default");
    {/if}
};
```

### If-Else-If Chains

```rust
let level = 2;

let code = ts_template! {
    {#if level == 1}
        console.log("Level 1");
    {:else if level == 2}
        console.log("Level 2");
    {:else}
        console.log("Other level");
    {/if}
};
```

<h2 id="pattern-matching">
    Pattern Matching: `&#123;#if let&#125;`
</h2>

Use `if let` for pattern matching on `Option`, `Result`, or other Rust enums:

```rust
let maybe_name: Option<&str> = Some("Alice");

let code = ts_template! {
    {#if let Some(name) = maybe_name}
        console.log("Hello, @{name}!");
    {:else}
        console.log("Hello, anonymous!");
    {/if}
};
```

**Generates:**

```typescript
console.log("Hello, Alice!");
```

This is useful when working with optional values from your IR:

```rust
let code = ts_template! {
    {#if let Some(default_val) = field.default_value}
        this.@{field.name} = @{default_val};
    {:else}
        this.@{field.name} = undefined;
    {/if}
};
```

<h2 id="match-expressions">
    Match Expressions: `&#123;#match&#125;`
</h2>

Use `match` for exhaustive pattern matching:

```rust
enum Visibility { Public, Private, Protected }
let visibility = Visibility::Public;

let code = ts_template! {
    {#match visibility}
        {:case Visibility::Public}
            public
        {:case Visibility::Private}
            private
        {:case Visibility::Protected}
            protected
    {/match}
    field: string;
};
```

**Generates:**

```typescript
public field: string;
```

### Match with Value Extraction

```rust
let result: Result<i32, &str> = Ok(42);

let code = ts_template! {
    const value = {#match result}
        {:case Ok(val)}
            @{val}
        {:case Err(msg)}
            throw new Error("@{msg}")
    {/match};
};
```

### Match with Wildcard

```rust
let count = 5;

let code = ts_template! {
    {#match count}
        {:case 0}
            console.log("none");
        {:case 1}
            console.log("one");
        {:case _}
            console.log("many");
    {/match}
};
```