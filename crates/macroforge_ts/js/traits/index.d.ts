/**
 * Trait interfaces for macroforge derive macros.
 * All members are readonly and use function-style signatures (for namespace compatibility).
 */
export interface Clone<T> {
    readonly clone: (self: T) => T;
}
export interface Debug<T> {
    readonly toString: (self: T) => string;
}
export interface Default<T> {
    readonly defaultValue: () => T;
}
export interface PartialEq<T> {
    readonly equals: (self: T, other: unknown) => boolean;
}
export interface Hash<T> {
    readonly hashCode: (self: T) => number;
}
export interface PartialOrd<T> {
    readonly compareTo: (self: T, other: unknown) => number | null;
}
export interface Ord<T> {
    readonly compareTo: (self: T, other: T) => number;
}
export interface Serialize<T> {
    readonly toStringifiedJSON: (self: T) => string;
    readonly toObject: (self: T) => Record<string, unknown>;
}
export interface Deserialize<T> {
    readonly fromStringifiedJSON: (json: string) => T;
    readonly fromObject: (obj: unknown) => T;
}
