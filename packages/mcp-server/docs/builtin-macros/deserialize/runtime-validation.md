## Runtime Validation

Deserialize validates the input data and throws descriptive errors:

<InteractiveMacro code={`/** @derive(Deserialize) */
class User {
  name: string;
  email: string;
}`} />

```typescript
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