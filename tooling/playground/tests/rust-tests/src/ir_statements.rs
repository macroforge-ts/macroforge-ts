//! IR Variant Coverage Tests - Statements
//!
//! Tests for: ForInStmt, ForOfStmt, EmptyStmt, ThrowStmt

use macroforge_ts_quote::ts_template;

// =============================================================================
// For-In Statements
// =============================================================================

#[test]
fn test_for_in_stmt_object() {
    let stream = ts_template! {
        for (const key in object) {
            console.log(key);
        }
    };
    let source = stream.source();
    assert!(source.contains("for"), "Expected 'for'. Got:\n{}", source);
    assert!(source.contains("in object") || source.contains("in  object"), "Expected 'in object'. Got:\n{}", source);
}

#[test]
fn test_for_in_stmt_with_index() {
    let stream = ts_template! {
        for (const index in array) {
            console.log(index, array[index]);
        }
    };
    let source = stream.source();
    assert!(source.contains("for") && source.contains("in array"), "Expected for-in. Got:\n{}", source);
}

#[test]
fn test_for_in_stmt_let() {
    let stream = ts_template! {
        for (let prop in obj) {
            if (obj.hasOwnProperty(prop)) {
                process(prop);
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("let prop") || source.contains("let  prop"), "Expected 'let prop'. Got:\n{}", source);
    assert!(source.contains("in obj") || source.contains("in  obj"), "Expected 'in obj'. Got:\n{}", source);
}

#[test]
fn test_for_in_stmt_var() {
    let stream = ts_template! {
        for (var key in config) {
            settings[key] = config[key];
        }
    };
    let source = stream.source();
    assert!(source.contains("var key") || source.contains("var  key"), "Expected 'var key'. Got:\n{}", source);
}

// =============================================================================
// For-Of Statements
// =============================================================================

#[test]
fn test_for_of_stmt_array() {
    let stream = ts_template! {
        for (const item of items) {
            process(item);
        }
    };
    let source = stream.source();
    assert!(source.contains("for"), "Expected 'for'. Got:\n{}", source);
    assert!(source.contains("of items") || source.contains("of  items"), "Expected 'of items'. Got:\n{}", source);
}

#[test]
fn test_for_of_stmt_destructure_array() {
    let stream = ts_template! {
        for (const [key, value] of entries) {
            map.set(key, value);
        }
    };
    let source = stream.source();
    assert!(source.contains("of entries") || source.contains("of  entries"), "Expected 'of entries'. Got:\n{}", source);
}

#[test]
fn test_for_of_stmt_destructure_object() {
    let stream = ts_template! {
        for (const { name, age } of users) {
            console.log(name, age);
        }
    };
    let source = stream.source();
    assert!(source.contains("of users") || source.contains("of  users"), "Expected 'of users'. Got:\n{}", source);
}

#[test]
fn test_for_of_stmt_string() {
    let stream = ts_template! {
        for (const char of text) {
            chars.push(char);
        }
    };
    let source = stream.source();
    assert!(source.contains("of text") || source.contains("of  text"), "Expected 'of text'. Got:\n{}", source);
}

#[test]
fn test_for_of_stmt_set() {
    let stream = ts_template! {
        for (const value of uniqueSet) {
            results.push(value);
        }
    };
    let source = stream.source();
    assert!(source.contains("of uniqueSet") || source.contains("of  uniqueSet"), "Expected 'of uniqueSet'. Got:\n{}", source);
}

#[test]
fn test_for_of_stmt_map() {
    let stream = ts_template! {
        for (const [k, v] of myMap) {
            console.log(k, v);
        }
    };
    let source = stream.source();
    assert!(source.contains("of myMap") || source.contains("of  myMap"), "Expected 'of myMap'. Got:\n{}", source);
}

// =============================================================================
// For-Await-Of Statements
// =============================================================================

#[test]
fn test_for_await_of_stmt() {
    let stream = ts_template! {
        async function consume() {
            for await (const chunk of stream) {
                process(chunk);
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("for await") || source.contains("for  await"), "Expected 'for await'. Got:\n{}", source);
}

#[test]
fn test_for_await_of_stmt_async_generator() {
    let stream = ts_template! {
        async function* generate() {
            yield 1;
            yield 2;
        }

        async function process() {
            for await (const num of generate()) {
                console.log(num);
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("for await") || source.contains("for  await"), "Expected 'for await'. Got:\n{}", source);
}

// =============================================================================
// Empty Statements
// =============================================================================

#[test]
fn test_empty_stmt_standalone() {
    let stream = ts_template! {
        ;
    };
    let source = stream.source();
    // Empty statement should produce a semicolon or be trimmed
    assert!(source.contains(";") || source.trim().is_empty() || source.is_empty(), "Expected empty statement. Got:\n{}", source);
}

#[test]
fn test_empty_stmt_in_for() {
    let stream = ts_template! {
        for (;;) {
            if (shouldBreak) break;
        }
    };
    let source = stream.source();
    assert!(source.contains("for") && source.contains(";;"), "Expected infinite for loop. Got:\n{}", source);
}

#[test]
fn test_empty_stmt_after_label() {
    let stream = ts_template! {
        outer:
        for (let i = 0; i < 10; i++) {
            continue outer;
        }
    };
    let source = stream.source();
    assert!(source.contains("outer") && source.contains("continue"), "Expected labeled statement. Got:\n{}", source);
}

// =============================================================================
// Throw Statements
// =============================================================================

#[test]
fn test_throw_stmt_error() {
    let stream = ts_template! {
        throw new Error("Something went wrong");
    };
    let source = stream.source();
    assert!(source.contains("throw"), "Expected 'throw'. Got:\n{}", source);
    assert!(source.contains("new Error") || source.contains("new  Error"), "Expected 'new Error'. Got:\n{}", source);
}

#[test]
fn test_throw_stmt_custom_error() {
    let stream = ts_template! {
        throw new ValidationError("Invalid input", { field: "email" });
    };
    let source = stream.source();
    assert!(source.contains("throw"), "Expected 'throw'. Got:\n{}", source);
    assert!(source.contains("ValidationError"), "Expected 'ValidationError'. Got:\n{}", source);
}

#[test]
fn test_throw_stmt_type_error() {
    let stream = ts_template! {
        throw new TypeError("Expected a string");
    };
    let source = stream.source();
    assert!(source.contains("throw") && source.contains("TypeError"), "Expected throw TypeError. Got:\n{}", source);
}

#[test]
fn test_throw_stmt_range_error() {
    let stream = ts_template! {
        throw new RangeError("Value out of range");
    };
    let source = stream.source();
    assert!(source.contains("throw") && source.contains("RangeError"), "Expected throw RangeError. Got:\n{}", source);
}

#[test]
fn test_throw_stmt_string() {
    let stream = ts_template! {
        throw "An error occurred";
    };
    let source = stream.source();
    assert!(source.contains("throw"), "Expected 'throw'. Got:\n{}", source);
}

#[test]
fn test_throw_stmt_in_function() {
    let stream = ts_template! {
        function validate(value: unknown): asserts value is string {
            if (typeof value !== "string") {
                throw new Error("Expected string");
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("throw"), "Expected 'throw'. Got:\n{}", source);
}

#[test]
fn test_throw_stmt_rethrow() {
    let stream = ts_template! {
        try {
            riskyOperation();
        } catch (e) {
            console.error(e);
            throw e;
        }
    };
    let source = stream.source();
    assert!(source.contains("throw e") || source.contains("throw  e"), "Expected rethrow. Got:\n{}", source);
}

#[test]
fn test_throw_stmt_conditional() {
    let stream = ts_template! {
        function assertNotNull<T>(value: T | null): T {
            if (value === null) {
                throw new Error("Value cannot be null");
            }
            return value;
        }
    };
    let source = stream.source();
    assert!(source.contains("throw") && source.contains("null"), "Expected conditional throw. Got:\n{}", source);
}

// =============================================================================
// Combined Statement Tests
// =============================================================================

#[test]
fn test_for_in_with_throw() {
    let stream = ts_template! {
        for (const key in required) {
            if (!(key in obj)) {
                throw new Error("Missing required key: " + key);
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("for") && source.contains("in required"), "Expected for-in. Got:\n{}", source);
    assert!(source.contains("throw"), "Expected throw. Got:\n{}", source);
}

#[test]
fn test_for_of_with_throw() {
    let stream = ts_template! {
        for (const item of items) {
            if (!isValid(item)) {
                throw new Error("Invalid item");
            }
            process(item);
        }
    };
    let source = stream.source();
    assert!(source.contains("for") && source.contains("of items"), "Expected for-of. Got:\n{}", source);
    assert!(source.contains("throw"), "Expected throw. Got:\n{}", source);
}

#[test]
fn test_nested_for_loops() {
    let stream = ts_template! {
        for (const row of matrix) {
            for (const cell of row) {
                console.log(cell);
            }
        }
    };
    let source = stream.source();
    let for_count = source.matches("for").count();
    assert!(for_count >= 2, "Expected at least 2 'for'. Got {} in:\n{}", for_count, source);
}
