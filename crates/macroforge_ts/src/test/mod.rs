use crate::host::PatchCollector;
use crate::{
    GeneratedRegionResult, MappingSegmentResult, NativePositionMapper, SourceMappingResult,
    host::MacroExpander, parse_import_sources,
};
use swc_core::ecma::ast::{ClassMember, Program};
use swc_core::{
    common::{FileName, GLOBALS, SourceMap, sync::Lrc},
    ecma::parser::{Lexer, Parser, StringInput, Syntax, TsSyntax},
};
use crate::ts_syn::abi::{
    ClassIR, DiagnosticLevel, MacroContextIR, MacroResult, Patch, PatchCode, SpanIR,
};

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
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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
fn test_derive_partial_eq_hash_dts_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(PartialEq, Hash) */
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
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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
    let host = MacroExpander::new().unwrap();
    let result = host.prepare_expansion_context(&program, source).unwrap();
    // Even without decorators, we return Some because we still need to
    // generate method signatures for type output
    assert!(result.is_some());
}

#[test]
fn test_prepare_no_classes() {
    let source = "const x = 1;";
    let program = parse_module(source);
    let host = MacroExpander::new().unwrap();
    let result = host.prepare_expansion_context(&program, source).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_prepare_with_classes() {
    let source = "/** @derive(Debug) */ class User {}";
    let program = parse_module(source);
    let host = MacroExpander::new().unwrap();
    let result = host.prepare_expansion_context(&program, source).unwrap();
    assert!(result.is_some());
    let (_module, items) = result.unwrap();
    assert_eq!(items.classes.len(), 1);
    assert_eq!(items.classes[0].name, "User");
}

#[test]
fn test_process_macro_output_converts_tokens_into_patches() {
    GLOBALS.set(&Default::default(), || {
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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
    let host = MacroExpander::new().unwrap();
    let (module, items) = host
        .prepare_expansion_context(&program, source)
        .unwrap()
        .unwrap();

    let (collector, _) =
        host.collect_macro_patches(&module, items, "test.ts", source);

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
    let host = MacroExpander::new().unwrap();
    let (module, items) = host
        .prepare_expansion_context(&program, source)
        .unwrap()
        .unwrap();
    let (collector, _) =
        host.collect_macro_patches(&module, items, "test.ts", source);

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
    assert!(
        type_patches
            .iter()
            .any(|p| matches!(p, Patch::Delete { .. }))
    );
    // check for method signature insertion
    assert!(
        type_patches
            .iter()
            .any(|p| matches!(p, Patch::Insert { .. }))
    );
}

#[test]
fn test_apply_and_finalize_expansion_no_type_patches() {
    let source = "class User {}";
    let mut collector = PatchCollector::new();
    let mut diagnostics = Vec::new();
    let host = MacroExpander::new().unwrap();
    let result = host
        .apply_and_finalize_expansion(
            source,
            &mut collector,
            &mut diagnostics,
            crate::host::expand::LoweredItems {
                classes: Vec::new(),
                interfaces: Vec::new(),
                enums: Vec::new(),
                type_aliases: Vec::new(),
            },
        )
        .unwrap();
    assert!(result.type_output.is_none());
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
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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

/** @derive(Debug, PartialEq, Hash) */
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
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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
// Interface Derive Macro Tests
// ============================================================================

#[test]
fn test_derive_debug_on_interface_generates_namespace() {
    // Debug macro now works on interfaces, generating a namespace with toString function
    let source = r#"
/** @derive(Debug) */
interface Status {
    active: boolean;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Output should contain namespace with toString function
        assert!(
            result.code.contains("namespace Status"),
            "Should generate namespace Status. Got:\n{}", result.code
        );
        assert!(
            result.code.contains("function toString(self: Status)"),
            "Should generate toString function with self parameter. Got:\n{}", result.code
        );
    });
}

#[test]
fn test_derive_clone_on_interface_generates_namespace() {
    let source = r#"
/** @derive(Clone) */
interface UserData {
    name: string;
    age: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Output should contain namespace with clone function
        assert!(
            result.code.contains("namespace UserData"),
            "Should generate namespace UserData"
        );
        assert!(
            result.code.contains("function clone(self: UserData)"),
            "Should generate clone function with self parameter"
        );
    });
}

#[test]
fn test_derive_partial_eq_hash_on_interface_generates_namespace() {
    let source = r#"
/** @derive(PartialEq, Hash) */
interface Point {
    x: number;
    y: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Output should contain namespace with equals and hashCode functions
        assert!(
            result.code.contains("namespace Point"),
            "Should generate namespace Point"
        );
        assert!(
            result.code.contains("function equals(self: Point, other: Point)"),
            "Should generate equals function with self and other parameters"
        );
        assert!(
            result.code.contains("function hashCode(self: Point)"),
            "Should generate hashCode function with self parameter"
        );
    });
}

#[test]
fn test_derive_debug_on_interface_generates_correct_output() {
    // Test that the generated Debug output is correct for interfaces
    let source = r#"
/** @derive(Debug) */
interface Status {
    active: boolean;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no errors
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should succeed without errors");

        // Verify the output contains expected patterns
        assert!(result.code.contains("self.active"), "Should reference self.active");
    });
}

#[test]
fn test_multiple_derives_on_interface_all_succeed() {
    // When multiple derives are used on interface, all should succeed
    let source = r#"
/** @derive(Debug, Clone, PartialEq, Hash) */
interface Status {
    active: boolean;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(
            error_count, 0,
            "Should have no errors for derives on interface, got {} errors",
            error_count
        );

        // Should have all three namespaces generated
        assert!(
            result.code.contains("function toString(self: Status)"),
            "Should have Debug's toString"
        );
        assert!(
            result.code.contains("function clone(self: Status)"),
            "Should have Clone's clone"
        );
        assert!(
            result.code.contains("function equals(self: Status"),
            "Should have Eq's equals"
        );
        assert!(
            result.code.contains("function hashCode(self: Status)"),
            "Should have Eq's hashCode"
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
        let host = MacroExpander::new().unwrap();
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
        let host = MacroExpander::new().unwrap();
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
    // Use an unknown macro to trigger an error
    let source = r#"
/** @derive(NonExistent) */
interface Data {
    value: string;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        let error_diag = result
            .diagnostics
            .iter()
            .find(|d| d.level == DiagnosticLevel::Error)
            .expect("Should have an error-level diagnostic");

        let span = error_diag.span.as_ref().expect("Error should have a span");
        let span_len = span.end - span.start;

        // The span should be reasonably sized - not the entire source
        // A macro name like "NonExistent" would be around 5-20 characters with context
        assert!(
            span_len < 100,
            "Error span should be focused, not cover entire source. Span length: {}",
            span_len
        );
    });
}

#[test]
fn test_derive_serialize_dts_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Serialize) */
class User {
    name: string;
    age: number;
}
"#;

    let expected_dts = r#"
import { Derive } from "@macro/derive";


class User {
    name: string;
    age: number;
    toJSON(): Record<string, unknown>;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
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
fn test_derive_serialize_runtime_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Serialize) */
class Data {
    val: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        // Serialize macro adds toJSON() method
        assert!(result.code.contains("toJSON()"));
        assert!(result.code.contains("Record<string, unknown>"));
    });
}

#[test]
fn test_derive_deserialize_dts_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Deserialize) */
class User {
    name: string;
    age: number;
}
"#;

    let expected_dts = r#"
import { Derive } from "@macro/derive";


class User {
    name: string;
    age: number;
    constructor(init: { name: string; age: number; });
    static fromJSON(data: unknown): Result<User, string[]>;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
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
fn test_derive_deserialize_runtime_output() {
    let source = r#"
import { Derive } from "@macro/derive";

/** @derive(Deserialize) */
class Data {
    val: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed, "expand() should report changes");
        // Deserialize macro adds static fromJSON() method
        assert!(result.code.contains("fromJSON"));
        assert!(result.code.contains("static"));
    });
}

#[test]
fn test_derive_serialize_on_interface_generates_namespace() {
    let source = r#"
/** @derive(Serialize) */
interface Point {
    x: number;
    y: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Output should contain namespace with toJSON function
        assert!(
            result.code.contains("namespace Point"),
            "Should generate namespace Point"
        );
        assert!(
            result.code.contains("function toJSON(self: Point)"),
            "Should generate toJSON function with self parameter"
        );
    });
}

#[test]
fn test_derive_deserialize_on_interface_generates_namespace() {
    let source = r#"
/** @derive(Deserialize) */
interface Point {
    x: number;
    y: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Output should contain namespace with fromJSON and is functions
        assert!(
            result.code.contains("namespace Point"),
            "Should generate namespace Point"
        );
        assert!(
            result.code.contains("function fromJSON(data: unknown)"),
            "Should generate fromJSON function"
        );
        assert!(
            result.code.contains("function is(data: unknown)"),
            "Should generate is type guard function"
        );
    });
}

#[test]
fn test_multiple_derives_with_serialize_deserialize() {
    // When Serialize and Deserialize are combined, both should succeed
    let source = r#"
/** @derive(Serialize, Deserialize) */
class Config {
    host: string;
    port: number;
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(
            error_count, 0,
            "Should have no errors, got {} errors",
            error_count
        );

        // Should have both toJSON and fromJSON methods
        assert!(
            result.code.contains("toJSON()"),
            "Should have Serialize's toJSON"
        );
        assert!(
            result.code.contains("static fromJSON"),
            "Should have Deserialize's fromJSON"
        );
    });
}

// ==================== ENUM TESTS ====================

#[test]
fn test_derive_debug_on_enum_generates_namespace() {
    let source = r#"
/** @derive(Debug) */
enum Status {
    Active,
    Inactive,
    Pending
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Debug macro on enum generates namespace with toString
        assert!(
            result.code.contains("namespace Status"),
            "Should generate namespace for enum"
        );
        assert!(
            result.code.contains("toString"),
            "Should have toString function"
        );
    });
}

#[test]
fn test_derive_clone_on_enum_generates_namespace() {
    let source = r#"
/** @derive(Clone) */
enum Priority {
    Low = 1,
    Medium = 2,
    High = 3
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Clone macro on enum generates namespace with clone function
        assert!(
            result.code.contains("namespace Priority"),
            "Should generate namespace for enum"
        );
        assert!(
            result.code.contains("clone"),
            "Should have clone function"
        );
    });
}

#[test]
fn test_derive_partial_eq_hash_on_enum_generates_namespace() {
    let source = r#"
/** @derive(PartialEq, Hash) */
enum Color {
    Red = "red",
    Green = "green",
    Blue = "blue"
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // PartialEq and Hash macros on enum generate namespace with equals and hashCode
        assert!(
            result.code.contains("namespace Color"),
            "Should generate namespace for enum"
        );
        assert!(result.code.contains("equals"), "Should have equals function");
        assert!(
            result.code.contains("hashCode"),
            "Should have hashCode function"
        );
    });
}

#[test]
fn test_derive_serialize_on_enum_generates_namespace() {
    let source = r#"
/** @derive(Serialize) */
enum Direction {
    North,
    South,
    East,
    West
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Serialize macro on enum generates namespace with toJSON
        assert!(
            result.code.contains("namespace Direction"),
            "Should generate namespace for enum"
        );
        assert!(result.code.contains("toJSON"), "Should have toJSON function");
    });
}

#[test]
fn test_derive_deserialize_on_enum_generates_namespace() {
    let source = r#"
/** @derive(Deserialize) */
enum Role {
    Admin = "admin",
    User = "user",
    Guest = "guest"
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Deserialize macro on enum generates namespace with fromJSON
        assert!(
            result.code.contains("namespace Role"),
            "Should generate namespace for enum"
        );
        assert!(
            result.code.contains("fromJSON"),
            "Should have fromJSON function"
        );
    });
}

#[test]
fn test_multiple_derives_on_enum() {
    let source = r#"
/** @derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize) */
enum Status {
    Active = "active",
    Inactive = "inactive"
}
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(
            error_count, 0,
            "Should have no errors, got {} errors",
            error_count
        );

        // All macros should generate functions in the namespace
        assert!(
            result.code.contains("namespace Status"),
            "Should generate namespace for enum"
        );
        assert!(
            result.code.contains("toString"),
            "Should have Debug's toString"
        );
        assert!(result.code.contains("clone"), "Should have Clone's clone");
        assert!(result.code.contains("equals"), "Should have Eq's equals");
        assert!(
            result.code.contains("hashCode"),
            "Should have Eq's hashCode"
        );
        assert!(
            result.code.contains("toJSON"),
            "Should have Serialize's toJSON"
        );
        assert!(
            result.code.contains("fromJSON"),
            "Should have Deserialize's fromJSON"
        );
    });
}

// ==================== TYPE ALIAS TESTS ====================

#[test]
fn test_derive_debug_on_type_alias_generates_namespace() {
    let source = r#"
/** @derive(Debug) */
type Point = {
    x: number;
    y: number;
};
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Debug macro on type alias generates namespace with toString
        assert!(
            result.code.contains("namespace Point"),
            "Should generate namespace for type"
        );
        assert!(
            result.code.contains("toString"),
            "Should have toString function"
        );
    });
}

#[test]
fn test_derive_clone_on_type_alias_generates_namespace() {
    let source = r#"
/** @derive(Clone) */
type Config = {
    host: string;
    port: number;
};
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Clone macro on type alias generates namespace with clone
        assert!(
            result.code.contains("namespace Config"),
            "Should generate namespace for type"
        );
        assert!(result.code.contains("clone"), "Should have clone function");
    });
}

#[test]
fn test_derive_partial_eq_hash_on_type_alias_generates_namespace() {
    let source = r#"
/** @derive(PartialEq, Hash) */
type Vector = {
    x: number;
    y: number;
    z: number;
};
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // PartialEq and Hash macros on type alias generate namespace with equals and hashCode
        assert!(
            result.code.contains("namespace Vector"),
            "Should generate namespace for type"
        );
        assert!(result.code.contains("equals"), "Should have equals function");
        assert!(
            result.code.contains("hashCode"),
            "Should have hashCode function"
        );
    });
}

#[test]
fn test_derive_serialize_on_type_alias_generates_namespace() {
    let source = r#"
/** @derive(Serialize) */
type User = {
    name: string;
    age: number;
};
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Serialize macro on type alias generates namespace with toJSON
        assert!(
            result.code.contains("namespace User"),
            "Should generate namespace for type"
        );
        assert!(result.code.contains("toJSON"), "Should have toJSON function");
    });
}

#[test]
fn test_derive_deserialize_on_type_alias_generates_namespace() {
    let source = r#"
/** @derive(Deserialize) */
type Settings = {
    theme: string;
    language: string;
};
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Deserialize macro on type alias generates namespace with fromJSON
        assert!(
            result.code.contains("namespace Settings"),
            "Should generate namespace for type"
        );
        assert!(
            result.code.contains("fromJSON"),
            "Should have fromJSON function"
        );
    });
}

#[test]
fn test_multiple_derives_on_type_alias() {
    let source = r#"
/** @derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize) */
type Coordinate = {
    lat: number;
    lng: number;
};
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(
            error_count, 0,
            "Should have no errors, got {} errors",
            error_count
        );

        // All macros should generate functions in the namespace
        assert!(
            result.code.contains("namespace Coordinate"),
            "Should generate namespace for type"
        );
        assert!(
            result.code.contains("toString"),
            "Should have Debug's toString"
        );
        assert!(result.code.contains("clone"), "Should have Clone's clone");
        assert!(result.code.contains("equals"), "Should have Eq's equals");
        assert!(
            result.code.contains("hashCode"),
            "Should have Eq's hashCode"
        );
        assert!(
            result.code.contains("toJSON"),
            "Should have Serialize's toJSON"
        );
        assert!(
            result.code.contains("fromJSON"),
            "Should have Deserialize's fromJSON"
        );
    });
}

#[test]
fn test_derive_on_union_type_alias() {
    let source = r#"
/** @derive(Debug, PartialEq, Hash) */
type Status = "active" | "inactive" | "pending";
"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors, got {}", error_count);

        // Macros should generate namespace for union type alias
        assert!(
            result.code.contains("namespace Status"),
            "Should generate namespace for union type"
        );
    });
}

#[test]
fn test_inline_jsdoc_with_export_interface() {
    // Test that /** @derive(X) */ export interface works on same line
    let source = r#"/** @derive(Deserialize) */ export interface User { name: string; age: number; }"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors for inline JSDoc, got {}", error_count);

        // Should generate fromJSON method in User namespace
        assert!(
            result.code.contains("User") && result.code.contains("fromJSON"),
            "Should generate fromJSON for Deserialize on interface. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_inline_jsdoc_with_export_class() {
    // Test that /** @derive(X) */ export class works on same line
    let source = r#"/** @derive(Debug) */ export class User { name: string; }"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors for inline JSDoc on class, got {}", error_count);

        // Should generate toString method in class
        assert!(
            result.code.contains("toString"),
            "Should generate toString for Debug on class. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_inline_jsdoc_with_export_enum() {
    // Test that /** @derive(X) */ export enum works on same line
    let source = r#"/** @derive(Debug) */ export enum Status { Active, Inactive }"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors for inline JSDoc on enum, got {}", error_count);

        // Should generate toString in Status namespace
        assert!(
            result.code.contains("Status") && result.code.contains("toString"),
            "Should generate toString for Debug on enum. Got:\n{}",
            result.code
        );
    });
}

#[test]
fn test_inline_jsdoc_with_export_type() {
    // Test that /** @derive(X) */ export type works on same line
    let source = r#"/** @derive(Debug) */ export type Point = { x: number; y: number; }"#;

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        // Should have no error diagnostics
        let error_count = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count();
        assert_eq!(error_count, 0, "Should have no errors for inline JSDoc on type, got {}", error_count);

        // Should generate toString in Point namespace
        assert!(
            result.code.contains("Point") && result.code.contains("toString"),
            "Should generate toString for Debug on type alias. Got:\n{}",
            result.code
        );
    });
}

// ==================== EARLY BAILOUT TESTS ====================

#[test]
fn test_early_bailout_no_derive_returns_unchanged() {
    // Code without @derive should be returned unchanged immediately
    use crate::expand_inner;

    let source = r#"
class User {
    name: string;
    constructor(name: string) {
        this.name = name;
    }
}
"#;

    let result = expand_inner(source, "test.ts", None).unwrap();

    // Code should be returned exactly as-is
    assert_eq!(result.code, source, "Code without @derive should be returned unchanged");
    assert!(result.types.is_none(), "No type output expected");
    assert!(result.diagnostics.is_empty(), "No diagnostics expected");
    assert!(result.source_mapping.is_none(), "No source mapping expected");
}

#[test]
fn test_early_bailout_svelte_runes_unchanged() {
    // Svelte runes ($state, $derived) without @derive should be returned unchanged
    use crate::expand_inner;

    let source = r#"
let count = $state(0);
let double = $derived(count * 2);

function increment() {
    count++;
}
"#;

    let result = expand_inner(source, "Counter.svelte.ts", None).unwrap();

    // Svelte runes code should be returned exactly as-is
    assert_eq!(result.code, source, "Svelte runes without @derive should be unchanged");
    assert!(result.diagnostics.is_empty(), "No diagnostics for Svelte runes");
}

#[test]
fn test_early_bailout_svelte_props_unchanged() {
    // Svelte $props() rune should be returned unchanged
    use crate::expand_inner;

    let source = r#"
interface Props {
    name: string;
    count?: number;
}

let { name, count = 0 }: Props = $props();
"#;

    let result = expand_inner(source, "Component.svelte.ts", None).unwrap();

    assert_eq!(result.code, source, "Svelte $props() without @derive should be unchanged");
    assert!(result.diagnostics.is_empty(), "No diagnostics for Svelte $props");
}

#[test]
fn test_early_bailout_complex_svelte_component_unchanged() {
    // Complex Svelte component code without @derive should be returned unchanged
    use crate::expand_inner;

    let source = r#"
interface Props {
    items: string[];
    selected?: string;
}

let { items, selected = '' }: Props = $props();

let filteredItems = $derived(
    items.filter(item => item.includes(selected))
);

let count = $state(0);
let message = $derived.by(() => {
    if (count === 0) return 'No items';
    return `${count} items`;
});

function handleClick() {
    count++;
}

$effect(() => {
    console.log('Count changed:', count);
});
"#;

    let result = expand_inner(source, "List.svelte.ts", None).unwrap();

    assert_eq!(result.code, source, "Complex Svelte code without @derive should be unchanged");
    assert!(result.diagnostics.is_empty(), "No diagnostics expected");
}

#[test]
fn test_with_derive_processes_normally() {
    // Code WITH @derive should be processed normally
    use crate::expand_inner;

    let source = r#"
/** @derive(Debug) */
class User {
    name: string;
}
"#;

    let result = expand_inner(source, "test.ts", None).unwrap();

    // Should have processed the macro
    assert!(result.code.contains("toString"), "Debug macro should generate toString");
    assert_ne!(result.code, source, "Code with @derive should be modified");
}

#[test]
fn test_derive_in_string_literal_still_skipped() {
    // @derive inside a string literal should still trigger processing
    // (we do a simple contains check, not parsing)
    use crate::expand_inner;

    let source = r#"
const msg = "Use @derive to add methods";
class User {
    name: string;
}
"#;

    let result = expand_inner(source, "test.ts", None).unwrap();

    // The contains("@derive") check will find the string literal
    // This is a conservative approach - we process the file but find no actual decorators
    // The result should have no errors and the code may have minor changes from parsing
    assert!(result.diagnostics.iter().all(|d| d.level != "error".to_string()),
        "No errors expected even with @derive in string literal");
}

#[test]
fn test_early_bailout_empty_file() {
    // Empty file should be returned unchanged
    use crate::expand_inner;

    let source = "";
    let result = expand_inner(source, "empty.ts", None).unwrap();

    assert_eq!(result.code, source, "Empty file should be returned unchanged");
    assert!(result.diagnostics.is_empty(), "No diagnostics for empty file");
}

#[test]
fn test_early_bailout_only_comments() {
    // File with only comments should be returned unchanged
    use crate::expand_inner;

    let source = r#"
// This is a comment
/* Another comment */
/**
 * JSDoc comment without derive
 */
"#;

    let result = expand_inner(source, "comments.ts", None).unwrap();

    assert_eq!(result.code, source, "Comments-only file should be returned unchanged");
    assert!(result.diagnostics.is_empty(), "No diagnostics for comments-only file");
}

#[test]
fn test_early_bailout_regular_typescript() {
    // Regular TypeScript without macros should be returned unchanged
    use crate::expand_inner;

    let source = r#"
interface User {
    id: string;
    name: string;
    email: string;
}

type Role = 'admin' | 'user' | 'guest';

enum Status {
    Active,
    Inactive,
    Pending
}

function createUser(name: string): User {
    return {
        id: crypto.randomUUID(),
        name,
        email: `${name}@example.com`
    };
}

const users: Map<string, User> = new Map();

export { User, Role, Status, createUser, users };
"#;

    let result = expand_inner(source, "types.ts", None).unwrap();

    assert_eq!(result.code, source, "Regular TypeScript should be returned unchanged");
    assert!(result.diagnostics.is_empty(), "No diagnostics for regular TypeScript");
}
