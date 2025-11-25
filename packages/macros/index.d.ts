export interface MacroDebug {
  toString(): string;
}

export interface MacroJSON {
  toJSON(): Record<string, unknown>;
}

export type MacroDeriveConstructor<T extends new (...args: any[]) => any> = new (
  ...args: ConstructorParameters<T>
) => InstanceType<T> & MacroDebug & MacroJSON;

export function Derive(
  ...features: Array<string | ((...args: any[]) => unknown)>
): <T extends new (...args: any[]) => any>(target: T) => MacroDeriveConstructor<T>;

export interface DebugDecoratorOptions {
  rename?: string;
  skip?: boolean;
}

export function Debug(options?: DebugDecoratorOptions): PropertyDecorator;
