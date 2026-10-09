/** import macro {Gigaform} from "@testground/macro"; */

import type { Coordinates } from './coordinates.svelte';

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Site {
    id: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    addressLine1: string;
    addressLine2: string | null;
    sublocalityLevel1: string | null;
    /** @endec({ validate: ["nonEmpty"] }) */
    locality: string;
    administrativeAreaLevel3: string | null;
    administrativeAreaLevel2: string | null;
    /** @endec({ validate: ["nonEmpty"] }) */
    administrativeAreaLevel1: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    country: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    postalCode: string;
    postalCodeSuffix: string | null;
    coordinates: Coordinates;
}
