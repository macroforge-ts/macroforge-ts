/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type Status = /** @default */ 'Scheduled' | 'OnDeck' | 'Waiting';
