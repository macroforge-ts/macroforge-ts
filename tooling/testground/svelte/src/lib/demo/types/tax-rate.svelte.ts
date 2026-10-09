/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface TaxRate {
    /** @hiddenController({}) */
    id: string;
    /** @textController({ label: "Name" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
    /** @textController({ label: "Tax Agency" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    taxAgency: string;
    /** @numberController({ label: "Zip", min: 0 }) */
    zip: number;
    /** @textController({ label: "City" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    city: string;
    /** @textController({ label: "County" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    county: string;
    /** @textController({ label: "State" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    state: string;
    /** @switchController({ label: "Active" }) */
    isActive: boolean;
    /** @textAreaController({ label: "Description" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    description: string;
    /** @hiddenController({}) */
    /** @default({}) */
    taxComponents: { [key: string]: number };
}
