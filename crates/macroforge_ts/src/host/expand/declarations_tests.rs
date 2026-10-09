use super::declaration_surface;

#[test]
fn bodies_and_initializers_are_left_out() {
    let typed_source = r#"export class Account {
    balance: number = 0;
    deposit(amount: number): void {
        this.balance += amount;
    }
    static defaultValue(): Account {
        return new Account();
    }
}
export function accountDefaultValue(): Account {
    return Account.defaultValue();
}
"#;
    let surface = declaration_surface(typed_source, "account.ts").expect("declares");

    assert!(surface.diagnostics.is_empty(), "{:?}", surface.diagnostics);
    assert!(!surface.code.contains("= 0"), "{}", surface.code);
    assert!(!surface.code.contains("this.balance"), "{}", surface.code);
    assert!(
        surface.code.contains("deposit(amount: number): void;"),
        "{}",
        surface.code
    );
    assert!(
        surface
            .code
            .contains("export declare function accountDefaultValue(): Account;"),
        "{}",
        surface.code
    );
}

#[test]
fn an_inferred_type_is_reported_by_name() {
    let surface = declaration_surface(
        "export function total(values: number[]) { return values.length; }\n",
        "total.ts",
    )
    .expect("declares");

    let [warning] = surface.diagnostics.as_slice() else {
        panic!("one warning, got {:?}", surface.diagnostics);
    };
    assert!(
        warning
            .message
            .starts_with("total.ts: declaration surface:"),
        "{}",
        warning.message
    );
}

#[test]
fn type_patches_that_do_not_parse_are_an_error_naming_the_line() {
    let error = declaration_surface("export const ok = 1;\nexport class {", "broken.ts")
        .err()
        .map(|error| error.to_string())
        .unwrap_or_default();
    assert!(error.contains("at line 2: `export class {`"), "{error}");
}

#[test]
fn a_declaration_file_keeps_its_declarations_and_loses_patched_bodies() {
    let typed_source = r#"export declare class Point {
    x: number;
    static clone(value: Point): Point {
        return value;
    }
}
"#;
    let surface = declaration_surface(typed_source, "point.d.ts").expect("declares");

    assert!(
        surface.code.contains("static clone(value: Point): Point;"),
        "{}",
        surface.code
    );
    assert!(!surface.code.contains("return value"), "{}", surface.code);
}

#[test]
fn a_member_after_another_members_signature_is_declared() {
    let typed_source = r#"export declare class Api {
    fetch(path: string): Promise<string>;
    static toString(value: Api): string {
        return "Api";
    }
}
"#;
    let surface = declaration_surface(typed_source, "api.d.ts").expect("declares");

    assert!(
        surface
            .code
            .contains("fetch(path: string): Promise<string>;"),
        "{}",
        surface.code
    );
    assert!(
        surface
            .code
            .contains("static toString(value: Api): string;"),
        "{}",
        surface.code
    );
}

#[test]
fn an_overload_implementation_is_still_left_out() {
    let typed_source = r#"export class Parser {
    parse(text: string): number;
    parse(text: string, radix: number): number;
    parse(text: string, radix?: number): number {
        return Number.parseInt(text, radix);
    }
}
"#;
    let surface = declaration_surface(typed_source, "parser.ts").expect("declares");

    assert_eq!(
        surface.code.matches("parse(").count(),
        2,
        "{}",
        surface.code
    );
}
