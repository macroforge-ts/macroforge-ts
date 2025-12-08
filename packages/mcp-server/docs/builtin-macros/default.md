# Default

*The `Default` macro generates a static `default()` factory method that creates instances with default field values.*

## Basic Usage

```typescript
/** @derive(Default) */
class Config {
  host: string;
  port: number;
  enabled: boolean;

  constructor(host: string, port: number, enabled: boolean) {
    this.host = host;
    this.port = port;
    this.enabled = enabled;
  }
}

const config = Config.default();
console.log(config.host);    // ""
console.log(config.port);    // 0
console.log(config.enabled); // false
```

## Generated Code

The Default macro generates a static factory method:

```typescript
static default(): Config {
    const instance = new Config();
    instance.host = "";
    instance.port = 0;
    instance.enabled = false;
    return instance;
}
```

## Automatic Default Values

The Default macro automatically determines default values based on field types:

- `string` → `""` (empty string)

- `number` → `0`

- `boolean` → `false`

- `bigint` → `0n`

- `Array<T>` or `T[]` → `[]` (empty array)

- `Map<K, V>` → `new Map()`

- `Set<T>` → `new Set()`

- `Date` → `new Date()`

- Custom types → `null as any`

## Custom Default Values

Use the `@defaultValue()` decorator to specify custom default values for fields:

```typescript
/** @derive(Default) */
class ServerConfig {
  /** @defaultValue("localhost") */
  host: string;

  /** @defaultValue(8080) */
  port: number;

  /** @defaultValue(true) */
  enabled: boolean;

  /** @defaultValue(["info", "error"]) */
  logLevels: string[];

  constructor(host: string, port: number, enabled: boolean, logLevels: string[]) {
    this.host = host;
    this.port = port;
    this.enabled = enabled;
    this.logLevels = logLevels;
  }
}

const config = ServerConfig.default();
console.log(config.host);      // "localhost"
console.log(config.port);      // 8080
console.log(config.enabled);   // true
console.log(config.logLevels); // ["info", "error"]
```

## Interface Support

Default also works with interfaces. For interfaces, a namespace is generated with a `default_()` function (note the underscore to avoid conflicts with the reserved word):

```typescript
/** @derive(Default) */
interface Point {
  x: number;
  y: number;
}

// Generated:
// export namespace Point {
//   export function default_(): Point {
//     return {
//       x: 0,
//       y: 0
//     } as Point;
//   }
// }

const origin = Point.default_();
console.log(origin); // { x: 0, y: 0 }
```

## Enum Support

Default works with enums. For enums, it returns the first variant as the default value:

```typescript
/** @derive(Default) */
enum Status {
  Pending = "pending",
  Active = "active",
  Completed = "completed",
}

// Generated:
// export namespace Status {
//   export function default_(): Status {
//     return Status.Pending;
//   }
// }

const defaultStatus = Status.default_();
console.log(defaultStatus); // "pending"
```

## Type Alias Support

Default works with type aliases. For object types, it creates an object with default field values:

```typescript
/** @derive(Default) */
type Dimensions = {
  width: number;
  height: number;
};

// Generated:
// export namespace Dimensions {
//   export function default_(): Dimensions {
//     return {
//       width: 0,
//       height: 0
//     } as Dimensions;
//   }
// }

const dims = Dimensions.default_();
console.log(dims); // { width: 0, height: 0 }
```

## Combining with Other Macros

```typescript
/** @derive(Default, Debug, Clone, PartialEq) */
class User {
  /** @defaultValue("Anonymous") */
  name: string;

  /** @defaultValue(0) */
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}

const user1 = User.default();
const user2 = user1.clone();

console.log(user1.toString());    // User { name: "Anonymous", age: 0 }
console.log(user1.equals(user2)); // true
```