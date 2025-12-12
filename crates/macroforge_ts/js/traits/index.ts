/**
 * Trait interfaces for macroforge derive macros.
 * All members are readonly and use function-style signatures (for namespace compatibility).
 */

// Clone trait - deep copy capability
export interface Clone<T> {
  readonly clone: (self: T) => T;
}

// Debug trait - string representation for debugging
export interface Debug<T> {
  readonly toString: (self: T) => string;
}

// Default trait - factory for default values
export interface Default<T> {
  readonly defaultValue: () => T;
}

// PartialEq trait - equality comparison
export interface PartialEq<T> {
  readonly equals: (self: T, other: unknown) => boolean;
}

// Hash trait - hash code generation
export interface Hash<T> {
  readonly hashCode: (self: T) => number;
}

// PartialOrd trait - partial ordering (may return null for incomparable values)
export interface PartialOrd<T> {
  readonly compareTo: (self: T, other: unknown) => number | null;
}

// Ord trait - total ordering (always returns a number)
export interface Ord<T> {
  readonly compareTo: (self: T, other: T) => number;
}

// Serialize trait - convert to JSON
export interface Serialize<T> {
  readonly toStringifiedJSON: (self: T) => string;
  readonly toObject: (self: T) => Record<string, unknown>;
}

// Deserialize trait - construct from JSON or object
export interface Deserialize<T> {
  readonly fromStringifiedJSON: (json: string) => T;
  readonly fromObject: (obj: unknown) => T;
}
