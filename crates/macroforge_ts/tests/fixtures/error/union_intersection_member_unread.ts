/** A union member that is an intersection no coder reads is an error, not a member left out. */
/** @derive(Encode, Decode) */
export interface Account {
    id: string;
}

/** @derive(Encode, Decode) */
export interface Owner {
    name: string;
}

/** The tag of the union is `__type`, and the member is tagged by `kind`. */
/** @derive(Encode, Decode) */
export type OtherTag = ({ kind: 'account' } & Account) | ({ kind: 'owner' } & Owner);

/** @derive(Decode) */
export type Untagged = (Account & { extra: number }) | Owner;

/** @derive(Encode) */
export type TwoTypes = ({ __type: 'both' } & Account & Owner) | string;

/** @derive(Decode) */
/** @endec({ tag: "kind" }) */
export type MoreFields = ({ kind: 'account'; note: string } & Account) | ({ kind: 'owner' } & Owner);

/** @derive(Decode) */
/** @endec({ tag: "kind" }) */
export type WideTag = ({ kind: string } & Account) | ({ kind: 'owner' } & Owner);

/** A member of the form is read, on either side of the `&`. */
/** @derive(Encode, Decode) */
/** @endec({ tag: "kind" }) */
export type Read = ({ kind: 'account' } & Account) | (Owner & { kind: "owner" });
