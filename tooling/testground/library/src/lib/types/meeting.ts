import type { DateTime } from 'effect';

/** @derive(Default, Encode, Decode) */
export interface Meeting {
    title: string;
    startsAt: DateTime.DateTime;
}
