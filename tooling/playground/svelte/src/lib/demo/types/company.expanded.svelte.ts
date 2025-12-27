import { colorsConfigDefaultValue } from "./colors-config.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { colorsConfigSerializeWithContext } from "./colors-config.svelte";
import { phoneNumberSerializeWithContext } from "./phone-number.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { colorsConfigDeserializeWithContext } from "./colors-config.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
import type { ArrayFieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { ColorsConfig } from './colors-config.svelte';
import type { PhoneNumber } from './phone-number.svelte';
import type { Site } from './site.svelte';
import type { TaxRate } from './tax-rate.svelte';


export interface Company {
    id: string;
    
    legalName: string;
    
    headquarters: string | Site;
    phones: Array<PhoneNumber>;
    
    fax: string;
    
    email: string;
    
    website: string;
    
    taxId: string;
    referenceNumber: number;
    
    postalCodeLookup: string;
    timeZone: string;
    
    defaultTax: string | TaxRate;
    
    defaultTaxLocation: string;
    defaultAreaCode: number;
    
    defaultAccountType: string;
    
    lookupFormatting: string;
    
    accountNameFormat: string;
    merchantServiceProvider: string | null;
    
    dateDisplayStyle: string;
    hasAutoCommission: boolean;
    hasAutoDaylightSavings: boolean;
    hasAutoFmsTracking: boolean;
    hasNotifications: boolean;
    hasRequiredLeadSource: boolean;
    hasRequiredEmail: boolean;
    hasSortServiceItemsAlphabetically: boolean;
    hasAttachOrderToAppointmentEmails: boolean;
    scheduleInterval: number;
    colorsConfig: ColorsConfig;
}

export function companyDefaultValue(): Company {
    return {
        id: "",
        legalName: "",
        headquarters: "",
        phones: [],
        fax: "",
        email: "",
        website: "",
        taxId: "",
        referenceNumber: 0,
        postalCodeLookup: "",
        timeZone: "",
        defaultTax: "",
        defaultTaxLocation: "",
        defaultAreaCode: 0,
        defaultAccountType: "",
        lookupFormatting: "",
        accountNameFormat: "",
        merchantServiceProvider: null,
        dateDisplayStyle: "",
        hasAutoCommission: false,
        hasAutoDaylightSavings: false,
        hasAutoFmsTracking: false,
        hasNotifications: false,
        hasRequiredLeadSource: false,
        hasRequiredEmail: false,
        hasSortServiceItemsAlphabetically: false,
        hasAttachOrderToAppointmentEmails: false,
        scheduleInterval: 0,
        colorsConfig: colorsConfigDefaultValue()
    } as Company;
}

export function companySerialize(value: Company): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(companySerializeWithContext(value, ctx));
}
export function companySerializeWithContext(value: Company, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Company"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"legalName"}`] = value.legalName;
    result[`${"headquarters"}`] = value.headquarters;
    result[`${"phones"}`] = value.phones.map((item)=>phoneNumberSerializeWithContext(item, ctx));
    result[`${"fax"}`] = value.fax;
    result[`${"email"}`] = value.email;
    result[`${"website"}`] = value.website;
    result[`${"taxId"}`] = value.taxId;
    result[`${"referenceNumber"}`] = value.referenceNumber;
    result[`${"postalCodeLookup"}`] = value.postalCodeLookup;
    result[`${"timeZone"}`] = value.timeZone;
    result[`${"defaultTax"}`] = value.defaultTax;
    result[`${"defaultTaxLocation"}`] = value.defaultTaxLocation;
    result[`${"defaultAreaCode"}`] = value.defaultAreaCode;
    result[`${"defaultAccountType"}`] = value.defaultAccountType;
    result[`${"lookupFormatting"}`] = value.lookupFormatting;
    result[`${"accountNameFormat"}`] = value.accountNameFormat;
    result[`${"merchantServiceProvider"}`] = value.merchantServiceProvider;
    result[`${"dateDisplayStyle"}`] = value.dateDisplayStyle;
    result[`${"hasAutoCommission"}`] = value.hasAutoCommission;
    result[`${"hasAutoDaylightSavings"}`] = value.hasAutoDaylightSavings;
    result[`${"hasAutoFmsTracking"}`] = value.hasAutoFmsTracking;
    result[`${"hasNotifications"}`] = value.hasNotifications;
    result[`${"hasRequiredLeadSource"}`] = value.hasRequiredLeadSource;
    result[`${"hasRequiredEmail"}`] = value.hasRequiredEmail;
    result[`${"hasSortServiceItemsAlphabetically"}`] = value.hasSortServiceItemsAlphabetically;
    result[`${"hasAttachOrderToAppointmentEmails"}`] = value.hasAttachOrderToAppointmentEmails;
    result[`${"scheduleInterval"}`] = value.scheduleInterval;
    result[`${"colorsConfig"}`] = colorsConfigSerializeWithContext(value.colorsConfig, ctx);
    return result;
}

export function companyDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Company } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = companyDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Company.deserialize: root cannot be a forward reference"
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return {
            success: true,
            value: resultOrRef
        };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return {
                success: false,
                errors: e.errors
            };
        }
        const message = e instanceof Error ? e.message : String(e);
        return {
            success: false,
            errors: [
                {
                    field: "_root",
                    message
                }
            ]
        };
    }
}
export function companyDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Company | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Company"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"id"}` in obj)) {
        errors.push({
            field: `${"id"}`,
            message: "missing required field"
        });
    }
    if (!(`${"legalName"}` in obj)) {
        errors.push({
            field: `${"legalName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"headquarters"}` in obj)) {
        errors.push({
            field: `${"headquarters"}`,
            message: "missing required field"
        });
    }
    if (!(`${"phones"}` in obj)) {
        errors.push({
            field: `${"phones"}`,
            message: "missing required field"
        });
    }
    if (!(`${"fax"}` in obj)) {
        errors.push({
            field: `${"fax"}`,
            message: "missing required field"
        });
    }
    if (!(`${"email"}` in obj)) {
        errors.push({
            field: `${"email"}`,
            message: "missing required field"
        });
    }
    if (!(`${"website"}` in obj)) {
        errors.push({
            field: `${"website"}`,
            message: "missing required field"
        });
    }
    if (!(`${"taxId"}` in obj)) {
        errors.push({
            field: `${"taxId"}`,
            message: "missing required field"
        });
    }
    if (!(`${"referenceNumber"}` in obj)) {
        errors.push({
            field: `${"referenceNumber"}`,
            message: "missing required field"
        });
    }
    if (!(`${"postalCodeLookup"}` in obj)) {
        errors.push({
            field: `${"postalCodeLookup"}`,
            message: "missing required field"
        });
    }
    if (!(`${"timeZone"}` in obj)) {
        errors.push({
            field: `${"timeZone"}`,
            message: "missing required field"
        });
    }
    if (!(`${"defaultTax"}` in obj)) {
        errors.push({
            field: `${"defaultTax"}`,
            message: "missing required field"
        });
    }
    if (!(`${"defaultTaxLocation"}` in obj)) {
        errors.push({
            field: `${"defaultTaxLocation"}`,
            message: "missing required field"
        });
    }
    if (!(`${"defaultAreaCode"}` in obj)) {
        errors.push({
            field: `${"defaultAreaCode"}`,
            message: "missing required field"
        });
    }
    if (!(`${"defaultAccountType"}` in obj)) {
        errors.push({
            field: `${"defaultAccountType"}`,
            message: "missing required field"
        });
    }
    if (!(`${"lookupFormatting"}` in obj)) {
        errors.push({
            field: `${"lookupFormatting"}`,
            message: "missing required field"
        });
    }
    if (!(`${"accountNameFormat"}` in obj)) {
        errors.push({
            field: `${"accountNameFormat"}`,
            message: "missing required field"
        });
    }
    if (!(`${"merchantServiceProvider"}` in obj)) {
        errors.push({
            field: `${"merchantServiceProvider"}`,
            message: "missing required field"
        });
    }
    if (!(`${"dateDisplayStyle"}` in obj)) {
        errors.push({
            field: `${"dateDisplayStyle"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasAutoCommission"}` in obj)) {
        errors.push({
            field: `${"hasAutoCommission"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasAutoDaylightSavings"}` in obj)) {
        errors.push({
            field: `${"hasAutoDaylightSavings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasAutoFmsTracking"}` in obj)) {
        errors.push({
            field: `${"hasAutoFmsTracking"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasNotifications"}` in obj)) {
        errors.push({
            field: `${"hasNotifications"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasRequiredLeadSource"}` in obj)) {
        errors.push({
            field: `${"hasRequiredLeadSource"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasRequiredEmail"}` in obj)) {
        errors.push({
            field: `${"hasRequiredEmail"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasSortServiceItemsAlphabetically"}` in obj)) {
        errors.push({
            field: `${"hasSortServiceItemsAlphabetically"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasAttachOrderToAppointmentEmails"}` in obj)) {
        errors.push({
            field: `${"hasAttachOrderToAppointmentEmails"}`,
            message: "missing required field"
        });
    }
    if (!(`${"scheduleInterval"}` in obj)) {
        errors.push({
            field: `${"scheduleInterval"}`,
            message: "missing required field"
        });
    }
    if (!(`${"colorsConfig"}` in obj)) {
        errors.push({
            field: `${"colorsConfig"}`,
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_id = obj[`${"id"}`] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_legalName = obj[`${"legalName"}`] as string;
        if (__raw_legalName.trim().length === 0) {
            errors.push({
                field: "legalName",
                message: "Company.legalName must not be empty"
            });
        }
        instance.legalName = __raw_legalName;
    }
    {
        const __raw_headquarters = obj[`${"headquarters"}`] as string | Site;
        instance.headquarters = __raw_headquarters;
    }
    {
        const __raw_phones = obj[`${"phones"}`] as Array<PhoneNumber>;
        if (Array.isArray(__raw_phones)) {
            instance.phones = __raw_phones as PhoneNumber[];
        }
    }
    {
        const __raw_fax = obj[`${"fax"}`] as string;
        if (__raw_fax.trim().length === 0) {
            errors.push({
                field: "fax",
                message: "Company.fax must not be empty"
            });
        }
        instance.fax = __raw_fax;
    }
    {
        const __raw_email = obj[`${"email"}`] as string;
        if (__raw_email.trim().length === 0) {
            errors.push({
                field: "email",
                message: "Company.email must not be empty"
            });
        }
        instance.email = __raw_email;
    }
    {
        const __raw_website = obj[`${"website"}`] as string;
        if (__raw_website.trim().length === 0) {
            errors.push({
                field: "website",
                message: "Company.website must not be empty"
            });
        }
        instance.website = __raw_website;
    }
    {
        const __raw_taxId = obj[`${"taxId"}`] as string;
        if (__raw_taxId.trim().length === 0) {
            errors.push({
                field: "taxId",
                message: "Company.taxId must not be empty"
            });
        }
        instance.taxId = __raw_taxId;
    }
    {
        const __raw_referenceNumber = obj[`${"referenceNumber"}`] as number;
        instance.referenceNumber = __raw_referenceNumber;
    }
    {
        const __raw_postalCodeLookup = obj[`${"postalCodeLookup"}`] as string;
        if (__raw_postalCodeLookup.trim().length === 0) {
            errors.push({
                field: "postalCodeLookup",
                message: "Company.postalCodeLookup must not be empty"
            });
        }
        instance.postalCodeLookup = __raw_postalCodeLookup;
    }
    {
        const __raw_timeZone = obj[`${"timeZone"}`] as string;
        instance.timeZone = __raw_timeZone;
    }
    {
        const __raw_defaultTax = obj[`${"defaultTax"}`] as string | TaxRate;
        instance.defaultTax = __raw_defaultTax;
    }
    {
        const __raw_defaultTaxLocation = obj[`${"defaultTaxLocation"}`] as string;
        if (__raw_defaultTaxLocation.trim().length === 0) {
            errors.push({
                field: "defaultTaxLocation",
                message: "Company.defaultTaxLocation must not be empty"
            });
        }
        instance.defaultTaxLocation = __raw_defaultTaxLocation;
    }
    {
        const __raw_defaultAreaCode = obj[`${"defaultAreaCode"}`] as number;
        instance.defaultAreaCode = __raw_defaultAreaCode;
    }
    {
        const __raw_defaultAccountType = obj[`${"defaultAccountType"}`] as string;
        if (__raw_defaultAccountType.trim().length === 0) {
            errors.push({
                field: "defaultAccountType",
                message: "Company.defaultAccountType must not be empty"
            });
        }
        instance.defaultAccountType = __raw_defaultAccountType;
    }
    {
        const __raw_lookupFormatting = obj[`${"lookupFormatting"}`] as string;
        if (__raw_lookupFormatting.trim().length === 0) {
            errors.push({
                field: "lookupFormatting",
                message: "Company.lookupFormatting must not be empty"
            });
        }
        instance.lookupFormatting = __raw_lookupFormatting;
    }
    {
        const __raw_accountNameFormat = obj[`${"accountNameFormat"}`] as string;
        if (__raw_accountNameFormat.trim().length === 0) {
            errors.push({
                field: "accountNameFormat",
                message: "Company.accountNameFormat must not be empty"
            });
        }
        instance.accountNameFormat = __raw_accountNameFormat;
    }
    {
        const __raw_merchantServiceProvider = obj[`${"merchantServiceProvider"}`] as string | null;
        instance.merchantServiceProvider = __raw_merchantServiceProvider;
    }
    {
        const __raw_dateDisplayStyle = obj[`${"dateDisplayStyle"}`] as string;
        if (__raw_dateDisplayStyle.trim().length === 0) {
            errors.push({
                field: "dateDisplayStyle",
                message: "Company.dateDisplayStyle must not be empty"
            });
        }
        instance.dateDisplayStyle = __raw_dateDisplayStyle;
    }
    {
        const __raw_hasAutoCommission = obj[`${"hasAutoCommission"}`] as boolean;
        instance.hasAutoCommission = __raw_hasAutoCommission;
    }
    {
        const __raw_hasAutoDaylightSavings = obj[`${"hasAutoDaylightSavings"}`] as boolean;
        instance.hasAutoDaylightSavings = __raw_hasAutoDaylightSavings;
    }
    {
        const __raw_hasAutoFmsTracking = obj[`${"hasAutoFmsTracking"}`] as boolean;
        instance.hasAutoFmsTracking = __raw_hasAutoFmsTracking;
    }
    {
        const __raw_hasNotifications = obj[`${"hasNotifications"}`] as boolean;
        instance.hasNotifications = __raw_hasNotifications;
    }
    {
        const __raw_hasRequiredLeadSource = obj[`${"hasRequiredLeadSource"}`] as boolean;
        instance.hasRequiredLeadSource = __raw_hasRequiredLeadSource;
    }
    {
        const __raw_hasRequiredEmail = obj[`${"hasRequiredEmail"}`] as boolean;
        instance.hasRequiredEmail = __raw_hasRequiredEmail;
    }
    {
        const __raw_hasSortServiceItemsAlphabetically = obj[`${"hasSortServiceItemsAlphabetically"}`] as boolean;
        instance.hasSortServiceItemsAlphabetically = __raw_hasSortServiceItemsAlphabetically;
    }
    {
        const __raw_hasAttachOrderToAppointmentEmails = obj[`${"hasAttachOrderToAppointmentEmails"}`] as boolean;
        instance.hasAttachOrderToAppointmentEmails = __raw_hasAttachOrderToAppointmentEmails;
    }
    {
        const __raw_scheduleInterval = obj[`${"scheduleInterval"}`] as number;
        instance.scheduleInterval = __raw_scheduleInterval;
    }
    {
        const __raw_colorsConfig = obj[`${"colorsConfig"}`] as ColorsConfig;
        {
            const __result = colorsConfigDeserializeWithContext(__raw_colorsConfig, ctx);
            ctx.assignOrDefer(instance, `${"colorsConfig"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Company;
}
export function companyValidateField<K extends keyof Company>(_field: K, _value: Company[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"legalName"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "legalName",
                message: "Company.legalName must not be empty"
            });
        }
    }
    if (_field === `${"fax"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "fax",
                message: "Company.fax must not be empty"
            });
        }
    }
    if (_field === `${"email"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "email",
                message: "Company.email must not be empty"
            });
        }
    }
    if (_field === `${"website"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "website",
                message: "Company.website must not be empty"
            });
        }
    }
    if (_field === `${"taxId"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "taxId",
                message: "Company.taxId must not be empty"
            });
        }
    }
    if (_field === `${"postalCodeLookup"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "postalCodeLookup",
                message: "Company.postalCodeLookup must not be empty"
            });
        }
    }
    if (_field === `${"defaultTaxLocation"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "defaultTaxLocation",
                message: "Company.defaultTaxLocation must not be empty"
            });
        }
    }
    if (_field === `${"defaultAccountType"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "defaultAccountType",
                message: "Company.defaultAccountType must not be empty"
            });
        }
    }
    if (_field === `${"lookupFormatting"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "lookupFormatting",
                message: "Company.lookupFormatting must not be empty"
            });
        }
    }
    if (_field === `${"accountNameFormat"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "accountNameFormat",
                message: "Company.accountNameFormat must not be empty"
            });
        }
    }
    if (_field === `${"dateDisplayStyle"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "dateDisplayStyle",
                message: "Company.dateDisplayStyle must not be empty"
            });
        }
    }
    return errors;
}
export function companyValidateFields(_partial: Partial<Company>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"legalName"}` in _partial && _partial.legalName !== undefined) {
        const __val = _partial.legalName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "legalName",
                message: "Company.legalName must not be empty"
            });
        }
    }
    if (`${"fax"}` in _partial && _partial.fax !== undefined) {
        const __val = _partial.fax as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "fax",
                message: "Company.fax must not be empty"
            });
        }
    }
    if (`${"email"}` in _partial && _partial.email !== undefined) {
        const __val = _partial.email as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "email",
                message: "Company.email must not be empty"
            });
        }
    }
    if (`${"website"}` in _partial && _partial.website !== undefined) {
        const __val = _partial.website as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "website",
                message: "Company.website must not be empty"
            });
        }
    }
    if (`${"taxId"}` in _partial && _partial.taxId !== undefined) {
        const __val = _partial.taxId as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "taxId",
                message: "Company.taxId must not be empty"
            });
        }
    }
    if (`${"postalCodeLookup"}` in _partial && _partial.postalCodeLookup !== undefined) {
        const __val = _partial.postalCodeLookup as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "postalCodeLookup",
                message: "Company.postalCodeLookup must not be empty"
            });
        }
    }
    if (`${"defaultTaxLocation"}` in _partial && _partial.defaultTaxLocation !== undefined) {
        const __val = _partial.defaultTaxLocation as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "defaultTaxLocation",
                message: "Company.defaultTaxLocation must not be empty"
            });
        }
    }
    if (`${"defaultAccountType"}` in _partial && _partial.defaultAccountType !== undefined) {
        const __val = _partial.defaultAccountType as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "defaultAccountType",
                message: "Company.defaultAccountType must not be empty"
            });
        }
    }
    if (`${"lookupFormatting"}` in _partial && _partial.lookupFormatting !== undefined) {
        const __val = _partial.lookupFormatting as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "lookupFormatting",
                message: "Company.lookupFormatting must not be empty"
            });
        }
    }
    if (`${"accountNameFormat"}` in _partial && _partial.accountNameFormat !== undefined) {
        const __val = _partial.accountNameFormat as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "accountNameFormat",
                message: "Company.accountNameFormat must not be empty"
            });
        }
    }
    if (`${"dateDisplayStyle"}` in _partial && _partial.dateDisplayStyle !== undefined) {
        const __val = _partial.dateDisplayStyle as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "dateDisplayStyle",
                message: "Company.dateDisplayStyle must not be empty"
            });
        }
    }
    return errors;
}
export function companyHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "legalName" in o && "headquarters" in o && "phones" in o && "fax" in o && "email" in o && "website" in o && "taxId" in o && "referenceNumber" in o && "postalCodeLookup" in o && "timeZone" in o && "defaultTax" in o && "defaultTaxLocation" in o && "defaultAreaCode" in o && "defaultAccountType" in o && "lookupFormatting" in o && "accountNameFormat" in o && "merchantServiceProvider" in o && "dateDisplayStyle" in o && "hasAutoCommission" in o && "hasAutoDaylightSavings" in o && "hasAutoFmsTracking" in o && "hasNotifications" in o && "hasRequiredLeadSource" in o && "hasRequiredEmail" in o && "hasSortServiceItemsAlphabetically" in o && "hasAttachOrderToAppointmentEmails" in o && "scheduleInterval" in o && "colorsConfig" in o';
}
export function companyIs(obj: unknown): obj is Company {
    if (!companyHasShape(obj)) {
        return false;
    }
    const result = companyDeserialize(obj);
    return result.success;
}

export function companyFromFormData(formData: FormData): Exit<Company, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get(`${"id"}`) ?? "";
    obj.legalName = formData.get(`${"legalName"}`) ?? "";
    obj.headquarters = formData.get(`${"headquarters"}`) ?? "";
    {
        const phonesItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while(formData.has(`${"phones"}.` + idx + ".") || idx === 0){
            const hasAny = Array.from(formData.keys()).some((k)=>k.startsWith(`${"phones"}.` + idx + "."));
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())){
                    if (key.startsWith(`${"phones"}.` + idx + ".")) {
                        const fieldName = key.slice(`${"phones"}.`.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                phonesItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.phones = phonesItems;
    }
    obj.fax = formData.get(`${"fax"}`) ?? "";
    obj.email = formData.get(`${"email"}`) ?? "";
    obj.website = formData.get(`${"website"}`) ?? "";
    obj.taxId = formData.get(`${"taxId"}`) ?? "";
    {
        const referenceNumberStr = formData.get(`${"referenceNumber"}`);
        obj.referenceNumber = referenceNumberStr ? parseFloat(referenceNumberStr as string) : $MfPh5;
        if (obj.referenceNumber !== undefined && isNaN(obj.referenceNumber as number)) obj.referenceNumber = "0";
    }
    obj.postalCodeLookup = formData.get(`${"postalCodeLookup"}`) ?? "";
    obj.timeZone = formData.get(`${"timeZone"}`) ?? "";
    obj.defaultTax = formData.get(`${"defaultTax"}`) ?? "";
    obj.defaultTaxLocation = formData.get(`${"defaultTaxLocation"}`) ?? "";
    {
        const defaultAreaCodeStr = formData.get(`${"defaultAreaCode"}`);
        obj.defaultAreaCode = defaultAreaCodeStr ? parseFloat(defaultAreaCodeStr as string) : $MfPh5;
        if (obj.defaultAreaCode !== undefined && isNaN(obj.defaultAreaCode as number)) obj.defaultAreaCode = "0";
    }
    obj.defaultAccountType = formData.get(`${"defaultAccountType"}`) ?? "";
    obj.lookupFormatting = formData.get(`${"lookupFormatting"}`) ?? "";
    obj.accountNameFormat = formData.get(`${"accountNameFormat"}`) ?? "";
    obj.merchantServiceProvider = formData.get(`${"merchantServiceProvider"}`) ?? "";
    obj.dateDisplayStyle = formData.get(`${"dateDisplayStyle"}`) ?? "";
    {
        const hasAutoCommissionVal = formData.get(`${"hasAutoCommission"}`);
        obj.hasAutoCommission = hasAutoCommissionVal === "true" || hasAutoCommissionVal === "on" || hasAutoCommissionVal === "1";
    }
    {
        const hasAutoDaylightSavingsVal = formData.get(`${"hasAutoDaylightSavings"}`);
        obj.hasAutoDaylightSavings = hasAutoDaylightSavingsVal === "true" || hasAutoDaylightSavingsVal === "on" || hasAutoDaylightSavingsVal === "1";
    }
    {
        const hasAutoFmsTrackingVal = formData.get(`${"hasAutoFmsTracking"}`);
        obj.hasAutoFmsTracking = hasAutoFmsTrackingVal === "true" || hasAutoFmsTrackingVal === "on" || hasAutoFmsTrackingVal === "1";
    }
    {
        const hasNotificationsVal = formData.get(`${"hasNotifications"}`);
        obj.hasNotifications = hasNotificationsVal === "true" || hasNotificationsVal === "on" || hasNotificationsVal === "1";
    }
    {
        const hasRequiredLeadSourceVal = formData.get(`${"hasRequiredLeadSource"}`);
        obj.hasRequiredLeadSource = hasRequiredLeadSourceVal === "true" || hasRequiredLeadSourceVal === "on" || hasRequiredLeadSourceVal === "1";
    }
    {
        const hasRequiredEmailVal = formData.get(`${"hasRequiredEmail"}`);
        obj.hasRequiredEmail = hasRequiredEmailVal === "true" || hasRequiredEmailVal === "on" || hasRequiredEmailVal === "1";
    }
    {
        const hasSortServiceItemsAlphabeticallyVal = formData.get(`${"hasSortServiceItemsAlphabetically"}`);
        obj.hasSortServiceItemsAlphabetically = hasSortServiceItemsAlphabeticallyVal === "true" || hasSortServiceItemsAlphabeticallyVal === "on" || hasSortServiceItemsAlphabeticallyVal === "1";
    }
    {
        const hasAttachOrderToAppointmentEmailsVal = formData.get(`${"hasAttachOrderToAppointmentEmails"}`);
        obj.hasAttachOrderToAppointmentEmails = hasAttachOrderToAppointmentEmailsVal === "true" || hasAttachOrderToAppointmentEmailsVal === "on" || hasAttachOrderToAppointmentEmailsVal === "1";
    }
    {
        const scheduleIntervalStr = formData.get(`${"scheduleInterval"}`);
        obj.scheduleInterval = scheduleIntervalStr ? parseFloat(scheduleIntervalStr as string) : $MfPh5;
        if (obj.scheduleInterval !== undefined && isNaN(obj.scheduleInterval as number)) obj.scheduleInterval = "0";
    }
    {
        const colorsConfigObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"colorsConfig"}.`)) {
                const fieldName = key.slice(`${"colorsConfig"}.`.length);
                const parts = fieldName.split(".");
                let current = colorsConfigObj;
                for(let i = 0; i < parts.length - 1; i++){
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.colorsConfig = colorsConfigObj;
    }
    return toExit("companyDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Company;
    readonly errors: CompanyErrors;
    readonly tainted: CompanyTainted;
    readonly fields: CompanyFieldControllers;
    validate(): Exit<Company, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Company>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function companyCreateForm(overrides: Partial<Company>): CompanyGigaform {}
let data = $state({
    ...companyDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as CompanyErrors);
let tainted = $state<$MfPh3>({} as CompanyTainted);
const fields = {} as CompanyFieldControllers;
fields.id = {
    label: `${"id"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.legalName = {
    label: `${"legalName"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.headquarters = {
    label: `${"headquarters"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.phones = {
    label: `${"phones"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.fax = {
    label: `${"fax"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.email = {
    label: `${"email"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.website = {
    label: `${"website"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.taxId = {
    label: `${"taxId"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.referenceNumber = {
    label: `${"referenceNumber"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.postalCodeLookup = {
    label: `${"postalCodeLookup"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.timeZone = {
    label: `${"timeZone"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.defaultTax = {
    label: `${"defaultTax"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.defaultTaxLocation = {
    label: `${"defaultTaxLocation"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.defaultAreaCode = {
    label: `${"defaultAreaCode"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.defaultAccountType = {
    label: `${"defaultAccountType"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.lookupFormatting = {
    label: `${"lookupFormatting"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.accountNameFormat = {
    label: `${"accountNameFormat"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.merchantServiceProvider = {
    label: `${"merchantServiceProvider"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.dateDisplayStyle = {
    label: `${"dateDisplayStyle"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.hasAutoCommission = {
    label: `${"hasAutoCommission"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.hasAutoDaylightSavings = {
    label: `${"hasAutoDaylightSavings"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.hasAutoFmsTracking = {
    label: `${"hasAutoFmsTracking"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.hasNotifications = {
    label: `${"hasNotifications"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.hasRequiredLeadSource = {
    label: `${"hasRequiredLeadSource"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.hasRequiredEmail = {
    label: `${"hasRequiredEmail"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.hasSortServiceItemsAlphabetically = {
    label: `${"hasSortServiceItemsAlphabetically"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.hasAttachOrderToAppointmentEmails = {
    label: `${"hasAttachOrderToAppointmentEmails"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.scheduleInterval = {
    label: `${"scheduleInterval"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.colorsConfig = {
    label: `${"colorsConfig"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<Company, Array<{
    field: string;
    message: string;
}>> {
    return toExit("companyDeserialize(data)");
    data = {
        ...companyDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Company = {
  defaultValue: companyDefaultValue,
  serialize: companySerialize,
  serializeWithContext: companySerializeWithContext,
  deserialize: companyDeserialize,
  deserializeWithContext: companyDeserializeWithContext,
  validateFields: companyValidateFields,
  hasShape: companyHasShape,
  is: companyIs,
  fromFormData: companyFromFormData,
  createForm: companyCreateForm
} as const;