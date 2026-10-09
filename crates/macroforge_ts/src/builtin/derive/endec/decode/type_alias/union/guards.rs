//! `{union}{Variant}Is` guards for the inline variants of a union, which have
//! no decoder of their own to carry one.

use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::members::Union;
use convert_case::{Case, Casing};

/// One guard per inline variant: internally tagged objects, `{ tag } & Type`
/// intersections and externally tagged objects, named from the union and tag.
pub(super) fn variant_guards(u: &Union) -> TsStream {
    let Union {
        object_variants,
        intersection_variants,
        external_object_variants,
        type_name,
        full_type_name,
        tag_field,
        ..
    } = u;
    let camel_name = type_name.to_case(Case::Camel);
    let mut guards: Vec<TsStream> = Vec::new();

    // Internally tagged inline object variants: discriminate by the tag
    // field equalling the variant's discriminant value.
    for ov in object_variants {
        let variant_pascal = ov.tag_value.to_case(Case::Pascal);
        let fn_ident = ts_ident!("{}{}Is", camel_name, variant_pascal);
        let tag = tag_field;
        let value = ov.tag_value.as_str();
        guards.push(ts_template! {
            export function @{fn_ident}(__v: unknown): __v is Extract<@{full_type_name}, { @{tag}: "@{value}" }> {
                return __v !== null
                    && typeof __v === "object"
                    && (__v as Record<string, unknown>)["@{tag}"] === "@{value}";
            }
        });
    }

    // Intersection variants (`{ tag: 'X' } & TypeRef`): same shape, the
    // tag field discriminates among union members.
    for iv in intersection_variants {
        let variant_pascal = iv.tag_value.to_case(Case::Pascal);
        let fn_ident = ts_ident!("{}{}Is", camel_name, variant_pascal);
        let tag = tag_field;
        let value = iv.tag_value.as_str();
        guards.push(ts_template! {
            export function @{fn_ident}(__v: unknown): __v is Extract<@{full_type_name}, { @{tag}: "@{value}" }> {
                return __v !== null
                    && typeof __v === "object"
                    && (__v as Record<string, unknown>)["@{tag}"] === "@{value}";
            }
        });
    }

    // Externally tagged inline objects (`{ TypeName: { ...fields } }`) :
    // discriminate on whether the variant's key is present.
    for ov in external_object_variants {
        let variant_pascal = ov.name.to_case(Case::Pascal);
        let fn_ident = ts_ident!("{}{}Is", camel_name, variant_pascal);
        let key = ov.name.as_str();
        guards.push(ts_template! {
            export function @{fn_ident}(__v: unknown): __v is Extract<@{full_type_name}, { @{key}: any }> {
                return __v !== null
                    && typeof __v === "object"
                    && "@{key}" in __v;
            }
        });
    }

    TsStream::merge_all(guards)
}
