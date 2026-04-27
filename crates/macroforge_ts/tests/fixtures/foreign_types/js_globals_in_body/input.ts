import { DateTime } from 'effect';

/** @derive(Default, Serialize, Deserialize) */
export interface Event {
    at: DateTime.Utc;
}
