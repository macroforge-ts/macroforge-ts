/** import macro {Gigaform} from "@playground/macro"; */

import type { Coordinates } from './coordinates.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Site {
    id: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    addressLine1: string;
    addressLine2: string | null;
    sublocalityLevel1: string | null;
    /** @serde({ validate: ["nonEmpty"] }) */
    locality: string;
    administrativeAreaLevel3: string | null;
    administrativeAreaLevel2: string | null;
    /** @serde({ validate: ["nonEmpty"] }) */
    administrativeAreaLevel1: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    country: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    postalCode: string;
    postalCodeSuffix: string | null;
    coordinates: Coordinates;
}
