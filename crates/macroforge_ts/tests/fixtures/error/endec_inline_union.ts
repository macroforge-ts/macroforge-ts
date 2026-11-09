/** @derive(Encode, Decode) */
interface Circle {
    radius: number;
}

/** @derive(Encode, Decode) */
interface Square {
    side: number;
}

/** @derive(Encode, Decode) */
interface Canvas {
    shape: Circle | Square;
    shapes: (Circle | Square)[];
    maybe: Circle | Square | null;
    when: Date | number;
    seen: Set<string> | string[];
    count: bigint | number[];
    tally: bigint | string;
    exact: 1n | 'one';
    held: string | (number | undefined)[];
    paired: string | [number | undefined];
    nested: Circle | null | (null | true)[];
    grouped: (Circle | null) | (null | true)[];
}
