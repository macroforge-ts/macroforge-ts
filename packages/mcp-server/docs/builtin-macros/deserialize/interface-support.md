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