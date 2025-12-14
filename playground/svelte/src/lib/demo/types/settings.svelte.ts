/** import macro {Gigaform} from "@playground/macro"; */

import { AppointmentNotifications } from './appointment-notifications.svelte';
import { ScheduleSettings } from './schedule-settings.svelte';
import { OverviewSettings } from './overview-settings.svelte';
import { Commissions } from './commissions.svelte';
import { Page } from './page.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Settings {
    appointmentNotifications: AppointmentNotifications | null;
    commissions: Commissions | null;
    scheduleSettings: ScheduleSettings;
    accountOverviewSettings: OverviewSettings;
    serviceOverviewSettings: OverviewSettings;
    appointmentOverviewSettings: OverviewSettings;
    leadOverviewSettings: OverviewSettings;
    packageOverviewSettings: OverviewSettings;
    productOverviewSettings: OverviewSettings;
    orderOverviewSettings: OverviewSettings;
    taxRateOverviewSettings: OverviewSettings;
    /** @default("UserHome") */
    homePage: Page;
}
