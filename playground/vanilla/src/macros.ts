/**
 * TypeScript macros module
 * These functions are replaced at compile time by the Rust plugin
 */

/**
 * Include the contents of a file as a string at compile time
 * @param path - Relative path to the file to include
 * @returns The file contents as a string
 */
export function IncludeStr(_path: string): string {
  throw new Error('IncludeStr should be replaced at compile time by the Rust macro plugin');
}

/**
 * Decorator that auto-generates methods for a class
 * @param features - Features to derive (e.g., 'Debug', 'JSON')
 */
export function Derive<T extends new (...args: any[]) => any>(..._features: ('Debug' | 'JSON' | 'JsonNative')[]):
  (target: T) => T & (new (...args: ConstructorParameters<T>) => InstanceType<T> & Debug & JSON) {
  return function(_target: T) {
    // This should never execute - it should be replaced at compile time
    throw new Error('Derive decorator should be replaced at compile time by the Rust macro plugin');
  } as any;
}

/**
 * Interface for Debug feature - provides toString() method
 */
export interface Debug {
  toString(): string;
}

/**
 * Interface for JSON feature - provides toJSON() method
 */
export interface JSON {
  toJSON(): object;
}
