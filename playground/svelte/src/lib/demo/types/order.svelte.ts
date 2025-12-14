/** import macro {Gigaform} from "@playground/macro"; */

import { Promotion } from './promotion.svelte';
import { Site } from './site.svelte';
import { Payment } from './payment.svelte';
import { Appointment } from './appointment.svelte';
import { Package } from './package.svelte';
import { Account } from './account.svelte';
import { Lead } from './lead.svelte';
import { Employee } from './employee.svelte';
import { BilledItem } from './billed-item.svelte';
import { OrderStage } from './order-stage.svelte';
import { Item } from './item.svelte';

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
    dateCreated: string;
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
    due: string;
    /** @hiddenController({}) */
    total: number;
    /** @siteFieldsetController({ label: "Site" }) */
    /** @default("") */
    site: string | Site;
    /** @arrayFieldsetController({ legend: "Billed Items" }) */
    billedItems: BilledItem[];
    /** @textAreaController({ label: "Memo" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    memo: string;
    /** @hiddenController({}) */
    discount: number;
    /** @hiddenController({}) */
    tip: number;
    /** @hiddenController({}) */
    commissions: number[];
}
