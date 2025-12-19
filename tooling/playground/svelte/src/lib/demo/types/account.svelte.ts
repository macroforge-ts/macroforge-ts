import type { AccountName } from './account-name.svelte';
import type { Colors } from './colors.svelte';
import type { Did } from './did.svelte';
import type { Email } from './email.svelte';
import type { Ordered } from './ordered.svelte';
import type { PhoneNumber } from './phone-number.svelte';
import type { Represents } from './represents.svelte';
import type { Sector } from './sector.svelte';
import type { Site } from './site.svelte';
import type { TaxRate } from './tax-rate.svelte';
/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Account {
    /** @hiddenController({}) */
    id: string;
    /** @comboboxController({ label: "Tax Rate", allowCustom: false, fetchUrls: ["/api/tax-rates"] }) */
    /** @default("") */
    taxRate: string | TaxRate;
    /** @siteFieldsetController({ label: "Site" }) */
    /** @default("") */
    site: string | Site;
    /** @comboboxMultipleController({ label: "Sales Rep", fetchUrls: ["/api/employees"] }) */
    salesRep: Array<Represents> | null;
    /** @hiddenController({}) */
    orders: Array<Ordered>;
    /** @hiddenController({}) */
    activity: Array<Did>;
    /** @arrayFieldsetController({ legend: "Custom Fields" }) */
    customFields: Array<[string, string]>;
    /** @enumFieldsetController({ legend: "Name", variants: { CompanyName: { label: "Company" }, PersonName: { label: "Person" } } }) */
    accountName: AccountName;
    /** @radioGroupController({ label: "Sector", options: [{ label: "Residential", value: "Residential" }, { label: "Commercial", value: "Commercial" }], orientation: "horizontal" }) */
    /** @default("Residential") */
    sector: Sector;
    /** @textAreaController({ label: "Memo" }) */
    memo: string | null;
    /** @arrayFieldsetController({ legend: "Phones" }) */
    phones: Array<PhoneNumber>;
    /** @emailFieldController({ label: "Email" }) */
    email: Email;
    /** @comboboxController({ label: "Lead Source", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    leadSource: string;
    /** @hiddenController({}) */
    colors: Colors;
    /** @toggleController({ label: "Needs Review" }) */
    needsReview: boolean;
    /** @toggleController({ label: "Has Alert" }) */
    hasAlert: boolean;
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
    tags: Array<string>;
    /** @hiddenController({}) */
    dateAdded: string;
}
