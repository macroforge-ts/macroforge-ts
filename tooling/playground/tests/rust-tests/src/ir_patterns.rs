//! IR Variant Coverage Tests - Patterns and Properties
//!
//! Tests for: ObjectPat, ObjectPatProp, ArrayPat, MethodProp, GetterProp,
//! SetterProp, IndexSignature

use macroforge_ts_quote::ts_template;

// =============================================================================
// Object Patterns (Destructuring)
// =============================================================================

#[test]
fn test_object_pat_simple() {
    let stream = ts_template! {
        const { name, age } = user;
    };
    let source = stream.source();
    assert!(source.contains("{") && source.contains("}"), "Expected object pattern. Got:\n{}", source);
    assert!(source.contains("name") && source.contains("age"), "Expected 'name' and 'age'. Got:\n{}", source);
}

#[test]
fn test_object_pat_with_alias() {
    let stream = ts_template! {
        const { firstName: first, lastName: last } = person;
    };
    let source = stream.source();
    assert!(source.contains("firstName") && source.contains("first"), "Expected alias pattern. Got:\n{}", source);
}

#[test]
fn test_object_pat_with_default() {
    let stream = ts_template! {
        const { name = "anonymous", age = 0 } = config;
    };
    let source = stream.source();
    assert!(source.contains("=") && source.contains("anonymous"), "Expected default value. Got:\n{}", source);
}

#[test]
fn test_object_pat_nested() {
    let stream = ts_template! {
        const { user: { name, email } } = response;
    };
    let source = stream.source();
    assert!(source.contains("user") && source.contains("name") && source.contains("email"), "Expected nested pattern. Got:\n{}", source);
}

#[test]
fn test_object_pat_with_rest() {
    let stream = ts_template! {
        const { id, ...rest } = data;
    };
    let source = stream.source();
    assert!(source.contains("...rest") || source.contains("... rest"), "Expected '...rest'. Got:\n{}", source);
}

#[test]
fn test_object_pat_in_function_param() {
    let stream = ts_template! {
        function process({ name, value }: Options) {
            return name + value;
        }
    };
    let source = stream.source();
    assert!(source.contains("{") && source.contains("name") && source.contains("value"), "Expected destructured param. Got:\n{}", source);
}

#[test]
fn test_object_pat_with_type_annotation() {
    let stream = ts_template! {
        const { x, y }: Point = getPoint();
    };
    let source = stream.source();
    assert!(source.contains("Point"), "Expected type annotation. Got:\n{}", source);
}

// =============================================================================
// Array Patterns (Destructuring)
// =============================================================================

#[test]
fn test_array_pat_simple() {
    let stream = ts_template! {
        const [first, second] = array;
    };
    let source = stream.source();
    assert!(source.contains("[") && source.contains("]"), "Expected array pattern. Got:\n{}", source);
    assert!(source.contains("first") && source.contains("second"), "Expected 'first' and 'second'. Got:\n{}", source);
}

#[test]
fn test_array_pat_with_skip() {
    let stream = ts_template! {
        const [first, , third] = array;
    };
    let source = stream.source();
    assert!(source.contains("first") && source.contains("third"), "Expected skipped element. Got:\n{}", source);
}

#[test]
fn test_array_pat_with_rest() {
    let stream = ts_template! {
        const [head, ...tail] = list;
    };
    let source = stream.source();
    assert!(source.contains("...tail") || source.contains("... tail"), "Expected '...tail'. Got:\n{}", source);
}

#[test]
fn test_array_pat_with_default() {
    let stream = ts_template! {
        const [first = 0, second = 0] = values;
    };
    let source = stream.source();
    assert!(source.contains("= 0"), "Expected default values. Got:\n{}", source);
}

#[test]
fn test_array_pat_nested() {
    let stream = ts_template! {
        const [[a, b], [c, d]] = matrix;
    };
    let source = stream.source();
    assert!(source.contains("[[") || source.contains("[ ["), "Expected nested array. Got:\n{}", source);
}

#[test]
fn test_array_pat_mixed() {
    let stream = ts_template! {
        const [name, { age, city }] = tuple;
    };
    let source = stream.source();
    assert!(source.contains("name") && source.contains("age") && source.contains("city"), "Expected mixed destructuring. Got:\n{}", source);
}

#[test]
fn test_array_pat_in_for_of() {
    let stream = ts_template! {
        for (const [key, value] of entries) {
            console.log(key, value);
        }
    };
    let source = stream.source();
    assert!(source.contains("[key, value]") || source.contains("[ key , value ]") || (source.contains("key") && source.contains("value")), "Expected destructuring in for-of. Got:\n{}", source);
}

// =============================================================================
// Method Properties
// =============================================================================

#[test]
fn test_method_prop_simple() {
    let stream = ts_template! {
        const obj = {
            greet() {
                return "hello";
            }
        };
    };
    let source = stream.source();
    assert!(source.contains("greet()") || source.contains("greet ()"), "Expected method 'greet()'. Got:\n{}", source);
}

#[test]
fn test_method_prop_with_params() {
    let stream = ts_template! {
        const calculator = {
            add(a: number, b: number): number {
                return a + b;
            }
        };
    };
    let source = stream.source();
    assert!(source.contains("add") && source.contains("number"), "Expected typed method. Got:\n{}", source);
}

#[test]
fn test_method_prop_async() {
    let stream = ts_template! {
        const api = {
            async fetchData() {
                return await fetch(url);
            }
        };
    };
    let source = stream.source();
    assert!(source.contains("async") && source.contains("fetchData"), "Expected 'async fetchData'. Got:\n{}", source);
}

#[test]
fn test_method_prop_generator() {
    let stream = ts_template! {
        const obj = {
            *items() {
                yield 1;
                yield 2;
            }
        };
    };
    let source = stream.source();
    assert!(source.contains("*items") || source.contains("* items") || source.contains("*\n"), "Expected generator method. Got:\n{}", source);
}

#[test]
fn test_method_prop_computed_name() {
    let stream = ts_template! {
        const key = "dynamicMethod";
        const obj = {
            [key]() {
                return "dynamic";
            }
        };
    };
    let source = stream.source();
    assert!(source.contains("[key]") || source.contains("[ key ]"), "Expected computed method name. Got:\n{}", source);
}

// =============================================================================
// Getter Properties
// =============================================================================

#[test]
fn test_getter_prop_simple() {
    let stream = ts_template! {
        const obj = {
            _value: 0,
            get value() {
                return this._value;
            }
        };
    };
    let source = stream.source();
    assert!(source.contains("get value") || source.contains("get  value"), "Expected 'get value'. Got:\n{}", source);
}

#[test]
fn test_getter_prop_with_type() {
    let stream = ts_template! {
        const obj = {
            get count(): number {
                return this._count;
            }
        };
    };
    let source = stream.source();
    assert!(source.contains("get count") || source.contains("get  count"), "Expected getter with return type. Got:\n{}", source);
}

#[test]
fn test_getter_prop_in_class() {
    let stream = ts_template! {
        class Person {
            private _name: string;

            get name(): string {
                return this._name;
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("get name") || source.contains("get  name"), "Expected class getter. Got:\n{}", source);
}

// =============================================================================
// Setter Properties
// =============================================================================

#[test]
fn test_setter_prop_simple() {
    let stream = ts_template! {
        const obj = {
            _value: 0,
            set value(v: number) {
                this._value = v;
            }
        };
    };
    let source = stream.source();
    assert!(source.contains("set value") || source.contains("set  value"), "Expected 'set value'. Got:\n{}", source);
}

#[test]
fn test_setter_prop_in_class() {
    let stream = ts_template! {
        class Person {
            private _age: number;

            set age(value: number) {
                if (value >= 0) {
                    this._age = value;
                }
            }
        }
    };
    let source = stream.source();
    assert!(source.contains("set age") || source.contains("set  age"), "Expected class setter. Got:\n{}", source);
}

#[test]
fn test_getter_setter_pair() {
    let stream = ts_template! {
        const reactive = {
            _data: null,
            get data() {
                return this._data;
            },
            set data(value) {
                this._data = value;
            }
        };
    };
    let source = stream.source();
    assert!(source.contains("get data") || source.contains("get  data"), "Expected getter. Got:\n{}", source);
    assert!(source.contains("set data") || source.contains("set  data"), "Expected setter. Got:\n{}", source);
}

// =============================================================================
// Index Signatures
// =============================================================================

#[test]
fn test_index_signature_string_key() {
    let stream = ts_template! {
        interface Dictionary {
            [key: string]: unknown;
        }
    };
    let source = stream.source();
    assert!(source.contains("[key:") || source.contains("[ key :") || source.contains("[key :"), "Expected index signature. Got:\n{}", source);
}

#[test]
fn test_index_signature_number_key() {
    let stream = ts_template! {
        interface StringArray {
            [index: number]: string;
        }
    };
    let source = stream.source();
    assert!(source.contains("[index:") || source.contains("[ index :") || source.contains("[index :"), "Expected numeric index. Got:\n{}", source);
}

#[test]
fn test_index_signature_readonly() {
    let stream = ts_template! {
        interface ReadonlyMap {
            readonly [key: string]: number;
        }
    };
    let source = stream.source();
    assert!(source.contains("readonly"), "Expected 'readonly'. Got:\n{}", source);
}

#[test]
fn test_index_signature_with_properties() {
    let stream = ts_template! {
        interface NamedDictionary {
            name: string;
            [key: string]: string | number;
        }
    };
    let source = stream.source();
    assert!(source.contains("name:") || source.contains("name :"), "Expected named property. Got:\n{}", source);
    assert!(source.contains("[key") || source.contains("[ key"), "Expected index signature. Got:\n{}", source);
}

#[test]
fn test_index_signature_in_type() {
    let stream = ts_template! {
        type Indexable = {
            [k: string]: any;
            length: number;
        };
    };
    let source = stream.source();
    assert!(source.contains("[k") || source.contains("[ k"), "Expected index signature in type. Got:\n{}", source);
}

#[test]
fn test_index_signature_symbol_key() {
    let stream = ts_template! {
        interface SymbolIndexable {
            [key: symbol]: string;
        }
    };
    let source = stream.source();
    assert!(source.contains("symbol"), "Expected symbol key type. Got:\n{}", source);
}
