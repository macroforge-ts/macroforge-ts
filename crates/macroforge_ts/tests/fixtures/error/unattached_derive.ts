/** @derive(Debug) */
export const value = 1;

/** @derive(Debug) */
export namespace Space {
    export const inside = 1;
}

/** @derive(Debug) */
// a note between the two
export interface Commented {
    a: string;
}

export interface Member {
    /** @derive(Debug) */
    field: string;
}

/** @derive(Debug) */

export interface Spaced {
    a: string;
}

/** @derive(Debug) */
/** A description. */
export interface Stacked {
    a: string;
}

export namespace Outer {
    /** @derive(Debug) */
    export interface Inside {
        a: string;
    }
}

export function local(): string {
    /** @derive(Debug, Clone) */
    class Local {
        a: string = 'local';
    }
    return new Local().a;
}
