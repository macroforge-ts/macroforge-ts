/** @derive(Encode, Decode) */
interface Marked {
    __id: number;
    __type: string;
    name: string;
}

/** @derive(Encode, Decode) */
class Linked {
    /** @endec({ rename: "__ref" }) */
    link: string = '';
}

/** @derive(Encode, Decode) */
/** @endec({ tag: "kind" }) */
interface Tagged {
    kind: string;
    /** @endec({ rename: "label" }) */
    __type: string;
}
