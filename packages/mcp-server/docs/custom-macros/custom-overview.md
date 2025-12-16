# Custom Macros
 *Macroforge allows you to create custom derive macros in Rust. Your macros have full access to the class AST and can generate any TypeScript code.*
 ## Overview
 Custom macros are written in Rust and compiled to native Node.js addons. The process involves:
 1. Creating a Rust crate with NAPI bindings
 2. Defining macro functions with `#[ts_macro_derive]`
 3. Using `macroforge_ts_quote` to generate TypeScript code
 4. Building and publishing as an npm package
 ## Quick Example
 ```
use macroforge_ts::macros::&#123;ts_macro_derive, body&#125;;
use macroforge_ts::ts_syn::&#123;Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input&#125;;

#[ts_macro_derive(
    JSON,
    description = "Generates toJSON() returning a plain object"
)]
pub fn derive_json(mut input: TsStream) -> Result&#x3C;TsStream, MacroforgeError> &#123;
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &#x26;input.data &#123;
        Data::Class(class) => &#123;
            Ok(body! &#123;
                toJSON(): Record&#x3C;string, unknown> &#123;
                    return &#123;
                        &#123;#for field in class.field_names()&#125;
                            @&#123;field&#125;: this.@&#123;field&#125;,
                        &#123;/for&#125;
                    &#125;;
                &#125;
            &#125;)
        &#125;
        _ => Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(JSON) only works on classes",
        )),
    &#125;
&#125;
``` ## Using Custom Macros
 Once your macro package is published, users can import and use it:
 ```
/** import macro &#123; JSON &#125; from "@my/macros"; */

/** @derive(JSON) */
class User &#123;
  name: string;
  age: number;

  constructor(name: string, age: number) &#123;
    this.name = name;
    this.age = age;
  &#125;
&#125;

const user = new User("Alice", 30);
console.log(user.toJSON()); // &#123; name: "Alice", age: 30 &#125;
``` > **Note:** The import macro comment tells Macroforge which package provides the macro. ## Getting Started
 Follow these guides to create your own macros:
 - [Set up a Rust macro crate](../docs/custom-macros/rust-setup)
 - [Learn the #[ts_macro_derive] attribute](../docs/custom-macros/ts-macro-derive)
 - [Learn the template syntax](../docs/custom-macros/ts-quote)