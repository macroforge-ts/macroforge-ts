/** @derive(Default) */
export interface Assignment {
    name: string;
    scores: { [key: string]: number };
    active: boolean;
}
