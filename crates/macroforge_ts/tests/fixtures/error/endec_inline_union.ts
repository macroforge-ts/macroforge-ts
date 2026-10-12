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
}
