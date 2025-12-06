use crate::{
    host::MacroHostIntegration, parse_import_sources, GeneratedRegionResult,
    MappingSegmentResult, NativePositionMapper, SourceMappingResult,
};
use swc_core::ecma::ast::{ClassMember, Program};
use swc_core::{
    common::{sync::Lrc, FileName, SourceMap, GLOBALS},
    ecma::parser::{Lexer, Parser, StringInput, Syntax, TsSyntax},
};
use ts_syn::abi::{
    ClassIR, DiagnosticLevel, MacroContextIR, MacroResult, Patch, PatchCode, SpanIR,
};
use crate::host::PatchCollector;

const DERIVE_MODULE_PATH: &str = "@macro/derive";

fn parse_module(source: &str) -> Program {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        FileName::Custom("test.ts".into()).into(),
        source.to_string(),
    );

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            decorators: true,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().expect("should parse");
    Program::Module(module)
}

trait StringExt {
    fn replace_whitespace(&self) -> String;
}

impl StringExt for str {
    fn replace_whitespace(&self) -> String {
        self.chars().filter(|c| !c.is_whitespace()).collect()
    }
}

fn base_class(name: &str) -> ClassIR {
    ClassIR {
        name: name.into(),
        span: SpanIR::new(0, 200),
        body_span: SpanIR::new(10, 190),
        is_abstract: false,
        type_params: vec![],
        heritage: vec![],
        decorators: vec![],
        decorators_ast: vec![],
        fields: vec![],
        methods: vec![],
        members: vec![],
    }
}

#[test]
fn test_derive_debug_runtime_output() {
    // Note: JSON macro is in playground-macros, not swc-napi-macros
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
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        // Debug macro adds toString() method
        assert!(result.code.contains("toString()"));
        assert!(result.code.contains("Data"));
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

    let expected_dts = r#"
import { Derive } from "@macro/derive";


class User {
    name: string;
    toString(): string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        let type_output = result.type_output.expect("should have type output");

        assert_eq!(
            type_output.replace_whitespace(),
            expected_dts.replace_whitespace()
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

    let expected_dts = r#"
import { Derive } from "@macro/derive";


class User {
    name: string;
    clone(): User;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        let type_output = result.type_output.expect("should have type output");

        assert_eq!(
            type_output.replace_whitespace(),
            expected_dts.replace_whitespace()
        );
    });
}

#[test]
fn test_derive_eq_dts_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Eq) */
class User {
    name: string;
}
"#;

    let expected_dts = r#"
import { Derive } from "@macro/derive";


class User {
    name: string;
    equals(other: unknown): boolean;
    hashCode(): number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        let type_output = result.type_output.expect("should have type output");

        assert_eq!(
            type_output.replace_whitespace(),
            expected_dts.replace_whitespace()
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

    let expected_dts = r#"
class MacroUser {
  id: string;
  name: string;
  role: string;
  favoriteMacro: "Derive" | "JsonNative";
  since: string;
  apiToken: string;

  constructor(
    id: string,
    name: string,
    role: string,
    favoriteMacro: "Derive" | "JsonNative",
    since: string,
    apiToken: string,
  );
  toString(): string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        let type_output = result.type_output.expect("should have type output");

        // Use whitespace-normalized comparison like other tests
        assert_eq!(
            type_output.replace_whitespace(),
            expected_dts.replace_whitespace()
        );
    });
}

#[test]
fn test_prepare_no_derive() {
    let source = "class User { name: string; }";
    let program = parse_module(source);
    let host = MacroHostIntegration::new().unwrap();
    let result = host.prepare_expansion_context(&program, source).unwrap();
    // Even without decorators, we return Some because we still need to
    // generate method signatures for type output
    assert!(result.is_some());
}

#[test]
fn test_prepare_no_classes() {
    let source = "const x = 1;";
    let program = parse_module(source);
    let host = MacroHostIntegration::new().unwrap();
    let result = host.prepare_expansion_context(&program, source).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_prepare_with_classes() {
    let source = "/** @derive(Debug) */ class User {}";
    let program = parse_module(source);
    let host = MacroHostIntegration::new().unwrap();
    let result = host.prepare_expansion_context(&program, source).unwrap();
    assert!(result.is_some());
    let (_module, classes, _interfaces) = result.unwrap();
    assert_eq!(classes.len(), 1);
    assert_eq!(classes[0].name, "User");
}

#[test]
fn test_process_macro_output_converts_tokens_into_patches() {
    GLOBALS.set(&Default::default(), || {
        let host = MacroHostIntegration::new().unwrap();
        let class_ir = base_class("TokenDriven");
        let ctx = MacroContextIR::new_derive_class(
            "Debug".into(),
            DERIVE_MODULE_PATH.into(),
            SpanIR::new(0, 5),
            class_ir.span,
            "token.ts".into(),
            class_ir.clone(),
            "class TokenDriven {}".into(),
        );

        // Use body marker since default is now "below"
        let mut result = MacroResult {
            tokens: Some(
                r#"/* @macroforge:body */
                        toString() { return `${this.value}`; }
                        constructor(value: string) { this.value = value; }
                    "#
                .into(),
            ),
            ..Default::default()
        };

        let (runtime, type_patches) = host
            .process_macro_output(&mut result, &ctx, &ctx.target_source)
            .expect("tokens should parse");

        assert_eq!(
            runtime.len(),
            2,
            "expected one runtime patch per generated member"
        );
        assert_eq!(
            type_patches.len(),
            2,
            "expected one type patch per generated member"
        );

        for patch in runtime {
            match patch {
                Patch::Insert {
                    code: PatchCode::ClassMember(_),
                    ..
                } => {}
                other => panic!("expected class member insert, got {:?}", other),
            }
        }

        for patch in type_patches {
            if let Patch::Insert {
                code: PatchCode::ClassMember(member),
                ..
            } = patch
            {
                match member {
                    ClassMember::Method(method) => assert!(
                        method.function.body.is_none(),
                        "type patch should strip method body"
                    ),
                    ClassMember::Constructor(cons) => assert!(
                        cons.body.is_none(),
                        "type patch should drop constructor body"
                    ),
                    _ => {}
                }
            } else {
                panic!("expected type patch insert");
            }
        }
    });
}

#[test]
fn test_process_macro_output_reports_parse_errors() {
    GLOBALS.set(&Default::default(), || {
        let host = MacroHostIntegration::new().unwrap();
        let class_ir = base_class("Broken");
        let ctx = MacroContextIR::new_derive_class(
            "Debug".into(),
            DERIVE_MODULE_PATH.into(),
            SpanIR::new(0, 5),
            class_ir.span,
            "broken.ts".into(),
            class_ir.clone(),
            "class Broken {}".into(),
        );

        // Use body marker to trigger class member parsing, which will fail
        let mut result = MacroResult {
            tokens: Some("/* @macroforge:body */this is not valid class member syntax".into()),
            ..Default::default()
        };

        let (_runtime, _types) = host
            .process_macro_output(&mut result, &ctx, &ctx.target_source)
            .expect("process_macro_output should succeed with raw insertion fallback");

        let diag = result
            .diagnostics
            .iter()
            .find(|d| d.message.contains("Failed to parse macro output"))
            .expect("diagnostic should mention parsing failure");
        assert_eq!(diag.level, DiagnosticLevel::Warning);
    });
}

#[test]
fn test_collect_constructor_patch() {
    let source = "class User { constructor(id: string) { this.id = id; } }";
    let program = parse_module(source);
    let host = MacroHostIntegration::new().unwrap();
    let (module, classes, interfaces) = host
        .prepare_expansion_context(&program, source)
        .unwrap()
        .unwrap();

    let (collector, _) =
        host.collect_macro_patches(&module, classes, interfaces, "test.ts", source);

    let type_patches = collector.get_type_patches();
    assert_eq!(type_patches.len(), 1);
    let patch = &type_patches[0];

    if let Patch::Replace { code, .. } = patch {
        match code {
            PatchCode::Text(text) => assert_eq!(text, "constructor(id: string);"),
            _ => panic!("Expected textual patch for constructor signature"),
        }
    } else {
        panic!("Expected a replace patch for constructor");
    }
}

#[test]
fn test_collect_derive_debug_patch() {
    let source = "/** @derive(Debug) */ class User { name: string; }";
    let program = parse_module(source);
    let host = MacroHostIntegration::new().unwrap();
    let (module, classes, interfaces) = host
        .prepare_expansion_context(&program, source)
        .unwrap()
        .unwrap();
    let (collector, _) =
        host.collect_macro_patches(&module, classes, interfaces, "test.ts", source);

    let type_patches = collector.get_type_patches();

    // Expecting 2 patches:
    // 1. Decorator removal for /** @derive(Debug) */
    // 2. toString signature insertion (from Derive(Debug) macro)
    // Note: find_macro_comment_span no longer incorrectly finds class-level decorators
    // for fields, so we don't get a spurious third patch
    assert_eq!(
        type_patches.len(),
        2,
        "Expected 2 patches, got {}",
        type_patches.len()
    );

    // check for decorator deletion
    assert!(type_patches
        .iter()
        .any(|p| matches!(p, Patch::Delete { .. })));
    // check for method signature insertion
    assert!(type_patches
        .iter()
        .any(|p| matches!(p, Patch::Insert { .. })));
}

#[test]
fn test_apply_and_finalize_expansion_no_type_patches() {
    let source = "class User {}";
    let mut collector = PatchCollector::new();
    let mut diagnostics = Vec::new();
    let host = MacroHostIntegration::new().unwrap();
    let result = host
        .apply_and_finalize_expansion(
            source,
            &mut collector,
            &mut diagnostics,
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
    assert!(result.type_output.is_none());
}

#[test]
fn test_complex_class_with_multiple_derives() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Debug, Clone, Eq) */
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

    let expected_dts = r#"
import { Derive } from "@macro/derive";

class Product {
    id: string;
    name: string;
    price: number;
    private secret: string;

    constructor(id: string, name: string, price: number, secret: string);

    getDisplayName(): string;

    static fromJSON(json: any): Product;

    toString(): string;
    clone(): Product;
    equals(other: unknown): boolean;
    hashCode(): number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        let type_output = result.type_output.expect("should have type output");

        assert_eq!(
            type_output.replace_whitespace(),
            expected_dts.replace_whitespace()
        );
    });
}

#[test]
fn test_complex_method_signatures() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Debug) */
class API {
    endpoint: string;

    constructor(endpoint: string) {
        this.endpoint = endpoint;
    }

    async fetch<T>(
        path: string,
        options?: { method?: string; body?: any }
    ): Promise<T> {
        return {} as T;
    }

    subscribe(
        event: "data" | "error",
        callback: (data: any) => void,
        thisArg?: any
    ): () => void {
        return () => {};
    }
}
"#;

    let expected_dts = r#"
import { Derive } from "@macro/derive";

class API {
    endpoint: string;

    constructor(endpoint: string);

    async fetch<T>(
        path: string,
        options?: { method?: string; body?: any }
    ): Promise<T>;

    subscribe(
        event: "data" | "error",
        callback: (data: any) => void,
        thisArg?: any
    ): () => void;

    toString(): string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        assert_eq!(
            type_output.replace_whitespace(),
            expected_dts.replace_whitespace()
        );
    });
}

#[test]
fn test_class_with_visibility_modifiers() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Clone) */
class Account {
    public username: string;
    protected password: string;
    private apiKey: string;

    constructor(username: string, password: string, apiKey: string) {
        this.username = username;
        this.password = password;
        this.apiKey = apiKey;
    }

    public login(): boolean {
        return true;
    }

    protected validatePassword(input: string): boolean {
        return this.password === input;
    }

    private getApiKey(): string {
        return this.apiKey;
    }
}
"#;

    let expected_dts = r#"
import { Derive } from "@macro/derive";

class Account {
    public username: string;
    protected password: string;
    private apiKey: string;

    constructor(username: string, password: string, apiKey: string);

    login(): boolean;

    protected validatePassword(input: string): boolean;

    private getApiKey(): string;

    clone(): Account;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        assert_eq!(
            type_output.replace_whitespace(),
            expected_dts.replace_whitespace()
        );
    });
}

#[test]
fn test_class_with_optional_and_readonly_fields() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Debug, Eq) */
class Config {
    readonly id: string;
    name: string;
    description?: string;
    readonly createdAt: Date;
    updatedAt?: Date;

    constructor(id: string, name: string, createdAt: Date) {
        this.id = id;
        this.name = name;
        this.createdAt = createdAt;
    }

    update(name: string, description?: string): void {
        this.name = name;
        this.description = description;
        this.updatedAt = new Date();
    }
}
"#;

    let expected_dts = r#"
import { Derive } from "@macro/derive";

class Config {
    readonly id: string;
    name: string;
    description?: string;
    readonly createdAt: Date;
    updatedAt?: Date;

    constructor(id: string, name: string, createdAt: Date);

    update(name: string, description?: string): void;

    toString(): string;
    equals(other: unknown): boolean;
    hashCode(): number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        assert_eq!(
            type_output.replace_whitespace(),
            expected_dts.replace_whitespace()
        );
    });
}

#[test]
fn test_empty_constructor_and_no_params_methods() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Debug) */
class Singleton {
    private static instance: Singleton;

    private constructor() {
        // Private constructor
    }

    static getInstance(): Singleton {
        if (!Singleton.instance) {
            Singleton.instance = new Singleton();
        }
        return Singleton.instance;
    }

    reset(): void {
        // Reset logic
    }
}
"#;

    let expected_dts = r#"
import { Derive } from "@macro/derive";

class Singleton {
    private static instance: Singleton;

    private constructor();

    static getInstance(): Singleton;

    reset(): void;

    toString(): string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        assert_eq!(
            type_output.replace_whitespace(),
            expected_dts.replace_whitespace()
        );
    });
}

#[test]
fn test_class_with_field_decorators_and_derive() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Debug) */
class ValidationExample {
    /** @debug({ rename: "userId" }) */
    id: string;

    name: string;

    /** @debug({ skip: true }) */
    internalFlag: boolean;

    constructor(id: string, name: string, internalFlag: boolean) {
        this.id = id;
        this.name = name;
        this.internalFlag = internalFlag;
    }
}
"#;

    let expected_dts = r#"
import { Derive } from "@macro/derive";

class ValidationExample {
    id: string;

    name: string;

    internalFlag: boolean;

    constructor(id: string, name: string, internalFlag: boolean);

    toString(): string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        assert_eq!(
            type_output.replace_whitespace(),
            expected_dts.replace_whitespace()
        );
    });
}

#[test]
fn test_generated_methods_on_separate_lines() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Debug, Clone) */
class User {
    id: number;
    name: string;

    constructor(id: number, name: string) {
        this.id = id;
        this.name = name;
    }
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        // Verify methods are on separate lines, not jammed together
        let lines: Vec<&str> = type_output.lines().collect();

        // Find the toString line
        let tostring_line = lines
            .iter()
            .position(|l| l.contains("toString()"))
            .expect("should have toString");
        // Find the clone line
        let clone_line = lines
            .iter()
            .position(|l| l.contains("clone()"))
            .expect("should have clone");

        // They should be on different lines
        assert_ne!(
            tostring_line, clone_line,
            "toString and clone should be on different lines"
        );

        // Verify no line contains multiple method signatures
        for line in &lines {
            let method_count = line.matches("(): ").count();
            assert!(
                method_count <= 1,
                "Line should not contain multiple methods: {}",
                line
            );
        }
    });
}

#[test]
fn test_proper_indentation_in_generated_code() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Debug) */
class User {
  id: number;
  name: string;

  constructor(id: number, name: string) {
    this.id = id;
    this.name = name;
  }
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        // Find the toString line
        let tostring_line = type_output
            .lines()
            .find(|l| l.contains("toString()"))
            .expect("should have toString method");

        // Verify it has proper indentation (2 spaces to match the class body)
        assert!(
            tostring_line.starts_with("  toString()")
                || tostring_line.trim().starts_with("toString()"),
            "toString should have proper indentation, got: '{}'",
            tostring_line
        );
    });
}

#[test]
fn test_default_parameter_values() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Debug) */
class ServerConfig {
    host: string;
    port: number;

    constructor(
        host: string = "localhost",
        port: number = 8080,
        secure: boolean = false
    ) {
        this.host = host;
        this.port = port;
    }

    connect(
        timeout: number = 5000,
        retries: number = 3,
        onError?: (err: Error) => void
    ): Promise<void> {
        return Promise.resolve();
    }

    static create(
        config: Partial<ServerConfig> = {},
        defaults: { host?: string; port?: number } = { host: "0.0.0.0", port: 3000 }
    ): ServerConfig {
        return new ServerConfig();
    }
}
"#;

    let expected_dts = r#"
import { Derive } from "@macro/derive";

class ServerConfig {
    host: string;
    port: number;

    constructor(
        host: string = "localhost",
        port: number = 8080,
        secure: boolean = false
    );

    connect(
        timeout: number = 5000,
        retries: number = 3,
        onError?: (err: Error) => void
    ): Promise<void>;

    static create(
        config: Partial<ServerConfig> = {},
        defaults: { host?: string; port?: number } = { host: "0.0.0.0", port: 3000 }
    ): ServerConfig;

    toString(): string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        assert_eq!(
            type_output.replace_whitespace(),
            expected_dts.replace_whitespace()
        );
    });
}

#[test]
fn test_rest_parameters_and_destructuring() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Clone) */
class EventEmitter {
    listeners: Map<string, Function[]>;

    constructor() {
        this.listeners = new Map();
    }

    on(event: string, ...callbacks: Array<(...args: any[]) => void>): void {
        const existing = this.listeners.get(event) || [];
        this.listeners.set(event, [...existing, ...callbacks]);
    }

    emit(event: string, ...args: any[]): void {
        const callbacks = this.listeners.get(event) || [];
        callbacks.forEach(cb => cb(...args));
    }
}
"#;

    let expected_dts = r#"
import { Derive } from "@macro/derive";

class EventEmitter {
    listeners: Map<string, Function[]>;

    constructor();

    on(event: string, ...callbacks: Array<(...args: any[]) => void>): void;

    emit(event: string, ...args: any[]): void;

    clone(): EventEmitter;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        assert_eq!(
            type_output.replace_whitespace(),
            expected_dts.replace_whitespace()
        );
    });
}

#[test]
fn test_source_mapping_produced() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Debug) */
class User {
    name: string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "Expansion should report changes");

        // Source mapping should be produced
        let mapping = result
            .source_mapping
            .expect("Source mapping should be produced");

        // Should have segments for unchanged regions
        assert!(!mapping.segments.is_empty(), "Should have mapping segments");

        // Should have a generated region for the toString implementation
        assert!(
            !mapping.generated_regions.is_empty(),
            "Should have generated regions"
        );
    });
}

#[test]
fn parse_import_sources_handles_aliases_and_defaults() {
    let code = r#"
import { Derive, Debug as Dbg } from "@macro/derive";
import DefaultMacro from "@macro/default";
import * as Everything from "@macro/all";
"#;

    let imports = parse_import_sources(code.to_string(), "test.ts".to_string())
        .expect("should parse imports");

    let map: std::collections::HashMap<_, _> = imports
        .into_iter()
        .map(|entry| (entry.local, entry.module))
        .collect();

    assert_eq!(map.get("Derive").map(String::as_str), Some("@macro/derive"));
    assert_eq!(map.get("Dbg").map(String::as_str), Some("@macro/derive"));
    assert_eq!(
        map.get("DefaultMacro").map(String::as_str),
        Some("@macro/default")
    );
    assert_eq!(
        map.get("Everything").map(String::as_str),
        Some("@macro/all")
    );
}

#[test]
fn native_position_mapper_matches_js_logic() {
    let mapping = SourceMappingResult {
        segments: vec![
            MappingSegmentResult {
                original_start: 0,
                original_end: 10,
                expanded_start: 0,
                expanded_end: 10,
            },
            MappingSegmentResult {
                original_start: 10,
                original_end: 20,
                expanded_start: 12,
                expanded_end: 22,
            },
        ],
        generated_regions: vec![GeneratedRegionResult {
            start: 10,
            end: 12,
            source_macro: "demo".into(),
        }],
    };

    let mapper = NativePositionMapper::new(mapping);

    assert_eq!(mapper.original_to_expanded(5), 5);
    assert_eq!(mapper.original_to_expanded(15), 17);

    assert_eq!(mapper.expanded_to_original(5), Some(5));
    assert_eq!(mapper.expanded_to_original(17), Some(15));
    assert_eq!(mapper.expanded_to_original(10), None);

    assert!(mapper.is_in_generated(10));
    assert_eq!(mapper.generated_by(11).as_deref(), Some("demo"));
    assert!(mapper.generated_by(25).is_none());

    let span = mapper.map_span_to_original(12, 2).expect("span should map");
    assert_eq!(span.start, 10);
    assert_eq!(span.length, 2);

    let expanded_span = mapper.map_span_to_expanded(8, 4);
    assert_eq!(expanded_span.start, 8);
    assert_eq!(expanded_span.length, 6);

    assert!(!mapper.is_empty());
}

#[test]
fn test_default_below_location_for_unmarked_tokens() {
    GLOBALS.set(&Default::default(), || {
        let host = MacroHostIntegration::new().unwrap();
        let class_ir = base_class("BelowTest");
        let ctx = MacroContextIR::new_derive_class(
            "Custom".into(),
            DERIVE_MODULE_PATH.into(),
            SpanIR::new(0, 5),
            class_ir.span,
            "below.ts".into(),
            class_ir.clone(),
            "class BelowTest {}".into(),
        );

        // Tokens without a marker should default to "below" location
        let mut result = MacroResult {
            tokens: Some("const helper = () => {};".into()),
            ..Default::default()
        };

        let (runtime, type_patches) = host
            .process_macro_output(&mut result, &ctx, &ctx.target_source)
            .expect("unmarked tokens should be inserted as text below class");

        // Should have 1 runtime and 1 type patch for "below" insertion
        assert_eq!(runtime.len(), 1, "should have 1 runtime patch for below");
        assert_eq!(type_patches.len(), 1, "should have 1 type patch for below");

        // Verify it's a text patch, not a class member patch
        for patch in runtime {
            match patch {
                Patch::Insert {
                    code: PatchCode::Text(_),
                    at,
                    ..
                } => {
                    // Below location should insert at class span end
                    assert_eq!(at.start, class_ir.span.end, "should insert at class end");
                }
                other => panic!("expected text insert for below, got {:?}", other),
            }
        }
    });
}

#[test]
fn test_explicit_body_marker_parses_as_class_members() {
    GLOBALS.set(&Default::default(), || {
        let host = MacroHostIntegration::new().unwrap();
        let class_ir = base_class("BodyTest");
        let ctx = MacroContextIR::new_derive_class(
            "Custom".into(),
            DERIVE_MODULE_PATH.into(),
            SpanIR::new(0, 5),
            class_ir.span,
            "body.ts".into(),
            class_ir.clone(),
            "class BodyTest {}".into(),
        );

        // Tokens WITH body marker should be parsed as class members
        let mut result = MacroResult {
            tokens: Some("/* @macroforge:body */getValue(): string { return \"test\"; }".into()),
            ..Default::default()
        };

        let (runtime, type_patches) = host
            .process_macro_output(&mut result, &ctx, &ctx.target_source)
            .expect("body-marked tokens should parse as class members");

        // Should have class member patches
        assert_eq!(runtime.len(), 1, "should have 1 runtime patch for body");
        assert_eq!(type_patches.len(), 1, "should have 1 type patch for body");

        for patch in runtime {
            match patch {
                Patch::Insert {
                    code: PatchCode::ClassMember(_),
                    ..
                } => {}
                other => panic!("expected class member insert for body, got {:?}", other),
            }
        }
    });
}

// ============================================================================
// Attribute Validation Error Tests
// ============================================================================

#[test]
fn test_derive_debug_on_interface_produces_error() {
    // Debug macro only works on classes, applying to interface should error
    let source = r#"
/** @derive(Debug) */
interface Status {
    active: boolean;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have diagnostics indicating the error
        assert!(
            !result.diagnostics.is_empty(),
            "Should produce diagnostics for invalid target"
        );

        // Find the error diagnostic
        let error_diag = result
            .diagnostics
            .iter()
            .find(|d| d.level == DiagnosticLevel::Error)
            .expect("Should have an error-level diagnostic");

        // Error message should mention that Debug can only be applied to classes
        assert!(
            error_diag.message.contains("class") || error_diag.message.contains("interface"),
            "Error should mention class or interface restriction, got: {}",
            error_diag.message
        );
    });
}

#[test]
fn test_derive_clone_on_interface_produces_error() {
    let source = r#"
/** @derive(Clone) */
interface UserData {
    name: string;
    age: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have diagnostics indicating the error
        assert!(
            !result.diagnostics.is_empty(),
            "Should produce diagnostics for invalid target"
        );

        // Find the error diagnostic
        let error_diag = result
            .diagnostics
            .iter()
            .find(|d| d.level == DiagnosticLevel::Error)
            .expect("Should have an error-level diagnostic");

        // Error message should mention interfaces
        assert!(
            error_diag.message.contains("interface") || error_diag.message.contains("Clone"),
            "Error should mention Clone or interfaces, got: {}",
            error_diag.message
        );
    });
}

#[test]
fn test_derive_eq_on_interface_produces_error() {
    let source = r#"
/** @derive(Eq) */
interface Point {
    x: number;
    y: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have diagnostics indicating the error
        assert!(
            !result.diagnostics.is_empty(),
            "Should produce diagnostics for invalid target"
        );

        // Find the error diagnostic
        let error_diag = result
            .diagnostics
            .iter()
            .find(|d| d.level == DiagnosticLevel::Error)
            .expect("Should have an error-level diagnostic");

        // Error message should mention interfaces
        assert!(
            error_diag.message.contains("interface") || error_diag.message.contains("Eq"),
            "Error should mention Eq or interfaces, got: {}",
            error_diag.message
        );
    });
}

#[test]
fn test_error_span_points_to_correct_attribute_for_interface() {
    // Test that error spans point to the decorator area, not the interface declaration
    let source = r#"
/** @derive(Debug) */
interface Status {
    active: boolean;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        let error_diag = result
            .diagnostics
            .iter()
            .find(|d| d.level == DiagnosticLevel::Error)
            .expect("Should have an error-level diagnostic");

        // The span should be present and point to a reasonable location
        let span = error_diag.span.as_ref().expect("Error should have a span");

        // The span should be within the source bounds
        assert!(
            span.start < source.len() as u32,
            "Span start should be within source"
        );
        assert!(
            span.end <= source.len() as u32,
            "Span end should be within source"
        );

        // The span should point to the decorator/comment area (before "interface")
        let interface_pos = source.find("interface").expect("source has interface");
        assert!(
            (span.start as usize) < interface_pos,
            "Error span should point to the decorator, not the interface. Span start: {}, interface pos: {}",
            span.start,
            interface_pos
        );
    });
}

#[test]
fn test_multiple_derives_all_invalid_on_interface_produces_multiple_errors() {
    // When multiple derives are used on interface and all are invalid,
    // we should get errors for each
    let source = r#"
/** @derive(Debug, Clone, Eq) */
interface Status {
    active: boolean;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have error diagnostics (Debug, Clone, and Eq all fail on interfaces)
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();

        assert!(
            error_count >= 1,
            "Should have at least one error for invalid derives on interface, got {} errors",
            error_count
        );
    });
}

#[test]
fn test_unknown_derive_macro_produces_error() {
    // A derive macro that doesn't exist should produce an error
    let source = r#"
/** @derive(NonExistentMacro) */
class User {
    name: string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have a diagnostic for unknown macro
        let unknown_error = result.diagnostics.iter().find(|d| {
            d.message.contains("NonExistentMacro")
                || d.message.contains("unknown")
                || d.message.contains("not found")
        });

        assert!(
            unknown_error.is_some(),
            "Should produce an error for unknown derive macro. Diagnostics: {:?}",
            result.diagnostics
        );
    });
}

#[test]
fn test_unknown_derive_macro_on_interface_produces_error() {
    // A derive macro that doesn't exist should produce an error for interfaces too
    let source = r#"
/** @derive(Serializable) */
interface Config {
    host: string;
    port: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have a diagnostic for unknown macro
        let unknown_error = result.diagnostics.iter().find(|d| {
            d.message.contains("Serializable")
                || d.message.contains("unknown")
                || d.message.contains("not found")
        });

        assert!(
            unknown_error.is_some(),
            "Should produce an error for unknown derive macro on interface. Diagnostics: {:?}",
            result.diagnostics
        );
    });
}

#[test]
fn test_error_span_covers_macro_name_not_entire_decorator() {
    // Verify the error span is reasonably sized (not covering the entire file)
    let source = r#"
/** @derive(Clone) */
interface Data {
    value: string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroHostIntegration::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        let error_diag = result
            .diagnostics
            .iter()
            .find(|d| d.level == DiagnosticLevel::Error)
            .expect("Should have an error-level diagnostic");

        let span = error_diag.span.as_ref().expect("Error should have a span");
        let span_len = span.end - span.start;

        // The span should be reasonably sized - not the entire source
        // A macro name like "Clone" would be around 5-20 characters with context
        assert!(
            span_len < 100,
            "Error span should be focused, not cover entire source. Span length: {}",
            span_len
        );
    });
}
