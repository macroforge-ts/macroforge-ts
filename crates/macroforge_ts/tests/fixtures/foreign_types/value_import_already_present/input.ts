import { DateTime } from 'effect';

/** @derive(Default, Serialize, Deserialize) */
export interface Tick {
    at: DateTime.Utc;
}
