## Combining with Serialize

Use both Serialize and Deserialize for complete JSON round-trip support:

<InteractiveMacro code={`/** @derive(Serialize, Deserialize) */
/** @serde({ rename_all: "camelCase" }) */
class UserProfile {
  user_name: string;
  created_at: Date;
  is_active: boolean;
}`} />

```typescript
// Create and serialize
const profile = new UserProfile();
profile.user_name = "Alice";
profile.created_at = new Date();
profile.is_active = true;

const json = JSON.stringify(profile);
// {"userName":"Alice","createdAt":"2024-...","isActive":true}

// Deserialize back
const restored = UserProfile.fromJSON(JSON.parse(json));
console.log(restored.user_name);              // "Alice"
console.log(restored.created_at instanceof Date); // true
```