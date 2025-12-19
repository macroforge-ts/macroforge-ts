/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface AppointmentNotifications {
    /** @serde({ validate: ["nonEmpty"] }) */
    personalScheduleChangeNotifications: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    allScheduleChangeNotifications: string;
}
