# Debug

*The `Debug` macro generates a `toString()` method that produces a human-readable string representation of your class.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const user = new User("Alice", 30);
console.log(user.toString());
// Output: User { name: Alice, age: 30 }
```

## Field Options

Use the `@debug` field decorator to customize behavior:

### Renaming Fields

<MacroExample before={data.examples.rename.before} after={data.examples.rename.after} />

```typescript
const user = new User(42, "Alice");
console.log(user.toString());
// Output: User { userId: 42, name: Alice }
```

### Skipping Fields

Use `skip: true` to exclude sensitive fields from the output:

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

```typescript
const user = new User("Alice", "alice@example.com", "secret", "tok_xxx");
console.log(user.toString());
// Output: User { name: Alice, email: alice@example.com }
// Note: password and authToken are not included
```

<Alert type="tip" title="Security">
Always skip sensitive fields like passwords, tokens, and API keys to prevent accidental logging.
</Alert>

## Combining Options

<InteractiveMacro code={`/** @derive(Debug) */
class ApiResponse {
  /** @debug({ rename: "statusCode" }) */
  status: number;

  data: unknown;

  /** @debug({ skip: true }) */
  internalMetadata: Record<string, unknown>;
}`} />

## All Options

| `rename` 
| `string` 
| Display a different name in the output 

| `skip` 
| `boolean` 
| Exclude this field from the output

## Interface Support

Debug also works with interfaces. For interfaces, a namespace is generated with a `toString` function:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const status: Status = { active: true, message: "OK" };
console.log(Status.toString(status));
// Output: Status { active: true, message: OK }
```

## Enum Support

Debug also works with enums. For enums, a namespace is generated with a `toString` function that displays the enum name and variant:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
console.log(Priority.toString(Priority.High));
// Output: Priority.High

console.log(Priority.toString(Priority.Low));
// Output: Priority.Low
```

Works with both numeric and string enums:

<InteractiveMacro code={`/** @derive(Debug) */
enum Status {
  Active = "active",
  Inactive = "inactive",
}`} />

## Type Alias Support

Debug works with type aliases. For object types, fields are displayed similar to interfaces:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const point: Point = { x: 10, y: 20 };
console.log(Point.toString(point));
// Output: Point { x: 10, y: 20 }
```

For union types, the value is displayed using JSON.stringify:

<InteractiveMacro code={`/** @derive(Debug) */
type ApiStatus = "loading" | "success" | "error";`} />

```typescript
console.log(ApiStatus.toString("success"));
// Output: ApiStatus("success")
```