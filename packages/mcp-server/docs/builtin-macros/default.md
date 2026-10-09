# Default

The `Default` macro generates a static `defaultValue()` factory method that creates instances with
default values. This is analogous to Rust's `Default` trait, providing a standard way to create
"zero" or "empty" instances of types.

## Generated Output

| Type       | Generated Code                                      | Description                                       |
| ---------- | --------------------------------------------------- | ------------------------------------------------- |
| Class      | `static defaultValue()` + `classNameDefaultValue()` | Static factory method + standalone function       |
| Enum       | `enumNameDefaultValue(): EnumName`                  | Standalone function returning `@default` variant  |
| Interface  | `ifaceNameDefaultValue(): InterfaceName`            | Standalone function returning object literal      |
| Type Alias | `typeNameDefaultValue(): TypeName`                  | Standalone function with type-appropriate default |

Names use **camelCase** conversion (e.g., `UserSettings` -> `userSettingsDefaultValue`).

## Default Values by Type

The macro uses Rust-like default semantics:

| Type              | Default Value                                                                     |
| ----------------- | --------------------------------------------------------------------------------- |
| `string`          | `""` (empty string)                                                               |
| `number`          | `0`                                                                               |
| `boolean`         | `false`                                                                           |
| `bigint`          | `0n`                                                                              |
| `T[]`             | `[]` (empty array)                                                                |
| `Array<T>`        | `[]` (empty array)                                                                |
| `Map<K,V>`        | `new Map()`                                                                       |
| `Set<T>`          | `new Set()`                                                                       |
| `Date`            | `new Date()` (current time)                                                       |
| `T \| null`       | `null`                                                                            |
| Unions (`A \| B`) | Default of the first primitive or literal member, else the first member's default |
| `CustomType`      | `customTypeDefaultValue()` (recursive standalone-function call)                   |

## Field-Level Options

The `@default` decorator allows specifying explicit default values:

- `@default(42)` - Use 42 as the default
- `@default("hello")` - Use "hello" as the default
- `@default([])` - Use empty array as the default
- `@default({ value: "test" })` - Named form for complex values

## Example

```typescript before
/** @derive(Default) */
class UserSettings {
    /** @default("light") */
    theme: string;

    /** @default(10) */
    pageSize: number;

    notifications: boolean; // Uses type default: false
}
```

```typescript after
class UserSettings {
    theme: string;

    pageSize: number;

    notifications: boolean; // Uses type default: false

    static defaultValue(): UserSettings {
        const instance: UserSettings = Object.create(UserSettings.prototype);
        instance.theme = 'light';
        instance.pageSize = 10;
        instance.notifications = false;
        return instance;
    }
}

export function userSettingsDefaultValue(): UserSettings {
    return UserSettings.defaultValue();
}
```

## Enum Defaults

For enums, mark one variant with `@default`:

```typescript before
/** @derive(Default) */
enum Status {
    /** @default */
    Pending,
    Active,
    Completed
}
```

```typescript after
enum Status {
    /** @default */
    Pending,
    Active,
    Completed
}

export function statusDefaultValue(): Status {
    return Status.Pending;
}
```

Like Rust's derive, the class default runs no constructor (so it works beside `Decode`'s, which
takes arguments) and assigns every required field. An optional field keeps no initializer value;
give it `@default(value)` to set one.

## Primitive Aliases

A type alias of a primitive, plain or branded with `$Newtype`, defaults to the primitive's default
above, or to its `@default(value)`, cast to the alias:

```typescript
/** @derive(Default) */
type Meters = $Newtype<number>;

export function metersDefaultValue(): Meters {
    return 0 as Meters;
}
```

An alias with `@endec` validators must give `@default(value)`: the primitive's default may fail
them, just as a Rust newtype with no valid zero derives no `Default`.

```typescript
/** @derive(Decode, Default) */
/** @endec(positive) */
/** @default(1) */
type Meters = $Newtype<number>;
```

## Error Handling

The macro reports an expansion error only if:

- An enum has no variant marked with `@default`
- A type alias that is not an object, union or primitive has no `@default(value)`
- A primitive alias with `@endec` validators has no `@default(value)`
- A field's `@default` expression fails to parse

Missing defaults on non-primitive fields are **not** expansion errors: every type is assumed to
implement Default (Rust-like philosophy), so the macro emits a `typeNameDefaultValue()` call for any
custom type. If a field's type is in the type registry but does not derive `Default`, a warning is
printed to stderr at expansion time and the call is generated anyway. It may then fail at runtime if
the function does not exist.
