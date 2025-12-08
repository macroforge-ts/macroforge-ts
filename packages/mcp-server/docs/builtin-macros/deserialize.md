# Deserialize

*The `Deserialize` macro generates a static `fromJSON()` method that parses JSON data into your class with runtime validation and automatic type conversion.*

## Basic Usage

```typescript
/** @derive(Deserialize) */
class User {
  name: string;
  age: number;
  createdAt: Date;
}

const json = '{"name":"Alice","age":30,"createdAt":"2024-01-15T10:30:00.000Z"}';
const user = User.fromJSON(JSON.parse(json));

console.log(user.name);                    // "Alice"
console.log(user.age);                     // 30
console.log(user.createdAt instanceof Date); // true
```

## Generated Code

```typescript
static fromJSON(data: unknown): User {
  if (typeof data !== "object" || data === null || Array.isArray(data)) {
    throw new Error("User.fromJSON: expected an object, got " + typeof data);
  }
  const obj = data as Record<string, unknown>;

  if (!("name" in obj)) {
    throw new Error("User.fromJSON: missing required field \\"name\\"");
  }
  if (!("age" in obj)) {
    throw new Error("User.fromJSON: missing required field \\"age\\"");
  }

  const instance = new User();
  instance.name = obj["name"] as string;
  instance.age = obj["age"] as number;
  instance.createdAt = new Date(obj["createdAt"] as string);
  return instance;
}
```

## Runtime Validation

Deserialize validates the input data and throws descriptive errors:

```typescript
/** @derive(Deserialize) */
class User {
  name: string;
  email: string;
}

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

```typescript
/** @derive(Deserialize) */
class User {
  /** @serde({ rename: "user_id" }) */
  id: string;

  /** @serde({ rename: "full_name" }) */
  name: string;
}

const user = User.fromJSON({ user_id: "123", full_name: "Alice" });
console.log(user.id);   // "123"
console.log(user.name); // "Alice"
```

### Default Values

```typescript
/** @derive(Deserialize) */
class Config {
  host: string;

  /** @serde({ default: "3000" }) */
  port: string;

  /** @serde({ default: "false" }) */
  debug: boolean;
}

const config = Config.fromJSON({ host: "localhost" });
console.log(config.port);  // "3000"
console.log(config.debug); // false
```

### Skipping Fields

```typescript
/** @derive(Deserialize) */
class User {
  name: string;
  email: string;

  /** @serde({ skip: true }) */
  cachedData: unknown;

  /** @serde({ skip_deserializing: true }) */
  computedField: string;
}
```

<Alert type="tip" title="skip vs skip_deserializing">
Use `skip: true` to exclude from both serialization and deserialization.
Use `skip_deserializing: true` to only skip during deserialization.
</Alert>

### Deny Unknown Fields

```typescript
/** @derive(Deserialize) */
/** @serde({ deny_unknown_fields: true }) */
class StrictUser {
  name: string;
  email: string;
}

// This will throw an error
StrictUser.fromJSON({ name: "Alice", email: "a@b.com", extra: "field" });
// Error: StrictUser.fromJSON: unknown field "extra"
```

### Flatten Nested Objects

```typescript
/** @derive(Deserialize) */
class Address {
  city: string;
  zip: string;
}

/** @derive(Deserialize) */
class User {
  name: string;

  /** @serde({ flatten: true }) */
  address: Address;
}

// Flat JSON structure
const user = User.fromJSON({
  name: "Alice",
  city: "NYC",
  zip: "10001"
});
console.log(user.address.city); // "NYC"
```

## Field Validation

Use the `validate` option to add runtime validation to fields. Validation errors are collected and returned as `Result.err(string[])`.

### Basic Validation

```typescript
/** @derive(Deserialize) */
class User {
  /** @serde({ validate: ["email"] }) */
  email: string;

  /** @serde({ validate: ["minLength(2)", "maxLength(50)"] }) */
  name: string;

  /** @serde({ validate: ["positive", "int"] }) */
  age: number;
}

const result = User.fromJSON({ email: "invalid", name: "A", age: -5 });
if (result.isErr()) {
  console.log(result.unwrapErr());
  // [
  //   'User.fromJSON: field "email" must be a valid email',
  //   'User.fromJSON: field "name" must have at least 2 characters',
  //   'User.fromJSON: field "age" must be positive',
  // ]
}
```

### Custom Error Messages

Use the object form to provide custom error messages:

```typescript
/** @derive(Deserialize) */
class Product {
  /** @serde({ validate: [
    { validate: "nonEmpty", message: "Product name is required" },
    { validate: "maxLength(100)", message: "Name too long (max 100 chars)" }
  ] }) */
  name: string;

  /** @serde({ validate: [
    { validate: "positive", message: "Price must be greater than zero" }
  ] }) */
  price: number;
}
```

### Custom Validator Functions

Use `custom(functionName)` to call your own validation function:

```typescript
function isValidSKU(value: string): boolean {
  return /^[A-Z]{3}-\\d{4}$/.test(value);
}

/** @derive(Deserialize) */
class Product {
  /** @serde({ validate: [
    { validate: "custom(isValidSKU)", message: "Invalid SKU format (expected XXX-0000)" }
  ] }) */
  sku: string;
}
```

### Available Validators

#### String Validators

| `email` 
| Must be a valid email address 

| `url` 
| Must be a valid URL 

| `uuid` 
| Must be a valid UUID 

| `nonEmpty` 
| Must not be empty string 

| `trimmed` 
| Must have no leading/trailing whitespace 

| `lowercase` 
| Must be all lowercase 

| `uppercase` 
| Must be all uppercase 

| `capitalized` 
| First character must be uppercase 

| `uncapitalized` 
| First character must be lowercase 

| `minLength(n)` 
| Must have at least n characters 

| `maxLength(n)` 
| Must have at most n characters 

| `length(n)` 
| Must have exactly n characters 

| `length(min, max)` 
| Must have between min and max characters 

| `pattern("regex")` 
| Must match the regular expression 

| `startsWith("prefix")` 
| Must start with the given prefix 

| `endsWith("suffix")` 
| Must end with the given suffix 

| `includes("substring")` 
| Must contain the substring

#### Number Validators

| `positive` 
| Must be greater than 0 

| `negative` 
| Must be less than 0 

| `nonNegative` 
| Must be 0 or greater 

| `nonPositive` 
| Must be 0 or less 

| `int` 
| Must be an integer 

| `finite` 
| Must be finite (not Infinity) 

| `nonNaN` 
| Must not be NaN 

| `uint8` 
| Must be integer 0-255 

| `greaterThan(n)` 
| Must be greater than n 

| `greaterThanOrEqualTo(n)` 
| Must be greater than or equal to n 

| `lessThan(n)` 
| Must be less than n 

| `lessThanOrEqualTo(n)` 
| Must be less than or equal to n 

| `between(min, max)` 
| Must be between min and max (inclusive) 

| `multipleOf(n)` 
| Must be a multiple of n

#### Array Validators

| `minItems(n)` 
| Must have at least n items 

| `maxItems(n)` 
| Must have at most n items 

| `itemsCount(n)` 
| Must have exactly n items

#### Date Validators

| `validDate` 
| Must be a valid date (not Invalid Date) 

| `greaterThanDate("ISO")` 
| Must be after the given date 

| `greaterThanOrEqualToDate("ISO")` 
| Must be on or after the given date 

| `lessThanDate("ISO")` 
| Must be before the given date 

| `lessThanOrEqualToDate("ISO")` 
| Must be on or before the given date 

| `betweenDate("ISO1", "ISO2")` 
| Must be between the two dates

#### BigInt Validators

| `positiveBigInt` 
| Must be greater than 0n 

| `negativeBigInt` 
| Must be less than 0n 

| `nonNegativeBigInt` 
| Must be 0n or greater 

| `nonPositiveBigInt` 
| Must be 0n or less 

| `greaterThanBigInt(n)` 
| Must be greater than BigInt(n) 

| `lessThanBigInt(n)` 
| Must be less than BigInt(n) 

| `betweenBigInt(min, max)` 
| Must be between BigInt(min) and BigInt(max)

#### Custom Validators

| `custom(fnName)` 
| Calls fnName(value), fails if it returns false

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
| `boolean` 
| Use TypeScript default if missing 

| `default: "expr"` 
| `string` 
| Custom default expression 

| `flatten` 
| `boolean` 
| Merge nested object fields from parent 

| `validate` 
| `string[] | object[]` 
| Array of validators to run during deserialization

## Interface Support

Deserialize also works with interfaces. For interfaces, a namespace is generated with `is` (type guard) and `fromJSON` functions:

```typescript
/** @derive(Deserialize) */
interface ApiResponse {
  status: number;
  message: string;
  timestamp: Date;
}

// Generated:
// export namespace ApiResponse {
//   export function is(data: unknown): data is ApiResponse {
//     if (typeof data !== "object" || data === null) return false;
//     const obj = data as Record<string, unknown>;
//     if (typeof obj["status"] !== "number") return false;
//     if (typeof obj["message"] !== "string") return false;
//     // ... additional checks
//     return true;
//   }
//
//   export function fromJSON(data: unknown): ApiResponse {
//     if (!is(data)) {
//       throw new Error("ApiResponse.fromJSON: validation failed");
//     }
//     return {
//       ...data,
//       timestamp: new Date(data.timestamp)
//     };
//   }
// }

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

```typescript
/** @derive(Deserialize) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}

// Generated:
// export namespace Status {
//   export function fromJSON(data: unknown): Status {
//     for (const key of Object.keys(Status)) {
//       if (Status[key as keyof typeof Status] === data) {
//         return data as Status;
//       }
//     }
//     throw new Error(\`Invalid Status value: \${data}\`);
//   }
// }

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

```typescript
/** @derive(Deserialize) */
enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
}

const priority = Priority.fromJSON(3);
console.log(priority); // Priority.High
```

## Type Alias Support

Deserialize works with type aliases. For object types, validation and type conversion is applied:

```typescript
/** @derive(Deserialize) */
type UserProfile = {
  id: string;
  name: string;
  createdAt: Date;
};

// Generated:
// export namespace UserProfile {
//   export function fromJSON(data: unknown): UserProfile {
//     if (typeof data !== "object" || data === null) {
//       throw new Error("UserProfile.fromJSON: expected object");
//     }
//     const obj = data as Record<string, unknown>;
//     return {
//       id: obj["id"] as string,
//       name: obj["name"] as string,
//       createdAt: new Date(obj["createdAt"] as string),
//     };
//   }
// }

const json = {
  id: "123",
  name: "Alice",
  createdAt: "2024-01-15T00:00:00.000Z"
};

const profile = UserProfile.fromJSON(json);
console.log(profile.createdAt instanceof Date); // true
```

For union types, basic validation is applied:

```typescript
/** @derive(Deserialize) */
type ApiStatus = "loading" | "success" | "error";

const status = ApiStatus.fromJSON("success");
console.log(status); // "success"
```

## Combining with Serialize

Use both Serialize and Deserialize for complete JSON round-trip support:

```typescript
/** @derive(Serialize, Deserialize) */
/** @serde({ rename_all: "camelCase" }) */
class UserProfile {
  user_name: string;
  created_at: Date;
  is_active: boolean;
}

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

```typescript
/** @derive(Deserialize) */
class User {
  name: string;
  email: string;
}

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