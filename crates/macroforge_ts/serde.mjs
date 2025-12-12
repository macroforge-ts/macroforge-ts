/**
 * Serde runtime helpers for macroforge Serialize/Deserialize macros.
 */

// ============================================================================
// Serialization Context
// ============================================================================

export const SerializeContext = {
  create() {
    const ids = new WeakMap();
    let nextId = 0;
    return {
      getId: (obj) => ids.get(obj),
      register: (obj) => {
        const id = nextId++;
        ids.set(obj, id);
        return id;
      },
    };
  },
};

// ============================================================================
// Deserialization Context
// ============================================================================

export const DeserializeContext = {
  create() {
    const registry = new Map();
    const patches = [];
    const toFreeze = [];
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
  },
};

// ============================================================================
// Pending Reference Marker
// ============================================================================

export const PendingRef = {
  create(id) {
    return { __pendingRef: true, id };
  },
  is(value) {
    return (
      value !== null &&
      typeof value === "object" &&
      value.__pendingRef === true &&
      typeof value.id === "number"
    );
  },
};

// ============================================================================
// Structured Error for Deserialization
// ============================================================================

/** Error class that carries structured field errors */
export class DeserializeError extends Error {
  constructor(errors) {
    const message = errors.map((e) => `${e.field}: ${e.message}`).join("; ");
    super(message);
    this.name = "DeserializeError";
    this.errors = errors;
  }
}
