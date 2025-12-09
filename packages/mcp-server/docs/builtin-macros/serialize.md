# Serialize

*The `Serialize` macro generates a `toJSON()` method that converts your object to a JSON-compatible format with automatic handling of complex types like Date, Map, Set, and nested objects.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const user = new User("Alice", 30);
console.log(JSON.stringify(user));
// {"name":"Alice","age":30,"createdAt":"2024-01-15T10:30:00.000Z"}
```

## Automatic Type Handling

Serialize automatically handles various TypeScript types:

| `string`, `number`, `boolean` 
| Direct copy 

| `Date` 
| `.toISOString()` 

| `T[]` 
| Maps items, calling `toJSON()` if available 

| `Map<K, V>` 
| `Object.fromEntries()` 

| `Set<T>` 
| `Array.from()` 

| Nested objects 
| Calls `toJSON()` if available

## Serde Options

Use the `@serde` decorator for fine-grained control over serialization:

### Renaming Fields

<MacroExample before={data.examples.rename.before} after={data.examples.rename.after} />

```typescript
const user = new User();
user.id = "123";
user.name = "Alice";
console.log(JSON.stringify(user));
// {"user_id":"123","full_name":"Alice"}
```

### Skipping Fields

<MacroExample before={data.examples.skip.before} after={data.examples.skip.after} />

<Alert type="tip" title="skip vs skip_serializing">
Use `skip: true` to exclude from both serialization and deserialization.
Use `skip_serializing: true` to only skip during serialization.
</Alert>

### Rename All Fields

Apply a naming convention to all fields at the container level:

<InteractiveMacro code={`/** @derive(Serialize) */
/** @serde({ rename_all: "camelCase" }) */
class ApiResponse {
  user_name: string;
  created_at: Date;
  is_active: boolean;
}`} />

Supported conventions:

- `camelCase`

- `snake_case`

- `PascalCase`

- `SCREAMING_SNAKE_CASE`

- `kebab-case`

### Flattening Nested Objects

<InteractiveMacro code={`/** @derive(Serialize) */
class Address {
  city: string;
  zip: string;
}

/** @derive(Serialize) */
class User {
  name: string;

  /** @serde({ flatten: true }) */
  address: Address;
}`} />

```typescript
const user = new User();
user.name = "Alice";
user.address = { city: "NYC", zip: "10001" };
console.log(JSON.stringify(user));
// {"name":"Alice","city":"NYC","zip":"10001"}
```

## All Options

### Container Options (on class/interface)

| `rename_all` 
| `string` 
| Apply naming convention to all fields

### Field Options (on properties)

| `rename` 
| `string` 
| Use a different JSON key 

| `skip` 
| `boolean` 
| Exclude from serialization and deserialization 

| `skip_serializing` 
| `boolean` 
| Exclude from serialization only 

| `flatten` 
| `boolean` 
| Merge nested object fields into parent

## Interface Support

Serialize also works with interfaces. For interfaces, a namespace is generated with a `toJSON` function:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const response: ApiResponse = {
  status: 200,
  message: "OK",
  timestamp: new Date()
};

console.log(JSON.stringify(ApiResponse.toJSON(response)));
// {"status":200,"message":"OK","timestamp":"2024-01-15T10:30:00.000Z"}
```

## Enum Support

Serialize also works with enums. The `toJSON` function returns the underlying enum value (string or number):

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
console.log(Status.toJSON(Status.Active));  // "active"
console.log(Status.toJSON(Status.Pending)); // "pending"
```

Works with numeric enums too:

<InteractiveMacro code={`/** @derive(Serialize) */
enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
}`} />

```typescript
console.log(Priority.toJSON(Priority.High)); // 3
```

## Type Alias Support

Serialize works with type aliases. For object types, fields are serialized with full type handling:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const profile: UserProfile = {
  id: "123",
  name: "Alice",
  createdAt: new Date("2024-01-15")
};

console.log(JSON.stringify(UserProfile.toJSON(profile)));
// {"id":"123","name":"Alice","createdAt":"2024-01-15T00:00:00.000Z"}
```

For union types, the value is returned directly:

<InteractiveMacro code={`/** @derive(Serialize) */
type ApiStatus = "loading" | "success" | "error";`} />

```typescript
console.log(ApiStatus.toJSON("success")); // "success"
```

## Combining with Deserialize

Use both Serialize and Deserialize for complete JSON round-trip support:

<InteractiveMacro code={`/** @derive(Serialize, Deserialize) */
class User {
  name: string;
  createdAt: Date;
}`} />

```typescript
// Serialize
const user = new User();
user.name = "Alice";
user.createdAt = new Date();
const json = JSON.stringify(user);

// Deserialize
const parsed = User.fromJSON(JSON.parse(json));
console.log(parsed.createdAt instanceof Date); // true
```