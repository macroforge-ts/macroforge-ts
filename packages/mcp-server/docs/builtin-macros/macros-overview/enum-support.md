## Enum Support

All built-in macros work with enums. For enums, methods are generated as functions in a namespace
with the same name:

TypeScript

```
/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}

// Generated standalone functions:
// export function statusToString(value: Status): string { ... }
// export function statusClone(value: Status): Status { ... }
// export function statusEquals(a: Status, b: Status): boolean { ... }
// export function statusHashCode(value: Status): number { ... }
// export function statusSerialize(value: Status): string { ... }
// export function statusDeserialize(input: unknown): Status { ... }

// Enums use namespace merging for the convenience names:
// namespace Status {
//   export const toString = statusToString;
//   export const serialize = statusSerialize;
// }

console.log(statusToString(Status.Active));                // "Status.Active"
console.log(statusEquals(Status.Active, Status.Active));   // true
const json = statusSerialize(Status.Pending);              // "pending"
// Note: enum deserialize throws on invalid input rather than
// returning a success/errors union.
const parsed = statusDeserialize("active");                // Status.Active
```
