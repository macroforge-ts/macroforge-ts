import { DateTime } from 'effect';

/** @derive(Default, Encode, Decode) */
export interface Event {
    at: DateTime.Utc;
}
