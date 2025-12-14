/** import macro {Gigaform} from "@playground/macro"; */

import { Number } from './number.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface PhoneNumber {
    /** @switchController({ label: "Main" }) */
    main: boolean;
    /** @comboboxController({ label: "Phone Type", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    phoneType: string;
    /** @textController({ label: "Number" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    number: string;
    /** @switchController({ label: "Can Text" }) */
    canText: boolean;
    /** @switchController({ label: "Can Call" }) */
    canCall: boolean;
}
