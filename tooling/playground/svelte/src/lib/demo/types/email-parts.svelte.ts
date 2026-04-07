/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface EmailParts {
    /** @serde({ validate: ["nonEmpty"] }) */
    local: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    domainName: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    topLevelDomain: string;
}
