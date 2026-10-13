/**
 * Directives whose arguments hold what a scan for brackets, commas and
 * option names could trip on, derives spread over several directives, and
 * fields a decoder never reads.
 */

/** @derive(Debug, Default, Encode, Decode) */
export interface DirectiveTexts {
    /** @endec({ rename: "skip" }) @debug({ rename: "flatten)" }) */
    renamed: string;
    /** @endec({ default: "\"(\"" }) @default(":(") */
    open: string;
    /** @endec({ default: "\"a, b]\"" }) @default("a, b]") */
    listed: string;
    /** @endec({ validate: ["pattern(\"^[(a-z, ]+$\")"] }) */
    matched: string;
    /** @endec({ default: "[\"skip\", \"flatten\"]" }) */
    words: string[];
}

/** @derive(Debug) */
/** @derive(Clone, PartialEq) */
export interface DirectiveStacked {
    name: string;
}

/**
 * A point, whose derives are written on two lines.
 * @derive(Debug, Clone)
 * @derive(PartialEq, Clone)
 */
export interface DirectiveRepeated {
    x: number;
}

/** @derive(Encode, Decode) */
export interface DirectiveSkipped {
    name: string;
    /** @endec({ skip: true }) */
    secret: string;
    /** @endec({ skipDecoding: true }) */
    count: number;
    /** @endec({ skip: true }) */
    tags: string[];
    /** @endec({ skip: true, default: "\"none\"" }) */
    note: string;
    /** @endec({ skip: true }) */
    optional?: string;
    /** @endec({ skipDecoding: true }) */
    maybe: string | null;
}

/** @derive(Encode, Decode) */
export class DirectiveSkippedClass {
    name: string = '';
    /** @endec({ skip: true }) */
    cache: Map<string, number> = new Map([['seed', 1]]);
    /** @endec({ skipDecoding: true }) */
    derived: number = 7;
}

/** @derive(Encode, Decode) */
export interface DirectiveInner {
    depth: number;
}

/** @derive(Encode, Decode) */
export interface DirectiveFlat {
    title: string;
    /** @endec(flatten) */
    inner: DirectiveInner | null;
}
