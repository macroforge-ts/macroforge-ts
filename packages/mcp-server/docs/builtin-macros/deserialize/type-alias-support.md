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