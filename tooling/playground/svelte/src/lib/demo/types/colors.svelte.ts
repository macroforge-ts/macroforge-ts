/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Colors {
    /** @serde({ validate: ["nonEmpty"] }) */
    main: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    hover: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    active: string;
}
