use super::*;

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

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        // Check for static method and standalone function
        assert!(type_output.contains("static toString(value: API): string"));
        assert!(type_output.contains("export function apiToString"));
        // Original methods should still be present
        assert!(type_output.contains("async fetch<T>"));
        assert!(type_output.contains("subscribe("));
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

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        // Check for static method and standalone function
        assert!(type_output.contains("static clone(value: Account): Account"));
        assert!(type_output.contains("export function accountClone"));
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

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        // Check for static methods and standalone functions
        assert!(type_output.contains("static toString(value: Config): string"));
        assert!(type_output.contains("static equals(a: Config, b: Config): boolean"));
        assert!(type_output.contains("static hashCode(value: Config): number"));
        assert!(type_output.contains("export function configToString"));
        assert!(type_output.contains("export function configEquals"));
        assert!(type_output.contains("export function configHashCode"));
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

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        // Check for static method and standalone function
        assert!(type_output.contains("static toString(value: Singleton): string"));
        assert!(type_output.contains("export function singletonToString"));
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

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        // Check for static method and standalone function
        assert!(type_output.contains("static toString(value: ValidationExample): string"));
        assert!(type_output.contains("export function validationExampleToString"));
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

        // Verify static methods are on separate lines
        let lines: Vec<&str> = type_output.lines().collect();

        // Find the static toString line
        let tostring_line = lines
            .iter()
            .position(|l| l.contains("static toString(value: User)"))
            .expect("should have static toString");
        // Find the static clone line
        let clone_line = lines
            .iter()
            .position(|l| l.contains("static clone(value: User)"))
            .expect("should have static clone");

        // They should be on different lines
        assert_ne!(
            tostring_line, clone_line,
            "toString and clone should be on different lines"
        );
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

        // Find the static toString line
        let tostring_line = type_output
            .lines()
            .find(|l| l.contains("static toString(value: User)"))
            .expect("should have static toString method");

        // Verify it has proper indentation (static method inside class)
        assert!(
            tostring_line.contains("static toString(value: User)"),
            "should have static toString method, got: '{}'",
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

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        // Check for static method in class
        assert!(
            type_output.contains("static toString(value: ServerConfig): string"),
            "should have static toString method, got:\n{}",
            type_output
        );
        // Check for standalone exported function
        assert!(
            type_output.contains("export function serverConfigToString"),
            "should have exported serverConfigToString function, got:\n{}",
            type_output
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

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "test.ts").unwrap();

        assert!(result.changed);
        let type_output = result.type_output.expect("should have type output");

        // Check for static method in class
        assert!(
            type_output.contains("static clone(value: EventEmitter): EventEmitter"),
            "should have static clone method, got:\n{}",
            type_output
        );
        // Check for standalone exported function
        assert!(
            type_output.contains("export function eventEmitterClone"),
            "should have exported eventEmitterClone function, got:\n{}",
            type_output
        );
    });
}
