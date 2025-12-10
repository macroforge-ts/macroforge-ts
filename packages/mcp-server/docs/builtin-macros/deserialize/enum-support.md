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