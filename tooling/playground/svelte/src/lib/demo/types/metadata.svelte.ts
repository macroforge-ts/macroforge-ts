/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Metadata {
    createdAt: string;
    lastLogin: string | null;
    isActive: boolean;
    roles: Array<string>;
}
