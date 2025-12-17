# Custom Macros
 *Macroforge allows you to create custom derive macros in Rust. Your macros have full access to the class AST and can generate any TypeScript code.*
 ## Overview
 Custom macros are written in Rust and compiled to native Node.js addons. The process involves:
 1. Creating a Rust crate with NAPI bindings
 2. Defining macro functions with <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#[ts_macro_derive]</code>
 3. Using <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">macroforge_ts_quote</code> to generate TypeScript code
 4. Building and publishing as an npm package
 ## Quick Example
 ```
use&nbsp;macroforge_ts::macros::&#123;ts_macro_derive,&nbsp;body&#125;;
use&nbsp;macroforge_ts::ts_syn::&#123;Data,&nbsp;DeriveInput,&nbsp;MacroforgeError,&nbsp;TsStream,&nbsp;parse_ts_macro_input&#125;;

#[ts_macro_derive(
&nbsp;&nbsp;&nbsp;&nbsp;JSON,
&nbsp;&nbsp;&nbsp;&nbsp;description&nbsp;=&nbsp;"Generates&nbsp;toJSON()&nbsp;returning&nbsp;a&nbsp;plain&nbsp;object"
)]
pub&nbsp;fn&nbsp;derive_json(mut&nbsp;input:&nbsp;TsStream)&nbsp;->&nbsp;Result&#x3C;TsStream,&nbsp;MacroforgeError>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;input&nbsp;=&nbsp;parse_ts_macro_input!(input&nbsp;as&nbsp;DeriveInput);

&nbsp;&nbsp;&nbsp;&nbsp;match&nbsp;&#x26;input.data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Class(class)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Ok(body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;toJSON():&nbsp;Record&#x3C;string,&nbsp;unknown>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;field&nbsp;in&nbsp;class.field_names()&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;@&#123;field&#125;:&nbsp;this.@&#123;field&#125;,
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;_&nbsp;=>&nbsp;Err(MacroforgeError::new(
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;input.decorator_span(),
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"@derive(JSON)&nbsp;only&nbsp;works&nbsp;on&nbsp;classes",
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;)),
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ## Using Custom Macros
 Once your macro package is published, users can import and use it:
 ```
/**&nbsp;import&nbsp;macro&nbsp;&#123;&nbsp;JSON&nbsp;&#125;&nbsp;from&nbsp;"@my/macros";&nbsp;*/

/**&nbsp;@derive(JSON)&nbsp;*/
class&nbsp;User&nbsp;&#123;
&nbsp;&nbsp;name:&nbsp;string;
&nbsp;&nbsp;age:&nbsp;number;

&nbsp;&nbsp;constructor(name:&nbsp;string,&nbsp;age:&nbsp;number)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;this.name&nbsp;=&nbsp;name;
&nbsp;&nbsp;&nbsp;&nbsp;this.age&nbsp;=&nbsp;age;
&nbsp;&nbsp;&#125;
&#125;

const&nbsp;user&nbsp;=&nbsp;new&nbsp;User("Alice",&nbsp;30);
console.log(user.toJSON());&nbsp;//&nbsp;&#123;&nbsp;name:&nbsp;"Alice",&nbsp;age:&nbsp;30&nbsp;&#125;
``` > **Note:** The import macro comment tells Macroforge which package provides the macro. ## Getting Started
 Follow these guides to create your own macros:
 - [Set up a Rust macro crate](../docs/custom-macros/rust-setup)
 - [Learn the #[ts_macro_derive] attribute](../docs/custom-macros/ts-macro-derive)
 - [Learn the template syntax](../docs/custom-macros/ts-quote)