/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface EmailParts {
    /** @endec({ validate: ["nonEmpty"] }) */
    local: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    domainName: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    topLevelDomain: string;
}
