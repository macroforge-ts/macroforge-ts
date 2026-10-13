/** @derive(Debug, PartialEq, Encode, Decode) */
export interface Merged {
    first: string;
}

export interface Merged {
    second: number;
}

/** @derive(Debug, Clone) */
export class Held {
    name: string = '';
}

export interface Held {
    extra: number;
}

/** @derive(Debug) */
export enum Level {
    Low
}

export enum Level {
    High = 1
}

/** @derive(Debug) */
export interface Single {
    only: string;
}
