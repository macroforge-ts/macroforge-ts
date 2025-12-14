/** import macro {Gigaform} from "@playground/macro"; */

import { PersonName } from './person-name.svelte';
import { Site } from './site.svelte';
import { PhoneNumber } from './phone-number.svelte';
import { Represents } from './represents.svelte';
import { Payment } from './payment.svelte';
import { CompanyName } from './company-name.svelte';
import { Account } from './account.svelte';
import { Custom } from './custom.svelte';
import { TaxRate } from './tax-rate.svelte';
import { Company } from './company.svelte';
import { Email } from './email.svelte';
import { Sector } from './sector.svelte';
import { Status } from './status.svelte';
import { NextStep } from './next-step.svelte';
import { LeadStage } from './lead-stage.svelte';
import { AccountName } from './account-name.svelte';
import { Priority } from './priority.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Lead {
    /** @hiddenController({}) */
    id: string;
    /** @hiddenController({}) */
    number: number | null;
    /** @hiddenController({}) */
    accepted: boolean;
    /** @numberController({ label: "Probability", min: 0, max: 100 }) */
    probability: number;
    /** @radioGroupController({ label: "Priority", options: [{ label: "High", value: "High" }, { label: "Medium", value: "Medium" }, { label: "Low", value: "Low" }], orientation: "horizontal" }) */
    /** @default("Medium") */
    priority: Priority;
    /** @dateTimeController({ label: "Due Date" }) */
    dueDate: string | null;
    /** @dateTimeController({ label: "Close Date" }) */
    closeDate: string | null;
    /** @numberController({ label: "Value", min: 0, step: 0.01 }) */
    value: number;
    /** @selectController({ label: "Stage", options: [{ label: "Open", value: "Open" }, { label: "Working", value: "Working" }, { label: "Lost", value: "Lost" }, { label: "Qualified", value: "Qualified" }, { label: "Estimate", value: "Estimate" }, { label: "Negotiation", value: "Negotiation" }] }) */
    /** @default("Open") */
    stage: LeadStage;
    /** @textController({ label: "Status" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    status: string;
    /** @textAreaController({ label: "Description" }) */
    description: string | null;
    /** @hiddenController({}) */
    /** @default("InitialContact") */
    nextStep: NextStep;
    /** @switchController({ label: "Favorite" }) */
    favorite: boolean;
    /** @hiddenController({}) */
    dateAdded: string | null;
    /** @comboboxController({ label: "Tax Rate", allowCustom: false, fetchUrls: ["/api/tax-rates"] }) */
    taxRate: (string | TaxRate) | null;
    /** @radioGroupController({ label: "Sector", options: [{ label: "Residential", value: "Residential" }, { label: "Commercial", value: "Commercial" }], orientation: "horizontal" }) */
    /** @default("Residential") */
    sector: Sector;
    /** @enumFieldsetController({ legend: "Name", variants: { CompanyName: { label: "Company" }, PersonName: { label: "Person" } } }) */
    leadName: AccountName;
    /** @arrayFieldsetController({ legend: "Phones" }) */
    phones: PhoneNumber[];
    /** @emailFieldController({ label: "Email" }) */
    email: Email;
    /** @comboboxController({ label: "Lead Source", allowCustom: true }) */
    leadSource: string | null;
    /** @siteFieldsetController({ label: "Site" }) */
    /** @default("") */
    site: string | Site;
    /** @textAreaController({ label: "Memo" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    memo: string;
    /** @toggleController({ label: "Needs Review" }) */
    needsReview: boolean;
    /** @toggleController({ label: "Has Alert" }) */
    hasAlert: boolean;
    /** @comboboxMultipleController({ label: "Sales Rep", fetchUrls: ["/api/employees"] }) */
    salesRep: Represents[] | null;
    /** @hiddenController({}) */
    color: string | null;
    /** @comboboxController({ label: "Account Type", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    accountType: string;
    /** @comboboxController({ label: "Subtype", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    subtype: string;
    /** @toggleController({ label: "Tax Exempt" }) */
    isTaxExempt: boolean;
    /** @comboboxController({ label: "Payment Terms", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    paymentTerms: string;
    /** @tagsController({ label: "Tags" }) */
    tags: string[];
    /** @arrayFieldsetController({ legend: "Custom Fields" }) */
    customFields: [string, string][];
}
