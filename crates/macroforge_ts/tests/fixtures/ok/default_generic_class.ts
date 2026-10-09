/** @derive(Default) */
export class Shelf<T> {
    items: T[] = [];
    label: string = '';
}

/** @derive(Default) */
export class Pinned<T extends object> {
    /** @default(null) */
    value: T | null;
}
