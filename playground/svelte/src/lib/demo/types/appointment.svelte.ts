/** import macro {Gigaform} from "@playground/macro"; */

import type { Site } from './site.svelte';
import type { Colors } from './colors.svelte';
import type { Employee } from './employee.svelte';
import type { RecurrenceRule } from './recurrence-rule.svelte';
import type { Status } from './status.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Appointment {
    /** @hiddenController({}) */
    id: string;
    /** @textController({ label: "Title" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    title: string;
    /** @selectController({ label: "Status", options: [{ label: "Scheduled", value: "Scheduled" }, { label: "On Deck", value: "OnDeck" }, { label: "Waiting", value: "Waiting" }] }) */
    /** @default("Scheduled") */
    status: Status;
    /** @dateTimeController({ label: "Begins" }) */
    begins: string;
    /** @numberController({ label: "Duration", min: 0 }) */
    duration: number;
    /** @textController({ label: "Time Zone" }) */
    timeZone: string;
    /** @hiddenController({}) */
    offsetMs: number;
    /** @switchController({ label: "All Day" }) */
    allDay: boolean;
    /** @switchController({ label: "Multi Day" }) */
    multiDay: boolean;
    /** @comboboxMultipleController({ label: "Employees", fetchUrls: ["/api/employees"] }) */
    employees: (string | Employee)[];
    /** @siteFieldsetController({ label: "Location" }) */
    /** @default("") */
    location: string | Site;
    /** @textAreaController({ label: "Description" }) */
    description: string | null;
    /** @hiddenController({}) */
    /** @default({ main: "#000000", hover: "#333333", active: "#666666" }) */
    colors: Colors;
    /** @hiddenController({}) */
    recurrenceRule: RecurrenceRule | null;
}
