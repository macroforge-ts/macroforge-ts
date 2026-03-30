use super::*;

#[test]
fn test_derive_debug_runtime_output() {
    // Note: JSON macro is in playground-macros, not macroforge
    // Testing Debug macro which generates toString() implementation
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Debug) */
class Data {
    val: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        // Debug macro adds static toString method and standalone function
        assert!(result.code.contains("static toString(value: Data)"));
        assert!(result.code.contains("dataToString"));
    });
}

#[test]
fn test_derive_debug_dts_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Debug) */
class User {
    name: string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        let type_output = result.type_output.expect("should have type output");
        // New format: static method + standalone function
        assert!(
            type_output.contains("static toString(value: User): string"),
            "should have static toString method"
        );
        assert!(
            type_output.contains("export function userToString"),
            "should have standalone function"
        );
    });
}

#[test]
fn test_derive_clone_dts_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Clone) */
class User {
    name: string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        let type_output = result.type_output.expect("should have type output");

        // New format: static method + standalone function
        assert!(
            type_output.contains("static clone(value: User): User"),
            "should have static clone method"
        );
        assert!(
            type_output.contains("export function userClone"),
            "should have standalone function"
        );
    });
}

#[test]
fn test_derive_partial_eq_hash_dts_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(PartialEq, Hash) */
class User {
    name: string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        let type_output = result.type_output.expect("should have type output");

        // New format: static methods + standalone functions
        assert!(
            type_output.contains("static equals(a: User, b: User): boolean"),
            "should have static equals method"
        );
        assert!(
            type_output.contains("static hashCode(value: User): number"),
            "should have static hashCode method"
        );
        assert!(
            type_output.contains("export function userEquals"),
            "should have standalone equals function"
        );
        assert!(
            type_output.contains("export function userHashCode"),
            "should have standalone hashCode function"
        );
    });
}

#[test]
fn test_derive_debug_complex_dts_output() {
    let source = r#"

/** @derive(Debug) */
class MacroUser {
  /** @debug({ rename: "userId" }) */
  id: string;

  name: string;
  role: string;
  favoriteMacro: "Derive" | "JsonNative";
  since: string;

  /** @debug({ skip: true }) */
  apiToken: string;

  constructor(
    id: string,
    name: string,
    role: string,
    favoriteMacro: "Derive" | "JsonNative",
    since: string,
    apiToken: string,
  ) {
    this.id = id;
    this.name = name;
    this.role = role;
    this.favoriteMacro = favoriteMacro;
    this.since = since;
    this.apiToken = apiToken;
  }
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        let type_output = result.type_output.expect("should have type output");

        // New format: static method + standalone function
        assert!(
            type_output.contains("static toString(value: MacroUser): string"),
            "should have static toString method"
        );
        assert!(
            type_output.contains("export function macroUserToString"),
            "should have standalone function"
        );
    });
}

#[test]
fn test_complex_class_with_multiple_derives() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Debug, Clone, PartialEq, Hash) */
class Product {
    id: string;
    name: string;
    price: number;
    private secret: string;

    constructor(id: string, name: string, price: number, secret: string) {
        this.id = id;
        this.name = name;
        this.price = price;
        this.secret = secret;
    }

    getDisplayName(): string {
        return `${this.name} - $${this.price}`;
    }

    static fromJSON(json: any): Product {
        return new Product(json.id, json.name, json.price, json.secret);
    }
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        let type_output = result.type_output.expect("should have type output");

        // Check for static methods on class
        assert!(type_output.contains("static toString(value: Product): string"));
        assert!(type_output.contains("static clone(value: Product): Product"));
        assert!(type_output.contains("static equals(a: Product, b: Product): boolean"));
        assert!(type_output.contains("static hashCode(value: Product): number"));
        // Check for standalone functions
        assert!(type_output.contains("export function productToString"));
        assert!(type_output.contains("export function productClone"));
        assert!(type_output.contains("export function productEquals"));
        assert!(type_output.contains("export function productHashCode"));
    });
}
