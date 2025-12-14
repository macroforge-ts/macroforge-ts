/**
 * # Macroforge Serde Module
 *
 * This module provides runtime helpers for the `Serialize` and `Deserialize` macros.
 * It handles complex serialization scenarios including:
 *
 * - **Cycle Detection**: Objects are assigned unique `__id` values during serialization.
 *   When the same object is encountered again, a `{ "__ref": id }` marker is emitted
 *   instead of re-serializing the object.
 *
 * - **Forward References**: During deserialization, references to objects that haven't
 *   been created yet are tracked as `PendingRef` markers. After all objects are
 *   instantiated, `applyPatches()` resolves these references.
 *
 * - **Validation Errors**: The `DeserializeError` class collects structured field-level
 *   errors that can be displayed to users.
 *
 * ## Serialization Flow
 *
 * ```typescript
 * // Generated code creates a context
 * const ctx = SerializeContext.create();
 *
 * // Each object gets registered with a unique ID
 * const id = ctx.register(obj);
 *
 * // Before serializing, check if already seen
 * const existingId = ctx.getId(obj);
 * if (existingId !== undefined) {
 *   return { __ref: existingId };  // Return reference marker
 * }
 * ```
 *
 * ## Deserialization Flow
 *
 * ```typescript
 * // Generated code creates a context
 * const ctx = DeserializeContext.create();
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
 * @module macroforge/serde
 */

// ============================================================================
// Serialization Context
// ============================================================================

/**
 * Context for tracking objects during serialization.
 *
 * The context assigns unique IDs to objects as they're serialized,
 * enabling cycle detection. When an object is encountered that has
 * already been serialized, a `{ "__ref": id }` marker is emitted instead.
 */
export interface SerializeContext {
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
 * Factory functions for creating serialization contexts.
 */
export namespace SerializeContext {
  /**
   * Creates a new serialization context.
   *
   * The context uses a `WeakMap` to track objects, so objects can be
   * garbage collected if no other references exist.
   *
   * @returns A new `SerializeContext` instance
   *
   * @example
   * ```typescript
   * const ctx = SerializeContext.create();
   * const id = ctx.register(myObject);
   * // Later...
   * if (ctx.getId(myObject) !== undefined) {
   *   // Object was already serialized, emit reference
   * }
   * ```
   */
  export function create(): SerializeContext {
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
// Deserialization Context
// ============================================================================

/**
 * Context for tracking objects and resolving references during deserialization.
 *
 * The context maintains a registry of objects by their `__id` values and
 * collects "patches" for forward references that need to be resolved after
 * all objects are created.
 *
 * ## Forward Reference Resolution
 *
 * When deserializing circular or forward references:
 *
 * 1. Object A references Object B (which hasn't been created yet)
 * 2. A `PendingRef` marker is stored temporarily
 * 3. Object B is created and registered with its ID
 * 4. `applyPatches()` resolves A's reference to point to B
 */
export interface DeserializeContext {
  /**
   * Registers an object with a known ID.
   * @param id - The object's ID from the `__id` field
   * @param instance - The deserialized object instance
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
   * Tracks an object for optional freezing after deserialization.
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
}

/**
 * Factory functions for creating deserialization contexts.
 */
export namespace DeserializeContext {
  /**
   * Creates a new deserialization context.
   *
   * The context maintains:
   * - A registry mapping IDs to deserialized objects
   * - A list of patches for forward references
   * - A list of objects to freeze (if immutability is enabled)
   *
   * @returns A new `DeserializeContext` instance
   *
   * @example
   * ```typescript
   * const ctx = DeserializeContext.create();
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
  export function create(): DeserializeContext {
    const registry = new Map<number, any>();
    const patches: Array<{ target: any; prop: string | number; refId: number }> = [];
    const toFreeze: object[] = [];

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
    };
  }
}

// ============================================================================
// Pending Reference Marker
// ============================================================================

/**
 * Marker interface for forward references that need patching.
 *
 * When deserializing a `{ "__ref": id }` marker for an object that hasn't
 * been created yet, a `PendingRef` is stored temporarily. After all objects
 * are created, `DeserializeContext.applyPatches()` resolves these markers.
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
 * Options for configuring deserialization behavior.
 */
export interface DeserializeOptions {
  /**
   * If `true`, all deserialized objects are frozen after patching.
   * This provides immutability guarantees but prevents modification.
   * @default false
   */
  freeze?: boolean;
}

// ============================================================================
// Structured Error for Deserialization
// ============================================================================

/**
 * Structured error for a single field validation failure.
 *
 * Used by the `Deserialize` macro to collect validation errors
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
 * Thrown by deserialization methods when validation fails. Contains
 * an array of `FieldError` objects that can be displayed to users
 * or used for form validation feedback.
 *
 * @example
 * ```typescript
 * try {
 *   const user = User.fromStringifiedJSON(json);
 * } catch (e) {
 *   if (e instanceof DeserializeError) {
 *     for (const { field, message } of e.errors) {
 *       console.error(`${field}: ${message}`);
 *     }
 *   }
 * }
 * ```
 */
export class DeserializeError extends Error {
  /**
   * Array of field-level validation errors.
   */
  public readonly errors: FieldError[];

  /**
   * Creates a new deserialization error.
   * @param errors - Array of field validation errors
   */
  constructor(errors: FieldError[]) {
    const message = errors.map((e) => `${e.field}: ${e.message}`).join("; ");
    super(message);
    this.name = "DeserializeError";
    this.errors = errors;
  }
}
