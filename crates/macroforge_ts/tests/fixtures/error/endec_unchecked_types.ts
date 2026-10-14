import type { Named, Dated } from './elsewhere';

/** A type no coder reads is refused, wherever in a field's type it stands. */
/** @derive(Encode, Decode) */
export interface Holder {
    pending: Promise<string>;
    both: Named & Dated;
    widened: Named & { extra: number };
    self: this;
    inline: import('./elsewhere').Named;
    weak: WeakMap<object, string>;
    boxed: String;
    anything: Object;
    failed: TypeError;
    left: Exclude<'a' | 'b', 'a'>;
    result: ReturnType<() => string>;
    shout: Uppercase<'a'>;
    either: { a: string } | { b: number };
    queue: Promise<string>[];
    byKey: Record<string, Promise<number>>;
    refs: Set<WeakRef<object>>;
}

/** A branded primitive, `unknown` and a type that only shares a utility type's name are read. */
/** @derive(Encode, Decode) */
export interface Read {
    id: string & { readonly __brand: 'Id' };
    carried: unknown;
    parameters: Parameters;
    at: URL;
}
