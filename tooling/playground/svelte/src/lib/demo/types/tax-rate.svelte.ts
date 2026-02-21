/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface TaxRate {
    /** @hiddenController({}) */
    id: string;
    /** @textController({ label: "Name" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    name: string;
    /** @textController({ label: "Tax Agency" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    taxAgency: string;
    /** @numberController({ label: "Zip", min: 0 }) */
    zip: number;
    /** @textController({ label: "City" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    city: string;
    /** @textController({ label: "County" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    county: string;
    /** @textController({ label: "State" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    state: string;
    /** @switchController({ label: "Active" }) */
    isActive: boolean;
    /** @textAreaController({ label: "Description" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    description: string;
    /** @hiddenController({}) */
    /** @default({}) */
    taxComponents: { [key: string]: number };
}
