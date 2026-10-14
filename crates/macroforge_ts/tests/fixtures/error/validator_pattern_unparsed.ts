/** A pattern that is no regular expression is refused where it is written. */
/** @derive(Decode) */
export interface Coded {
    /** @endec(pattern("(")) */
    open: string;
    /** @endec({ validate: ["pattern(\"[a-\")"] }) */
    range: string;
    /** @endec(pattern("^[A-Z]{3}/\\d+$")) */
    fine: string;
}
