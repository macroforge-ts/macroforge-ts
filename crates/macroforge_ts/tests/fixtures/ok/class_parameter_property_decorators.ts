/** A parameter property takes the decorators written above it, like a field. */
/** @derive(Default, Encode, Decode) */
class Tagged {
    constructor(
        /** @default(9) */ public count: number = 7,
        /** @endec({ rename: "label" }) */
        public name: string = 'unnamed',
        /** @endec({ skip: true }) */ readonly cache: number = 0,
        readonly plain: boolean = false
    ) {}
}
