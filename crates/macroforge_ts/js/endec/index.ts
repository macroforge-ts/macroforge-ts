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
   */
  register(id: number, instance: any): void;

  /**
   * Gets an object by ID, or returns a `PendingRef` if not yet available.
   * @param refId - The ID from the `__ref` field
   * @returns The object if already registered, or a `PendingRef` marker
   */
  getOrDefer(refId: number): any;

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
    const registry = new Map<number, any>();
    const patches: Array<
      { target: any; prop: string | number; refId: number }
    > = [];
    const toFreeze: object[] = [];
    const errors: FieldError[] = [];
    const scopes: string[] = [];

    return {
      register: (id, instance) => {
        registry.set(id, instance);
      },

      getOrDefer: (refId) => {
        if (registry.has(refId)) {
          return registry.get(refId);
        }
        return PendingRef.create(refId);
      },

      assignOrDefer: (target, prop, value) => {
        if (PendingRef.is(value)) {
          target[prop] = null;
          patches.push({ target, prop, refId: value.id });
        } else {
          target[prop] = value;
        }
      },

      addPatch: (target, prop, refId) => {
        patches.push({ target, prop, refId });
      },

      trackForFreeze: (obj) => {
        toFreeze.push(obj);
      },

      applyPatches: () => {
        for (const { target, prop, refId } of patches) {
          if (!registry.has(refId)) {
            throw new Error(`Unresolved reference: __ref ${refId}`);
          }
          target[prop] = registry.get(refId);
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
        const prefix = scopes.length > 0 ? scopes.join(".") : "";
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
}

/**
 * Factory and type guard functions for `PendingRef`.
 */
export namespace PendingRef {
  /**
   * Creates a new pending reference marker.
   * @param id - The ID of the referenced object
   * @returns A `PendingRef` marker
   */
  export function create(id: number): PendingRef {
    return { __pendingRef: true, id };
  }

  /**
   * Type guard to check if a value is a `PendingRef`.
   * @param value - The value to check
   * @returns `true` if the value is a `PendingRef`
   */
  export function is(value: any): value is PendingRef {
    return (
      value !== null &&
      typeof value === "object" &&
      value.__pendingRef === true &&
      typeof value.id === "number"
    );
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
