/** import macro {Gigaform} from "@playground/macro"; */

import type { AppointmentNotifications } from './appointment-notifications.svelte';
import type { ScheduleSettings } from './schedule-settings.svelte';
import type { OverviewSettings } from './overview-settings.svelte';
import type { Commissions } from './commissions.svelte';
import type { Page } from './page.svelte';

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
