/**
 * Serde runtime helpers for macroforge Serialize/Deserialize macros.
 * These are used by the generated code to handle cycles, forward references,
 * and polymorphic deserialization.
 */
export interface SerializeContext {
    /** Get the ID for an already-registered object, or undefined if not seen */
    getId(obj: object): number | undefined;
    /** Register an object and return its assigned ID */
    register(obj: object): number;
}
export declare namespace SerializeContext {
    function create(): SerializeContext;
}
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
export declare namespace DeserializeContext {
    function create(): DeserializeContext;
}
/** Marker interface for forward references that need patching */
export interface PendingRef {
    readonly __pendingRef: true;
    readonly id: number;
}
export declare namespace PendingRef {
    function create(id: number): PendingRef;
    function is(value: any): value is PendingRef;
}
export interface DeserializeOptions {
    /** If true, freeze all deserialized objects after patching */
    freeze?: boolean;
}
/** Structured field error for validation failures */
export interface FieldError {
    field: string;
    message: string;
}
/** Error class that carries structured field errors */
export declare class DeserializeError extends Error {
    readonly errors: FieldError[];
    constructor(errors: FieldError[]);
}
