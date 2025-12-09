# Default

*The `Default` macro generates a static `default()` factory method that creates instances with default field values.*

## Basic Usage

<MacroExample before={data.examples.basic.before} after={data.examples.basic.after} />

```typescript
const config = Config.default();
console.log(config.host);    // ""
console.log(config.port);    // 0
console.log(config.enabled); // false
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

<MacroExample before={data.examples.custom.before} after={data.examples.custom.after} />

```typescript
const config = ServerConfig.default();
console.log(config.host);      // "localhost"
console.log(config.port);      // 8080
console.log(config.enabled);   // true
console.log(config.logLevels); // ["info", "error"]
```

## Interface Support

Default also works with interfaces. For interfaces, a namespace is generated with a `default_()` function (note the underscore to avoid conflicts with the reserved word):

<MacroExample before={data.examples.interface.before} after={data.examples.interface.after} />

```typescript
const origin = Point.default_();
console.log(origin); // { x: 0, y: 0 }
```

## Enum Support

Default works with enums. For enums, it returns the first variant as the default value:

<MacroExample before={data.examples.enum.before} after={data.examples.enum.after} />

```typescript
const defaultStatus = Status.default_();
console.log(defaultStatus); // "pending"
```

## Type Alias Support

Default works with type aliases. For object types, it creates an object with default field values:

<MacroExample before={data.examples.typeAlias.before} after={data.examples.typeAlias.after} />

```typescript
const dims = Dimensions.default_();
console.log(dims); // { width: 0, height: 0 }
```

## Combining with Other Macros

<InteractiveMacro code={`/** @derive(Default, Debug, Clone, PartialEq) */
class User {
  /** @defaultValue("Anonymous") */
  name: string;

  /** @defaultValue(0) */
  age: number;

  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}`} />

```typescript
const user1 = User.default();
const user2 = user1.clone();

console.log(user1.toString());    // User { name: "Anonymous", age: 0 }
console.log(user1.equals(user2)); // true
```