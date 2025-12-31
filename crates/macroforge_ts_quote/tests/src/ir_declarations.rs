//! IR Variant Coverage Tests - Declarations
//!
//! Tests for: EnumDecl, EnumMember, Decorator, ImportDecl, NamedImport,
//! DefaultImport, NamespaceImport, ExportDecl, NamedExport, ExportSpecifier,
//! ExportDefaultExpr, ExportAll

use macroforge_ts_quote::ts_template;

// =============================================================================
// Enum Declarations
// =============================================================================

#[test]
fn test_enum_decl_simple() {
    let stream = ts_template! {
        enum Status {
            Active,
            Inactive
        }
    };
    let source = stream.source();
    assert!(source.contains("enum Status"), "Expected 'enum Status'. Got:\n{}", source);
    assert!(source.contains("Active"), "Expected 'Active'. Got:\n{}", source);
    assert!(source.contains("Inactive"), "Expected 'Inactive'. Got:\n{}", source);
}

#[test]
fn test_enum_decl_with_numeric_values() {
    let stream = ts_template! {
        enum Priority {
            Low = 0,
            Medium = 1,
            High = 2
        }
    };
    let source = stream.source();
    assert!(source.contains("enum Priority"), "Expected 'enum Priority'. Got:\n{}", source);
    assert!(source.contains("Low") && source.contains("0"), "Expected 'Low = 0'. Got:\n{}", source);
}

#[test]
fn test_enum_decl_with_string_values() {
    let stream = ts_template! {
        enum Direction {
            Up = "UP",
            Down = "DOWN",
            Left = "LEFT",
            Right = "RIGHT"
        }
    };
    let source = stream.source();
    assert!(source.contains("enum Direction"), "Expected 'enum Direction'. Got:\n{}", source);
    assert!(source.contains("UP") || source.contains("Up"), "Expected string values. Got:\n{}", source);
}

#[test]
fn test_enum_decl_const() {
    let stream = ts_template! {
        const enum Color {
            Red,
            Green,
            Blue
        }
    };
    let source = stream.source();
    assert!(source.contains("const enum") || source.contains("const  enum"), "Expected 'const enum'. Got:\n{}", source);
    assert!(source.contains("Color"), "Expected 'Color'. Got:\n{}", source);
}

#[test]
fn test_enum_decl_exported() {
    let stream = ts_template! {
        export enum Visibility {
            Public,
            Private,
            Protected
        }
    };
    let source = stream.source();
    assert!(source.contains("export") && source.contains("enum"), "Expected 'export enum'. Got:\n{}", source);
    assert!(source.contains("Visibility"), "Expected 'Visibility'. Got:\n{}", source);
}

// =============================================================================
// Decorators
// =============================================================================

#[test]
fn test_decorator_simple() {
    let stream = ts_template! {
        @observable
        class Store {
            value: number;
        }
    };
    let source = stream.source();
    assert!(source.contains("@observable") || source.contains("@ observable"), "Expected '@observable'. Got:\n{}", source);
    assert!(source.contains("class Store"), "Expected 'class Store'. Got:\n{}", source);
}

#[test]
fn test_decorator_with_call() {
    let stream = ts_template! {
        @Component({
            selector: "app-root",
            template: "<div>Hello</div>"
        })
        class AppComponent {}
    };
    let source = stream.source();
    assert!(source.contains("@Component") || source.contains("@ Component"), "Expected '@Component'. Got:\n{}", source);
    assert!(source.contains("selector"), "Expected 'selector'. Got:\n{}", source);
}

#[test]
fn test_decorator_on_method() {
    let stream = ts_template! {
        class Controller {
            @Get("/users")
            getUsers(): User[] {
                return [];
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("@Get") || source.contains("@ Get"), "Expected '@Get'. Got:\n{}", source);
    assert!(source.contains("getUsers"), "Expected 'getUsers'. Got:\n{}", source);
}

#[test]
fn test_decorator_on_property() {
    let stream = ts_template! {
        class User {
            @Column("varchar")
            name: string;

            @Column("int")
            age: number;
        }
    };
    let source = stream.source();
    assert!(source.contains("@Column") || source.contains("@ Column"), "Expected '@Column'. Got:\n{}", source);
}

#[test]
fn test_decorator_multiple() {
    let stream = ts_template! {
        @Injectable()
        @Singleton
        class Service {}
    };
    let source = stream.source();
    assert!(source.contains("@Injectable") || source.contains("@ Injectable"), "Expected '@Injectable'. Got:\n{}", source);
    assert!(source.contains("@Singleton") || source.contains("@ Singleton") || source.contains("Singleton"), "Expected '@Singleton'. Got:\n{}", source);
}

// =============================================================================
// Import Declarations
// =============================================================================

#[test]
fn test_import_named() {
    let stream = ts_template! {
        import { Component, Injectable } from "@angular/core";
    };
    let source = stream.source();
    assert!(source.contains("import"), "Expected 'import'. Got:\n{}", source);
    assert!(source.contains("Component"), "Expected 'Component'. Got:\n{}", source);
    assert!(source.contains("@angular/core") || source.contains("angular/core"), "Expected module path. Got:\n{}", source);
}

#[test]
fn test_import_default() {
    let stream = ts_template! {
        import React from "react";
    };
    let source = stream.source();
    assert!(source.contains("import") && source.contains("React"), "Expected 'import React'. Got:\n{}", source);
    assert!(source.contains("react"), "Expected 'react' module. Got:\n{}", source);
}

#[test]
fn test_import_namespace() {
    let stream = ts_template! {
        import * as fs from "fs";
    };
    let source = stream.source();
    assert!(source.contains("*") && source.contains("as") && source.contains("fs"), "Expected '* as fs'. Got:\n{}", source);
}

#[test]
fn test_import_type_only() {
    let stream = ts_template! {
        import type { User, Config } from "./types";
    };
    let source = stream.source();
    assert!(source.contains("import") && source.contains("type"), "Expected 'import type'. Got:\n{}", source);
    assert!(source.contains("User"), "Expected 'User'. Got:\n{}", source);
}

#[test]
fn test_import_with_alias() {
    let stream = ts_template! {
        import { foo as bar, baz as qux } from "module";
    };
    let source = stream.source();
    assert!(source.contains("as bar") || source.contains("as  bar"), "Expected 'as bar'. Got:\n{}", source);
}

#[test]
fn test_import_default_and_named() {
    let stream = ts_template! {
        import React, { useState, useEffect } from "react";
    };
    let source = stream.source();
    assert!(source.contains("React"), "Expected 'React'. Got:\n{}", source);
    assert!(source.contains("useState"), "Expected 'useState'. Got:\n{}", source);
}

#[test]
fn test_import_side_effect() {
    let stream = ts_template! {
        import "./polyfills";
    };
    let source = stream.source();
    assert!(source.contains("import") && source.contains("polyfills"), "Expected side-effect import. Got:\n{}", source);
}

// =============================================================================
// Export Declarations
// =============================================================================

#[test]
fn test_export_named() {
    let stream = ts_template! {
        const x = 1;
        const y = 2;
        export { x, y };
    };
    let source = stream.source();
    assert!(source.contains("export"), "Expected 'export'. Got:\n{}", source);
    assert!(source.contains("x") && source.contains("y"), "Expected 'x' and 'y'. Got:\n{}", source);
}

#[test]
fn test_export_with_alias() {
    let stream = ts_template! {
        const internalName = "value";
        export { internalName as publicName };
    };
    let source = stream.source();
    assert!(source.contains("as publicName") || source.contains("as  publicName"), "Expected 'as publicName'. Got:\n{}", source);
}

#[test]
fn test_export_default_object() {
    let stream = ts_template! {
        export default {
            name: "default",
            value: 42
        };
    };
    let source = stream.source();
    assert!(source.contains("export default") || source.contains("export  default"), "Expected 'export default'. Got:\n{}", source);
}

#[test]
fn test_export_default_function() {
    let stream = ts_template! {
        export default function handler() {
            return "handled";
        }
    };
    let source = stream.source();
    assert!(source.contains("export default") || source.contains("export  default"), "Expected 'export default'. Got:\n{}", source);
    assert!(source.contains("function") && source.contains("handler"), "Expected function. Got:\n{}", source);
}

#[test]
fn test_export_default_class() {
    let stream = ts_template! {
        export default class Component {
            render() {}
        }
    };
    let source = stream.source();
    assert!(source.contains("export default") || source.contains("export  default"), "Expected 'export default'. Got:\n{}", source);
    assert!(source.contains("class Component") || source.contains("class  Component"), "Expected class. Got:\n{}", source);
}

#[test]
fn test_export_all() {
    let stream = ts_template! {
        export * from "./module";
    };
    let source = stream.source();
    assert!(source.contains("export") && source.contains("*"), "Expected 'export *'. Got:\n{}", source);
    assert!(source.contains("from"), "Expected 'from'. Got:\n{}", source);
}

#[test]
fn test_export_all_as_namespace() {
    let stream = ts_template! {
        export * as utils from "./utils";
    };
    let source = stream.source();
    assert!(source.contains("*") && source.contains("as") && source.contains("utils"), "Expected '* as utils'. Got:\n{}", source);
}

#[test]
fn test_export_type() {
    let stream = ts_template! {
        export type { User, Config };
    };
    let source = stream.source();
    assert!(source.contains("export") && source.contains("type"), "Expected 'export type'. Got:\n{}", source);
}

#[test]
fn test_export_from() {
    let stream = ts_template! {
        export { foo, bar } from "./other";
    };
    let source = stream.source();
    assert!(source.contains("export") && source.contains("from"), "Expected re-export. Got:\n{}", source);
    assert!(source.contains("foo") && source.contains("bar"), "Expected 'foo' and 'bar'. Got:\n{}", source);
}
