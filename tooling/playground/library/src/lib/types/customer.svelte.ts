import type { PersonName } from './person-name.js';

/** @derive(Default, Serialize, Deserialize) */
export interface Customer {
    name: PersonName;
    email: string;
}
