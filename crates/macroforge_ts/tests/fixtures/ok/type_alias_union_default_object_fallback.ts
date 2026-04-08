/** @derive(Default) */
export type Status = { kind: 'Active'; data: string } | { kind: 'Inactive'; reason: string };
