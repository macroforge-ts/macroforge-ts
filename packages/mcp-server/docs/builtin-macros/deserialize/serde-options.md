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