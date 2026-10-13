/** @derive(Default) */
export interface Chain {
    next: Chain;
}

/** @derive(Default) */
export interface Left {
    right: Right;
}

/** @derive(Default) */
export interface Right {
    left: Left;
}

/** @derive(Default) */
export class Boxed {
    inner: [Boxed, number];
    kept: Boxed | null = null;
}

/** @derive(Default) */
export interface Fine {
    next: Fine | null;
    all: Fine[];
    maybe?: Fine;
    /** @default(null) */
    given: Fine | null;
}
