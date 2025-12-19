/** import macro {Gigaform} from "@playground/macro"; */

import type { DateTime } from 'effect';
import type { Option } from 'effect/Option';
import type { Promotion } from './promotion.svelte';
import type { Site } from './site.svelte';
import type { Payment } from './payment.svelte';
import type { Appointment } from './appointment.svelte';
import type { Package } from './package.svelte';
import type { Account } from './account.svelte';
import type { Lead } from './lead.svelte';
import type { Employee } from './employee.svelte';
import type { BilledItem } from './billed-item.svelte';
import type { OrderStage } from './order-stage.svelte';
import type { Item } from './item.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Order {
    /** @hiddenController({}) */
    id: string;
    /** @comboboxController({ label: "Account", allowCustom: false, fetchUrls: ["/api/accounts"] }) */
    /** @default("") */
    account: string | Account;
    /** @selectController({ label: "Stage", options: [{ label: "Estimate", value: "Estimate" }, { label: "Active", value: "Active" }, { label: "Invoice", value: "Invoice" }] }) */
    /** @default("Estimate") */
    stage: OrderStage;
    /** @hiddenController({}) */
    number: number;
    /** @hiddenController({}) */
    payments: (string | Payment)[];
    /** @textController({ label: "Opportunity" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    opportunity: string;
    /** @textController({ label: "Reference" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    reference: string;
    /** @comboboxController({ label: "Lead Source", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    leadSource: string;
    /** @comboboxController({ label: "Sales Rep", allowCustom: false, fetchUrls: ["/api/employees"] }) */
    /** @default("") */
    salesRep: string | Employee;
    /** @comboboxController({ label: "Group", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    group: string;
    /** @comboboxController({ label: "Subgroup", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    subgroup: string;
    /** @switchController({ label: "Posted" }) */
    isPosted: boolean;
    /** @switchController({ label: "Needs Review" }) */
    needsReview: boolean;
    /** @textController({ label: "Action Item" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    actionItem: string;
    /** @hiddenController({}) */
    upsale: number;
    /** @hiddenController({}) */
    dateCreated: DateTime.DateTime;
    /** @comboboxController({ label: "Appointment", allowCustom: false, fetchUrls: ["/api/appointments"] }) */
    /** @default("") */
    appointment: string | Appointment;
    /** @comboboxMultipleController({ label: "Technicians", fetchUrls: ["/api/employees"] }) */
    lastTechs: (string | Employee)[];
    /** @hiddenController({}) */
    package: (string | Package)[] | null;
    /** @hiddenController({}) */
    promotion: (string | Promotion)[] | null;
    /** @hiddenController({}) */
    balance: number;
    /** @dateTimeController({ label: "Due" }) */
    due: DateTime.DateTime;
    /** @hiddenController({}) */
    total: number;
    /** @siteFieldsetController({ label: "Site" }) */
    /** @default("") */
    site: string | Site;
    /** @arrayFieldsetController({ legend: "Billed Items" }) */
    billedItems: BilledItem[];
    /** @textAreaController({ label: "Memo" }) */
    memo: Option<string>;
    /** @hiddenController({}) */
    discount: number;
    /** @hiddenController({}) */
    tip: number;
    /** @hiddenController({}) */
    commissions: number[];
}
