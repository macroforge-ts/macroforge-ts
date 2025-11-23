declare module 'module' {
  interface NodeRequireFunction {
    (id: string): any
    resolve(id: string): string
  }

  export function createRequire(path: string | URL): NodeRequireFunction
}
