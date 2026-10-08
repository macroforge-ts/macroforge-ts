## Enum Support

All built-in macros work with enums. For enums, methods are generated as functions in a namespace
with the same name:

TypeScript

```
/** @derive(Debug, Clone, PartialEq, Encode, Decode) */
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
// export function statusEncode(value: Status): string { ... }
// export function statusDecode(input: unknown): Status { ... }

// Enums use namespace merging for the convenience names:
// namespace Status {
//   export const toString = statusToString;
//   export const encode = statusEncode;
// }

console.log(statusToString(Status.Active));                // "Status.Active"
console.log(statusEquals(Status.Active, Status.Active));   // true
const json = statusEncode(Status.Pending);              // "pending"
// Note: enum decode throws on invalid input rather than
// returning a success/errors union.
const parsed = statusDecode("active");                // Status.Active
```
