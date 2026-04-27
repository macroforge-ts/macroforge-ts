import { DateTime } from 'effect';

/** @derive(Default, Serialize, Deserialize) */
export interface Foo {
    /** @default("place:holder") */
    id: string;
    createdAt: DateTime.Utc;
}
