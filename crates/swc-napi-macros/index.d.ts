export interface TransformResult {
  code: string
  map?: string
}

export function transformSync(code: string, filepath: string): TransformResult