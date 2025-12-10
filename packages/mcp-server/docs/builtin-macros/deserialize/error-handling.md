## Error Handling

Handle deserialization errors gracefully:

<InteractiveMacro code={`/** @derive(Deserialize) */
class User {
  name: string;
  email: string;
}`} />

```typescript
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