/**
 * Parameter properties that carry decorators: each is read as the field's,
 * as one above a property is.
 */

/** @derive(Default, Encode, Decode) */
export class ClassTagged {
    constructor(
        /** @default(9) */ public count: number = 7,
        /** @endec({ rename: "label" }) */
        public name: string = 'unnamed',
        readonly plain: boolean = false
    ) {}
}
