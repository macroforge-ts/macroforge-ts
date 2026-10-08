import { DateTime } from 'effect';

/** @derive(Default, Encode, Decode) */
export interface Foo {
    /** @default("place:holder") */
    id: string;
    createdAt: DateTime.Utc;
}
