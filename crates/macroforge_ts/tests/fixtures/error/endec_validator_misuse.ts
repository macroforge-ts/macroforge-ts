/** @derive(Decode) */
export interface Misused {
    /** @endec(email) */
    count: number;
    /** @endec(positive) */
    name: string;
    /** @endec(positive) */
    scores: number[];
    /** @endec(email) */
    when: Date;
    /** @endec(nonEmpty) */
    flag: boolean;
    /** @endec(minLength(1)) */
    byName: Record<string, string>;
    /** @endec(emial) */
    address: string;
    /** @endec({ renme: "n" }) */
    renamed: string;
    /** @endec(minLength(2)) */
    maybe: string | null;
    /** @endec(nonEmpty) */
    tags: string[];
    /** @endec(nonEmpty) */
    code: `c-${number}`;
    /** @endec(positive) */
    later?: number;
    /** @endec(email) */
    named: Address;
}

/** @derive(Decode) */
/** @endec(email) */
export type Meters = number;
