# Built-in Macros
 *Macroforge comes with built-in derive macros that cover the most common code generation needs. All macros work with classes, interfaces, enums, and type aliases.*
 ## Overview
 | Macro | Generates | Description |
| --- | --- | --- |
| [Debug](../docs/builtin-macros/debug) | toString(): string | Human-readable string representation |
| [Clone](../docs/builtin-macros/clone) | clone():T | Creates a deep copy of the object |
| [Default](../docs/builtin-macros/default) | staticdefault():T | Creates an instance with default values |
| [Hash](../docs/builtin-macros/hash) | hashCode(): number | Generates a hash code for the object |
| [PartialEq](../docs/builtin-macros/partial-eq) | equals(other:T): boolean | Value equality comparison |
| [Ord](../docs/builtin-macros/ord) | compare(other:T): number | Total ordering comparison (-1, 0, 1) |
| [PartialOrd](../docs/builtin-macros/partial-ord) | partialCompare(other:T): number|null | Partial ordering comparison |
| [Serialize](../docs/builtin-macros/serialize) | toJSON(): Record&lt;string, unknown&gt; | JSON serialization with type handling |
| [Deserialize](../docs/builtin-macros/deserialize) | staticfromJSON(data: unknown):T | JSON deserialization with validation |
 ## Using Built-in Macros
 Built-in macros don't require imports. Just use them with <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code>:
 ```
/** @derive(Debug, Clone, PartialEq) */
class User &#123;
  name: string;
  age: number;

  constructor(name: string, age: number) &#123;
    this.name = name;
    this.age = age;
  &#125;
&#125;
``` ## Interface Support
 All built-in macros work with interfaces. For interfaces, methods are generated as functions in a namespace with the same name, using <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">self</code> as the first parameter:
 ```
/** @derive(Debug, Clone, PartialEq) */
interface Point &#123;
  x: number;
  y: number;
&#125;

// Generated namespace:
// namespace Point &#123;
//   export function toString(self: Point): string &#123; ... &#125;
//   export function clone(self: Point): Point &#123; ... &#125;
//   export function equals(self: Point, other: Point): boolean &#123; ... &#125;
//   export function hashCode(self: Point): number &#123; ... &#125;
// &#125;

const point: Point = &#123; x: 10, y: 20 &#125;;

// Use the namespace functions
console.log(Point.toString(point));     // "Point &#123; x: 10, y: 20 &#125;"
const copy = Point.clone(point);        // &#123; x: 10, y: 20 &#125;
console.log(Point.equals(point, copy)); // true
``` ## Enum Support
 All built-in macros work with enums. For enums, methods are generated as functions in a namespace with the same name:
 ```
/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
enum Status &#123;
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
&#125;

// Generated namespace:
// namespace Status &#123;
//   export function toString(value: Status): string &#123; ... &#125;
//   export function clone(value: Status): Status &#123; ... &#125;
//   export function equals(a: Status, b: Status): boolean &#123; ... &#125;
//   export function hashCode(value: Status): number &#123; ... &#125;
//   export function toJSON(value: Status): string | number &#123; ... &#125;
//   export function fromJSON(data: unknown): Status &#123; ... &#125;
// &#125;

// Use the namespace functions
console.log(Status.toString(Status.Active));     // "Status.Active"
console.log(Status.equals(Status.Active, Status.Active)); // true
const json = Status.toJSON(Status.Pending);      // "pending"
const parsed = Status.fromJSON("active");        // Status.Active
``` ## Type Alias Support
 All built-in macros work with type aliases. For object type aliases, field-aware methods are generated in a namespace:
 ```
/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
type Point = &#123;
  x: number;
  y: number;
&#125;;

// Generated namespace:
// namespace Point &#123;
//   export function toString(value: Point): string &#123; ... &#125;
//   export function clone(value: Point): Point &#123; ... &#125;
//   export function equals(a: Point, b: Point): boolean &#123; ... &#125;
//   export function hashCode(value: Point): number &#123; ... &#125;
//   export function toJSON(value: Point): Record&#x3C;string, unknown> &#123; ... &#125;
//   export function fromJSON(data: unknown): Point &#123; ... &#125;
// &#125;

const point: Point = &#123; x: 10, y: 20 &#125;;
console.log(Point.toString(point));     // "Point &#123; x: 10, y: 20 &#125;"
const copy = Point.clone(point);        // &#123; x: 10, y: 20 &#125;
console.log(Point.equals(point, copy)); // true
``` Union type aliases also work, using JSON-based implementations:
 ```
/** @derive(Debug, PartialEq) */
type ApiStatus = "loading" | "success" | "error";

const status: ApiStatus = "success";
console.log(ApiStatus.toString(status)); // "ApiStatus(\\"success\\")"
console.log(ApiStatus.equals("success", "success")); // true
``` ## Combining Macros
 All macros can be used together. They don't conflict and each generates independent methods:
 ```
const user = new User("Alice", 30);

// Debug
console.log(user.toString());
// "User &#123; name: Alice, age: 30 &#125;"

// Clone
const copy = user.clone();
console.log(copy.name); // "Alice"

// Eq
console.log(user.equals(copy)); // true
``` ## Detailed Documentation
 Each macro has its own options and behaviors:
 - [**Debug**](../docs/builtin-macros/debug) - Customizable field renaming and skipping
 - [**Clone**](../docs/builtin-macros/clone) - Deep copying for all field types
 - [**Default**](../docs/builtin-macros/default) - Default value generation with field attributes
 - [**Hash**](../docs/builtin-macros/hash) - Hash code generation for use in maps and sets
 - [**PartialEq**](../docs/builtin-macros/partial-eq) - Value-based equality comparison
 - [**Ord**](../docs/builtin-macros/ord) - Total ordering for sorting
 - [**PartialOrd**](../docs/builtin-macros/partial-ord) - Partial ordering comparison
 - [**Serialize**](../docs/builtin-macros/serialize) - JSON serialization with serde-style options
 - [**Deserialize**](../docs/builtin-macros/deserialize) - JSON deserialization with validation