/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Colors {
    /** @endec({ validate: ["nonEmpty"] }) */
    main: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    hover: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    active: string;
}
