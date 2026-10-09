//! # Newtype Macro Implementation
//!
//! `$Newtype<T>` is a type-position macro that gives `T` a nominal brand. Two
//! newtypes over the same base are distinct types, and neither accepts a bare `T`
//! without going through a constructor or guard.
//!
//! ## Generated Output
//!
//! ```typescript
//! export type Meters = $Newtype<number>;
//! ```
//!
//! expands to
//!
//! ```typescript
//! export type Meters = number & { readonly [__mf_newtype_42]: true };
//! declare const __mf_newtype_42: unique symbol;
//! ```
//!
//! Every expansion declares its own `unique symbol`, named from the expansion's
//! id, so the brand needs no name and cannot collide, even for a `$Newtype` that
//! another macro emits. The `declare const` is type-only and erases at runtime. A base
//! with a top-level `|` or `&`, or a function or conditional type, is parenthesized.
//!
//! `$Newtype` is built in and needs no import. It nests, so
//! `$Newtype<$Newtype<number>>` carries both brands.
//!
//! The brand does not involve the alias's own type parameters: with
//! `type Id<T> = $Newtype<string>`, `Id<User>` and `Id<Order>` are the same type.
//! A brand that should differ per parameter names it in the type, as in
//! `type Id<T> = string & { readonly [IdOf]: T }` with a `declare const IdOf: unique symbol`.
//!
//! ## Constructing Values
//!
//! A newtype has no constructor of its own. Derive `Decode` on the alias: for a
//! primitive base, `decode` checks the `typeof` and runs any validators written on
//! the alias, which makes it the validated constructor, and `is` the narrowing guard.
//!
//! ```typescript
//! /** @derive(Encode, Decode) */
//! /** @endec(nonNegative, finite) */
//! export type Meters = $Newtype<number>;
//!
//! const parsed = Meters.decode(input);   // { success, value } | { success, errors }
//! if (Meters.is(distance)) {
//!     travel(distance);                   // narrowed to Meters
//! }
//! ```
//!
//! `Encode` writes the bare primitive, and `Default` returns the primitive's
//! default (or the alias's `@default(value)`) cast to the newtype.
//!
//! ## Error Handling
//!
//! The macro reports an expansion error when it does not receive exactly one type
//! argument.

use oxc::allocator::Allocator;
use oxc::ast::ast::{Statement, TSType};
use oxc::parser::Parser;
use oxc::span::{GetSpan, SourceType};

use crate::builtin::derive::common::rendered;
use crate::macros::{ts_macro, ts_template};
use crate::ts_syn::abi::ir::{NEWTYPE_BRAND_PREFIX, NEWTYPE_MACRO};
use crate::ts_syn::{MacroforgeError, Patch, SpanIR, TsStream, ts_ident};

#[cfg(test)]
mod tests;

#[ts_macro(
    Newtype,
    description = "Brand a type nominally with a per-expansion unique symbol"
)]
pub fn newtype_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let ctx = input
        .context()
        .ok_or_else(|| MacroforgeError::new_global("$Newtype: no macro context available"))?;
    let base = single_type_argument(input.source())
        .map_err(|message| MacroforgeError::new(ctx.target_span, format!("$Newtype {message}")))?;

    let symbol = ts_ident!("{}{}", NEWTYPE_BRAND_PREFIX, ctx.expansion_id);
    let mut out = ts_template! { @{base} & { readonly [@{&symbol}]: true } };
    out.runtime_patches.push(Patch::Insert {
        at: SpanIR::new(1, 1),
        code: rendered(ts_template! { declare const @{&symbol}: unique symbol; }) + "\n",
        source_macro: Some(NEWTYPE_MACRO.to_string()),
    });
    Ok(out)
}

/// Parse the macro's type-argument text and return it as the left operand of
/// an intersection, parenthesized where `&` would otherwise bind into it.
fn single_type_argument(args: &str) -> Result<String, String> {
    if args.trim().is_empty() {
        return Err("takes exactly one type argument, got 0".to_string());
    }
    let wrapped = format!("type __mf_Args = __mf_Wrap<{args}>;");
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, &wrapped, SourceType::ts()).parse();
    if !parsed.diagnostics.is_empty() {
        return Err(format!("could not parse type argument `{}`", args.trim()));
    }
    let Some(Statement::TSTypeAliasDeclaration(alias)) = parsed.program.body.first() else {
        return Err(format!("could not parse type argument `{}`", args.trim()));
    };
    let TSType::TSTypeReference(reference) = &alias.type_annotation else {
        return Err(format!("could not parse type argument `{}`", args.trim()));
    };
    let params = reference
        .type_arguments
        .as_ref()
        .map(|arguments| arguments.params.as_slice())
        .unwrap_or_default();
    let [param] = params else {
        return Err(format!(
            "takes exactly one type argument, got {}",
            params.len()
        ));
    };
    let span = param.span();
    let text = &wrapped[span.start as usize..span.end as usize];
    let needs_parens = matches!(
        param,
        TSType::TSUnionType(_)
            | TSType::TSIntersectionType(_)
            | TSType::TSFunctionType(_)
            | TSType::TSConstructorType(_)
            | TSType::TSConditionalType(_)
    );
    Ok(if needs_parens {
        rendered(ts_template! { (@{text}) })
    } else {
        text.to_string()
    })
}
