/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type Status = /** @default */ 'Scheduled' | 'OnDeck' | 'Waiting';
