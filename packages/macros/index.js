const importMetaEnv =
  typeof import.meta !== 'undefined' && import.meta && import.meta.env ? import.meta.env : undefined

const isDev =
  (typeof process !== 'undefined' &&
    typeof process.env !== 'undefined' &&
    process.env.NODE_ENV !== 'production') ||
  (importMetaEnv && (importMetaEnv.DEV || importMetaEnv.MODE !== 'production'))

function warn(message) {
  if (!isDev || typeof console === 'undefined') {
    return
  }
  console.warn(message)
}

export function Derive(...features) {
  warn(
    `[ts-macros] Derive fallback executed. Features: ${
      features.length ? features.map((f) => (typeof f === 'string' ? f : '<callable>')).join(', ') : '(none)'
    }`
  )
  return function deriveDecorator(target) {
    return target
  }
}

export function Debug(options) {
  const rename = typeof options?.rename === 'string' ? options.rename : undefined
  const skip = options?.skip === true
  warn(
    `[ts-macros] Debug decorator fallback executed${
      skip ? ' (skip)' : ''
    }${rename ? `, rename -> ${rename}` : ''}. Check macro wiring.`
  )
  return function debugDecorator() {
    return
  }
}
