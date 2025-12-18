## Enum Support

All built-in macros work with enums. For enums, methods are generated as functions in a namespace with the same name:

TypeScript

```
/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}

// Generated namespace:
// namespace Status {
//   export function toString(value: Status): string { ... }
//   export function clone(value: Status): Status { ... }
//   export function equals(a: Status, b: Status): boolean { ... }
//   export function hashCode(value: Status): number { ... }
//   export function toJSON(value: Status): string | number { ... }
//   export function fromJSON(data: unknown): Status { ... }
// }

// Use the namespace functions
console.log(Status.toString(Status.Active));     // "Status.Active"
console.log(Status.equals(Status.Active, Status.Active)); // true
const json = Status.toJSON(Status.Pending);      // "pending"
const parsed = Status.fromJSON("active");        // Status.Active
```