/** A default that is no expression is reported in the parser's own words. */
/** @derive(Default) */
export interface Settings {
    /** @default(1 +) */
    retries: number;
}

/** @derive(Default) */
export class Options {
    /** @default(new) */
    level: number;
}

/** @derive(Default) */
export type Limits = {
    /** @default(a b) */
    upper: number;
};
