/**
 * TypeScript definitions for Rust-powered macros
 * These functions are replaced at compile time by the Rust plugin
 */

/**
 * Decorator that auto-generates methods for a class
 * @param features - Features to derive (e.g., 'Debug', 'JSON')
 */
export declare function Derive(...features: ('Debug' | 'JSON')[]): ClassDecorator

export interface DebugDecoratorOptions {
  rename?: string
  skip?: boolean
}

/**
 * Field-level overrides for the Debug derive
 */
export declare function Debug(options?: DebugDecoratorOptions): PropertyDecorator

/**
 * Interface extensions for generated methods
 * These are exported for the TypeScript plugin to use when augmenting classes
 */
export interface Debug {
  toString(): string
}

export interface JSON {
  toJSON(): object
}
