/**
 * Module augmentation for Node.js `module` package.
 *
 * @remarks
 * This declaration file provides TypeScript type definitions for the `createRequire`
 * function from Node.js's built-in `module` package. The augmentation is necessary
 * because:
 *
 * 1. **ESM/CJS Interoperability**: In ESM (ES Modules) contexts, `require` is not
 *    available by default. The `createRequire` function allows creating a `require`
 *    function that can load CommonJS modules from an ESM context.
 *
 * 2. **Native Module Loading**: The Macroforge Rust binary (loaded via N-API) and
 *    other native dependencies may require `require()` to be available for proper
 *    module resolution.
 *
 * 3. **Type Safety**: Provides proper TypeScript types for the dynamically created
 *    `require` function, including the `resolve` method for path resolution.
 *
 * @see {@link https://nodejs.org/api/module.html#modulecreaterequirefilename | Node.js createRequire documentation}
 *
 * @packageDocumentation
 */

declare module 'module' {
  /**
   * Interface representing a Node.js-compatible require function.
   *
   * @remarks
   * This interface describes the shape of the function returned by `createRequire`.
   * It can be called directly to load modules, and includes a `resolve` method
   * for resolving module paths without loading.
   */
  interface NodeRequireFunction {
    /**
     * Loads and returns the exports of a module.
     *
     * @param id - The module identifier (path or package name)
     * @returns The module's exports
     */
    (id: string): any

    /**
     * Resolves the path to a module without loading it.
     *
     * @param id - The module identifier to resolve
     * @returns The absolute path to the module
     */
    resolve(id: string): string
  }

  /**
   * Creates a `require` function that can be used to load CommonJS modules.
   *
   * @remarks
   * This is particularly useful in ES modules where `require` is not available.
   * The created function uses the specified path as its resolution base.
   *
   * @param path - The file path or URL to use as the resolution base
   * @returns A require function scoped to the given path
   *
   * @example
   * ```typescript
   * import { createRequire } from 'module';
   * const require = createRequire(import.meta.url);
   * const nativeModule = require('some-native-module');
   * ```
   */
  export function createRequire(path: string | URL): NodeRequireFunction
}
