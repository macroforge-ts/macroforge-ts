import { DateTime } from 'effect';

/** @derive(Default) */
export interface Foo {
    createdAt: DateTime.Utc;
}
