/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface EmailParts {
    /** @endec({ validate: ["nonEmpty"] }) */
    local: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    domainName: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    topLevelDomain: string;
}
