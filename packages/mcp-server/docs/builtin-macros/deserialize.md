# Deserialize

*The `Deserialize` macro generates a static `fromJSON()` method that parses JSON data into your class with runtime validation and automatic type conversion.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const json = '{"name":"Alice","age":30,"createdAt":"2024-01-15T10:30:00.000Z"}';
const user = User.fromJSON(JSON.parse(json));

console.log(user.name);                    // "Alice"
console.log(user.age);                     // 30
console.log(user.createdAt instanceof Date); // true
```

## Runtime Validation

Deserialize validates the input data and throws descriptive errors:

<InteractiveMacro code={`/** @derive(Deserialize) */
class User {
  name: string;
  email: string;
}`} />

```typescript
// Missing required field
User.fromJSON({ name: "Alice" });
// Error: User.fromJSON: missing required field "email"

// Wrong type
User.fromJSON("not an object");
// Error: User.fromJSON: expected an object, got string

// Array instead of object
User.fromJSON([1, 2, 3]);
// Error: User.fromJSON: expected an object, got array
```

## Automatic Type Conversion

Deserialize automatically converts JSON types to their TypeScript equivalents:

| string/number/boolean 
| `string`/`number`/`boolean` 
| Direct assignment 

| ISO string 
| `Date` 
| `new Date(string)` 

| array 
| `T[]` 
| Maps items with auto-detection 

| object 
| `Map<K, V>` 
| `new Map(Object.entries())` 

| array 
| `Set<T>` 
| `new Set(array)` 

| object 
| Nested class 
| Calls `fromJSON()` if available

## Serde Options

Use the `@serde` decorator to customize deserialization:

### Renaming Fields

<MacroExample before={data.examples.rename.before} after={data.examples.rename.after} />

```typescript
const user = User.fromJSON({ user_id: "123", full_name: "Alice" });
console.log(user.id);   // "123"
console.log(user.name); // "Alice"
```

### Default Values

<MacroExample before={data.examples.default.before} after={data.examples.default.after} />

```typescript
const config = Config.fromJSON({ host: "localhost" });
console.log(config.port);  // "3000"
console.log(config.debug); // false
```

### Skipping Fields

<InteractiveMacro code={`/** @derive(Deserialize) */
class User {
  name: string;
  email: string;

  /** @serde({ skip: true }) */
  cachedData: unknown;

  /** @serde({ skip_deserializing: true }) */
  computedField: string;
}`} />

<Alert type="tip" title="skip vs skip_deserializing">
Use `skip: true` to exclude from both serialization and deserialization.
Use `skip_deserializing: true` to only skip during deserialization.
</Alert>

### Deny Unknown Fields

<InteractiveMacro code={`/** @derive(Deserialize) */
/** @serde({ deny_unknown_fields: true }) */
class StrictUser {
  name: string;
  email: string;
}`} />

```typescript
// This will throw an error
StrictUser.fromJSON({ name: "Alice", email: "a@b.com", extra: "field" });
// Error: StrictUser.fromJSON: unknown field "extra"
```

### Flatten Nested Objects

<InteractiveMacro code={`/** @derive(Deserialize) */
class Address {
  city: string;
  zip: string;
}

/** @derive(Deserialize) */
class User {
  name: string;

  /** @serde({ flatten: true }) */
  address: Address;
}`} />

```typescript
// Flat JSON structure
const user = User.fromJSON({
  name: "Alice",
  city: "NYC",
  zip: "10001"
});
console.log(user.address.city); // "NYC"
```

## All Options

### Container Options (on class/interface)

| `rename_all` 
| `string` 
| Apply naming convention to all fields 

| `deny_unknown_fields` 
| `boolean` 
| Throw error if JSON has unknown keys

### Field Options (on properties)

| `rename` 
| `string` 
| Use a different JSON key 

| `skip` 
| `boolean` 
| Exclude from serialization and deserialization 

| `skip_deserializing` 
| `boolean` 
| Exclude from deserialization only 

| `default` 
| `boolean | string` 
| Use TypeScript default or custom expression if missing 

| `flatten` 
| `boolean` 
| Merge nested object fields from parent

## Interface Support

Deserialize also works with interfaces. For interfaces, a namespace is generated with `is` (type guard) and `fromJSON` functions:

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const json = { status: 200, message: "OK", timestamp: "2024-01-15T10:30:00.000Z" };

// Type guard
if (ApiResponse.is(json)) {
  console.log(json.status); // TypeScript knows this is ApiResponse
}

// Deserialize with validation
const response = ApiResponse.fromJSON(json);
console.log(response.timestamp instanceof Date); // true
```

## Enum Support

Deserialize also works with enums. The `fromJSON` function validates that the input matches one of the enum values:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
const status = Status.fromJSON("active");
console.log(status); // Status.Active

// Invalid values throw an error
try {
  Status.fromJSON("invalid");
} catch (e) {
  console.log(e.message); // "Invalid Status value: invalid"
}
```

Works with numeric enums too:

<InteractiveMacro code={`/** @derive(Deserialize) */
enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
}`} />

```typescript
const priority = Priority.fromJSON(3);
console.log(priority); // Priority.High
```

## Type Alias Support

Deserialize works with type aliases. For object types, validation and type conversion is applied:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const json = {
  id: "123",
  name: "Alice",
  createdAt: "2024-01-15T00:00:00.000Z"
};

const profile = UserProfile.fromJSON(json);
console.log(profile.createdAt instanceof Date); // true
```

For union types, basic validation is applied:

<InteractiveMacro code={`/** @derive(Deserialize) */
type ApiStatus = "loading" | "success" | "error";`} />

```typescript
const status = ApiStatus.fromJSON("success");
console.log(status); // "success"
```

## Combining with Serialize

Use both Serialize and Deserialize for complete JSON round-trip support:

<InteractiveMacro code={`/** @derive(Serialize, Deserialize) */
/** @serde({ rename_all: "camelCase" }) */
class UserProfile {
  user_name: string;
  created_at: Date;
  is_active: boolean;
}`} />

```typescript
// Create and serialize
const profile = new UserProfile();
profile.user_name = "Alice";
profile.created_at = new Date();
profile.is_active = true;

const json = JSON.stringify(profile);
// {"userName":"Alice","createdAt":"2024-...","isActive":true}

// Deserialize back
const restored = UserProfile.fromJSON(JSON.parse(json));
console.log(restored.user_name);              // "Alice"
console.log(restored.created_at instanceof Date); // true
```

## Error Handling

Handle deserialization errors gracefully:

<InteractiveMacro code={`/** @derive(Deserialize) */
class User {
  name: string;
  email: string;
}`} />

```typescript
function parseUser(json: unknown): User | null {
  try {
    return User.fromJSON(json);
  } catch (error) {
    console.error("Failed to parse user:", error.message);
    return null;
  }
}

const user = parseUser({ name: "Alice" });
// Logs: Failed to parse user: User.fromJSON: missing required field "email"
// Returns: null
```