/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface AppointmentNotifications {
    /** @endec({ validate: ["nonEmpty"] }) */
    personalScheduleChangeNotifications: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    allScheduleChangeNotifications: string;
}
