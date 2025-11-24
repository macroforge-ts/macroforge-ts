/**
 * Type-friendly placeholders for the Rust-powered macros.
 * The Vite plugin replaces every usage at build time, so these should never run.
 */

export type DeriveFeature = 'Debug' | 'JSON'

export interface MacroDebug {
	toString(): string
}

export interface MacroJSON {
	toJSON(): Record<string, unknown>
}

type MacroDeriveConstructor<T extends new (...args: any[]) => any> = new (
	...args: ConstructorParameters<T>
) => InstanceType<T> & MacroDebug & MacroJSON

export function IncludeStr(_path: string): string {
	if (import.meta.env.DEV) {
		console.warn('[ts-macros] IncludeStr fallback executed. Check that the NAPI plugin is enabled.')
	}
	return ''
}

export function Derive(..._features: DeriveFeature[]) {
	return function <T extends new (...args: any[]) => any>(target: T): MacroDeriveConstructor<T> {
		if (import.meta.env.DEV) {
			console.warn('[ts-macros] Derive fallback executed. Check that the NAPI plugin is enabled.')
		}
		return target as unknown as MacroDeriveConstructor<T>
	}
}
