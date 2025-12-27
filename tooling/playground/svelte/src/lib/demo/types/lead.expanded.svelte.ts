import { accountNameDefaultValue } from "./account-name.svelte";
import { emailDefaultValue } from "./email.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { accountNameSerializeWithContext } from "./account-name.svelte";
import { emailSerializeWithContext } from "./email.svelte";
import { leadStageSerializeWithContext } from "./lead-stage.svelte";
import { nextStepSerializeWithContext } from "./next-step.svelte";
import { phoneNumberSerializeWithContext } from "./phone-number.svelte";
import { prioritySerializeWithContext } from "./priority.svelte";
import { sectorSerializeWithContext } from "./sector.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { accountNameDeserializeWithContext } from "./account-name.svelte";
import { emailDeserializeWithContext } from "./email.svelte";
import { leadStageDeserializeWithContext } from "./lead-stage.svelte";
import { nextStepDeserializeWithContext } from "./next-step.svelte";
import { priorityDeserializeWithContext } from "./priority.svelte";
import { sectorDeserializeWithContext } from "./sector.svelte";
/** import macro {Gigaform} from "@playground/macro"; */

import type { AccountName } from './account-name.svelte';
import type { Email } from './email.svelte';
import type { LeadStage } from './lead-stage.svelte';
import type { NextStep } from './next-step.svelte';
import type { PhoneNumber } from './phone-number.svelte';
import type { Priority } from './priority.svelte';
import type { Represents } from './represents.svelte';
import type { Sector } from './sector.svelte';
import type { Site } from './site.svelte';
import type { TaxRate } from './tax-rate.svelte';


export interface Lead {
    
    id: string;
    
    number: number | null;
    
    accepted: boolean;
    
    probability: number;
    
    
    priority: Priority;
    
    dueDate: string | null;
    
    closeDate: string | null;
    
    value: number;
    
    
    stage: LeadStage;
    
    
    status: string;
    
    description: string | null;
    
    
    nextStep: NextStep;
    
    favorite: boolean;
    
    dateAdded: string | null;
    
    taxRate: (string | TaxRate) | null;
    
    
    sector: Sector;
    
    leadName: AccountName;
    
    phones: Array<PhoneNumber>;
    
    email: Email;
    
    leadSource: string | null;
    
    
    site: string | Site;
    
    
    memo: string;
    
    needsReview: boolean;
    
    hasAlert: boolean;
    
    salesRep: Array<Represents> | null;
    
    color: string | null;
    
    
    accountType: string;
    
    
    subtype: string;
    
    isTaxExempt: boolean;
    
    
    paymentTerms: string;
    
    tags: Array<string>;
    
    customFields: Array<[string, string]>;
}

export function leadDefaultValue(): Lead {
    return {
        id: "",
        number: null,
        accepted: false,
        probability: 0,
        priority: "Medium",
        dueDate: null,
        closeDate: null,
        value: 0,
        stage: "Open",
        status: "",
        description: null,
        nextStep: "InitialContact",
        favorite: false,
        dateAdded: null,
        taxRate: null,
        sector: "Residential",
        leadName: accountNameDefaultValue(),
        phones: [],
        email: emailDefaultValue(),
        leadSource: null,
        site: "",
        memo: "",
        needsReview: false,
        hasAlert: false,
        salesRep: null,
        color: null,
        accountType: "",
        subtype: "",
        isTaxExempt: false,
        paymentTerms: "",
        tags: [],
        customFields: []
    } as Lead;
}

export function leadSerialize(value: Lead): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(leadSerializeWithContext(value, ctx));
}
export function leadSerializeWithContext(value: Lead, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Lead"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"number"}`] = value.number;
    result[`${"accepted"}`] = value.accepted;
    result[`${"probability"}`] = value.probability;
    result[`${"priority"}`] = prioritySerializeWithContext(value.priority, ctx);
    result[`${"dueDate"}`] = value.dueDate;
    result[`${"closeDate"}`] = value.closeDate;
    result[`${"value"}`] = value.value;
    result[`${"stage"}`] = leadStageSerializeWithContext(value.stage, ctx);
    result[`${"status"}`] = value.status;
    result[`${"description"}`] = value.description;
    result[`${"nextStep"}`] = nextStepSerializeWithContext(value.nextStep, ctx);
    result[`${"favorite"}`] = value.favorite;
    result[`${"dateAdded"}`] = value.dateAdded;
    if (value.taxRate !== null) {
        result[`${"taxRate"}`] = value.taxRate;
    }
    result[`${"sector"}`] = sectorSerializeWithContext(value.sector, ctx);
    result[`${"leadName"}`] = accountNameSerializeWithContext(value.leadName, ctx);
    result[`${"phones"}`] = value.phones.map((item)=>phoneNumberSerializeWithContext(item, ctx));
    result[`${"email"}`] = emailSerializeWithContext(value.email, ctx);
    result[`${"leadSource"}`] = value.leadSource;
    result[`${"site"}`] = value.site;
    result[`${"memo"}`] = value.memo;
    result[`${"needsReview"}`] = value.needsReview;
    result[`${"hasAlert"}`] = value.hasAlert;
    if (value.salesRep !== null) {
        result[`${"salesRep"}`] = value.salesRep;
    }
    result[`${"color"}`] = value.color;
    result[`${"accountType"}`] = value.accountType;
    result[`${"subtype"}`] = value.subtype;
    result[`${"isTaxExempt"}`] = value.isTaxExempt;
    result[`${"paymentTerms"}`] = value.paymentTerms;
    result[`${"tags"}`] = value.tags;
    result[`${"customFields"}`] = value.customFields;
    return result;
}

export function leadDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Lead } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = leadDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Lead.deserialize: root cannot be a forward reference"
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
export function leadDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Lead | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Lead"}.deserializeWithContext: expected an object`
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
    if (!(`${"number"}` in obj)) {
        errors.push({
            field: `${"number"}`,
            message: "missing required field"
        });
    }
    if (!(`${"accepted"}` in obj)) {
        errors.push({
            field: `${"accepted"}`,
            message: "missing required field"
        });
    }
    if (!(`${"probability"}` in obj)) {
        errors.push({
            field: `${"probability"}`,
            message: "missing required field"
        });
    }
    if (!(`${"priority"}` in obj)) {
        errors.push({
            field: `${"priority"}`,
            message: "missing required field"
        });
    }
    if (!(`${"dueDate"}` in obj)) {
        errors.push({
            field: `${"dueDate"}`,
            message: "missing required field"
        });
    }
    if (!(`${"closeDate"}` in obj)) {
        errors.push({
            field: `${"closeDate"}`,
            message: "missing required field"
        });
    }
    if (!(`${"value"}` in obj)) {
        errors.push({
            field: `${"value"}`,
            message: "missing required field"
        });
    }
    if (!(`${"stage"}` in obj)) {
        errors.push({
            field: `${"stage"}`,
            message: "missing required field"
        });
    }
    if (!(`${"status"}` in obj)) {
        errors.push({
            field: `${"status"}`,
            message: "missing required field"
        });
    }
    if (!(`${"description"}` in obj)) {
        errors.push({
            field: `${"description"}`,
            message: "missing required field"
        });
    }
    if (!(`${"nextStep"}` in obj)) {
        errors.push({
            field: `${"nextStep"}`,
            message: "missing required field"
        });
    }
    if (!(`${"favorite"}` in obj)) {
        errors.push({
            field: `${"favorite"}`,
            message: "missing required field"
        });
    }
    if (!(`${"dateAdded"}` in obj)) {
        errors.push({
            field: `${"dateAdded"}`,
            message: "missing required field"
        });
    }
    if (!(`${"taxRate"}` in obj)) {
        errors.push({
            field: `${"taxRate"}`,
            message: "missing required field"
        });
    }
    if (!(`${"sector"}` in obj)) {
        errors.push({
            field: `${"sector"}`,
            message: "missing required field"
        });
    }
    if (!(`${"leadName"}` in obj)) {
        errors.push({
            field: `${"leadName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"phones"}` in obj)) {
        errors.push({
            field: `${"phones"}`,
            message: "missing required field"
        });
    }
    if (!(`${"email"}` in obj)) {
        errors.push({
            field: `${"email"}`,
            message: "missing required field"
        });
    }
    if (!(`${"leadSource"}` in obj)) {
        errors.push({
            field: `${"leadSource"}`,
            message: "missing required field"
        });
    }
    if (!(`${"site"}` in obj)) {
        errors.push({
            field: `${"site"}`,
            message: "missing required field"
        });
    }
    if (!(`${"memo"}` in obj)) {
        errors.push({
            field: `${"memo"}`,
            message: "missing required field"
        });
    }
    if (!(`${"needsReview"}` in obj)) {
        errors.push({
            field: `${"needsReview"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasAlert"}` in obj)) {
        errors.push({
            field: `${"hasAlert"}`,
            message: "missing required field"
        });
    }
    if (!(`${"salesRep"}` in obj)) {
        errors.push({
            field: `${"salesRep"}`,
            message: "missing required field"
        });
    }
    if (!(`${"color"}` in obj)) {
        errors.push({
            field: `${"color"}`,
            message: "missing required field"
        });
    }
    if (!(`${"accountType"}` in obj)) {
        errors.push({
            field: `${"accountType"}`,
            message: "missing required field"
        });
    }
    if (!(`${"subtype"}` in obj)) {
        errors.push({
            field: `${"subtype"}`,
            message: "missing required field"
        });
    }
    if (!(`${"isTaxExempt"}` in obj)) {
        errors.push({
            field: `${"isTaxExempt"}`,
            message: "missing required field"
        });
    }
    if (!(`${"paymentTerms"}` in obj)) {
        errors.push({
            field: `${"paymentTerms"}`,
            message: "missing required field"
        });
    }
    if (!(`${"tags"}` in obj)) {
        errors.push({
            field: `${"tags"}`,
            message: "missing required field"
        });
    }
    if (!(`${"customFields"}` in obj)) {
        errors.push({
            field: `${"customFields"}`,
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
        const __raw_number = obj[`${"number"}`] as number | null;
        instance.number = __raw_number;
    }
    {
        const __raw_accepted = obj[`${"accepted"}`] as boolean;
        instance.accepted = __raw_accepted;
    }
    {
        const __raw_probability = obj[`${"probability"}`] as number;
        instance.probability = __raw_probability;
    }
    {
        const __raw_priority = obj[`${"priority"}`] as Priority;
        {
            const __result = priorityDeserializeWithContext(__raw_priority, ctx);
            ctx.assignOrDefer(instance, `${"priority"}`, __result);
        }
    }
    {
        const __raw_dueDate = obj[`${"dueDate"}`] as string | null;
        instance.dueDate = __raw_dueDate;
    }
    {
        const __raw_closeDate = obj[`${"closeDate"}`] as string | null;
        instance.closeDate = __raw_closeDate;
    }
    {
        const __raw_value = obj[`${"value"}`] as number;
        instance.value = __raw_value;
    }
    {
        const __raw_stage = obj[`${"stage"}`] as LeadStage;
        {
            const __result = leadStageDeserializeWithContext(__raw_stage, ctx);
            ctx.assignOrDefer(instance, `${"stage"}`, __result);
        }
    }
    {
        const __raw_status = obj[`${"status"}`] as string;
        if (__raw_status.trim().length === 0) {
            errors.push({
                field: "status",
                message: "Lead.status must not be empty"
            });
        }
        instance.status = __raw_status;
    }
    {
        const __raw_description = obj[`${"description"}`] as string | null;
        instance.description = __raw_description;
    }
    {
        const __raw_nextStep = obj[`${"nextStep"}`] as NextStep;
        {
            const __result = nextStepDeserializeWithContext(__raw_nextStep, ctx);
            ctx.assignOrDefer(instance, `${"nextStep"}`, __result);
        }
    }
    {
        const __raw_favorite = obj[`${"favorite"}`] as boolean;
        instance.favorite = __raw_favorite;
    }
    {
        const __raw_dateAdded = obj[`${"dateAdded"}`] as string | null;
        instance.dateAdded = __raw_dateAdded;
    }
    {
        const __raw_taxRate = obj[`${"taxRate"}`] as (string | TaxRate) | null;
        if (__raw_taxRate === null) {
            instance.taxRate = null;
        } else {
            instance.taxRate = __raw_taxRate;
        }
    }
    {
        const __raw_sector = obj[`${"sector"}`] as Sector;
        {
            const __result = sectorDeserializeWithContext(__raw_sector, ctx);
            ctx.assignOrDefer(instance, `${"sector"}`, __result);
        }
    }
    {
        const __raw_leadName = obj[`${"leadName"}`] as AccountName;
        {
            const __result = accountNameDeserializeWithContext(__raw_leadName, ctx);
            ctx.assignOrDefer(instance, `${"leadName"}`, __result);
        }
    }
    {
        const __raw_phones = obj[`${"phones"}`] as Array<PhoneNumber>;
        if (Array.isArray(__raw_phones)) {
            instance.phones = __raw_phones as PhoneNumber[];
        }
    }
    {
        const __raw_email = obj[`${"email"}`] as Email;
        {
            const __result = emailDeserializeWithContext(__raw_email, ctx);
            ctx.assignOrDefer(instance, `${"email"}`, __result);
        }
    }
    {
        const __raw_leadSource = obj[`${"leadSource"}`] as string | null;
        instance.leadSource = __raw_leadSource;
    }
    {
        const __raw_site = obj[`${"site"}`] as string | Site;
        instance.site = __raw_site;
    }
    {
        const __raw_memo = obj[`${"memo"}`] as string;
        if (__raw_memo.trim().length === 0) {
            errors.push({
                field: "memo",
                message: "Lead.memo must not be empty"
            });
        }
        instance.memo = __raw_memo;
    }
    {
        const __raw_needsReview = obj[`${"needsReview"}`] as boolean;
        instance.needsReview = __raw_needsReview;
    }
    {
        const __raw_hasAlert = obj[`${"hasAlert"}`] as boolean;
        instance.hasAlert = __raw_hasAlert;
    }
    {
        const __raw_salesRep = obj[`${"salesRep"}`] as Array<Represents> | null;
        if (__raw_salesRep === null) {
            instance.salesRep = null;
        } else {
            instance.salesRep = __raw_salesRep;
        }
    }
    {
        const __raw_color = obj[`${"color"}`] as string | null;
        instance.color = __raw_color;
    }
    {
        const __raw_accountType = obj[`${"accountType"}`] as string;
        if (__raw_accountType.trim().length === 0) {
            errors.push({
                field: "accountType",
                message: "Lead.accountType must not be empty"
            });
        }
        instance.accountType = __raw_accountType;
    }
    {
        const __raw_subtype = obj[`${"subtype"}`] as string;
        if (__raw_subtype.trim().length === 0) {
            errors.push({
                field: "subtype",
                message: "Lead.subtype must not be empty"
            });
        }
        instance.subtype = __raw_subtype;
    }
    {
        const __raw_isTaxExempt = obj[`${"isTaxExempt"}`] as boolean;
        instance.isTaxExempt = __raw_isTaxExempt;
    }
    {
        const __raw_paymentTerms = obj[`${"paymentTerms"}`] as string;
        if (__raw_paymentTerms.trim().length === 0) {
            errors.push({
                field: "paymentTerms",
                message: "Lead.paymentTerms must not be empty"
            });
        }
        instance.paymentTerms = __raw_paymentTerms;
    }
    {
        const __raw_tags = obj[`${"tags"}`] as Array<string>;
        if (Array.isArray(__raw_tags)) {
            instance.tags = __raw_tags as string[];
        }
    }
    {
        const __raw_customFields = obj[`${"customFields"}`] as Array<[string, string]>;
        if (Array.isArray(__raw_customFields)) {
            instance.customFields = __raw_customFields as [string, string][];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Lead;
}
export function leadValidateField<K extends keyof Lead>(_field: K, _value: Lead[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"status"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "status",
                message: "Lead.status must not be empty"
            });
        }
    }
    if (_field === `${"memo"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "memo",
                message: "Lead.memo must not be empty"
            });
        }
    }
    if (_field === `${"accountType"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "accountType",
                message: "Lead.accountType must not be empty"
            });
        }
    }
    if (_field === `${"subtype"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "subtype",
                message: "Lead.subtype must not be empty"
            });
        }
    }
    if (_field === `${"paymentTerms"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "paymentTerms",
                message: "Lead.paymentTerms must not be empty"
            });
        }
    }
    return errors;
}
export function leadValidateFields(_partial: Partial<Lead>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"status"}` in _partial && _partial.status !== undefined) {
        const __val = _partial.status as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "status",
                message: "Lead.status must not be empty"
            });
        }
    }
    if (`${"memo"}` in _partial && _partial.memo !== undefined) {
        const __val = _partial.memo as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "memo",
                message: "Lead.memo must not be empty"
            });
        }
    }
    if (`${"accountType"}` in _partial && _partial.accountType !== undefined) {
        const __val = _partial.accountType as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "accountType",
                message: "Lead.accountType must not be empty"
            });
        }
    }
    if (`${"subtype"}` in _partial && _partial.subtype !== undefined) {
        const __val = _partial.subtype as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "subtype",
                message: "Lead.subtype must not be empty"
            });
        }
    }
    if (`${"paymentTerms"}` in _partial && _partial.paymentTerms !== undefined) {
        const __val = _partial.paymentTerms as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "paymentTerms",
                message: "Lead.paymentTerms must not be empty"
            });
        }
    }
    return errors;
}
export function leadHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "number" in o && "accepted" in o && "probability" in o && "priority" in o && "dueDate" in o && "closeDate" in o && "value" in o && "stage" in o && "status" in o && "description" in o && "nextStep" in o && "favorite" in o && "dateAdded" in o && "taxRate" in o && "sector" in o && "leadName" in o && "phones" in o && "email" in o && "leadSource" in o && "site" in o && "memo" in o && "needsReview" in o && "hasAlert" in o && "salesRep" in o && "color" in o && "accountType" in o && "subtype" in o && "isTaxExempt" in o && "paymentTerms" in o && "tags" in o && "customFields" in o';
}
export function leadIs(obj: unknown): obj is Lead {
    if (!leadHasShape(obj)) {
        return false;
    }
    const result = leadDeserialize(obj);
    return result.success;
}

export const Lead = {
  defaultValue: leadDefaultValue,
  serialize: leadSerialize,
  serializeWithContext: leadSerializeWithContext,
  deserialize: leadDeserialize,
  deserializeWithContext: leadDeserializeWithContext,
  validateFields: leadValidateFields,
  hasShape: leadHasShape,
  is: leadIs
} as const;