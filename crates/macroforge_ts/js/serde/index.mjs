// js/serde/index.ts
var SerializeContext;
((SerializeContext) => {
  function create() {
    const ids = new WeakMap;
    let nextId = 0;
    return {
      getId: (obj) => ids.get(obj),
      register: (obj) => {
        const id = nextId++;
        ids.set(obj, id);
        return id;
      }
    };
  }
  SerializeContext.create = create;
})(SerializeContext ||= {});
var DeserializeContext;
((DeserializeContext) => {
  function create() {
    const registry = new Map;
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
      deferPatch: (refId, setter) => {
        patches.push({ refId, setter });
      },
      trackForFreeze: (obj) => {
        toFreeze.push(obj);
      },
      applyPatches: () => {
        for (const { refId, setter } of patches) {
          if (!registry.has(refId)) {
            throw new Error(`Unresolved reference: __ref ${refId}`);
          }
          setter(registry.get(refId));
        }
      },
      freezeAll: () => {
        for (const obj of toFreeze) {
          Object.freeze(obj);
        }
      }
    };
  }
  DeserializeContext.create = create;
})(DeserializeContext ||= {});
var PendingRef;
((PendingRef) => {
  function create(id) {
    return { __pendingRef: true, id };
  }
  PendingRef.create = create;
  function is(value) {
    return value !== null && typeof value === "object" && value.__pendingRef === true && typeof value.id === "number";
  }
  PendingRef.is = is;
})(PendingRef ||= {});

class DeserializeError extends Error {
  errors;
  constructor(errors) {
    const message = errors.map((e) => `${e.field}: ${e.message}`).join("; ");
    super(message);
    this.name = "DeserializeError";
    this.errors = errors;
  }
}
export {
  SerializeContext,
  PendingRef,
  DeserializeError,
  DeserializeContext
};
