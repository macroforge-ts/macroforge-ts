/** The members of a union that derives only `Encode` are told by their tag, or by their own fields. */
/** @derive(Encode) */
export interface Cat {
    kind: 'cat';
    born: Date;
}

/** @derive(Encode) */
export interface Fish {
    fins: number;
    seen: Date;
}

/** @derive(Encode) */
export enum Mood {
    Calm = 'calm',
    Wild = 'wild',
}

/** @derive(Encode) */
export type Pet = Cat | Fish | Mood | { kind: 'bird'; at: Date } | ({ __type: 'Mouse' } & Fish);

/** @derive(Encode) */
/** @endec({ externallyTagged: true }) */
export type Sighting = 'None' | { Spotted: Cat } | { Heard: Date };
