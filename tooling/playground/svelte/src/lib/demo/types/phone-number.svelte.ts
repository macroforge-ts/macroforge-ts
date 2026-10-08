/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface PhoneNumber {
    /** @switchController({ label: "Main" }) */
    main: boolean;
    /** @comboboxController({ label: "Phone Type", allowCustom: true }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    phoneType: string;
    /** @textController({ label: "Number" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    number: string;
    /** @switchController({ label: "Can Text" }) */
    canText: boolean;
    /** @switchController({ label: "Can Call" }) */
    canCall: boolean;
}
