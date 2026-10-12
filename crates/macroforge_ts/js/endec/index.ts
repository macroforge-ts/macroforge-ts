/**
 * # Macroforge Endec Module
 *
 * This module provides runtime helpers for the `Encode` and `Decode` macros.
 * It handles complex encoding scenarios including:
 *
 * - **Cycle Detection**: Objects are assigned unique `__id` values during encoding.
 *   When the same object is encountered again, a `{ "__ref": id }` marker is emitted
 *   instead of re-encoding the object.
 *
 * - **Forward References**: During decoding, references to objects that haven't
 *   been created yet are tracked as `PendingRef` markers. After all objects are
 *   instantiated, `applyPatches()` resolves these references.
 *
 * - **Validation Errors**: The `DecodeError` class collects structured field-level
 *   errors that can be displayed to users.
 *
 * ## Encoding Flow
 *
 * ```typescript
 * // Generated code creates a context
 * const ctx = EncodeContext.create();
 *
 * // Each object gets registered with a unique ID
 * const id = ctx.register(obj);
 *
 * // Before encoding, check if already seen
 * const existingId = ctx.getId(obj);
 * if (existingId !== undefined) {
 *   return { __ref: existingId };  // Return reference marker
 * }
 * ```
 *
 * ## Decoding Flow
 *
 * ```typescript
 * // Generated code creates a context
 * const ctx = DecodeContext.create();
 *
 * // Register objects as they're created
 * ctx.register(id, instance);
 *
 * // References are resolved immediately if available, or deferred
 * const value = ctx.getOrDefer(refId);
 *
 * // After all objects are created, resolve pending references
 * ctx.applyPatches();
 * ```
 *
 * @module @macroforge/core/endec
 */

// ============================================================================
// Encoding Context
// ============================================================================

/**
 * Context for tracking objects during encoding.
 *
 * The context assigns unique IDs to objects as they're encoded,
 * enabling cycle detection. When an object is encountered that has
 * already been encoded, a `{ "__ref": id }` marker is emitted instead.
 */
export interface EncodeContext {
  /**
   * Gets the ID for an already-registered object.
   * @param obj - The object to look up
   * @returns The object's ID, or `undefined` if not yet registered
   */
  getId(obj: object): number | undefined;

  /**
   * Registers an object and assigns it a unique ID.
   * @param obj - The object to register
   * @returns The newly assigned ID
   */
  register(obj: object): number;
}

/**
 * Factory functions for creating encoding contexts.
 */
export namespace EncodeContext {
  /**
   * Creates a new encoding context.
   *
   * The context uses a `WeakMap` to track objects, so objects can be
   * garbage collected if no other references exist.
   *
   * @returns A new `EncodeContext` instance
   *
   * @example
   * ```typescript
   * const ctx = EncodeContext.create();
   * const id = ctx.register(myObject);
   * // Later...
   * if (ctx.getId(myObject) !== undefined) {
   *   // Object was already encoded, emit reference
   * }
   * ```
   */
  export function create(): EncodeContext {
    const ids = new WeakMap<object, number>();
    let nextId = 0;
    return {
      getId: (obj) => ids.get(obj),
      register: (obj) => {
        const id = nextId++;
        ids.set(obj, id);
        return id;
      },
    };
  }
}

// ============================================================================
// Decoding Context
// ============================================================================

/**
 * Context for tracking objects and resolving references during decoding.
 *
 * The context maintains a registry of objects by their `__id` values and
 * collects "patches" for forward references that need to be resolved after
 * all objects are created.
 *
 * ## Forward Reference Resolution
 *
 * When decoding circular or forward references:
 *
 * 1. Object A references Object B (which hasn't been created yet)
 * 2. A `PendingRef` marker is stored temporarily
 * 3. Object B is created and registered with its ID
 * 4. `applyPatches()` resolves A's reference to point to B
 */
export interface DecodeContext {
  /**
   * Registers an object with a known ID.
   * @param id - The object's ID from the `__id` field
   * @param instance - The decoded object instance
   * @param kind - The type the object was decoded as, which a reference to
   *   it has to expect
   */
  register(id: number, instance: any, kind?: string): void;

  /**
   * Gets an object by ID, or returns a `PendingRef` if not yet available.
   * @param refId - The ID from the `__ref` field
   * @param kind - The type the reference stands for
   * @returns The object if already registered, or a `PendingRef` marker
   * @throws DecodeError if the object was registered as another type: a
   *   reference is input, and may not stand an object in for one of a type
   *   it was never checked against
   */
  getOrDefer(refId: number, kind?: string): any;

  /**
   * Assigns a value to a property, deferring if it's a `PendingRef`.
   * If the value is a `PendingRef`, the assignment is recorded as a patch
   * to be applied later.
   * @param target - The object to assign to
   * @param prop - The property name or index
   * @param value - The value to assign (may be a `PendingRef`)
   */
  assignOrDefer(target: any, prop: string | number, value: any): void;

  /**
   * Manually adds a patch for later resolution.
   * @param target - The object containing the reference
   * @param prop - The property name or index
   * @param refId - The ID of the referenced object
   */
  addPatch(target: any, prop: string | number, refId: number): void;

  /**
   * Hands the object a reference names to `settle` once it is known, for a
   * place a property assignment cannot reach, such as a `Set` or a `Map`.
   * @param refId - The ID of the referenced object
   * @param settle - Receives the object when patches are applied
   * @param kind - The type the reference stands for
   */
  defer(refId: number, settle: (value: any) => void, kind?: string): void;

  /**
   * Tracks an object for optional freezing after decoding.
   * @param obj - The object to track
   */
  trackForFreeze(obj: object): void;

  /**
   * Applies all deferred patches, resolving forward references.
   * Call this after all objects have been created.
   * @throws Error if any referenced ID is not in the registry
   */
  applyPatches(): void;

  /**
   * Freezes all tracked objects for immutability.
   * Call this after `applyPatches()` if immutable objects are desired.
   */
  freezeAll(): void;

  /**
   * Pushes a field name onto the scope stack.
   * Nested decoders call this before processing so that any errors
   * they push are automatically prefixed with the full path.
   * @param name - The field name to push (e.g., "colors")
   */
  pushScope(name: string): void;

  /**
   * Pops the last field name from the scope stack.
   * Must be called after the nested decoder returns.
   */
  popScope(): void;

  /**
   * Pushes validation errors onto the context.
   * Called by `decodeWithContext` instead of throwing.
   * Errors are automatically prefixed with the current scope stack.
   * @param errors - Array of field validation errors to accumulate
   */
  pushErrors(errors: FieldError[]): void;

  /**
   * Returns all accumulated validation errors.
   * Called by the wrapper `decode` function after processing.
   * @returns Array of all accumulated field errors
   */
  getErrors(): FieldError[];
}

/**
 * Factory functions for creating decoding contexts.
 */
export namespace DecodeContext {
  /**
   * Creates a new decoding context.
   *
   * The context maintains:
   * - A registry mapping IDs to decoded objects
   * - A list of patches for forward references
   * - A list of objects to freeze (if immutability is enabled)
   *
   * @returns A new `DecodeContext` instance
   *
   * @example
   * ```typescript
   * const ctx = DecodeContext.create();
   *
   * // Register objects as they're created
   * ctx.register(1, user);
   * ctx.register(2, friend);
   *
   * // Resolve forward references
   * ctx.applyPatches();
   *
   * // Optionally freeze for immutability
   * ctx.freezeAll();
   * ```
   */
  export function create(): DecodeContext {
    const registry = new Map<number, { instance: any; kind: string | undefined }>();
    const patches: Array<
      { refId: number; settle: (value: any) => void; kind: string | undefined }
    > = [];
    // The object a reference of `kind` to `refId` stands for, once it is
    // registered. The two kinds have to agree where both are known.
    const referenced = (refId: number, kind: string | undefined): any => {
      const entry = registry.get(refId);
      if (entry === undefined) {
        throw new Error(`Unresolved reference: __ref ${refId}`);
      }
      if (kind !== undefined && entry.kind !== undefined && entry.kind !== kind) {
        throw new DecodeError([{
          field: "_root",
          message: `__ref ${refId} refers to a ${entry.kind}, where a ${kind} is expected`,
        }]);
      }
      return entry.instance;
    };
    const toFreeze: object[] = [];
    const errors: FieldError[] = [];
    const scopes: string[] = [];

    return {
      register: (id, instance, kind) => {
        registry.set(id, { instance, kind });
      },

      getOrDefer: (refId, kind) => {
        if (registry.has(refId)) {
          return referenced(refId, kind);
        }
        return PendingRef.create(refId, kind);
      },

      assignOrDefer: (target, prop, value) => {
        if (PendingRef.is(value)) {
          target[prop] = null;
          patches.push({
            refId: value.id,
            settle: (resolved) => {
              target[prop] = resolved;
            },
            kind: value.kind,
          });
        } else {
          target[prop] = value;
        }
      },

      addPatch: (target, prop, refId) => {
        patches.push({
          refId,
          settle: (resolved) => {
            target[prop] = resolved;
          },
          kind: undefined,
        });
      },

      defer: (refId, settle, kind) => {
        patches.push({ refId, settle, kind });
      },

      trackForFreeze: (obj) => {
        toFreeze.push(obj);
      },

      applyPatches: () => {
        for (const { refId, settle, kind } of patches) {
          settle(referenced(refId, kind));
        }
      },

      freezeAll: () => {
        for (const obj of toFreeze) {
          Object.freeze(obj);
        }
      },

      pushScope: (name) => {
        scopes.push(name);
      },

      popScope: () => {
        scopes.pop();
      },

      pushErrors: (errs) => {
        // An index follows its collection directly: `tags[0]`, not `tags.[0]`.
        // A value decoded as the root of its own type is `_root` there, and
        // has no name of its own inside another value.
        const prefix = scopes.reduce(
          (path, scope) => {
            if (path === "") return scope;
            if (scope === "_root") return path;
            return scope.startsWith("[") ? path + scope : `${path}.${scope}`;
          },
          "",
        );
        for (const e of errs) {
          const field = e.field === "_root"
            ? (prefix || "_root")
            : (prefix ? prefix + "." + e.field : e.field);
          errors.push({ field, message: e.message });
        }
      },

      getErrors: () => errors,
    };
  }
}

// ============================================================================
// Pending Reference Marker
// ============================================================================

/**
 * Marker interface for forward references that need patching.
 *
 * When decoding a `{ "__ref": id }` marker for an object that hasn't
 * been created yet, a `PendingRef` is stored temporarily. After all objects
 * are created, `DecodeContext.applyPatches()` resolves these markers.
 */
export interface PendingRef {
  /** Discriminant field to identify pending references */
  readonly __pendingRef: true;
  /** The ID of the referenced object */
  readonly id: number;
  /** The type the reference stands for, when its decoder says */
  readonly kind?: string | undefined;
}

// The markers `PendingRef.create` made. A decoded value is input: one that
// merely has a marker's shape must not pass for one.
const pendingRefs = new WeakSet<object>();

/**
 * Factory and type guard functions for `PendingRef`.
 */
export namespace PendingRef {
  /**
   * Creates a new pending reference marker.
   * @param id - The ID of the referenced object
   * @param kind - The type the reference stands for
   * @returns A `PendingRef` marker
   */
  export function create(id: number, kind?: string): PendingRef {
    const marker: PendingRef = { __pendingRef: true, id, kind };
    pendingRefs.add(marker);
    return marker;
  }

  /**
   * Type guard to check if a value is a `PendingRef`: a marker `create`
   * made, not a value that looks like one.
   * @param value - The value to check
   * @returns `true` if the value is a `PendingRef`
   */
  export function is(value: unknown): value is PendingRef {
    return typeof value === "object" && value !== null && pendingRefs.has(value);
  }
}

// ============================================================================
// Options for fromStringifiedJSON
// ============================================================================

/**
 * Options for configuring decoding behavior.
 */
export interface DecodeOptions {
  /**
   * If `true`, all decoded objects are frozen after patching.
   * This provides immutability guarantees but prevents modification.
   * @default false
   */
  freeze?: boolean;
}

// ============================================================================
// Structured Error for Decoding
// ============================================================================

/**
 * Structured error for a single field validation failure.
 *
 * Used by the `Decode` macro to collect validation errors
 * in a format suitable for display to users.
 */
export interface FieldError {
  /**
   * The field path that failed validation.
   * For nested fields, uses dot notation (e.g., `"address.street"`).
   */
  field: string;

  /**
   * Human-readable error message describing the validation failure.
   * @example "must be a valid email"
   * @example "must have at least 3 characters"
   */
  message: string;
}

/**
 * Error class that carries structured field validation errors.
 *
 * Used internally by generated `decodeWithContext` implementations.
 * The generated top-level `decode` functions do not throw it: they
 * return `{ success: false, errors }` with the collected `FieldError`
 * objects instead, suitable for display or form validation feedback.
 *
 * @example
 * ```typescript
 * const result = User.decode(json);
 * if (!result.success) {
 *   for (const { field, message } of result.errors) {
 *     console.error(`${field}: ${message}`);
 *   }
 * }
 * ```
 */
export class DecodeError extends Error {
  /**
   * Array of field-level validation errors.
   */
  public readonly errors: FieldError[];

  /**
   * Creates a new decoding error.
   * @param errors - Array of field validation errors
   */
  constructor(errors: FieldError[]) {
    const message = errors.map((e) => `${e.field}: ${e.message}`).join("; ");
    super(message);
    this.name = "DecodeError";
    this.errors = errors;
  }
}

/**
 * The value a literal default decodes to, for a field whose type validates
 * its values, such as a newtype. A literal its own type rejects is a broken
 * declaration, so it throws rather than yield a default that is invalid.
 */
export function decodedLiteral<T>(
  decoded:
    | { success: true; value: T }
    | { success: false; errors: Array<FieldError> },
): T {
  if (decoded.success) return decoded.value;
  throw new DecodeError(decoded.errors);
}

/**
 * Decodes one raw value with a context. A reference to an object not read
 * yet comes back pending.
 */
export type Decoder<Value> = (raw: unknown, ctx: DecodeContext) => Value | PendingRef;

function mismatch(expected: string): DecodeError {
  return new DecodeError([{ field: "_root", message: `expected ${expected}` }]);
}

/** Decodes a string held in a collection. */
export function decodeString(raw: unknown): string {
  if (typeof raw !== "string") throw mismatch("string");
  return raw;
}

/** Decodes a number held in a collection. */
export function decodeNumber(raw: unknown): number {
  if (typeof raw !== "number") throw mismatch("number");
  return raw;
}

/** Decodes a boolean held in a collection. */
export function decodeBoolean(raw: unknown): boolean {
  if (typeof raw !== "boolean") throw mismatch("boolean");
  return raw;
}

/** Decodes a bigint held in a collection. JSON has none, so only a parsed value carries one. */
export function decodeBigInt(raw: unknown): bigint {
  if (typeof raw !== "bigint") throw mismatch("bigint");
  return raw;
}

/** Decodes a date held in a collection: a `Date`, or the ISO text of one. */
export function decodeDate(raw: unknown): Date {
  if (raw instanceof Date) return raw;
  if (typeof raw !== "string") throw mismatch("an ISO date string");
  return new Date(raw);
}

/**
 * The number a `Map`'s JSON key spells. Only the spelling a number is
 * written out as counts, so no two keys of one object spell one number.
 */
export function decodeNumberKey(key: string): number {
  const decoded = Number(key);
  if (Number.isNaN(decoded) || String(decoded) !== key) throw mismatch("a number key");
  return decoded;
}

/**
 * A value held in a collection whose type is a literal, a primitive or a
 * union of those, checked by `accepted`, which the caller works out from the
 * type. `expected` names the type in the error.
 */
export function decodeChecked(raw: unknown, accepted: boolean, expected: string): unknown {
  if (!accepted) throw mismatch(expected);
  return raw;
}

/** The bigint a `Map`'s JSON key spells. */
export function decodeBigIntKey(key: string): bigint {
  if (!/^-?\d+$/.test(key)) throw mismatch("a bigint key");
  return BigInt(key);
}

/** The boolean a `Map`'s JSON key spells. */
export function decodeBooleanKey(key: string): boolean {
  if (key === "true") return true;
  if (key === "false") return false;
  throw mismatch("a boolean key");
}

/**
 * Runs `decode` for one member of a collection under `scope`. A member that
 * fails to decode reports its errors and is left out. Whatever a decoder
 * throws is the member's failure, as the top-level `decode` takes it.
 */
function inScope(ctx: DecodeContext, scope: string, decode: () => void): void {
  ctx.pushScope(scope);
  try {
    decode();
  } catch (error) {
    ctx.pushErrors(
      error instanceof DecodeError
        ? error.errors
        : [{ field: "_root", message: error instanceof Error ? error.message : String(error) }],
    );
  } finally {
    ctx.popScope();
  }
}

/**
 * Hands a decoded member to `place`: at once, or when the object it refers
 * to has been read.
 */
function settle<Value>(
  ctx: DecodeContext,
  decoded: Value | PendingRef,
  place: (value: Value) => void,
): void {
  if (PendingRef.is(decoded)) ctx.defer(decoded.id, place, decoded.kind);
  else place(decoded);
}

function isRawObject(raw: unknown): raw is Record<string, unknown> {
  return typeof raw === "object" && raw !== null && !Array.isArray(raw);
}

/** Decodes an array, each element with `item`. */
export function decodeArray<Item>(
  raw: unknown,
  ctx: DecodeContext,
  item: Decoder<Item>,
): Item[] {
  if (!Array.isArray(raw)) throw mismatch("an array");
  const items: Item[] = [];
  items.length = raw.length;
  raw.forEach((element: unknown, index) => {
    inScope(ctx, `[${index}]`, () => {
      settle(ctx, item(element, ctx), (value) => {
        items[index] = value;
      });
    });
  });
  return items;
}

/** Decodes a `Set` from the array it is encoded as, each element with `item`. */
export function decodeSet<Item>(
  raw: unknown,
  ctx: DecodeContext,
  item: Decoder<Item>,
): Set<Item> {
  if (!Array.isArray(raw)) throw mismatch("an array");
  const items = new Set<Item>();
  raw.forEach((element: unknown, index) => {
    inScope(ctx, `[${index}]`, () => {
      settle(ctx, item(element, ctx), (value) => {
        items.add(value);
      });
    });
  });
  return items;
}

/**
 * Decodes a `Map` from the object it is encoded as: each key with `key`,
 * each value with `value`. An entry whose value is a reference ahead of its
 * object is added once that object has been read.
 */
export function decodeMap<Key, Value>(
  raw: unknown,
  ctx: DecodeContext,
  key: (text: string) => Key,
  value: Decoder<Value>,
): Map<Key, Value> {
  if (!isRawObject(raw)) throw mismatch("an object");
  const entries = new Map<Key, Value>();
  for (const [text, member] of Object.entries(raw)) {
    inScope(ctx, text, () => {
      const decodedKey = key(text);
      settle(ctx, value(member, ctx), (decoded) => {
        entries.set(decodedKey, decoded);
      });
    });
  }
  return entries;
}

/** Sets `key` on `target` by defining it, so a `__proto__` key stays a key. */
function defineMember<Value>(target: Record<string, Value>, key: string, value: Value): void {
  Object.defineProperty(target, key, {
    value,
    writable: true,
    enumerable: true,
    configurable: true,
  });
}

/** Decodes a `Record`, each value with `value`. */
export function decodeRecord<Value>(
  raw: unknown,
  ctx: DecodeContext,
  value: Decoder<Value>,
): Record<string, Value> {
  if (!isRawObject(raw)) throw mismatch("an object");
  const members: Record<string, Value> = {};
  for (const [key, member] of Object.entries(raw)) {
    inScope(ctx, key, () => {
      settle(ctx, value(member, ctx), (decoded) => {
        defineMember(members, key, decoded);
      });
    });
  }
  return members;
}

/**
 * Decodes an object whose type is written out member by member: each with
 * its decoder in `members`, those named in `optional` only when they are
 * there. A member the type does not declare is left behind.
 */
export function decodeObject(
  raw: unknown,
  ctx: DecodeContext,
  members: Readonly<Record<string, Decoder<unknown>>>,
  optional: ReadonlyArray<string> = [],
): Record<string, unknown> {
  if (!isRawObject(raw)) throw mismatch("an object");
  const decoded: Record<string, unknown> = {};
  for (const [key, decode] of Object.entries(members)) {
    const member = raw[key];
    inScope(ctx, key, () => {
      if (member === undefined) {
        if (optional.includes(key)) return;
        throw new DecodeError([{ field: "_root", message: "missing required field" }]);
      }
      settle(ctx, decode(member, ctx), (value) => {
        defineMember(decoded, key, value);
      });
    });
  }
  return decoded;
}

/**
 * Decodes a tuple: its leading elements with `items`, of which the first
 * `required` have to be there, and any beyond them with `rest`. The caller
 * knows which tuple type its decoders build.
 */
export function decodeTuple(
  raw: unknown,
  ctx: DecodeContext,
  items: ReadonlyArray<Decoder<unknown>>,
  required: number,
  rest?: Decoder<unknown>,
): unknown[] {
  if (!Array.isArray(raw)) throw mismatch("an array");
  if (raw.length < required || (rest === undefined && raw.length > items.length)) {
    throw mismatch(`a tuple of ${tupleLength(items.length, required, rest !== undefined)} elements`);
  }
  const elements: unknown[] = [];
  elements.length = raw.length;
  raw.forEach((element: unknown, index) => {
    const decode = items[index] ?? rest;
    if (decode === undefined) return;
    inScope(ctx, `[${index}]`, () => {
      settle(ctx, decode(element, ctx), (value) => {
        elements[index] = value;
      });
    });
  });
  return elements;
}

function tupleLength(declared: number, required: number, open: boolean): string {
  if (open) return `at least ${required}`;
  return required === declared ? `${required}` : `${required} to ${declared}`;
}

/**
 * Runs `encode` for `value`, a value holding only some of its type's fields.
 * Such a value is written out wherever it is held, so one that holds itself
 * has no end.
 * @throws {TypeError} If `value` is already being encoded.
 */
export function encodeSelection<Encoded>(
  ctx: EncodeContext,
  value: object,
  encode: () => Encoded,
): Encoded {
  const encoding = selectionsBeingEncoded.get(ctx) ?? new Set<object>();
  selectionsBeingEncoded.set(ctx, encoding);
  if (encoding.has(value)) {
    throw new TypeError(
      "a Partial, Pick or Omit of a value that holds itself has no encoding",
    );
  }
  encoding.add(value);
  try {
    return encode();
  } finally {
    encoding.delete(value);
  }
}

const selectionsBeingEncoded = new WeakMap<EncodeContext, Set<object>>();

/**
 * The static `encodeWithContext` of the class `value` is an instance of, when
 * that class derives `Encode`. Only the class's own static counts, so a
 * subclass that does not derive `Encode` is not encoded as its parent.
 */
export function derivedEncoder(
  value: unknown,
): ((value: unknown, ctx: EncodeContext) => unknown) | undefined {
  if (typeof value !== "object" || value === null) return undefined;
  const constructor: unknown = value.constructor;
  if (typeof constructor !== "function") return undefined;
  if (!Object.prototype.hasOwnProperty.call(constructor, "encodeWithContext")) {
    return undefined;
  }
  const encoder: unknown = Reflect.get(constructor, "encodeWithContext");
  if (typeof encoder !== "function") return undefined;
  return (item, ctx) => encoder(item, ctx);
}

/**
 * The fields of a type a value has to have: only those named, or every one
 * but those named. A `Pick` and an `Omit` of the type, as values.
 */
export type FieldSelection<Field extends PropertyKey = string> =
  | { readonly only: ReadonlyArray<Field> }
  | { readonly except: ReadonlyArray<Field> };

/**
 * What a selection of a type's fields means for one of them: whether a value
 * of that shape has the field at all, and whether it has to.
 */
export interface FieldRequirement {
  /** Whether the shape has the field. One it lacks is neither read nor written. */
  keeps(field: string): boolean;
  /** Whether a value has to have the field, where its type requires it. */
  requires(field: string): boolean;
}

/**
 * The requirement `fields` makes. Without `fields` every field is kept and
 * none is required, which is how a `Partial` of a type is coded.
 */
export function fieldRequirement(
  fields: FieldSelection<PropertyKey> | undefined,
): FieldRequirement {
  if (fields === undefined) return { keeps: () => true, requires: () => false };
  if ("only" in fields) {
    const only = new Set(fields.only);
    const selected = (field: string) => only.has(field);
    return { keeps: selected, requires: selected };
  }
  const except = new Set(fields.except);
  const selected = (field: string) => !except.has(field);
  return { keeps: selected, requires: selected };
}

/**
 * The JSON text of an encoded value.
 * @throws {TypeError} If the value has no JSON text, as `undefined` has none.
 */
export function encodedText(encoded: unknown): string {
  return jsonText(JSON.stringify(encoded));
}

/**
 * The JSON text of an encoded value without its `tagField` and `__id`
 * members, at every depth.
 * @throws {TypeError} If the value has no JSON text, as `undefined` has none.
 */
export function encodedTextWithoutMetadata(encoded: unknown, tagField: string): string {
  return jsonText(
    JSON.stringify(
      encoded,
      (key, member: unknown) =>
        key === tagField || key === "__id" ? undefined : replacerMember(member),
    ),
  );
}

function jsonText(text: string | undefined): string {
  if (text === undefined) {
    throw new TypeError("the encoded value has no JSON text: JSON cannot represent it");
  }
  return text;
}

/**
 * `member` as a `JSON.stringify` replacer hands it on. A function, a symbol
 * or `undefined` is left out, as JSON leaves it out.
 * @throws {TypeError} For a `bigint`, which JSON cannot represent.
 */
function replacerMember(
  member: unknown,
): string | number | boolean | null | Record<string, unknown> | unknown[] | undefined {
  if (typeof member === "string" || typeof member === "number" || typeof member === "boolean") {
    return member;
  }
  if (Array.isArray(member) || isJsonObject(member)) return member;
  if (typeof member === "bigint") throw new TypeError("Do not know how to serialize a BigInt");
  return member === null ? null : undefined;
}

function isJsonObject(member: unknown): member is Record<string, unknown> {
  return typeof member === "object" && member !== null;
}

/**
 * Whether `value` is a multiple of `divisor`, the `multipleOf(n)` validator's
 * check. Decimal operands are compared exactly, so `0.3` is a multiple of
 * `0.1`; a value that is not finite is never a multiple. Matches Effect's
 * `Schema.isMultipleOf`.
 */
export function isMultipleOf(value: number, divisor: number): boolean {
  return remainder(value, divisor) === 0;
}

/**
 * The remainder of `dividend / divisor`, signed like the dividend and exact
 * for decimal operands. `NaN` when either operand is not finite or the
 * divisor is zero. A port of Effect's `Number.remainder`.
 */
export function remainder(dividend: number, divisor: number): number {
  if (!Number.isFinite(dividend) || !Number.isFinite(divisor) || divisor === 0) {
    return NaN;
  }
  if (Number.isInteger(dividend) && Number.isInteger(divisor)) {
    return dividend % divisor;
  }
  const [dividendCoefficient, dividendExponent] = toScientificInteger(dividend);
  const [divisorCoefficient, divisorExponent] = toScientificInteger(divisor);
  const exponent = Math.min(dividendExponent, divisorExponent);
  const dividendInteger = dividendCoefficient * 10n ** BigInt(dividendExponent - exponent);
  const divisorInteger = divisorCoefficient * 10n ** BigInt(divisorExponent - exponent);
  const out = dividendInteger % divisorInteger;
  if (out === 0n) {
    return dividend < 0 || Object.is(dividend, -0) ? -0 : 0;
  }
  const rest = Number(`${out}e${exponent}`);
  return rest === 0 ? Math.sign(dividend) * Number.MIN_VALUE : rest;
}

/** `n` as an integer coefficient and a power of ten: `n = coefficient * 10^exponent`. */
function toScientificInteger(n: number): readonly [bigint, number] {
  if (Number.isInteger(n)) {
    return [BigInt(n), 0];
  }
  const scientific = Math.abs(n).toExponential();
  const eIndex = scientific.indexOf("e");
  const digits = scientific.slice(0, eIndex).replace(".", "");
  const coefficient = BigInt(digits) * (n < 0 ? -1n : 1n);
  return [coefficient, Number(scientific.slice(eIndex + 1)) - digits.length + 1];
}
