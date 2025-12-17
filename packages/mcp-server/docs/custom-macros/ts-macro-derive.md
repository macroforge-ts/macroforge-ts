# ts_macro_derive
 *The <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">#[ts_macro_derive]</code> attribute is a Rust procedural macro that registers your function as a Macroforge derive macro.*
 ## Basic Syntax
 ```
use&nbsp;macroforge_ts::macros::ts_macro_derive;
use&nbsp;macroforge_ts::ts_syn::&#123;TsStream,&nbsp;MacroforgeError&#125;;

#[ts_macro_derive(MacroName)]
pub&nbsp;fn&nbsp;my_macro(mut&nbsp;input:&nbsp;TsStream)&nbsp;->&nbsp;Result&#x3C;TsStream,&nbsp;MacroforgeError>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Macro&nbsp;implementation
&#125;
``` ## Attribute Options
 ### Name (Required)
 The first argument is the macro name that users will reference in <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">@<span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">derive<span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">()</code>:
 ```
#[ts_macro_derive(JSON)]&nbsp;&nbsp;//&nbsp;Users&nbsp;write:&nbsp;@derive(JSON)
pub&nbsp;fn&nbsp;derive_json(...)
``` ### Description
 Provides documentation for the macro:
 ```
#[ts_macro_derive(
&nbsp;&nbsp;&nbsp;&nbsp;JSON,
&nbsp;&nbsp;&nbsp;&nbsp;description&nbsp;=&nbsp;"Generates&nbsp;toJSON()&nbsp;returning&nbsp;a&nbsp;plain&nbsp;object"
)]
pub&nbsp;fn&nbsp;derive_json(...)
``` ### Attributes
 Declare which field-level decorators your macro accepts:
 ```
#[ts_macro_derive(
&nbsp;&nbsp;&nbsp;&nbsp;Debug,
&nbsp;&nbsp;&nbsp;&nbsp;description&nbsp;=&nbsp;"Generates&nbsp;toString()",
&nbsp;&nbsp;&nbsp;&nbsp;attributes(debug)&nbsp;&nbsp;//&nbsp;Allows&nbsp;@debug(&#123;&nbsp;...&nbsp;&#125;)&nbsp;on&nbsp;fields
)]
pub&nbsp;fn&nbsp;derive_debug(...)
``` > **Note:** Declared attributes become available as @attributeName({ options }) decorators in TypeScript. ## Function Signature
 ```
pub&nbsp;fn&nbsp;my_macro(mut&nbsp;input:&nbsp;TsStream)&nbsp;->&nbsp;Result&#x3C;TsStream,&nbsp;MacroforgeError>
``` | Parameter | Description |
| --- | --- |
| input: TsStream | Token stream containing the class/interface AST |
| Result<TsStream, MacroforgeError> | Returns generated code or an error with source location |
 ## Parsing Input
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#B392F0;--shiki-light:#6F42C1">parse_ts_macro_input!</code> to convert the token stream:
 ```
use&nbsp;macroforge_ts::ts_syn::&#123;Data,&nbsp;DeriveInput,&nbsp;parse_ts_macro_input&#125;;

#[ts_macro_derive(MyMacro)]
pub&nbsp;fn&nbsp;my_macro(mut&nbsp;input:&nbsp;TsStream)&nbsp;->&nbsp;Result&#x3C;TsStream,&nbsp;MacroforgeError>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;input&nbsp;=&nbsp;parse_ts_macro_input!(input&nbsp;as&nbsp;DeriveInput);

&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Access&nbsp;class&nbsp;data
&nbsp;&nbsp;&nbsp;&nbsp;match&nbsp;&#x26;input.data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Class(class)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;class_name&nbsp;=&nbsp;input.name();
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;fields&nbsp;=&nbsp;class.fields();
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;...
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Interface(interface)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Handle&nbsp;interfaces
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Enum(_)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Handle&nbsp;enums&nbsp;(if&nbsp;supported)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ## DeriveInput Structure
 ```
struct&nbsp;DeriveInput&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;ident:&nbsp;Ident,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;The&nbsp;type&nbsp;name
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;span:&nbsp;SpanIR,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Span&nbsp;of&nbsp;the&nbsp;type&nbsp;definition
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;attrs:&nbsp;Vec&#x3C;Attribute>,&nbsp;&nbsp;//&nbsp;Decorators&nbsp;(excluding&nbsp;@derive)
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;data:&nbsp;Data,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;The&nbsp;parsed&nbsp;type&nbsp;data
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;context:&nbsp;MacroContextIR,&nbsp;//&nbsp;Macro&nbsp;context&nbsp;with&nbsp;spans

&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Helper&nbsp;methods
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;name(&#x26;self)&nbsp;->&nbsp;&#x26;str;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Get&nbsp;the&nbsp;type&nbsp;name
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;decorator_span(&#x26;self)&nbsp;->&nbsp;SpanIR;&nbsp;&nbsp;//&nbsp;Span&nbsp;of&nbsp;@derive&nbsp;decorator
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;as_class(&#x26;self)&nbsp;->&nbsp;Option&#x3C;&#x26;DataClass>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;as_interface(&#x26;self)&nbsp;->&nbsp;Option&#x3C;&#x26;DataInterface>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;as_enum(&#x26;self)&nbsp;->&nbsp;Option&#x3C;&#x26;DataEnum>;
&#125;

enum&nbsp;Data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;Class(DataClass),
&nbsp;&nbsp;&nbsp;&nbsp;Interface(DataInterface),
&nbsp;&nbsp;&nbsp;&nbsp;Enum(DataEnum),
&nbsp;&nbsp;&nbsp;&nbsp;TypeAlias(DataTypeAlias),
&#125;

impl&nbsp;DataClass&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;fields(&#x26;self)&nbsp;->&nbsp;&#x26;[FieldIR];
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;methods(&#x26;self)&nbsp;->&nbsp;&#x26;[MethodSigIR];
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;field_names(&#x26;self)&nbsp;->&nbsp;impl&nbsp;Iterator&#x3C;Item&nbsp;=&nbsp;&#x26;str>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;field(&#x26;self,&nbsp;name:&nbsp;&#x26;str)&nbsp;->&nbsp;Option&#x3C;&#x26;FieldIR>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;body_span(&#x26;self)&nbsp;->&nbsp;SpanIR;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;For&nbsp;inserting&nbsp;code&nbsp;into&nbsp;class&nbsp;body
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;type_params(&#x26;self)&nbsp;->&nbsp;&#x26;[String];&nbsp;//&nbsp;Generic&nbsp;type&nbsp;parameters
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;heritage(&#x26;self)&nbsp;->&nbsp;&#x26;[String];&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;extends/implements&nbsp;clauses
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;is_abstract(&#x26;self)&nbsp;->&nbsp;bool;
&#125;

impl&nbsp;DataInterface&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;fields(&#x26;self)&nbsp;->&nbsp;&#x26;[InterfaceFieldIR];
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;methods(&#x26;self)&nbsp;->&nbsp;&#x26;[InterfaceMethodIR];
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;field_names(&#x26;self)&nbsp;->&nbsp;impl&nbsp;Iterator&#x3C;Item&nbsp;=&nbsp;&#x26;str>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;field(&#x26;self,&nbsp;name:&nbsp;&#x26;str)&nbsp;->&nbsp;Option&#x3C;&#x26;InterfaceFieldIR>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;body_span(&#x26;self)&nbsp;->&nbsp;SpanIR;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;type_params(&#x26;self)&nbsp;->&nbsp;&#x26;[String];
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;heritage(&#x26;self)&nbsp;->&nbsp;&#x26;[String];&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;extends&nbsp;clauses
&#125;

impl&nbsp;DataEnum&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;variants(&#x26;self)&nbsp;->&nbsp;&#x26;[EnumVariantIR];
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;variant_names(&#x26;self)&nbsp;->&nbsp;impl&nbsp;Iterator&#x3C;Item&nbsp;=&nbsp;&#x26;str>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;variant(&#x26;self,&nbsp;name:&nbsp;&#x26;str)&nbsp;->&nbsp;Option&#x3C;&#x26;EnumVariantIR>;
&#125;

impl&nbsp;DataTypeAlias&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;body(&#x26;self)&nbsp;->&nbsp;&#x26;TypeBody;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;type_params(&#x26;self)&nbsp;->&nbsp;&#x26;[String];
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;is_union(&#x26;self)&nbsp;->&nbsp;bool;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;is_object(&#x26;self)&nbsp;->&nbsp;bool;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;as_union(&#x26;self)&nbsp;->&nbsp;Option&#x3C;&#x26;[TypeMember]>;
&nbsp;&nbsp;&nbsp;&nbsp;fn&nbsp;as_object(&#x26;self)&nbsp;->&nbsp;Option&#x3C;&#x26;[InterfaceFieldIR]>;
&#125;
``` ## Accessing Field Data
 ### Class Fields (FieldIR)
 ```
struct&nbsp;FieldIR&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;name:&nbsp;String,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Field&nbsp;name
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;span:&nbsp;SpanIR,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Field&nbsp;span
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;ts_type:&nbsp;String,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;TypeScript&nbsp;type&nbsp;annotation
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;optional:&nbsp;bool,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Whether&nbsp;field&nbsp;has&nbsp;?
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;readonly:&nbsp;bool,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Whether&nbsp;field&nbsp;is&nbsp;readonly
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;visibility:&nbsp;Visibility,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Public,&nbsp;Protected,&nbsp;Private
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;decorators:&nbsp;Vec&#x3C;DecoratorIR>,&nbsp;//&nbsp;Field&nbsp;decorators
&#125;
``` ### Interface Fields (InterfaceFieldIR)
 ```
struct&nbsp;InterfaceFieldIR&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;name:&nbsp;String,
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;span:&nbsp;SpanIR,
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;ts_type:&nbsp;String,
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;optional:&nbsp;bool,
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;readonly:&nbsp;bool,
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;decorators:&nbsp;Vec&#x3C;DecoratorIR>,
&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Note:&nbsp;No&nbsp;visibility&nbsp;field&nbsp;(interfaces&nbsp;are&nbsp;always&nbsp;public)
&#125;
``` ### Enum Variants (EnumVariantIR)
 ```
struct&nbsp;EnumVariantIR&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;name:&nbsp;String,
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;span:&nbsp;SpanIR,
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;value:&nbsp;EnumValue,&nbsp;&nbsp;//&nbsp;Auto,&nbsp;String(String),&nbsp;or&nbsp;Number(f64)
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;decorators:&nbsp;Vec&#x3C;DecoratorIR>,
&#125;
``` ### Decorator Structure
 ```
struct&nbsp;DecoratorIR&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;name:&nbsp;String,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;e.g.,&nbsp;"serde"
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;args_src:&nbsp;String,&nbsp;&nbsp;//&nbsp;Raw&nbsp;args&nbsp;text,&nbsp;e.g.,&nbsp;"skip,&nbsp;rename:&nbsp;'id'"
&nbsp;&nbsp;&nbsp;&nbsp;pub&nbsp;span:&nbsp;SpanIR,
&#125;
``` > **Note:** To check for decorators, iterate through field.decorators and check decorator.name. For parsing options, you can write helper functions like the built-in macros do. ## Adding Imports
 If your macro generates code that requires imports, use the <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">add_import</code> method on <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">TsStream</code>:
 ```
//&nbsp;Add&nbsp;an&nbsp;import&nbsp;to&nbsp;be&nbsp;inserted&nbsp;at&nbsp;the&nbsp;top&nbsp;of&nbsp;the&nbsp;file
let&nbsp;mut&nbsp;output&nbsp;=&nbsp;body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;validate():&nbsp;ValidationResult&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;validateFields(this);
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;;

//&nbsp;This&nbsp;will&nbsp;add:&nbsp;import&nbsp;&#123;&nbsp;validateFields,&nbsp;ValidationResult&nbsp;&#125;&nbsp;from&nbsp;"my-validation-lib";
output.add_import("validateFields",&nbsp;"my-validation-lib");
output.add_import("ValidationResult",&nbsp;"my-validation-lib");

Ok(output)
``` > **Note:** Imports are automatically deduplicated. If the same import already exists in the file, it won't be added again. ## Returning Errors
 Use <code class="shiki-inline"><span class="line"><span style="--shiki-dark:#E1E4E8;--shiki-light:#24292E">MacroforgeError</code> to report errors with source locations:
 ```
#[ts_macro_derive(ClassOnly)]
pub&nbsp;fn&nbsp;class_only(mut&nbsp;input:&nbsp;TsStream)&nbsp;->&nbsp;Result&#x3C;TsStream,&nbsp;MacroforgeError>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;input&nbsp;=&nbsp;parse_ts_macro_input!(input&nbsp;as&nbsp;DeriveInput);

&nbsp;&nbsp;&nbsp;&nbsp;match&nbsp;&#x26;input.data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Class(_)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//&nbsp;Generate&nbsp;code...
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Ok(body!&nbsp;&#123;&nbsp;/*&nbsp;...&nbsp;*/&nbsp;&#125;)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;_&nbsp;=>&nbsp;Err(MacroforgeError::new(
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;input.decorator_span(),
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"@derive(ClassOnly)&nbsp;can&nbsp;only&nbsp;be&nbsp;used&nbsp;on&nbsp;classes",
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;)),
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ## Complete Example
 ```
use&nbsp;macroforge_ts::macros::&#123;ts_macro_derive,&nbsp;body&#125;;
use&nbsp;macroforge_ts::ts_syn::&#123;
&nbsp;&nbsp;&nbsp;&nbsp;Data,&nbsp;DeriveInput,&nbsp;FieldIR,&nbsp;MacroforgeError,&nbsp;TsStream,&nbsp;parse_ts_macro_input,
&#125;;

//&nbsp;Helper&nbsp;function&nbsp;to&nbsp;check&nbsp;if&nbsp;a&nbsp;field&nbsp;has&nbsp;a&nbsp;decorator
fn&nbsp;has_decorator(field:&nbsp;&#x26;FieldIR,&nbsp;name:&nbsp;&#x26;str)&nbsp;->&nbsp;bool&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;field.decorators.iter().any(|d|&nbsp;d.name.eq_ignore_ascii_case(name))
&#125;

#[ts_macro_derive(
&nbsp;&nbsp;&nbsp;&nbsp;Validate,
&nbsp;&nbsp;&nbsp;&nbsp;description&nbsp;=&nbsp;"Generates&nbsp;a&nbsp;validate()&nbsp;method",
&nbsp;&nbsp;&nbsp;&nbsp;attributes(validate)
)]
pub&nbsp;fn&nbsp;derive_validate(mut&nbsp;input:&nbsp;TsStream)&nbsp;->&nbsp;Result&#x3C;TsStream,&nbsp;MacroforgeError>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;input&nbsp;=&nbsp;parse_ts_macro_input!(input&nbsp;as&nbsp;DeriveInput);

&nbsp;&nbsp;&nbsp;&nbsp;match&nbsp;&#x26;input.data&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Data::Class(class)&nbsp;=>&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let&nbsp;validations:&nbsp;Vec&#x3C;_>&nbsp;=&nbsp;class.fields()
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;.iter()
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;.filter(|f|&nbsp;has_decorator(f,&nbsp;"validate"))
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;.collect();

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Ok(body!&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;validate():&nbsp;string[]&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;const&nbsp;errors:&nbsp;string[]&nbsp;=&nbsp;[];
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;#for&nbsp;field&nbsp;in&nbsp;validations&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;if&nbsp;(!this.@&#123;field.name&#125;)&nbsp;&#123;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;errors.push("@&#123;field.name&#125;&nbsp;is&nbsp;required");
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#123;/for&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;return&nbsp;errors;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;_&nbsp;=>&nbsp;Err(MacroforgeError::new(
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;input.decorator_span(),
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"@derive(Validate)&nbsp;only&nbsp;works&nbsp;on&nbsp;classes",
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;)),
&nbsp;&nbsp;&nbsp;&nbsp;&#125;
&#125;
``` ## Next Steps
 - [Learn the template syntax](../../docs/custom-macros/ts-quote)