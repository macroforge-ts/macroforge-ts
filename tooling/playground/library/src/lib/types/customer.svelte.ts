import type { PersonName } from './person-name.js';

/** @derive(Default, Encode, Decode) */
export interface Customer {
    name: PersonName;
    email: string;
}
