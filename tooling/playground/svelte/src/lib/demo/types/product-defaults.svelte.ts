/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface ProductDefaults {
    /** @numberController({ label: "Price", min: 0, step: 0.01 }) */
    price: number;
    /** @textAreaController({ label: "Description" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    description: string;
}
