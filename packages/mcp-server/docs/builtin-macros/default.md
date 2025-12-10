# Default

*The `Default` macro generates a static `defaultValue()` factory method that creates instances with default field values. It works like Rust's `#[derive(Default)]` trait.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const config = Config.defaultValue();
console.log(config.host);    // ""
console.log(config.port);    // 0
console.log(config.enabled); // false
```

## Automatic Default Values

Like Rust's `Default` trait, the macro automatically determines default values for primitive types and common collections:

| `string` | `""` | `String::default()` 

| `number` | `0` | `i32::default()` 

| `boolean` | `false` | `bool::default()` 

| `bigint` | `0n` | `i64::default()` 

| `T[]` / `Array<T>` | `[]` | `Vec::default()` 

| `Map<K, V>` | `new Map()` | `HashMap::default()` 

| `Set<T>` | `new Set()` | `HashSet::default()` 

| `Date` | `new Date()` | — 

| `T | null` / `T | undefined` | `null` | `Option::default()` 

| Custom types | **Error** | **Error** (needs `impl Default`)

## Nullable Types (like Rust's Option)

Just like Rust's `Option<T>` defaults to `None`, nullable TypeScript types automatically default to `null`:

<MacroExample before={data.examples.nullable.before} after={data.examples.nullable.after} />

```typescript
const user = User.defaultValue();
console.log(user.name);     // ""
console.log(user.email);    // null (nullable type)
console.log(user.age);      // 0
console.log(user.metadata); // null (nullable type)
```

## Custom Types Require @default

Just like Rust requires `impl Default` for custom types, Macroforge requires the `@default()` decorator on fields with non-primitive types:

<MacroExample before={data.examples.customType.before} after={data.examples.customType.after} />

<p class="text-red-500 text-sm mt-2">
Without `@default` on custom type fields, the macro will emit an error:
</p>

```text
// Error: @derive(Default) cannot determine default for non-primitive fields.
// Add @default(value) to: settings, permissions
```

## Custom Default Values

Use the `@default()` decorator to specify custom default values for any field:

<MacroExample before={data.examples.custom.before} after={data.examples.custom.after} />

```typescript
const config = ServerConfig.defaultValue();
console.log(config.host);      // "localhost"
console.log(config.port);      // 8080
console.log(config.enabled);   // true
console.log(config.logLevels); // ["info", "error"]
```

## Interface Support

Default also works with interfaces. For interfaces, a namespace is generated with a `defaultValue()` function:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const origin = Point.defaultValue();
console.log(origin); // { x: 0, y: 0 }
```

## Enum Support

Default works with enums. For enums, it returns the first variant as the default value:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
const defaultStatus = Status.defaultValue();
console.log(defaultStatus); // "pending"
```

## Type Alias Support

Default works with type aliases. For object types, it creates an object with default field values:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const dims = Dimensions.defaultValue();
console.log(dims); // { width: 0, height: 0 }
```

## Combining with Other Macros

<InteractiveMacro code={`/** @derive(Default, Debug, Clone, PartialEq) */
class User {
  /** @default("Anonymous") */
  name: string;

  /** @default(0) */
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}`} />

```typescript
const user1 = User.defaultValue();
const user2 = user1.clone();

console.log(user1.toString());    // User { name: "Anonymous", age: 0 }
console.log(user1.equals(user2)); // true
```