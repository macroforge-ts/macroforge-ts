//! IR Variant Coverage Tests - Expressions
//!
//! Tests for: ClassExpr, FnExpr, UnaryExpr, UpdateExpr, AwaitExpr, YieldExpr,
//! ParenExpr, SeqExpr, OptChainExpr, TsAsExpr, TsNonNullExpr, TsSatisfiesExpr,
//! TsInstantiation, SuperExpr, TaggedTpl, PrivateName, BigIntLit

use macroforge_ts_quote::ts_template;

// =============================================================================
// Class Expressions
// =============================================================================

#[test]
fn test_class_expr_anonymous() {
    let stream = ts_template! {
        const MyClass = class {
            value: number;
            constructor() {
                this.value = 0;
            }
        };
    };
    let source = stream.source();
    assert!(source.contains("= class") || source.contains("=class"), "Expected '= class'. Got:\n{}", source);
}

#[test]
fn test_class_expr_named() {
    let stream = ts_template! {
        const Factory = class FactoryImpl {
            create() {
                return new FactoryImpl();
            }
        };
    };
    let source = stream.source();
    assert!(source.contains("class FactoryImpl") || source.contains("class  FactoryImpl"), "Expected 'class FactoryImpl'. Got:\n{}", source);
}

#[test]
fn test_class_expr_with_extends() {
    let stream = ts_template! {
        const Child = class extends Parent {
            constructor() {
                super();
            }
        };
    };
    let source = stream.source();
    assert!(source.contains("extends Parent") || source.contains("extends  Parent"), "Expected 'extends Parent'. Got:\n{}", source);
}

// =============================================================================
// Function Expressions
// =============================================================================

#[test]
fn test_fn_expr_anonymous() {
    let stream = ts_template! {
        const handler = function(event: Event): void {
            console.log(event);
        };
    };
    let source = stream.source();
    assert!(source.contains("function(") || source.contains("function ("), "Expected 'function('. Got:\n{}", source);
}

#[test]
fn test_fn_expr_named() {
    let stream = ts_template! {
        const factorial = function fact(n: number): number {
            return n <= 1 ? 1 : n * fact(n - 1);
        };
    };
    let source = stream.source();
    assert!(source.contains("function fact") || source.contains("function  fact"), "Expected 'function fact'. Got:\n{}", source);
}

#[test]
fn test_fn_expr_async() {
    let stream = ts_template! {
        const fetchData = async function() {
            return await fetch(url);
        };
    };
    let source = stream.source();
    assert!(source.contains("async") && source.contains("function"), "Expected 'async function'. Got:\n{}", source);
}

#[test]
fn test_fn_expr_generator() {
    let stream = ts_template! {
        const gen = function*() {
            yield 1;
            yield 2;
        };
    };
    let source = stream.source();
    assert!(source.contains("function*") || source.contains("function *"), "Expected 'function*'. Got:\n{}", source);
}

// =============================================================================
// Unary Expressions
// =============================================================================

#[test]
fn test_unary_expr_minus() {
    let stream = ts_template! {
        const negative = -value;
    };
    let source = stream.source();
    assert!(source.contains("-value") || source.contains("- value"), "Expected '-value'. Got:\n{}", source);
}

#[test]
fn test_unary_expr_plus() {
    let stream = ts_template! {
        const num = +stringValue;
    };
    let source = stream.source();
    assert!(source.contains("+stringValue") || source.contains("+ stringValue"), "Expected '+stringValue'. Got:\n{}", source);
}

#[test]
fn test_unary_expr_not() {
    let stream = ts_template! {
        const inverted = !condition;
    };
    let source = stream.source();
    assert!(source.contains("!condition") || source.contains("! condition"), "Expected '!condition'. Got:\n{}", source);
}

#[test]
fn test_unary_expr_bitwise_not() {
    let stream = ts_template! {
        const flipped = ~bits;
    };
    let source = stream.source();
    assert!(source.contains("~bits") || source.contains("~ bits"), "Expected '~bits'. Got:\n{}", source);
}

#[test]
fn test_unary_expr_typeof() {
    let stream = ts_template! {
        const typeStr = typeof value;
    };
    let source = stream.source();
    assert!(source.contains("typeof value") || source.contains("typeof  value"), "Expected 'typeof value'. Got:\n{}", source);
}

#[test]
fn test_unary_expr_void() {
    let stream = ts_template! {
        void runSideEffect();
    };
    let source = stream.source();
    assert!(source.contains("void runSideEffect") || source.contains("void  runSideEffect"), "Expected 'void runSideEffect'. Got:\n{}", source);
}

#[test]
fn test_unary_expr_delete() {
    let stream = ts_template! {
        delete obj.property;
    };
    let source = stream.source();
    assert!(source.contains("delete obj") || source.contains("delete  obj"), "Expected 'delete obj'. Got:\n{}", source);
}

// =============================================================================
// Update Expressions
// =============================================================================

#[test]
fn test_update_expr_postfix_increment() {
    let stream = ts_template! {
        counter++;
    };
    let source = stream.source();
    assert!(source.contains("counter++") || source.contains("counter ++"), "Expected 'counter++'. Got:\n{}", source);
}

#[test]
fn test_update_expr_prefix_increment() {
    let stream = ts_template! {
        const next = ++counter;
    };
    let source = stream.source();
    assert!(source.contains("++counter") || source.contains("++ counter"), "Expected '++counter'. Got:\n{}", source);
}

#[test]
fn test_update_expr_postfix_decrement() {
    let stream = ts_template! {
        remaining--;
    };
    let source = stream.source();
    assert!(source.contains("remaining--") || source.contains("remaining --"), "Expected 'remaining--'. Got:\n{}", source);
}

#[test]
fn test_update_expr_prefix_decrement() {
    let stream = ts_template! {
        const prev = --counter;
    };
    let source = stream.source();
    assert!(source.contains("--counter") || source.contains("-- counter"), "Expected '--counter'. Got:\n{}", source);
}

// =============================================================================
// Await Expressions
// =============================================================================

#[test]
fn test_await_expr_simple() {
    let stream = ts_template! {
        async function load() {
            const data = await fetch(url);
            return data;
        }
    };
    let source = stream.source();
    assert!(source.contains("await fetch") || source.contains("await  fetch"), "Expected 'await fetch'. Got:\n{}", source);
}

#[test]
fn test_await_expr_chained() {
    let stream = ts_template! {
        async function getJson() {
            const response = await fetch(url);
            const json = await response.json();
            return json;
        }
    };
    let source = stream.source();
    assert!(source.contains("await") && source.contains("json"), "Expected await and json. Got:\n{}", source);
}

// =============================================================================
// Yield Expressions
// =============================================================================

#[test]
fn test_yield_expr_simple() {
    let stream = ts_template! {
        function* generator() {
            yield 1;
            yield 2;
            yield 3;
        }
    };
    let source = stream.source();
    assert!(source.contains("yield"), "Expected 'yield'. Got:\n{}", source);
}

#[test]
fn test_yield_expr_with_value() {
    let stream = ts_template! {
        function* range(start: number, end: number) {
            for (let i = start; i < end; i++) {
                yield i;
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("yield i") || source.contains("yield  i"), "Expected 'yield i'. Got:\n{}", source);
}

#[test]
fn test_yield_expr_delegate() {
    let stream = ts_template! {
        function* composed() {
            yield* [1, 2, 3];
        }
    };
    let source = stream.source();
    assert!(source.contains("yield*") || source.contains("yield *"), "Expected 'yield*'. Got:\n{}", source);
}

// =============================================================================
// Parenthesized Expressions
// =============================================================================

#[test]
fn test_paren_expr_arithmetic() {
    let stream = ts_template! {
        const result = (a + b) * c;
    };
    let source = stream.source();
    // Parentheses should be present for correct precedence
    assert!(source.contains("(") && source.contains(")"), "Expected parentheses. Got:\n{}", source);
    assert!(source.contains("a") && source.contains("b") && source.contains("c"), "Expected a, b, c. Got:\n{}", source);
}

#[test]
fn test_paren_expr_for_precedence() {
    let stream = ts_template! {
        const value = (condition ? a : b).property;
    };
    let source = stream.source();
    assert!(source.contains("condition"), "Expected 'condition'. Got:\n{}", source);
    assert!(source.contains("property"), "Expected 'property'. Got:\n{}", source);
}

// =============================================================================
// Sequence Expressions
// =============================================================================

#[test]
fn test_seq_expr_simple() {
    let stream = ts_template! {
        const last = (a = 1, b = 2, c = 3);
    };
    let source = stream.source();
    assert!(source.contains("a") && source.contains("b") && source.contains("c"), "Expected a, b, c. Got:\n{}", source);
}

#[test]
fn test_seq_expr_in_for() {
    let stream = ts_template! {
        for (let i = 0, j = 10; i < j; i++, j--) {
            console.log(i, j);
        }
    };
    let source = stream.source();
    assert!(source.contains("i") && source.contains("j"), "Expected 'i' and 'j'. Got:\n{}", source);
}

// =============================================================================
// Optional Chaining
// =============================================================================

#[test]
fn test_opt_chain_property() {
    let stream = ts_template! {
        const name = user?.profile?.name;
    };
    let source = stream.source();
    assert!(source.contains("?."), "Expected '?.'. Got:\n{}", source);
}

#[test]
fn test_opt_chain_element() {
    let stream = ts_template! {
        const first = array?.[0];
    };
    let source = stream.source();
    assert!(source.contains("?.["), "Expected '?.['. Got:\n{}", source);
}

#[test]
fn test_opt_chain_call() {
    let stream = ts_template! {
        const result = callback?.();
    };
    let source = stream.source();
    assert!(source.contains("?.("), "Expected '?.('. Got:\n{}", source);
}

// =============================================================================
// TypeScript Type Assertions
// =============================================================================

#[test]
fn test_ts_as_expr() {
    let stream = ts_template! {
        const str = value as string;
    };
    let source = stream.source();
    assert!(source.contains("as string") || source.contains("as  string"), "Expected 'as string'. Got:\n{}", source);
}

#[test]
fn test_ts_as_const() {
    let stream = ts_template! {
        const tuple = [1, 2, 3] as const;
    };
    let source = stream.source();
    assert!(source.contains("as const") || source.contains("as  const"), "Expected 'as const'. Got:\n{}", source);
}

#[test]
fn test_ts_as_any() {
    let stream = ts_template! {
        const unsafe = unknownValue as any;
    };
    let source = stream.source();
    assert!(source.contains("as any") || source.contains("as  any"), "Expected 'as any'. Got:\n{}", source);
}

// =============================================================================
// Non-Null Assertions
// =============================================================================

#[test]
fn test_ts_non_null_expr() {
    let stream = ts_template! {
        const definite = maybeNull!;
    };
    let source = stream.source();
    assert!(source.contains("maybeNull!") || source.contains("maybeNull !"), "Expected non-null assertion. Got:\n{}", source);
}

#[test]
fn test_ts_non_null_chained() {
    let stream = ts_template! {
        const value = obj!.property!.nested;
    };
    let source = stream.source();
    // Should have multiple non-null assertions
    let count = source.matches("!").count();
    assert!(count >= 2, "Expected at least 2 '!' assertions. Got {} in:\n{}", count, source);
}

// =============================================================================
// Satisfies Expressions
// =============================================================================

#[test]
fn test_ts_satisfies_expr() {
    let stream = ts_template! {
        const config = { name: "test" } satisfies Config;
    };
    let source = stream.source();
    assert!(source.contains("satisfies Config") || source.contains("satisfies  Config"), "Expected 'satisfies Config'. Got:\n{}", source);
}

#[test]
fn test_ts_satisfies_complex() {
    let stream = ts_template! {
        const palette = {
            red: [255, 0, 0],
            green: "#00ff00"
        } satisfies Record<string, string | number[]>;
    };
    let source = stream.source();
    assert!(source.contains("satisfies"), "Expected 'satisfies'. Got:\n{}", source);
}

// =============================================================================
// Type Instantiation
// =============================================================================

#[test]
fn test_ts_instantiation() {
    let stream = ts_template! {
        const specificFn = genericFn<string>;
    };
    let source = stream.source();
    assert!(source.contains("genericFn<string>") || source.contains("genericFn< string >") || source.contains("genericFn <string>"), "Expected type instantiation. Got:\n{}", source);
}

#[test]
fn test_ts_instantiation_multiple() {
    let stream = ts_template! {
        const mapFn = transform<Input, Output>;
    };
    let source = stream.source();
    assert!(source.contains("Input") && source.contains("Output"), "Expected Input, Output. Got:\n{}", source);
}

// =============================================================================
// Super Expressions
// =============================================================================

#[test]
fn test_super_call() {
    let stream = ts_template! {
        class Child extends Parent {
            constructor() {
                super();
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("super()") || source.contains("super ()"), "Expected 'super()'. Got:\n{}", source);
}

#[test]
fn test_super_call_with_args() {
    let stream = ts_template! {
        class Child extends Parent {
            constructor(name: string) {
                super(name);
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("super(name)") || source.contains("super( name )"), "Expected 'super(name)'. Got:\n{}", source);
}

#[test]
fn test_super_property() {
    let stream = ts_template! {
        class Child extends Parent {
            method() {
                return super.method();
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("super.method"), "Expected 'super.method'. Got:\n{}", source);
}

// =============================================================================
// Tagged Template Literals
// =============================================================================
// Note: Backticks (`) are not valid Rust tokens, so tagged template literals
// cannot be tested directly in ts_template! macros. These would need to be
// tested via string-based template compilation instead.

// =============================================================================
// Private Names
// =============================================================================

#[test]
fn test_private_name_field() {
    let stream = ts_template! {
        class Counter {
            #count: number = 0;

            increment() {
                this.#count++;
            }

            get value() {
                return this.#count;
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("#count"), "Expected '#count'. Got:\n{}", source);
}

#[test]
fn test_private_name_method() {
    let stream = ts_template! {
        class Service {
            #validate(input: string): boolean {
                return input.length > 0;
            }

            process(input: string) {
                if (this.#validate(input)) {
                    return input;
                }
                return null;
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("#validate"), "Expected '#validate'. Got:\n{}", source);
}

#[test]
fn test_private_name_static() {
    let stream = ts_template! {
        class Singleton {
            static #instance: Singleton;

            static getInstance() {
                if (!Singleton.#instance) {
                    Singleton.#instance = new Singleton();
                }
                return Singleton.#instance;
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("#instance"), "Expected '#instance'. Got:\n{}", source);
}

// =============================================================================
// BigInt Literals
// =============================================================================

#[test]
fn test_bigint_lit_simple() {
    let stream = ts_template! {
        const big = 9007199254740991n;
    };
    let source = stream.source();
    assert!(source.contains("9007199254740991n"), "Expected BigInt literal. Got:\n{}", source);
}

#[test]
fn test_bigint_lit_zero() {
    let stream = ts_template! {
        const zero = 0n;
    };
    let source = stream.source();
    assert!(source.contains("0n"), "Expected '0n'. Got:\n{}", source);
}

#[test]
fn test_bigint_lit_operations() {
    let stream = ts_template! {
        const result = 100n + 200n;
    };
    let source = stream.source();
    assert!(source.contains("100n") && source.contains("200n"), "Expected BigInt operations. Got:\n{}", source);
}

#[test]
fn test_bigint_lit_negative() {
    let stream = ts_template! {
        const negative = -42n;
    };
    let source = stream.source();
    assert!(source.contains("42n"), "Expected '42n'. Got:\n{}", source);
}
