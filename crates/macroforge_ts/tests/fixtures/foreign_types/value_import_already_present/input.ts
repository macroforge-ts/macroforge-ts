import { DateTime } from 'effect';

/** @derive(Default, Encode, Decode) */
export interface Tick {
    at: DateTime.Utc;
}
