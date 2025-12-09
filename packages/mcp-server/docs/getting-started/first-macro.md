# Your First Macro

*Let's create a class that uses Macroforge's derive macros to automatically generate useful methods.*

## Creating a Class with Derive Macros

Start by creating a simple `User` class. We'll use the `@derive` decorator to automatically generate methods.

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

## Using the Generated Methods

```typescript
const user = new User("Alice", 30, "alice@example.com");

// Debug: toString()
console.log(user.toString());
// Output: User { name: Alice, age: 30, email: alice@example.com }

// Clone: clone()
const copy = user.clone();
console.log(copy.name); // "Alice"

// Eq: equals()
console.log(user.equals(copy)); // true

const different = new User("Bob", 25, "bob@example.com");
console.log(user.equals(different)); // false
```

## Customizing Behavior

You can customize how macros work using field-level decorators. For example, with the Debug macro:

<MacroExample before={data.examples.customizing.before} after={data.examples.customizing.after} />

```typescript
const user = new User(42, "Alice", "secret123");
console.log(user.toString());
// Output: User { userId: 42, name: Alice }
// Note: 'id' is renamed to 'userId', 'password' is skipped
```

<Alert type="tip" title="Field-level decorators">
Field-level decorators let you control exactly how each field is handled by the macro.
</Alert>

## Next Steps

- [Learn how macros work under the hood]({base}/docs/concepts)

- [Explore all Debug options]({base}/docs/builtin-macros/debug)

- [Create your own custom macros]({base}/docs/custom-macros)