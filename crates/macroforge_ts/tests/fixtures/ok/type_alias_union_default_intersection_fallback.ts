interface ActiveData {
    since: string;
}
interface InactiveData {
    reason: string;
}

/** @derive(Default) */
export type Status =
    | ({ kind: 'Active' } & ActiveData)
    | ({ kind: 'Inactive' } & InactiveData);
