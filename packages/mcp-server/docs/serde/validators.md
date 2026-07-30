# Validators

Validators run during deserialization. Failures are reported through the `errors` array of the result union rather than thrown, so a single call reports every problem at once.

## Three ways to apply them

```typescript
/** @derive(Deserialize) */
class User {
  /** @serde(validate: ["email", "maxLength(255)"]) */
  email: string;

  /** @serde(validate: [{ validate: "minLength(2)", message: "Name too short" }]) */
  name: string;

  /** @serde(minLength(8), maxLength(64)) */
  password: string;
}
```

1. **Array of strings** — `validate: ["email", "maxLength(255)"]`
2. **Array of objects** — adds a custom `message` per validator
3. **Shorthand** — bare validator calls directly in `@serde(...)`

`validate` must be an **array**. An object literal such as `validate: { email: true }` is rejected with *"validate must be an array"*.

Misspelled names are caught with a suggestion: the parser does a Levenshtein match against the known list and tells you what you probably meant.

## String validators

| Validator | Checks |
| --- | --- |
| `email` | Valid email address |
| `url` | Valid URL |
| `uuid` | Valid UUID |
| `pattern("re")` | Matches the regular expression |
| `minLength(n)` | Length ≥ n |
| `maxLength(n)` | Length ≤ n |
| `length(n)` | Length exactly n |
| `length(min, max)` | Length within range |
| `nonEmpty` | Not the empty string |
| `trimmed` | No leading/trailing whitespace |
| `lowercase` | All lowercase |
| `uppercase` | All uppercase |
| `capitalized` | First character uppercase |
| `uncapitalized` | First character lowercase |
| `startsWith(s)` | Has the prefix |
| `endsWith(s)` | Has the suffix |
| `includes(s)` | Contains the substring |

## Number validators

| Validator | Checks |
| --- | --- |
| `greaterThan(n)` | > n |
| `greaterThanOrEqualTo(n)` | ≥ n |
| `lessThan(n)` | &lt; n |
| `lessThanOrEqualTo(n)` | ≤ n |
| `between(min, max)` | Within range |
| `int` | Integer |
| `nonNegativeInt` | Integer **and** ≥ 0 |
| `uint8` | Integer in 0–255 |
| `multipleOf(n)` | Divisible by n |
| `positive` | > 0 |
| `nonNegative` | ≥ 0 |
| `negative` | &lt; 0 |
| `nonPositive` | ≤ 0 |
| `finite` | Not Infinity |
| `nonNaN` | Not NaN |

## BigInt validators

| Validator | Checks |
| --- | --- |
| `positiveBigInt` | > 0n |
| `nonNegativeBigInt` | ≥ 0n |
| `negativeBigInt` | &lt; 0n |
| `nonPositiveBigInt` | ≤ 0n |
| `greaterThanBigInt(n)` | > n |
| `greaterThanOrEqualToBigInt(n)` | ≥ n |
| `lessThanBigInt(n)` | &lt; n |
| `lessThanOrEqualToBigInt(n)` | ≤ n |
| `betweenBigInt(min, max)` | Within range |

## Date validators

Note the names — they are comparison-shaped, not `after`/`before`:

| Validator | Checks |
| --- | --- |
| `validDate` | Parses to a valid Date |
| `greaterThanDate(d)` | After d |
| `greaterThanOrEqualToDate(d)` | At or after d |
| `lessThanDate(d)` | Before d |
| `lessThanOrEqualToDate(d)` | At or before d |
| `betweenDate(start, end)` | Within range |

## Array validators

| Validator | Checks |
| --- | --- |
| `minItems(n)` | At least n elements |
| `maxItems(n)` | At most n elements |
| `itemsCount(n)` | Exactly n elements |

## Custom validators

`custom(fnName)` calls a function you supply, which should return `true` when the value is valid.

```typescript
/** @serde(validate: ["custom(isStrongPassword)"]) */
password: string;
```

## Validating without deserializing

Generated types also expose the validators directly, which is useful for form validation:

```typescript
User.validateField("email", "not-an-email");
User.validateFields({ email: "a@b.com", name: "" });
```
