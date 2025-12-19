/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Metadata {
    createdAt: string;
    lastLogin: string | null;
    isActive: boolean;
    roles: string[];
}
