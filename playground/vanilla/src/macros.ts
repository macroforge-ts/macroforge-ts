/**
 * TypeScript macros module
 * These functions are replaced at compile time by the Rust plugin
 */

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

export interface DebugDecoratorOptions {
  rename?: string;
  skip?: boolean;
}

export function Debug(_options?: DebugDecoratorOptions): PropertyDecorator {
  return function() {
    if (import.meta.env?.DEV ?? true) {
      console.warn('Debug field decorator executed at runtime. Check that the macro plugin ran.');
    }
  };
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
