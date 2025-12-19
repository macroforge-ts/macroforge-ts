// --- Utilities ---
import type * as EffectOption from 'effect/Option';
import type { Duration } from 'effect/Duration';
import type { DurationEncoded } from 'effect/Schema';

type Key = string | number;

type DurationTuple = readonly [seconds: number, nanos: number];

type EffectSchemaEncoded<T> = T extends {
    Type: unknown;
    Encoded: infer Encoded;
}
    ? Encoded
    : T extends { constructor: infer C }
      ? C extends { Encoded: infer InstanceEncoded }
          ? InstanceEncoded
          : T
      : T;
type ResolveOption<T> = T extends EffectOption.Option<infer Value> ? Value : T;
type NormalizeDuration<T> = T extends Duration ? DurationEncoded | DurationTuple : T;
type NormalizeTarget<T> = NormalizeDuration<ResolveOption<EffectSchemaEncoded<T>>>;

// ---------------- Core helpers ----------------

// ---- callable / constructable detection (lint-clean) ----
type IsCallable<T> = T extends (...args: Array<never>) => unknown ? true : false;
type IsConstructable<T> = T extends abstract new (...args: Array<never>) => unknown ? true : false;

// ---- non-function "object" without using the 'object' type ----
type HasPipe<T> =
    ResolveOption<EffectSchemaEncoded<T>> extends {
        pipe: (...args: Array<unknown>) => unknown;
    }
        ? true
        : false;

type HasSymbolKey<T> =
    Extract<keyof ResolveOption<EffectSchemaEncoded<T>>, symbol> extends never ? false : true;

type IsNonFunctionRecord<T> =
    IsCallable<T> extends true
        ? false
        : IsConstructable<T> extends true
          ? false
          : HasPipe<T> extends true
            ? false
            : HasSymbolKey<T> extends true
              ? false
              : NormalizeTarget<T> extends object
                ? keyof NormalizeTarget<T> extends never
                    ? false
                    : true
                : false;

// ---- cycle tracker ----
// IMPORTANT: use a NON-distributive check so unions don't slip through.
type SeenHas<Seen, T> = [T] extends [never] ? false : [T] extends [Seen] ? true : false;
type SeenAdd<Seen, T> = Seen | T;

type _IsTuple<P extends ReadonlyArray<unknown>> = number extends P['length'] ? false : true;
type TupleKeys<P extends ReadonlyArray<unknown>> = {
    [I in Exclude<keyof P, keyof ReadonlyArray<unknown>>]: I extends `${infer N extends number}`
        ? N
        : I extends number
          ? I
          : never;
}[Exclude<keyof P, keyof ReadonlyArray<unknown>>];
type ToReadonlyTuple<P extends ReadonlyArray<Key>> = {
    readonly [I in keyof P]: P[I];
};

type StopOnSeen<T, Seen, Fallback, Next> = SeenHas<Seen, T> extends true ? Fallback : Next;

type OnCycleKeys<T> =
    NormalizeTarget<T> extends infer Base
        ? Base extends ReadonlyArray<unknown>
            ? number
            : IsNonFunctionRecord<Base> extends true
              ? keyof Base
              : never
        : never;

// Optional: primitive helper to avoid accidental descent
type Primitive = string | number | boolean | symbol | bigint | null | undefined;

// ---- Step-narrowing helpers ----
// These types enable incremental path validation rather than generating all paths upfront

/**
 * Get the normalized base type, handling Effect Schema classes and Options
 */
type Base<T> = NormalizeTarget<T>;

/**
 * Get valid keys at the current position in a type.
 * Returns `number` for arrays, property keys for objects, `never` for primitives or cycles.
 */
type ValidKeyAt<T, Seen = never> = Base<T> extends infer B
    ? [B] extends [Seen]
        ? never // Cycle detected - stop here
        : B extends ReadonlyArray<unknown>
          ? number
          : IsNonFunctionRecord<B> extends true
            ? PropertyKeyOf<B>
            : never
    : never;

/**
 * Get the type at a specific key after normalization.
 * For arrays, returns the element type. For objects, returns the property type.
 */
type TypeAt<T, K extends Key> = Base<T> extends infer B
    ? K extends number
        ? B extends ReadonlyArray<infer E>
            ? E
            : never
        : K extends keyof B
          ? B[K]
          : never
    : never;

/**
 * Validate and narrow a path step-by-step.
 * Each segment is checked against what's actually valid at that position.
 * Returns the path if valid, `never` if invalid.
 */
type NarrowPath<T, P extends ReadonlyArray<Key>, Seen = never, D extends number = 5> = [D] extends [
    0
]
    ? P extends readonly []
        ? readonly []
        : never
    : P extends readonly []
      ? readonly []
      : P extends readonly [infer H extends Key, ...infer R extends ReadonlyArray<Key>]
        ? H extends ValidKeyAt<T, Seen>
            ? R extends readonly []
                ? readonly [H]
                : readonly [H, ...NarrowPath<TypeAt<T, H>, R, Seen | Base<T>, Dec<D>>]
            : never
        : never;

// ----------------- DeepValue -----------------
type PropertyKeyOf<T> = T extends infer O
    ? O extends object
        ? Extract<keyof O, string | number>
        : never
    : never;

// Internal alias with default depth for backwards-compatible internal usages
type DeepPathInternal<T, D extends number = 5, Seen = never> = DeepPath<T, D, Seen>;

type DeepValueInternal<
    T,
    Seen = never,
    P extends ReadonlyArray<Key> = DeepPathInternal<T>
> = NormalizeTarget<T> extends infer Base
    ? StopOnSeen<
          Base,
          Seen,
          unknown,
          P extends readonly []
              ? Base
              : P extends readonly [infer H, ...infer R]
                ? H extends number
                    ? Base extends ReadonlyArray<infer Elem>
                        ? _IsTuple<Base> extends true
                            ? H extends TupleKeys<Base> & number
                                ? R extends ReadonlyArray<Key>
                                    ? DeepValueInternal<Base[H], SeenAdd<Seen, Base>, R>
                                    : never
                                : number extends H
                                  ? {
                                        [I in TupleKeys<Base> &
                                            number]: R extends ReadonlyArray<Key>
                                            ? DeepValueInternal<Base[I], SeenAdd<Seen, Base>, R>
                                            : never;
                                    }[TupleKeys<Base> & number]
                                  : never
                            : R extends ReadonlyArray<Key>
                              ? DeepValueInternal<Elem, SeenAdd<Seen, Base>, R>
                              : never
                        : never
                    : H extends PropertyKeyOf<Base>
                      ? Extract<Base, Record<H & PropertyKey, unknown>> extends infer TH
                          ? TH extends unknown
                              ? H extends keyof TH
                                  ? R extends ReadonlyArray<Key>
                                      ? DeepValueInternal<TH[H], SeenAdd<Seen, TH>, R>
                                      : never
                                  : never
                              : never
                          : never
                      : never
                : number extends P[number]
                  ? Base extends ReadonlyArray<infer Elem>
                      ? DeepValueInternal<Elem, SeenAdd<Seen, Base>, P>
                      : never
                  : P[number] extends PropertyKeyOf<Base>
                    ? {
                          [K in P[number] & PropertyKeyOf<Base>]: Extract<
                              Base,
                              Record<K, unknown>
                          > extends infer TK
                              ? TK extends unknown
                                  ? K extends keyof TK
                                      ? DeepValueInternal<TK[K], SeenAdd<Seen, TK>, P>
                                      : never
                                  : never
                              : never;
                      }[P[number] & PropertyKeyOf<Base>]
                    : never
      >
    : never;
export type ResolveDeepPath<_T, P, _Seen = never> = P extends ReadonlyArray<Key> ? P : never;
type DeepValueResult<T, P, Seen> = DeepValueInternal<T, Seen, ResolveDeepPath<T, P, Seen>>;
type Simplify<T> = T extends object ? { [K in keyof T]: T[K] } : T;

export type DeepValue<
    T,
    P extends (DeepPath<T, D> | ResolvedDeepPath) & ReadonlyArray<Key>,
    Seen = never,
    D extends number = 5
> = DeepValueResult<T, P, Seen> extends infer Result ? Simplify<Result> : never;
// ---- DeepValueUnion ----
export type DeepValueUnion<T, Seen = never> = NormalizeTarget<T> extends infer Base
    ? SeenHas<Seen, Base> extends true
        ? never
        : Base extends ReadonlyArray<infer E>
          ? DeepValueUnion<E, SeenAdd<Seen, Base>>
          : Base extends Array<infer E>
            ? DeepValueUnion<E, SeenAdd<Seen, Base>>
            : IsNonFunctionRecord<Base> extends true
              ? {
                    [K in keyof Base]-?: DeepValueUnion<Base[K], SeenAdd<Seen, Base>>;
                }[keyof Base]
              : Base
    : never;

// ---- DeepLeafValueUnion ----
export type DeepLeafValueUnion<T, Seen = never> = NormalizeTarget<T> extends infer Base
    ? SeenHas<Seen, Base> extends true
        ? never
        : Base extends ReadonlyArray<infer E>
          ? DeepLeafValueUnion<E, SeenAdd<Seen, Base>>
          : Base extends Array<infer E>
            ? DeepLeafValueUnion<E, SeenAdd<Seen, Base>>
            : IsNonFunctionRecord<Base> extends true
              ? {
                    [K in keyof Base]-?: DeepLeafValueUnion<Base[K], SeenAdd<Seen, Base>>;
                }[keyof Base]
              : Base
    : never;

// ---- DeepKey (union-aware, cycle-safe, optional depth) ----
interface DepthMap {
    0: 0;
    1: 0;
    2: 1;
    3: 2;
    4: 3;
    5: 4;
    6: 5;
}
type Dec<N extends number> = N extends keyof DepthMap ? DepthMap[N] : 0;

export type DeepKey<T, Seen = never, D extends number = 3> = NormalizeTarget<T> extends infer Base
    ? StopOnSeen<
          Base,
          Seen,
          D extends 0
              ? OnCycleKeys<Base>
              : Base extends unknown
                ? Base extends ReadonlyArray<infer E>
                    ? number | DeepKey<E, SeenAdd<Seen, Base>, Dec<D>>
                    : IsNonFunctionRecord<Base> extends true
                      ?
                            | keyof Base
                            | {
                                  [K in keyof Base]-?: DeepKey<
                                      Base[K],
                                      SeenAdd<Seen, Base>,
                                      Dec<D>
                                  >;
                              }[keyof Base]
                      : never
                : never,
          D extends 0
              ? OnCycleKeys<Base>
              : Base extends unknown
                ? Base extends ReadonlyArray<infer E>
                    ? number | DeepKey<E, SeenAdd<Seen, Base>, Dec<D>>
                    : IsNonFunctionRecord<Base> extends true
                      ?
                            | keyof Base
                            | {
                                  [K in keyof Base]-?: DeepKey<
                                      Base[K],
                                      SeenAdd<Seen, Base>,
                                      Dec<D>
                                  >;
                              }[keyof Base]
                      : never
                : never
      >
    : never;

// ---- DeepPath (union-aware, depth-limited) ----
/**
 * Generates all valid paths for a type T up to depth D.
 * Relies purely on depth limiting rather than cycle detection to prevent infinite recursion.
 * This ensures consistent behavior regardless of type structure.
 *
 * @example
 * ```ts
 * type Path = DeepPath<Account, 4>; // Paths up to depth 4
 * ```
 *
 * Note: D (depth) must be explicitly specified.
 */
export type DeepPath<T, D extends number = 5, _Seen = never> = NormalizeTarget<T> extends infer Base
    ? D extends 0
        ? []
        : [] | DeepPathNonEmpty<Base, D>
    : never;

type DeepPathNonEmpty<T, D extends number = 5> = NormalizeTarget<T> extends infer Base
    ? Base extends unknown
        ? Base extends Primitive
            ? never
            : Base extends ReadonlyArray<infer Elem>
              ? _IsTuple<Base> extends true
                  ? {
                        [I in TupleKeys<Base> & number]-?: readonly [
                            I,
                            ...DeepPath<Base[I], Dec<D>>
                        ];
                    }[TupleKeys<Base> & number]
                  : readonly [number, ...DeepPath<Elem, Dec<D>>]
              : IsNonFunctionRecord<Base> extends true
                ? {
                      [K in PropertyKeyOf<Base>]-?: IsCallable<Base[K]> extends true
                          ? never
                          : IsConstructable<Base[K]> extends true
                            ? never
                            : readonly [K, ...DeepPath<Base[K], Dec<D>>];
                  }[PropertyKeyOf<Base>]
                : never
        : never
    : never;

type NormalizeDepth<Depth extends number, Fallback extends number> = Depth extends keyof DepthMap
    ? Depth
    : Fallback;

type PathDepth<
    P extends ReadonlyArray<unknown>,
    Fallback extends number = 5
> = number extends P['length'] ? Fallback : NormalizeDepth<P['length'], Fallback>;

/**
 * Resolves a path P to a valid DeepPath for type T.
 * Extracts the path from generated DeepPath if found, otherwise generates with appropriate depth.
 */
export type DeepPathFor<T, P extends ReadonlyArray<Key>, Fallback extends number = 3> = Extract<
    DeepPathInternal<T>,
    ToReadonlyTuple<P>
> extends never
    ? DeepPath<T, PathDepth<P, Fallback>>
    : Extract<DeepPathInternal<T>, ToReadonlyTuple<P>>;

// Export step-narrowing types for external use
export type { ValidKeyAt, TypeAt, NarrowPath };

/**
 * Type-erased versions (unchanged semantics)
 */
export type ResolvedDeepKey = DeepKey<Record<Key, unknown>>;

/**
 * A type-erased path that allows any sequence of string/number keys.
 * This is more permissive than DeepPath<T> because it doesn't enforce
 * path validity against a specific type structure.
 *
 * Used when the exact type structure is not known at compile time,
 * such as in generic traversal functions.
 */
export type ResolvedDeepPath = ReadonlyArray<Key>;

export type ResolvedDeepValue = DeepValue<Record<Key, unknown>, ResolvedDeepPath>;

// ---- DeepLeafPath (uses fixed SeenHas) ----
type IsLeaf<T, Seen = never> = NormalizeTarget<T> extends infer Base
    ? SeenHas<Seen, Base> extends true
        ? true
        : Base extends ReadonlyArray<unknown>
          ? false
          : Base extends Array<unknown>
            ? false
            : IsNonFunctionRecord<Base> extends true
              ? false
              : true
    : true;

type DeepLeafPathHelper<T, Seen = never> = NormalizeTarget<T> extends infer Base
    ? SeenHas<Seen, Base> extends true
        ? never
        : Base extends ReadonlyArray<infer E>
          ? readonly [number, ...DeepLeafPathHelper<E, SeenAdd<Seen, Base>>]
          : Base extends Array<infer E>
            ? readonly [number, ...DeepLeafPathHelper<E, SeenAdd<Seen, Base>>]
            : IsNonFunctionRecord<Base> extends true
              ? {
                    [K in keyof Base]-?: IsLeaf<Base[K], SeenAdd<Seen, Base>> extends true
                        ? readonly [K]
                        : readonly [K, ...DeepLeafPathHelper<Base[K], SeenAdd<Seen, Base>>];
                }[keyof Base]
              : readonly []
    : never;

export type DeepLeafPath<T> = (IsLeaf<T> extends true ? readonly [] : DeepLeafPathHelper<T>) &
    DeepPathInternal<T>;

// ---- DeepProp (inherits non-distributive SeenHas fix) ----
export type DeepProp<T, K extends PropertyKey, Seen = never> = NormalizeTarget<T> extends infer Base
    ? SeenHas<Seen, Base> extends true
        ? never
        : Base extends ReadonlyArray<infer E>
          ? DeepProp<E, K, SeenAdd<Seen, Base>>
          : Base extends Array<infer E>
            ? DeepProp<E, K, SeenAdd<Seen, Base>>
            : IsNonFunctionRecord<Base> extends true
              ?
                    | (K extends keyof Base ? Base[K] : never)
                    | {
                          [P in keyof Base]-?: DeepProp<Base[P], K, SeenAdd<Seen, Base>>;
                      }[keyof Base]
              : never
    : never;

// --- setAtPath (mutable) ---
// Mutates `obj` and returns it. Strongly typed path + value.

export const isObjectLike = (value: unknown): value is Record<PropertyKey, unknown> =>
    typeof value === 'object' && value !== null;

/**
 * Runtime type guard to check if a value is a valid ResolvedDeepPath for a given object.
 * Validates that the input is an array containing only strings and numbers,
 * and that the path actually exists in the provided object.
 *
 * @param obj - The object to validate the path against
 * @param value - The value to check
 * @returns True if the value is a valid path that exists in the object
 *
 * @example
 * ```ts
 * const obj = { user: { address: [{ street: 'Main St' }] } };
 * const path = ['user', 'address', 0, 'street'];
 *
 * if (isResolvedDeepPath(obj, path)) {
 *   // path is now typed as ResolvedDeepPath and guaranteed to exist in obj
 *   const value = getNestedValue(obj, path); // won't be undefined
 * }
 * ```
 */
export function isResolvedDeepPath(
    obj: object,
    value: ReadonlyArray<Key>
): value is ResolvedDeepPath {
    if (!Array.isArray(value)) {
        return false;
    }

    for (const element of value) {
        const type = typeof element;
        if (type !== 'string' && type !== 'number') {
            return false;
        }
    }

    // Validate path exists in the object
    let current: unknown = obj;

    for (const segment of value) {
        if (current === null || current === undefined) return false;

        if (typeof segment === 'number') {
            if (!Array.isArray(current)) return false;
            if (segment < 0 || segment >= current.length) return false;
            current = current[segment];
        } else {
            if (!isObjectLike(current)) return false;
            if (!hasOwn(current, segment)) return false;
            current = current[segment];
        }
    }

    return true;
}

function isIndex(k: Key): k is number {
    return typeof k === 'number' && Number.isInteger(k);
}

// Create {} or [] depending on next path segment
function makeIntermediate(nextKey: Key): Record<Key, unknown> | Array<unknown> {
    return isIndex(nextKey) ? [] : {};
}

/**
 * Sets a nested value in an object using an array of keys (path).
 * Creates intermediate objects as needed and provides detailed error messages.
 *
 * @template T - The type of the object being modified
 * @param {T} obj - The object to modify
 * @param {ResolvedDeepPath} path - A branded array of strings/numbers representing the keys to traverse
 * @param {unknown} value - The value to set at the specified path
 * @returns {T} The original object (modified in place)
 * @throws {Error} If inputs are invalid or traversal fails
 */
export function setNestedValue<T extends object>(
    obj: T,
    path: ResolvedDeepPath,
    value: unknown
): T {
    if (path.length === 0) {
        throw new Error('Path cannot be empty');
    }

    // Cursor is either a record or an array as we traverse.
    // We start by viewing `obj` through a safe union.
    let cursor: Record<Key, unknown> | Array<unknown> = Array.isArray(obj)
        ? (obj as Array<unknown>)
        : (obj as unknown as Record<Key, unknown>);

    const lastIdx = path.length - 1;

    // Traverse to parent container
    for (let i = 0; i < lastIdx; i++) {
        const key = path[i];
        const nextKey = path[i + 1];

        if (key && Array.isArray(cursor)) {
            if (!isIndex(key)) {
                throw new Error(
                    `Cannot use string key '${key}' on array at path index ${String(i)}`
                );
            }

            if (nextKey === undefined) {
                continue;
            }

            // Ensure element exists
            const at = (() => {
                if (key) {
                    return cursor[key];
                } else {
                    throw new Error(`No key`);
                }
            })();
            if (key && !isObjectLike(at)) {
                // create the appropriate intermediate container
                const created = makeIntermediate(nextKey);
                // write back
                cursor[key] = created;
                cursor = created;
            } else {
                // at is object or array
                cursor = Array.isArray(at) ? at : (at as Record<Key, unknown>);
            }
        } else if (key && isObjectLike(cursor)) {
            // Plain object record
            if (nextKey === undefined) {
                continue;
            }

            const rec = cursor as Record<Key, unknown>;
            const recordKey = key;
            const at = rec[recordKey];
            if (!isObjectLike(at)) {
                const created = makeIntermediate(nextKey);
                rec[recordKey] = created;
                cursor = created;
            } else {
                cursor = Array.isArray(at)
                    ? (at as Array<unknown>)
                    : (at as unknown as Record<Key, unknown>);
            }
        }
    }

    // Set final segment
    const finalKey = path[lastIdx];

    if (finalKey && Array.isArray(cursor)) {
        if (!isIndex(finalKey)) {
            throw new Error(
                `Cannot use string key '${finalKey}' on array at final path index ${String(lastIdx)}`
            );
        }

        const arr = cursor;
        // Expand once if needed
        if (finalKey >= arr.length) {
            arr.length = finalKey + 1;
        }
        arr[finalKey] = value;
    } else if (finalKey && isObjectLike(cursor)) {
        cursor[finalKey] = value;
    }

    return obj;
}

/**
 * Gets a nested value from an object using an array of keys (path).
 *
 * @template T - The type of the object being read
 * @template P - The path type (must be a ResolvedDeepPath)
 * @param {T} obj - The object to read from
 * @param {P} path - A branded array of strings/numbers representing the keys to traverse
 * @returns {DeepValue<T, P>} The value at the path, or undefined if not found
 * @throws {TypeError} If path is not an array
 */
export function getNestedValue<T extends object, D extends number, const P extends DeepPath<T, D>>(
    obj: T,
    path: P
): DeepValue<T, P> | undefined {
    if (!Array.isArray(path)) {
        throw new TypeError(`Invalid argument: 'path' must be an array. Received: ${typeof path}`);
    }
    if (path.length === 0) {
        return obj as DeepValue<T, P>;
    }

    let cur: unknown = obj;

    for (const segment of path) {
        if (cur === null || cur === undefined) return undefined;

        if (typeof segment === 'number') {
            if (!Array.isArray(cur)) return undefined;
            const arr = cur;
            if (segment < 0 || segment >= arr.length) return undefined;
            cur = arr[segment];
        } else {
            if (!isObjectLike(cur)) return undefined;
            const rec = cur;
            if (!hasOwn(rec, segment)) return undefined;
            cur = rec[segment];
        }
    }

    return cur as DeepValue<T, P> | undefined;
}

function hasOwn(obj: Record<PropertyKey, unknown>, k: PropertyKey): boolean {
    return Object.hasOwn(obj, k);
}

/**
 * Runtime type guard to check if a value is a valid DeepValue for a given object and path.
 * Validates that the value's type matches what exists at the path in the object.
 *
 * @param obj - The object containing the path
 * @param path - The path to validate against
 * @param value - The value to check
 * @returns True if the value is type-compatible with what's at the path
 *
 * @example
 * ```ts
 * const obj = { user: { name: 'John', age: 30 } };
 * const path = ['user', 'age'] as const;
 * const newValue: unknown = 25;
 *
 * if (isResolvedDeepValue(obj, path, newValue)) {
 *   // newValue is typed as DeepValue<typeof obj, typeof path> (number)
 *   setNestedValue(obj, path, newValue);
 * }
 * ```
 */
export function isResolvedDeepValue<T extends object, const P extends ResolvedDeepPath>(
    obj: T,
    path: P,
    value: unknown
): value is DeepValue<T, P> {
    if (!isResolvedDeepPath(obj, path)) {
        // Path doesn't exist, can't validate value type
        return false;
    }

    const existingValue = getNestedValue(obj, path as unknown as DeepPathInternal<T>);

    // If path exists but value is undefined, allow any value (path might be optional)
    if (existingValue === undefined) {
        return true;
    }

    const existingType = typeof existingValue;
    const newType = typeof value;

    // Check primitive type compatibility
    if (existingType !== 'object' && newType !== 'object') {
        return existingType === newType;
    }

    // Both are objects/arrays
    if (existingType === 'object' && newType === 'object') {
        // Check array compatibility
        if (Array.isArray(existingValue) && Array.isArray(value)) {
            return true;
        }
        // Check both are objects (not arrays or null)
        if (
            !Array.isArray(existingValue) &&
            !Array.isArray(value) &&
            existingValue !== null &&
            value !== null
        ) {
            return true;
        }
    }

    return false;
}

/**
 * Returns all keys at any depth in an object, similar to Object.keys but for nested structures.
 * Traverses arrays and objects recursively, collecting all property keys and array indices.
 *
 * @param obj - The object to extract keys from
 * @returns Array of all keys (strings and numbers) found at any depth
 *
 * @example
 * ```ts
 * const obj = { user: { name: 'John', age: 30 }, items: [1, 2] };
 * deepKeys(obj);
 * // Returns: ['user', 'name', 'age', 'items', 0, 1]
 * ```
 */
export function deepKeys(obj: object): Array<ResolvedDeepKey> {
    const keys: Array<ResolvedDeepKey> = [];
    const seen = new Set<object>();

    function traverse(current: unknown): void {
        // Avoid circular references
        if (isObjectLike(current)) {
            if (seen.has(current)) {
                return;
            }
            seen.add(current);
        }

        const isLeaf = !isObjectLike(current) && !Array.isArray(current);

        // Don't traverse further if it's a leaf
        if (isLeaf) {
            return;
        }

        // Handle arrays
        if (Array.isArray(current)) {
            for (const [i, element] of current.entries()) {
                keys.push(i);
                traverse(element);
            }
        }
        // Handle objects
        else if (isObjectLike(current)) {
            for (const key in current) {
                if (hasOwn(current, key)) {
                    keys.push(key);
                    traverse(current[key]);
                }
            }
        }
    }

    traverse(obj);
    return keys;
}
