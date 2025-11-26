ts_quote! Macro Syntax Guide

The ts_quote! macro is an ergonomic wrapper around swc_core::quote!. It allows you to write TypeScript/JavaScript code templates with inline variable bindings, similar to string interpolation in modern languages.

1. Basic Structure

The macro takes a string literal containing the code template, optionally followed by an as Type cast to specify which AST node to generate.

// Syntax: ts_quote!("code" as TargetASTType)
let stmt = ts_quote!("const x = 1;" as Stmt);

Common Target Types:

Expr (Expression) - Values, function calls, object literals.

Stmt (Statement) - Variable declarations, loops, return statements.

ModuleItem - Export/Import declarations, top-level statements.

Pat (Pattern) - Destructuring patterns, function arguments.

2. Variable Interpolation $(...)

Instead of the verbose argument mapping required by vanilla SWC, ts_quote! supports inline injection.

A. Simple Identifier Injection

Inject a Rust variable that holds an Ident (identifier).

let my_var = ident!("myVar");

// Generates: const myVar = 10;
ts_quote!("const $(my_var) = 10;" as Stmt)

B. Typed Injection

If the Rust variable is an Expr, Stmt, or other AST node, you must provide a type hint inside the interpolation.

let val_expr: Expr = 100.into();

// Generates: const x = 100;
// Maps Rust `val_expr` to the template slot
ts_quote!("const x = $(val_expr: Expr);" as Stmt)

C. Explicit Property Access

You can inject object properties directly.

let field_name = ident!("id");

// Generates: this.id = 5;
ts_quote!("this.$(field_name) = 5;" as Stmt)

3. Object Literals

When generating object literals (Expr::Object), standard JavaScript parsing rules apply.

Rule: An object literal { key: val } at the start of a statement is parsed as a block. You must wrap object literals in parentheses ({}) to force them to be parsed as expressions.

let val_expr = ts_quote!("1" as Expr);

// ❌ WRONG: Parsed as a block with label 'a'
// ts_quote!("{ a: $(val_expr: Expr) }" as Expr)

// ✅ CORRECT: Parsed as object expression
ts_quote!("({ a: $(val_expr: Expr) })" as Expr)

Shorthand Properties:

let name = ident!("name");
// Generates: { name }
ts_quote!("({ $(name) })" as Expr)

Computed Properties:

let key = ts_quote!("'dynamic'" as Expr);
// Generates: { ['dynamic']: 1 }
ts_quote!("({ [$(key: Expr)]: 1 })" as Expr)

4. Complex Examples

Class Method Generation

let class_name = ident!("User");
let method_name = ident!("save");
let body_stmt = ts_quote!("return true;" as Stmt);

ts_quote!(
"$(class_name).prototype.$(method_name) = function() {
console.log('Saving...');
$(body_stmt: Stmt)
};" as Stmt
)

Array Literals

let item1 = ts_quote!("1" as Expr);
let item2 = ts_quote!("2" as Expr);

ts_quote!("[$(item1: Expr), $(item2: Expr)]" as Expr)

5. Comparison with Vanilla SWC

Feature

Vanilla swc_core::quote!

ts_quote!

Binding Syntax

$var

$(var)

Mapping

Manual (var = rust_var) at end

Inline ($(rust_var))

Type Hints

var: Type = rust_var

$(rust_var: Type)

Repetition

Variable names often typed 3 times

Variable typed once

// Vanilla
quote!(
"const $name = $val;" as Stmt,
name = rust_name_var,
val: Expr = rust_val_var
);

// ts_quote!
ts_quote!(
"const $(rust_name_var) = $(rust_val_var: Expr);" as Stmt
);
