//! IR Variant Coverage Tests - Type Nodes
//!
//! Tests for: QualifiedName, LiteralType, IntersectionType, TupleType, OptionalType,
//! RestType, FnType, ConstructorType, ObjectType, ParenType, TypeofType, KeyofType,
//! IndexedAccessType, ConditionalType, InferType, MappedType, ImportType, ThisType

use macroforge_ts_quote::ts_template;

// =============================================================================
// Qualified Name Types
// =============================================================================

#[test]
fn test_qualified_name_type() {
    let stream = ts_template! {
        type Handler = Express.RequestHandler;
    };
    let source = stream.source();
    assert!(source.contains("Express.RequestHandler") || source.contains("Express .RequestHandler"), "Expected 'Express.RequestHandler'. Got:\n{}", source);
}

#[test]
fn test_qualified_name_nested() {
    let stream = ts_template! {
        type DeepType = Namespace.SubNamespace.Type;
    };
    let source = stream.source();
    assert!(source.contains("Namespace") && source.contains("SubNamespace") && source.contains("Type"), "Expected nested namespace. Got:\n{}", source);
}

// =============================================================================
// Literal Types
// =============================================================================

#[test]
fn test_literal_type_string() {
    let stream = ts_template! {
        type Direction = "up" | "down" | "left" | "right";
    };
    let source = stream.source();
    // Check for string literals - may be quoted differently
    assert!(source.contains("up") && source.contains("down"), "Expected string literals. Got:\n{}", source);
}

#[test]
fn test_literal_type_number() {
    let stream = ts_template! {
        type HttpStatus = 200 | 404 | 500;
    };
    let source = stream.source();
    assert!(source.contains("200") && source.contains("404") && source.contains("500"), "Expected number literals. Got:\n{}", source);
}

#[test]
fn test_literal_type_boolean() {
    let stream = ts_template! {
        type AlwaysTrue = true;
    };
    let source = stream.source();
    assert!(source.contains("true"), "Expected 'true'. Got:\n{}", source);
}

// Note: Template literal types use backticks which are not valid Rust tokens.
// test_literal_type_template cannot be tested directly in ts_template! macros.

// =============================================================================
// Intersection Types
// =============================================================================

#[test]
fn test_intersection_type_simple() {
    let stream = ts_template! {
        type Combined = TypeA & TypeB;
    };
    let source = stream.source();
    assert!(source.contains("&"), "Expected '&'. Got:\n{}", source);
    assert!(source.contains("TypeA") && source.contains("TypeB"), "Expected TypeA and TypeB. Got:\n{}", source);
}

#[test]
fn test_intersection_type_multiple() {
    let stream = ts_template! {
        type Full = Base & Mixin1 & Mixin2 & Mixin3;
    };
    let source = stream.source();
    let count = source.matches("&").count();
    assert!(count >= 3, "Expected at least 3 '&'. Got {} in:\n{}", count, source);
}

#[test]
fn test_intersection_type_with_object() {
    let stream = ts_template! {
        type Extended = Base & { extra: string };
    };
    let source = stream.source();
    assert!(source.contains("&") && source.contains("extra"), "Expected intersection with object. Got:\n{}", source);
}

// =============================================================================
// Tuple Types
// =============================================================================

#[test]
fn test_tuple_type_simple() {
    let stream = ts_template! {
        type Pair = [string, number];
    };
    let source = stream.source();
    assert!(source.contains("[") && source.contains("]"), "Expected tuple brackets. Got:\n{}", source);
    assert!(source.contains("string") && source.contains("number"), "Expected string and number. Got:\n{}", source);
}

#[test]
fn test_tuple_type_labeled() {
    let stream = ts_template! {
        type Point = [x: number, y: number, z: number];
    };
    let source = stream.source();
    assert!(source.contains("x") && source.contains("y") && source.contains("z"), "Expected labeled tuple. Got:\n{}", source);
}

#[test]
fn test_tuple_type_optional() {
    let stream = ts_template! {
        type Args = [string, number?, boolean?];
    };
    let source = stream.source();
    assert!(source.contains("?"), "Expected optional elements. Got:\n{}", source);
}

#[test]
fn test_tuple_type_with_rest() {
    let stream = ts_template! {
        type VarArgs = [first: string, ...rest: number[]];
    };
    let source = stream.source();
    assert!(source.contains("..."), "Expected rest element. Got:\n{}", source);
}

#[test]
fn test_tuple_type_readonly() {
    let stream = ts_template! {
        type Frozen = readonly [number, number];
    };
    let source = stream.source();
    assert!(source.contains("readonly"), "Expected 'readonly'. Got:\n{}", source);
}

// =============================================================================
// Function Types
// =============================================================================

#[test]
fn test_fn_type_simple() {
    let stream = ts_template! {
        type Callback = (value: string) => void;
    };
    let source = stream.source();
    assert!(source.contains("=>"), "Expected '=>'. Got:\n{}", source);
    assert!(source.contains("void"), "Expected 'void'. Got:\n{}", source);
}

#[test]
fn test_fn_type_generic() {
    let stream = ts_template! {
        type Mapper<T, U> = (input: T) => U;
    };
    let source = stream.source();
    assert!(source.contains("=>"), "Expected '=>'. Got:\n{}", source);
    assert!(source.contains("T") && source.contains("U"), "Expected T and U. Got:\n{}", source);
}

#[test]
fn test_fn_type_no_params() {
    let stream = ts_template! {
        type Factory = () => object;
    };
    let source = stream.source();
    assert!(source.contains("()") && source.contains("=>"), "Expected '() =>'. Got:\n{}", source);
}

#[test]
fn test_fn_type_rest_params() {
    let stream = ts_template! {
        type VarFunc = (...args: string[]) => void;
    };
    let source = stream.source();
    assert!(source.contains("...args") || source.contains("... args"), "Expected '...args'. Got:\n{}", source);
}

// =============================================================================
// Constructor Types
// =============================================================================

#[test]
fn test_constructor_type() {
    let stream = ts_template! {
        type Constructor<T> = new (...args: any[]) => T;
    };
    let source = stream.source();
    assert!(source.contains("new"), "Expected 'new'. Got:\n{}", source);
    assert!(source.contains("=>"), "Expected '=>'. Got:\n{}", source);
}

#[test]
fn test_constructor_type_simple() {
    let stream = ts_template! {
        type Newable = new () => object;
    };
    let source = stream.source();
    assert!(source.contains("new") && source.contains("()"), "Expected 'new ()'. Got:\n{}", source);
}

// =============================================================================
// Object Types (Type Literals)
// =============================================================================

#[test]
fn test_object_type_simple() {
    let stream = ts_template! {
        type Config = { name: string; value: number };
    };
    let source = stream.source();
    assert!(source.contains("name") && source.contains("string"), "Expected 'name: string'. Got:\n{}", source);
    assert!(source.contains("value") && source.contains("number"), "Expected 'value: number'. Got:\n{}", source);
}

#[test]
fn test_object_type_optional() {
    let stream = ts_template! {
        type Options = { required: string; optional?: number };
    };
    let source = stream.source();
    assert!(source.contains("optional?") || source.contains("optional ?"), "Expected 'optional?'. Got:\n{}", source);
}

#[test]
fn test_object_type_readonly() {
    let stream = ts_template! {
        type Frozen = { readonly id: string; readonly name: string };
    };
    let source = stream.source();
    assert!(source.contains("readonly"), "Expected 'readonly'. Got:\n{}", source);
}

#[test]
fn test_object_type_index_signature() {
    let stream = ts_template! {
        type Dict = { [key: string]: unknown };
    };
    let source = stream.source();
    assert!(source.contains("[key") || source.contains("[ key"), "Expected index signature. Got:\n{}", source);
}

#[test]
fn test_object_type_method() {
    let stream = ts_template! {
        type Service = {
            init(): void;
            process(data: string): boolean;
        };
    };
    let source = stream.source();
    assert!(source.contains("init") && source.contains("process"), "Expected methods. Got:\n{}", source);
}

// =============================================================================
// Parenthesized Types
// =============================================================================

#[test]
fn test_paren_type_union() {
    let stream = ts_template! {
        type Nullable = (string | number) | null;
    };
    let source = stream.source();
    assert!(source.contains("(") && source.contains(")"), "Expected parentheses. Got:\n{}", source);
}

#[test]
fn test_paren_type_intersection() {
    let stream = ts_template! {
        type Complex = (A | B) & (C | D);
    };
    let source = stream.source();
    assert!(source.contains("A") && source.contains("B") && source.contains("C") && source.contains("D"), "Expected A, B, C, D. Got:\n{}", source);
}

// =============================================================================
// Typeof Types
// =============================================================================

#[test]
fn test_typeof_type_variable() {
    let stream = ts_template! {
        const config = { name: "app" };
        type ConfigType = typeof config;
    };
    let source = stream.source();
    assert!(source.contains("typeof config") || source.contains("typeof  config"), "Expected 'typeof config'. Got:\n{}", source);
}

#[test]
fn test_typeof_type_import() {
    let stream = ts_template! {
        type ModuleType = typeof import("./module");
    };
    let source = stream.source();
    assert!(source.contains("typeof") && source.contains("import"), "Expected 'typeof import'. Got:\n{}", source);
}

// =============================================================================
// Keyof Types
// =============================================================================

#[test]
fn test_keyof_type_simple() {
    let stream = ts_template! {
        type Keys = keyof User;
    };
    let source = stream.source();
    assert!(source.contains("keyof User") || source.contains("keyof  User"), "Expected 'keyof User'. Got:\n{}", source);
}

#[test]
fn test_keyof_type_in_generic() {
    let stream = ts_template! {
        function getProperty<T, K extends keyof T>(obj: T, key: K): T[K] {
            return obj[key];
        }
    };
    let source = stream.source();
    assert!(source.contains("keyof"), "Expected 'keyof'. Got:\n{}", source);
}

// =============================================================================
// Indexed Access Types
// =============================================================================

#[test]
fn test_indexed_access_type_string() {
    let stream = ts_template! {
        type Name = User["name"];
    };
    let source = stream.source();
    // Check for indexed access - quotes might vary
    assert!(source.contains("User[") && source.contains("name"), "Expected indexed access. Got:\n{}", source);
}

#[test]
fn test_indexed_access_type_generic() {
    let stream = ts_template! {
        type PropType<T, K extends keyof T> = T[K];
    };
    let source = stream.source();
    assert!(source.contains("T[K]") || source.contains("T[ K ]") || source.contains("T [K]"), "Expected 'T[K]'. Got:\n{}", source);
}

#[test]
fn test_indexed_access_type_number() {
    let stream = ts_template! {
        type First = Tuple[0];
    };
    let source = stream.source();
    assert!(source.contains("Tuple[0]") || source.contains("Tuple[ 0 ]"), "Expected 'Tuple[0]'. Got:\n{}", source);
}

// =============================================================================
// Conditional Types
// =============================================================================

#[test]
fn test_conditional_type_simple() {
    let stream = ts_template! {
        type IsString<T> = T extends string ? true : false;
    };
    let source = stream.source();
    assert!(source.contains("extends"), "Expected 'extends'. Got:\n{}", source);
    assert!(source.contains("?") && source.contains(":"), "Expected '?' and ':'. Got:\n{}", source);
}

#[test]
fn test_conditional_type_nested() {
    let stream = ts_template! {
        type TypeName<T> =
            T extends string ? "string" :
            T extends number ? "number" :
            T extends boolean ? "boolean" :
            "object";
    };
    let source = stream.source();
    let extends_count = source.matches("extends").count();
    assert!(extends_count >= 3, "Expected at least 3 'extends'. Got {} in:\n{}", extends_count, source);
}

#[test]
fn test_conditional_type_distributive() {
    let stream = ts_template! {
        type NonNullable<T> = T extends null | undefined ? never : T;
    };
    let source = stream.source();
    assert!(source.contains("extends") && source.contains("never"), "Expected conditional with never. Got:\n{}", source);
}

// =============================================================================
// Infer Types
// =============================================================================

#[test]
fn test_infer_type_return() {
    let stream = ts_template! {
        type ReturnType<T> = T extends (...args: any[]) => infer R ? R : never;
    };
    let source = stream.source();
    assert!(source.contains("infer R") || source.contains("infer  R"), "Expected 'infer R'. Got:\n{}", source);
}

#[test]
fn test_infer_type_array() {
    let stream = ts_template! {
        type Flatten<T> = T extends (infer U)[] ? U : T;
    };
    let source = stream.source();
    assert!(source.contains("infer U") || source.contains("infer  U"), "Expected 'infer U'. Got:\n{}", source);
}

#[test]
fn test_infer_type_promise() {
    let stream = ts_template! {
        type Awaited<T> = T extends Promise<infer U> ? U : T;
    };
    let source = stream.source();
    assert!(source.contains("infer"), "Expected 'infer'. Got:\n{}", source);
}

// =============================================================================
// Mapped Types
// =============================================================================

#[test]
fn test_mapped_type_simple() {
    let stream = ts_template! {
        type ReadonlyType<T> = { readonly [K in keyof T]: T[K] };
    };
    let source = stream.source();
    assert!(source.contains("[K in keyof T]") || source.contains("[ K in keyof T ]") || (source.contains("in") && source.contains("keyof")), "Expected mapped type. Got:\n{}", source);
}

#[test]
fn test_mapped_type_optional() {
    let stream = ts_template! {
        type Partial<T> = { [K in keyof T]?: T[K] };
    };
    let source = stream.source();
    assert!(source.contains("?") || source.contains("?:"), "Expected optional modifier. Got:\n{}", source);
}

#[test]
fn test_mapped_type_remove_readonly() {
    let stream = ts_template! {
        type Mutable<T> = { -readonly [K in keyof T]: T[K] };
    };
    let source = stream.source();
    assert!(source.contains("-readonly") || source.contains("- readonly"), "Expected '-readonly'. Got:\n{}", source);
}

#[test]
fn test_mapped_type_remove_optional() {
    let stream = ts_template! {
        type Required<T> = { [K in keyof T]-?: T[K] };
    };
    let source = stream.source();
    assert!(source.contains("-?") || source.contains("- ?"), "Expected '-?'. Got:\n{}", source);
}

#[test]
fn test_mapped_type_as_clause() {
    // Note: Template literal keys use backticks which are not valid Rust tokens.
    // Using a simpler 'as' clause example instead.
    let stream = ts_template! {
        type Filtered<T> = {
            [K in keyof T as K extends "id" ? never : K]: T[K]
        };
    };
    let source = stream.source();
    assert!(source.contains("as"), "Expected 'as' clause. Got:\n{}", source);
}

// =============================================================================
// Import Types
// =============================================================================

#[test]
fn test_import_type_simple() {
    let stream = ts_template! {
        type Config = import("./config").Config;
    };
    let source = stream.source();
    assert!(source.contains("import(") || source.contains("import ("), "Expected 'import('. Got:\n{}", source);
}

#[test]
fn test_import_type_with_typeof() {
    let stream = ts_template! {
        type ModuleType = typeof import("./module");
    };
    let source = stream.source();
    assert!(source.contains("typeof") && source.contains("import"), "Expected 'typeof import'. Got:\n{}", source);
}

#[test]
fn test_import_type_in_type_alias() {
    let stream = ts_template! {
        type Db = import("./database").Database;
    };
    let source = stream.source();
    assert!(source.contains("import") && source.contains("Database"), "Expected import type. Got:\n{}", source);
}

// =============================================================================
// This Types
// =============================================================================

#[test]
fn test_this_type_return() {
    let stream = ts_template! {
        interface Builder {
            setName(name: string): this;
            setValue(value: number): this;
            build(): Result;
        }
    };
    let source = stream.source();
    assert!(source.contains(": this") || source.contains(":this"), "Expected ': this'. Got:\n{}", source);
}

#[test]
fn test_this_type_in_class() {
    let stream = ts_template! {
        class Fluent {
            chain(): this {
                return this;
            }
        }
    };
    let source = stream.source();
    assert!(source.contains(": this") || source.contains(":this"), "Expected ': this' return type. Got:\n{}", source);
}

#[test]
fn test_this_type_constraint() {
    let stream = ts_template! {
        interface Clonable {
            clone(): this;
        }
    };
    let source = stream.source();
    assert!(source.contains("clone") && source.contains("this"), "Expected clone returning this. Got:\n{}", source);
}
