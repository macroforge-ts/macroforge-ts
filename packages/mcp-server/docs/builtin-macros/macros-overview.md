# Built-in Macros
 *Macroforge comes with built-in derive macros that cover the most common code generation needs. All macros work with classes, interfaces, enums, and type aliases.*
 ## Overview
 | Macro | Generates | Description |
| --- | --- | --- |
| [Debug](../docs/builtin-macros/debug) | toString(): string | Human-readable string representation |
| [Clone](../docs/builtin-macros/clone) | clone(): T | Creates a deep copy of the object |
| [Default](../docs/builtin-macros/default) | static default(): T | Creates an instance with default values |
| [Hash](../docs/builtin-macros/hash) | hashCode(): number | Generates a hash code for the object |
| [PartialEq](../docs/builtin-macros/partial-eq) | equals(other: T): boolean | Value equality comparison |
| [Ord](../docs/builtin-macros/ord) | compare(other: T): number | Total ordering comparison (-1, 0, 1) |
| [PartialOrd](../docs/builtin-macros/partial-ord) | partialCompare(other: T): number | null | Partial ordering comparison |
| [Serialize](../docs/builtin-macros/serialize) | toJSON(): Record<string, unknown> | JSON serialization with type handling |
| [Deserialize](../docs/builtin-macros/deserialize) | static fromJSON(data: unknown): T | JSON deserialization with validation |
 ## Using Built-in Macros
 Built-in macros don't require imports. Just use them with <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@derive</code>:
 ```
/**&nbsp;@derive(Debug,&nbsp;Clone,&nbsp;PartialEq)&nbsp;*/
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&nbsp;&nbsp;age:&nbsp;number;

&nbsp;&nbsp;constructor(name:&nbsp;string,&nbsp;age:&nbsp;number)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;this.name&nbsp;=&nbsp;name;
&nbsp;&nbsp;&nbsp;&nbsp;this.age&nbsp;=&nbsp;age;
&nbsp;&nbsp;&#125;
&#125;
``` ## Interface Support
 All built-in macros work with interfaces. For interfaces, methods are generated as functions in a namespace with the same name, using <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">self</code> as the first parameter:
 ```
/**&nbsp;@derive(Debug,&nbsp;Clone,&nbsp;PartialEq)&nbsp;*/
interface&nbsp;Point&nbsp;&#123;
&nbsp;&nbsp;x:&nbsp;number;
&nbsp;&nbsp;y:&nbsp;number;
&#125;

//&nbsp;Generated&nbsp;namespace:
//&nbsp;namespace&nbsp;Point&nbsp;&#123;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;toString(self:&nbsp;Point):&nbsp;string&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;clone(self:&nbsp;Point):&nbsp;Point&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;equals(self:&nbsp;Point,&nbsp;other:&nbsp;Point):&nbsp;boolean&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;hashCode(self:&nbsp;Point):&nbsp;number&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&#125;

const&nbsp;point:&nbsp;Point&nbsp;=&nbsp;&#123;&nbsp;x:&nbsp;10,&nbsp;y:&nbsp;20&nbsp;&#125;;

//&nbsp;Use&nbsp;the&nbsp;namespace&nbsp;functions
console.log(Point.toString(point));&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;"Point&nbsp;&#123;&nbsp;x:&nbsp;10,&nbsp;y:&nbsp;20&nbsp;&#125;"
const&nbsp;copy&nbsp;=&nbsp;Point.clone(point);&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;&#123;&nbsp;x:&nbsp;10,&nbsp;y:&nbsp;20&nbsp;&#125;
console.log(Point.equals(point,&nbsp;copy));&nbsp;//&nbsp;true
``` ## Enum Support
 All built-in macros work with enums. For enums, methods are generated as functions in a namespace with the same name:
 ```
/**&nbsp;@derive(Debug,&nbsp;Clone,&nbsp;PartialEq,&nbsp;Serialize,&nbsp;Deserialize)&nbsp;*/
enum&nbsp;Status&nbsp;&#123;
&nbsp;&nbsp;Active&nbsp;=&nbsp;"active",
&nbsp;&nbsp;Inactive&nbsp;=&nbsp;"inactive",
&nbsp;&nbsp;Pending&nbsp;=&nbsp;"pending",
&#125;

//&nbsp;Generated&nbsp;namespace:
//&nbsp;namespace&nbsp;Status&nbsp;&#123;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;toString(value:&nbsp;Status):&nbsp;string&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;clone(value:&nbsp;Status):&nbsp;Status&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;equals(a:&nbsp;Status,&nbsp;b:&nbsp;Status):&nbsp;boolean&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;hashCode(value:&nbsp;Status):&nbsp;number&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;toJSON(value:&nbsp;Status):&nbsp;string&nbsp;|&nbsp;number&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;fromJSON(data:&nbsp;unknown):&nbsp;Status&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&#125;

//&nbsp;Use&nbsp;the&nbsp;namespace&nbsp;functions
console.log(Status.toString(Status.Active));&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;"Status.Active"
console.log(Status.equals(Status.Active,&nbsp;Status.Active));&nbsp;//&nbsp;true
const&nbsp;json&nbsp;=&nbsp;Status.toJSON(Status.Pending);&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;"pending"
const&nbsp;parsed&nbsp;=&nbsp;Status.fromJSON("active");&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Status.Active
``` ## Type Alias Support
 All built-in macros work with type aliases. For object type aliases, field-aware methods are generated in a namespace:
 ```
/**&nbsp;@derive(Debug,&nbsp;Clone,&nbsp;PartialEq,&nbsp;Serialize,&nbsp;Deserialize)&nbsp;*/
type&nbsp;Point&nbsp;=&nbsp;&#123;
&nbsp;&nbsp;x:&nbsp;number;
&nbsp;&nbsp;y:&nbsp;number;
&#125;;

//&nbsp;Generated&nbsp;namespace:
//&nbsp;namespace&nbsp;Point&nbsp;&#123;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;toString(value:&nbsp;Point):&nbsp;string&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;clone(value:&nbsp;Point):&nbsp;Point&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;equals(a:&nbsp;Point,&nbsp;b:&nbsp;Point):&nbsp;boolean&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;hashCode(value:&nbsp;Point):&nbsp;number&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;toJSON(value:&nbsp;Point):&nbsp;Record&#x3C;string,&nbsp;unknown>&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&nbsp;&nbsp;export&nbsp;function&nbsp;fromJSON(data:&nbsp;unknown):&nbsp;Point&nbsp;&#123;&nbsp;...&nbsp;&#125;
//&nbsp;&#125;

const&nbsp;point:&nbsp;Point&nbsp;=&nbsp;&#123;&nbsp;x:&nbsp;10,&nbsp;y:&nbsp;20&nbsp;&#125;;
console.log(Point.toString(point));&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;"Point&nbsp;&#123;&nbsp;x:&nbsp;10,&nbsp;y:&nbsp;20&nbsp;&#125;"
const&nbsp;copy&nbsp;=&nbsp;Point.clone(point);&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;&#123;&nbsp;x:&nbsp;10,&nbsp;y:&nbsp;20&nbsp;&#125;
console.log(Point.equals(point,&nbsp;copy));&nbsp;//&nbsp;true
``` Union type aliases also work, using JSON-based implementations:
 ```
/**&nbsp;@derive(Debug,&nbsp;PartialEq)&nbsp;*/
type&nbsp;ApiStatus&nbsp;=&nbsp;"loading"&nbsp;|&nbsp;"success"&nbsp;|&nbsp;"error";

const&nbsp;status:&nbsp;ApiStatus&nbsp;=&nbsp;"success";
console.log(ApiStatus.toString(status));&nbsp;//&nbsp;"ApiStatus(\\"success\\")"
console.log(ApiStatus.equals("success",&nbsp;"success"));&nbsp;//&nbsp;true
``` ## Combining Macros
 All macros can be used together. They don't conflict and each generates independent methods:
 ```
const&nbsp;user&nbsp;=&nbsp;new&nbsp;User("Alice",&nbsp;30);

//&nbsp;Debug
console.log(user.toString());
//&nbsp;"User&nbsp;&#123;&nbsp;name:&nbsp;Alice,&nbsp;age:&nbsp;30&nbsp;&#125;"

//&nbsp;Clone
const&nbsp;copy&nbsp;=&nbsp;user.clone();
console.log(copy.name);&nbsp;//&nbsp;"Alice"

//&nbsp;Eq
console.log(user.equals(copy));&nbsp;//&nbsp;true
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