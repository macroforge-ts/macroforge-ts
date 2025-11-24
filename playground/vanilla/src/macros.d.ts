/**
 * TypeScript definitions for Rust-powered macros
 * These functions are replaced at compile time by the Rust plugin
 */

/**
 * Include the contents of a file as a string at compile time
 * @param path - Relative path to the file to include
 * @returns The file contents as a string
 */
export declare function IncludeStr(path: string): string

/**
 * Decorator that auto-generates methods for a class
 * @param features - Features to derive (e.g., 'Debug', 'JSON')
 */
export declare function Derive(...features: ('Debug' | 'JSON')[]): ClassDecorator

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