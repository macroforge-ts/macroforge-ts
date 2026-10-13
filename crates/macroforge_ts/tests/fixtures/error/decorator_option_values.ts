/** @derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq) */
export interface Values {
    /** @debug({ rename: 5 }) */
    numbered: string;
    /** @debug({ rename: "" }) */
    unnamed: string;
    /** @debug({ skip: "yes" }) */
    worded: string;
    /** @hash({ skip: 1 }) */
    counted: string;
    /** @partialEq({ skip: null }) */
    emptied: string;
    /** @ord({ skip: "false" }) */
    quoted: string;
    /** @debug("shown") */
    lone: string;
    /** @debug({ skip: false, rename: "skip" }) */
    kept: string;
    /** @debug({ rename: "a" }) @debug({ rename: "b" }) */
    twice: string;
    /** @hash({ skip: true, skip: false }) */
    both: string;
}
