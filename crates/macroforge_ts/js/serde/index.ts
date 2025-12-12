/**
 * Serde runtime helpers for macroforge Serialize/Deserialize macros.
 * These are used by the generated code to handle cycles, forward references,
 * and polymorphic deserialization.
 */

// ============================================================================
// Serialization Context
// ============================================================================

export interface SerializeContext {
  /** Get the ID for an already-registered object, or undefined if not seen */
  getId(obj: object): number | undefined;
  /** Register an object and return its assigned ID */
  register(obj: object): number;
}

export namespace SerializeContext {
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

export interface DeserializeContext {
  /** Register an object with a known ID */
  register(id: number, instance: any): void;
  /** Get an object by ID, or return a PendingRef if not yet available */
  getOrDefer(refId: number): any;
  /** Assign a value to a property, deferring if it's a PendingRef */
  assignOrDefer(target: any, prop: string | number, value: any): void;
  /** Manually add a patch for later resolution */
  addPatch(target: any, prop: string | number, refId: number): void;
  /** Track an object for optional freezing */
  trackForFreeze(obj: object): void;
  /** Apply all deferred patches (call after deserialization is complete) */
  applyPatches(): void;
  /** Freeze all tracked objects (call after applyPatches if immutability is desired) */
  freezeAll(): void;
}

export namespace DeserializeContext {
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

/** Marker interface for forward references that need patching */
export interface PendingRef {
  readonly __pendingRef: true;
  readonly id: number;
}

export namespace PendingRef {
  export function create(id: number): PendingRef {
    return { __pendingRef: true, id };
  }

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

export interface DeserializeOptions {
  /** If true, freeze all deserialized objects after patching */
  freeze?: boolean;
}

// ============================================================================
// Structured Error for Deserialization
// ============================================================================

/** Structured field error for validation failures */
export interface FieldError {
  field: string;
  message: string;
}

/** Error class that carries structured field errors */
export class DeserializeError extends Error {
  public readonly errors: FieldError[];

  constructor(errors: FieldError[]) {
    const message = errors.map((e) => `${e.field}: ${e.message}`).join("; ");
    super(message);
    this.name = "DeserializeError";
    this.errors = errors;
  }
}
