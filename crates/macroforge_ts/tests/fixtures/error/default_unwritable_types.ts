/** A type no default can be written for is refused, not given a placeholder. */
/** @derive(Default) */
export interface Holder {
    keys: keyof Elsewhere;
    seeded: typeof seed;
    mode: Elsewhere['mode'];
    self: this;
    inline: import('./elsewhere').Named;
    both: Named & Dated;
    id: string & { readonly __brand: 'Id' };
    left: Exclude<'a' | 'b', 'a'>;
    result: ReturnType<() => string>;
    flags: { [K in 'a' | 'b']: boolean };
    mark: unique symbol;
    none: never;
    at: URL;
    pair: [string, Promise<this>];
    held: { at: URL };
}

/** @derive(Decode) */
export interface Filled {
    /** @endec({ default: true }) */
    at: URL;
}
