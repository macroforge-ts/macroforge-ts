# Serialize

*The `Serialize` macro generates a `toJSON()` method that converts your object to a JSON-compatible format with automatic handling of complex types like Date, Map, Set, and nested objects.*

## Basic Usage

```typescript
/** @derive(Serialize) */
class User {
  name: string;
  age: number;
  createdAt: Date;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
    this.createdAt = new Date();
  }
}

const user = new User("Alice", 30);
console.log(JSON.stringify(user));
// {"name":"Alice","age":30,"createdAt":"2024-01-15T10:30:00.000Z"}
```

## Generated Code

```typescript
toJSON(): Record<string, unknown> {
  const result: Record<string, unknown> = {};
  result["name"] = this.name;
  result["age"] = this.age;
  result["createdAt"] = this.createdAt.toISOString();
  return result;
}
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

```typescript
/** @derive(Serialize) */
class DataContainer {
  items: string[];
  metadata: Map<string, number>;
  tags: Set<string>;
  nested: User;
}
```

## Serde Options

Use the `@serde` decorator for fine-grained control over serialization:

### Renaming Fields

```typescript
/** @derive(Serialize) */
class User {
  /** @serde({ rename: "user_id" }) */
  id: string;

  /** @serde({ rename: "full_name" }) */
  name: string;
}

const user = new User();
user.id = "123";
user.name = "Alice";
console.log(JSON.stringify(user));
// {"user_id":"123","full_name":"Alice"}
```

### Skipping Fields

```typescript
/** @derive(Serialize) */
class User {
  name: string;
  email: string;

  /** @serde({ skip: true }) */
  password: string;

  /** @serde({ skip_serializing: true }) */
  internalId: string;
}
```

<Alert type="tip" title="skip vs skip_serializing">
Use `skip: true` to exclude from both serialization and deserialization.
Use `skip_serializing: true` to only skip during serialization.
</Alert>

### Rename All Fields

Apply a naming convention to all fields at the container level:

```typescript
/** @derive(Serialize) */
/** @serde({ rename_all: "camelCase" }) */
class ApiResponse {
  user_name: string;    // becomes "userName"
  created_at: Date;     // becomes "createdAt"
  is_active: boolean;   // becomes "isActive"
}
```

Supported conventions:

- `camelCase`

- `snake_case`

- `PascalCase`

- `SCREAMING_SNAKE_CASE`

- `kebab-case`

### Flattening Nested Objects

```typescript
/** @derive(Serialize) */
class Address {
  city: string;
  zip: string;
}

/** @derive(Serialize) */
class User {
  name: string;

  /** @serde({ flatten: true }) */
  address: Address;
}

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

```typescript
/** @derive(Serialize) */
interface ApiResponse {
  status: number;
  message: string;
  timestamp: Date;
}

// Generated:
// export namespace ApiResponse {
//   export function toJSON(self: ApiResponse): Record<string, unknown> {
//     const result: Record<string, unknown> = {};
//     result["status"] = self.status;
//     result["message"] = self.message;
//     result["timestamp"] = self.timestamp.toISOString();
//     return result;
//   }
// }

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

```typescript
/** @derive(Serialize) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}

// Generated:
// export namespace Status {
//   export function toJSON(value: Status): string | number {
//     return value;
//   }
// }

console.log(Status.toJSON(Status.Active));  // "active"
console.log(Status.toJSON(Status.Pending)); // "pending"
```

Works with numeric enums too:

```typescript
/** @derive(Serialize) */
enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
}

console.log(Priority.toJSON(Priority.High)); // 3
```

## Type Alias Support

Serialize works with type aliases. For object types, fields are serialized with full type handling:

```typescript
/** @derive(Serialize) */
type UserProfile = {
  id: string;
  name: string;
  createdAt: Date;
};

// Generated:
// export namespace UserProfile {
//   export function toJSON(value: UserProfile): Record<string, unknown> {
//     const result: Record<string, unknown> = {};
//     result["id"] = value.id;
//     result["name"] = value.name;
//     result["createdAt"] = value.createdAt.toISOString();
//     return result;
//   }
// }

const profile: UserProfile = {
  id: "123",
  name: "Alice",
  createdAt: new Date("2024-01-15")
};

console.log(JSON.stringify(UserProfile.toJSON(profile)));
// {"id":"123","name":"Alice","createdAt":"2024-01-15T00:00:00.000Z"}
```

For union types, the value is returned directly:

```typescript
/** @derive(Serialize) */
type ApiStatus = "loading" | "success" | "error";

console.log(ApiStatus.toJSON("success")); // "success"
```

## Combining with Deserialize

Use both Serialize and Deserialize for complete JSON round-trip support:

```typescript
/** @derive(Serialize, Deserialize) */
class User {
  name: string;
  createdAt: Date;
}

// Serialize
const user = new User();
user.name = "Alice";
user.createdAt = new Date();
const json = JSON.stringify(user);

// Deserialize
const parsed = User.fromJSON(JSON.parse(json));
console.log(parsed.createdAt instanceof Date); // true
```