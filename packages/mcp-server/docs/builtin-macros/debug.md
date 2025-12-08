# Debug

*The `Debug` macro generates a `toString()` method that produces a human-readable string representation of your class.*

## Basic Usage

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

const user = new User("Alice", 30);
console.log(user.toString());
// Output: User { name: Alice, age: 30 }
```

## Generated Code

```typescript
toString(): string {
  const parts: string[] = [];
  parts.push("name: " + this.name);
  parts.push("age: " + this.age);
  return "User { " + parts.join(", ") + " }";
}
```

## Field Options

Use the `@debug` field decorator to customize behavior:

### Renaming Fields

```typescript
/** @derive(Debug) */
class User {
  /** @debug({ rename: "userId" }) */
  id: number;

  name: string;

  constructor(id: number, name: string) {
    this.id = id;
    this.name = name;
  }
}

const user = new User(42, "Alice");
console.log(user.toString());
// Output: User { userId: 42, name: Alice }
```

### Skipping Fields

Use `skip: true` to exclude sensitive fields from the output:

```typescript
/** @derive(Debug) */
class User {
  name: string;
  email: string;

  /** @debug({ skip: true }) */
  password: string;

  /** @debug({ skip: true }) */
  authToken: string;

  constructor(name: string, email: string, password: string, authToken: string) {
    this.name = name;
    this.email = email;
    this.password = password;
    this.authToken = authToken;
  }
}

const user = new User("Alice", "alice@example.com", "secret", "tok_xxx");
console.log(user.toString());
// Output: User { name: Alice, email: alice@example.com }
// Note: password and authToken are not included
```

<Alert type="tip" title="Security">
Always skip sensitive fields like passwords, tokens, and API keys to prevent accidental logging.
</Alert>

## Combining Options

```typescript
/** @derive(Debug) */
class ApiResponse {
  /** @debug({ rename: "statusCode" }) */
  status: number;

  data: unknown;

  /** @debug({ skip: true }) */
  internalMetadata: Record<string, unknown>;
}
```

## All Options

| `rename` 
| `string` 
| Display a different name in the output 

| `skip` 
| `boolean` 
| Exclude this field from the output

## Interface Support

Debug also works with interfaces. For interfaces, a namespace is generated with a `toString` function:

```typescript
/** @derive(Debug) */
interface Status {
  active: boolean;
  message: string;
}

// Generated:
// export namespace Status {
//   export function toString(self: Status): string {
//     const parts: string[] = [];
//     parts.push("active: " + self.active);
//     parts.push("message: " + self.message);
//     return "Status { " + parts.join(", ") + " }";
//   }
// }

const status: Status = { active: true, message: "OK" };
console.log(Status.toString(status));
// Output: Status { active: true, message: OK }
```

## Enum Support

Debug also works with enums. For enums, a namespace is generated with a `toString` function that displays the enum name and variant:

```typescript
/** @derive(Debug) */
enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
}

// Generated:
// export namespace Priority {
//   export function toString(value: Priority): string {
//     const key = Priority[value as unknown as keyof typeof Priority];
//     if (key !== undefined) {
//       return "Priority." + key;
//     }
//     return "Priority(" + String(value) + ")";
//   }
// }

console.log(Priority.toString(Priority.High));
// Output: Priority.High

console.log(Priority.toString(Priority.Low));
// Output: Priority.Low
```

Works with both numeric and string enums:

```typescript
/** @derive(Debug) */
enum Status {
  Active = "active",
  Inactive = "inactive",
}

console.log(Status.toString(Status.Active));
// Output: Status.Active
```

## Type Alias Support

Debug works with type aliases. For object types, fields are displayed similar to interfaces:

```typescript
/** @derive(Debug) */
type Point = {
  x: number;
  y: number;
};

// Generated:
// export namespace Point {
//   export function toString(value: Point): string {
//     const parts: string[] = [];
//     parts.push("x: " + value.x);
//     parts.push("y: " + value.y);
//     return "Point { " + parts.join(", ") + " }";
//   }
// }

const point: Point = { x: 10, y: 20 };
console.log(Point.toString(point));
// Output: Point { x: 10, y: 20 }
```

For union types, the value is displayed using JSON.stringify:

```typescript
/** @derive(Debug) */
type ApiStatus = "loading" | "success" | "error";

console.log(ApiStatus.toString("success"));
// Output: ApiStatus("success")
```