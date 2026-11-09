/** @derive(Clone, Encode, Decode) */
interface Sneaky {
    '__proto__': string;
    id: string;
}

/** @derive(Debug) */
type Aliased = { __proto__: number };

/** @derive(Hash) */
class Carrier {
    ['__proto__']: string = '';
    __proto__: number = 0;
}
