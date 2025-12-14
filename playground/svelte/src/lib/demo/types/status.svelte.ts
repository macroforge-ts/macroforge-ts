/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Status = /** @default */ 'Scheduled' | 'OnDeck' | 'Waiting';
