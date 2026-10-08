/** @derive(Encode, Decode) */
export enum Stage {
    Draft = 0,
    Active = 1,
    'Needs Review' = 2
}

/** @derive(Encode, Decode) */
export interface Job {
    stage: Stage;
}
