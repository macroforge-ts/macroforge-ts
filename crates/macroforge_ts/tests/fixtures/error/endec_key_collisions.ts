/** @derive(Encode, Decode) */
interface Renamed {
    /** @endec({ rename: "name" }) */
    title: string;
    name: string;
}

/** @derive(Encode, Decode) */
/** @endec({ renameAll: "snake_case" }) */
interface Cased {
    userName: string;
    user_name: string;
}

/** @derive(Encode, Decode) */
interface Inner {
    id: string;
    note: string;
}

/** @derive(Encode, Decode) */
interface Flat {
    id: number;
    /** @endec(flatten) */
    inner: Inner;
}
