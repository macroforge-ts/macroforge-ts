//! IR Variant Coverage Tests - Declarations
//!
//! Tests for: EnumDecl, EnumMember, Decorator, ImportDecl, NamedImport,
//! DefaultImport, NamespaceImport, ExportDecl, NamedExport, ExportSpecifier,
//! ExportDefaultExpr, ExportAll

#[test]
fn test_enum_decl_simple() {
    let stream = macroforge_ts_quote::ts_template! {
        enum Status {
            Active,
            Inactive
        }
    };
    let source = stream.source();
    assert!(
        source.contains("enum Status"),
        "Expected 'enum Status'. Got:\n{}",
        source
    );
}

#[test]
fn test_enum_decl_with_numeric_values() {
    let stream = macroforge_ts_quote::ts_template! {
        enum Priority {
            Low = 0,
            Medium = 1,
            High = 2
        }
    };
    let source = stream.source();
    assert!(
        source.contains("enum Priority"),
        "Expected 'enum Priority'. Got:\n{}",
        source
    );
}

#[test]
fn test_import_named() {
    let stream = macroforge_ts_quote::ts_template! {
        import { Component, Injectable } from "@angular/core";
    };
    let source = stream.source();
    assert!(
        source.contains("import"),
        "Expected 'import'. Got:\n{}",
        source
    );
}

#[test]
fn test_import_default() {
    let stream = macroforge_ts_quote::ts_template! {
        import React from "react";
    };
    let source = stream.source();
    assert!(
        source.contains("React"),
        "Expected 'React'. Got:\n{}",
        source
    );
}

#[test]
fn test_import_namespace() {
    let stream = macroforge_ts_quote::ts_template! {
        import * as fs from "fs";
    };
    let source = stream.source();
    assert!(source.contains("fs"), "Expected 'fs'. Got:\n{}", source);
}

#[test]
fn test_export_named() {
    let stream = macroforge_ts_quote::ts_template! {
        const x = 1;
        const y = 2;
        export { x, y };
    };
    let source = stream.source();
    assert!(
        source.contains("export"),
        "Expected 'export'. Got:\n{}",
        source
    );
}

#[test]
fn test_export_all() {
    let stream = macroforge_ts_quote::ts_template! {
        export * from "./module";
    };
    let source = stream.source();
    assert!(
        source.contains("export"),
        "Expected 'export'. Got:\n{}",
        source
    );
}

#[test]
fn test_export_default_function() {
    let stream = macroforge_ts_quote::ts_template! {
        export default function handler() {
            return "handled";
        }
    };
    let source = stream.source();
    assert!(
        source.contains("export default"),
        "Expected 'export default'. Got:\n{}",
        source
    );
}

#[test]
fn test_decorator_simple() {
    let stream = macroforge_ts_quote::ts_template! {
        @observable
        class Store {
            value: number;
        }
    };
    let source = stream.source();
    assert!(
        source.contains("class Store"),
        "Expected 'class Store'. Got:\n{}",
        source
    );
}

#[test]
fn test_decorator_with_call() {
    let stream = macroforge_ts_quote::ts_template! {
        @Component({
            selector: "app-root",
            template: "<div>Hello</div>"
        })
        class AppComponent {}
    };
    let source = stream.source();
    assert!(
        source.contains("selector"),
        "Expected 'selector'. Got:\n{}",
        source
    );
}

// Previously caused hang - testing if fix works
#[test]
fn test_decorator_on_method() {
    let stream = macroforge_ts_quote::ts_template! {
        class Controller {
            @Get("/users")
            getUsers(): User[] {
                return [];
            }
        }
    };
    let source = stream.source();
    assert!(
        source.contains("getUsers"),
        "Expected 'getUsers'. Got:\n{}",
        source
    );
}
