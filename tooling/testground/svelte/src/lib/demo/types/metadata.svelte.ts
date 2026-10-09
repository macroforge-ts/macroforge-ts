/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Metadata {
    createdAt: string;
    lastLogin: string | null;
    isActive: boolean;
    roles: Array<string>;
}
