/** @derive(Encode, Decode) */
interface Holder {
    callback: (value: number) => void;
    mark: symbol;
    keys: keyof Holder;
    mode: Config['mode'];
    flags: { [K in 'a' | 'b']: boolean };
    listeners: Array<() => void>;
    describe(): string;
    /** @endec({ skip: true }) */
    skipped: () => void;
    carried: unknown;
    version: `v-${Version}`;
    pair: `${number}${number}`;
    known: `v-${1 | 2}-${'a' | 'b'}`;
}

/** @derive(Encode, Decode) */
type Handler = (value: number) => string;

/** @derive(Decode) */
type Picked<T> = T extends string ? { text: T } : { other: T };

/** @derive(Default) */
interface Patterned {
    id: `id-${Kind}`;
    described(): string;
}
