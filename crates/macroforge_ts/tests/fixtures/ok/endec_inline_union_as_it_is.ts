/** @derive(Encode, Decode) */
interface Circle {
    radius: number;
}

/** @derive(Encode, Decode) */
type Shape = Circle | 'none';

/** @derive(Encode, Decode) */
type Slot<T> = T | Circle;

/** @derive(Encode, Decode) */
interface Canvas {
    when: Date | string;
    tags: string | string[];
    size: number | 'auto';
    shape: Shape;
    slot: Slot<Date>;
    link: string | Circle;
    callback: () => Circle | null;
    byDay: { [key: string]: Date };
    counts: { [key: string]: number } | null;
}
