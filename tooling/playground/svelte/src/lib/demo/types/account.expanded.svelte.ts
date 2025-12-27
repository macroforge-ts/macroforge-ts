import { accountNameDefaultValue } from "./account-name.svelte";
import { colorsDefaultValue } from "./colors.svelte";
import { emailDefaultValue } from "./email.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { accountNameSerializeWithContext } from "./account-name.svelte";
import { colorsSerializeWithContext } from "./colors.svelte";
import { didSerializeWithContext } from "./did.svelte";
import { emailSerializeWithContext } from "./email.svelte";
import { orderedSerializeWithContext } from "./ordered.svelte";
import { phoneNumberSerializeWithContext } from "./phone-number.svelte";
import { sectorSerializeWithContext } from "./sector.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { accountNameDeserializeWithContext } from "./account-name.svelte";
import { colorsDeserializeWithContext } from "./colors.svelte";
import { emailDeserializeWithContext } from "./email.svelte";
import { sectorDeserializeWithContext } from "./sector.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
import type { ArrayFieldController } from "@playground/macro/gigaform";
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


export interface Account {
    
    id: string;
    
    
    taxRate: string | TaxRate;
    
    
    site: string | Site;
    
    salesRep: Array<Represents> | null;
    
    orders: Array<Ordered>;
    
    activity: Array<Did>;
    
    customFields: Array<[string, string]>;
    
    accountName: AccountName;
    
    
    sector: Sector;
    
    memo: string | null;
    
    phones: Array<PhoneNumber>;
    
    email: Email;
    
    
    leadSource: string;
    
    colors: Colors;
    
    needsReview: boolean;
    
    hasAlert: boolean;
    
    
    accountType: string;
    
    
    subtype: string;
    
    isTaxExempt: boolean;
    
    
    paymentTerms: string;
    
    tags: Array<string>;
    
    dateAdded: string;
}

export function accountDefaultValue(): Account {
    return {
        id: "",
        taxRate: "",
        site: "",
        salesRep: null,
        orders: [],
        activity: [],
        customFields: [],
        accountName: accountNameDefaultValue(),
        sector: "Residential",
        memo: null,
        phones: [],
        email: emailDefaultValue(),
        leadSource: "",
        colors: colorsDefaultValue(),
        needsReview: false,
        hasAlert: false,
        accountType: "",
        subtype: "",
        isTaxExempt: false,
        paymentTerms: "",
        tags: [],
        dateAdded: ""
    } as Account;
}

export function accountSerialize(value: Account): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(accountSerializeWithContext(value, ctx));
}
export function accountSerializeWithContext(value: Account, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Account"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"taxRate"}`] = value.taxRate;
    result[`${"site"}`] = value.site;
    if (value.salesRep !== null) {
        result[`${"salesRep"}`] = value.salesRep;
    }
    result[`${"orders"}`] = value.orders.map((item)=>orderedSerializeWithContext(item, ctx));
    result[`${"activity"}`] = value.activity.map((item)=>didSerializeWithContext(item, ctx));
    result[`${"customFields"}`] = value.customFields;
    result[`${"accountName"}`] = accountNameSerializeWithContext(value.accountName, ctx);
    result[`${"sector"}`] = sectorSerializeWithContext(value.sector, ctx);
    result[`${"memo"}`] = value.memo;
    result[`${"phones"}`] = value.phones.map((item)=>phoneNumberSerializeWithContext(item, ctx));
    result[`${"email"}`] = emailSerializeWithContext(value.email, ctx);
    result[`${"leadSource"}`] = value.leadSource;
    result[`${"colors"}`] = colorsSerializeWithContext(value.colors, ctx);
    result[`${"needsReview"}`] = value.needsReview;
    result[`${"hasAlert"}`] = value.hasAlert;
    result[`${"accountType"}`] = value.accountType;
    result[`${"subtype"}`] = value.subtype;
    result[`${"isTaxExempt"}`] = value.isTaxExempt;
    result[`${"paymentTerms"}`] = value.paymentTerms;
    result[`${"tags"}`] = value.tags;
    result[`${"dateAdded"}`] = value.dateAdded;
    return result;
}

export function accountDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Account } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = accountDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Account.deserialize: root cannot be a forward reference"
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
export function accountDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Account | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Account"}.deserializeWithContext: expected an object`
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
    if (!(`${"taxRate"}` in obj)) {
        errors.push({
            field: `${"taxRate"}`,
            message: "missing required field"
        });
    }
    if (!(`${"site"}` in obj)) {
        errors.push({
            field: `${"site"}`,
            message: "missing required field"
        });
    }
    if (!(`${"salesRep"}` in obj)) {
        errors.push({
            field: `${"salesRep"}`,
            message: "missing required field"
        });
    }
    if (!(`${"orders"}` in obj)) {
        errors.push({
            field: `${"orders"}`,
            message: "missing required field"
        });
    }
    if (!(`${"activity"}` in obj)) {
        errors.push({
            field: `${"activity"}`,
            message: "missing required field"
        });
    }
    if (!(`${"customFields"}` in obj)) {
        errors.push({
            field: `${"customFields"}`,
            message: "missing required field"
        });
    }
    if (!(`${"accountName"}` in obj)) {
        errors.push({
            field: `${"accountName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"sector"}` in obj)) {
        errors.push({
            field: `${"sector"}`,
            message: "missing required field"
        });
    }
    if (!(`${"memo"}` in obj)) {
        errors.push({
            field: `${"memo"}`,
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
    if (!(`${"colors"}` in obj)) {
        errors.push({
            field: `${"colors"}`,
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
    if (!(`${"dateAdded"}` in obj)) {
        errors.push({
            field: `${"dateAdded"}`,
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
        const __raw_taxRate = obj[`${"taxRate"}`] as string | TaxRate;
        instance.taxRate = __raw_taxRate;
    }
    {
        const __raw_site = obj[`${"site"}`] as string | Site;
        instance.site = __raw_site;
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
        const __raw_orders = obj[`${"orders"}`] as Array<Ordered>;
        if (Array.isArray(__raw_orders)) {
            instance.orders = __raw_orders as Ordered[];
        }
    }
    {
        const __raw_activity = obj[`${"activity"}`] as Array<Did>;
        if (Array.isArray(__raw_activity)) {
            instance.activity = __raw_activity as Did[];
        }
    }
    {
        const __raw_customFields = obj[`${"customFields"}`] as Array<[string, string]>;
        if (Array.isArray(__raw_customFields)) {
            instance.customFields = __raw_customFields as [string, string][];
        }
    }
    {
        const __raw_accountName = obj[`${"accountName"}`] as AccountName;
        {
            const __result = accountNameDeserializeWithContext(__raw_accountName, ctx);
            ctx.assignOrDefer(instance, `${"accountName"}`, __result);
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
        const __raw_memo = obj[`${"memo"}`] as string | null;
        instance.memo = __raw_memo;
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
        const __raw_leadSource = obj[`${"leadSource"}`] as string;
        if (__raw_leadSource.trim().length === 0) {
            errors.push({
                field: "leadSource",
                message: "Account.leadSource must not be empty"
            });
        }
        instance.leadSource = __raw_leadSource;
    }
    {
        const __raw_colors = obj[`${"colors"}`] as Colors;
        {
            const __result = colorsDeserializeWithContext(__raw_colors, ctx);
            ctx.assignOrDefer(instance, `${"colors"}`, __result);
        }
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
        const __raw_accountType = obj[`${"accountType"}`] as string;
        if (__raw_accountType.trim().length === 0) {
            errors.push({
                field: "accountType",
                message: "Account.accountType must not be empty"
            });
        }
        instance.accountType = __raw_accountType;
    }
    {
        const __raw_subtype = obj[`${"subtype"}`] as string;
        if (__raw_subtype.trim().length === 0) {
            errors.push({
                field: "subtype",
                message: "Account.subtype must not be empty"
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
                message: "Account.paymentTerms must not be empty"
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
        const __raw_dateAdded = obj[`${"dateAdded"}`] as string;
        instance.dateAdded = __raw_dateAdded;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Account;
}
export function accountValidateField<K extends keyof Account>(_field: K, _value: Account[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"leadSource"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "leadSource",
                message: "Account.leadSource must not be empty"
            });
        }
    }
    if (_field === `${"accountType"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "accountType",
                message: "Account.accountType must not be empty"
            });
        }
    }
    if (_field === `${"subtype"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "subtype",
                message: "Account.subtype must not be empty"
            });
        }
    }
    if (_field === `${"paymentTerms"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "paymentTerms",
                message: "Account.paymentTerms must not be empty"
            });
        }
    }
    return errors;
}
export function accountValidateFields(_partial: Partial<Account>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"leadSource"}` in _partial && _partial.leadSource !== undefined) {
        const __val = _partial.leadSource as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "leadSource",
                message: "Account.leadSource must not be empty"
            });
        }
    }
    if (`${"accountType"}` in _partial && _partial.accountType !== undefined) {
        const __val = _partial.accountType as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "accountType",
                message: "Account.accountType must not be empty"
            });
        }
    }
    if (`${"subtype"}` in _partial && _partial.subtype !== undefined) {
        const __val = _partial.subtype as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "subtype",
                message: "Account.subtype must not be empty"
            });
        }
    }
    if (`${"paymentTerms"}` in _partial && _partial.paymentTerms !== undefined) {
        const __val = _partial.paymentTerms as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "paymentTerms",
                message: "Account.paymentTerms must not be empty"
            });
        }
    }
    return errors;
}
export function accountHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "taxRate" in o && "site" in o && "salesRep" in o && "orders" in o && "activity" in o && "customFields" in o && "accountName" in o && "sector" in o && "memo" in o && "phones" in o && "email" in o && "leadSource" in o && "colors" in o && "needsReview" in o && "hasAlert" in o && "accountType" in o && "subtype" in o && "isTaxExempt" in o && "paymentTerms" in o && "tags" in o && "dateAdded" in o';
}
export function accountIs(obj: unknown): obj is Account {
    if (!accountHasShape(obj)) {
        return false;
    }
    const result = accountDeserialize(obj);
    return result.success;
}

export function accountFromFormData(formData: FormData): Exit<Account, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get(`${"id"}`) ?? "";
    obj.taxRate = formData.get(`${"taxRate"}`) ?? "";
    obj.site = formData.get(`${"site"}`) ?? "";
    obj.salesRep = formData.get(`${"salesRep"}`) ?? "";
    {
        const ordersItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while(formData.has(`${"orders"}.` + idx + ".") || idx === 0){
            const hasAny = Array.from(formData.keys()).some((k)=>k.startsWith(`${"orders"}.` + idx + "."));
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())){
                    if (key.startsWith(`${"orders"}.` + idx + ".")) {
                        const fieldName = key.slice(`${"orders"}.`.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                ordersItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.orders = ordersItems;
    }
    {
        const activityItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while(formData.has(`${"activity"}.` + idx + ".") || idx === 0){
            const hasAny = Array.from(formData.keys()).some((k)=>k.startsWith(`${"activity"}.` + idx + "."));
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())){
                    if (key.startsWith(`${"activity"}.` + idx + ".")) {
                        const fieldName = key.slice(`${"activity"}.`.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                activityItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.activity = activityItems;
    }
    {
        const customFieldsItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while(formData.has(`${"customFields"}.` + idx + ".") || idx === 0){
            const hasAny = Array.from(formData.keys()).some((k)=>k.startsWith(`${"customFields"}.` + idx + "."));
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())){
                    if (key.startsWith(`${"customFields"}.` + idx + ".")) {
                        const fieldName = key.slice(`${"customFields"}.`.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                customFieldsItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.customFields = customFieldsItems;
    }
    {
        const accountNameObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"accountName"}.`)) {
                const fieldName = key.slice(`${"accountName"}.`.length);
                const parts = fieldName.split(".");
                let current = accountNameObj;
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
        obj.accountName = accountNameObj;
    }
    {
        const sectorObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"sector"}.`)) {
                const fieldName = key.slice(`${"sector"}.`.length);
                const parts = fieldName.split(".");
                let current = sectorObj;
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
        obj.sector = sectorObj;
    }
    obj.memo = formData.get(`${"memo"}`) ?? "";
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
    {
        const emailObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"email"}.`)) {
                const fieldName = key.slice(`${"email"}.`.length);
                const parts = fieldName.split(".");
                let current = emailObj;
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
        obj.email = emailObj;
    }
    obj.leadSource = formData.get(`${"leadSource"}`) ?? "";
    {
        const colorsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"colors"}.`)) {
                const fieldName = key.slice(`${"colors"}.`.length);
                const parts = fieldName.split(".");
                let current = colorsObj;
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
        obj.colors = colorsObj;
    }
    {
        const needsReviewVal = formData.get(`${"needsReview"}`);
        obj.needsReview = needsReviewVal === "true" || needsReviewVal === "on" || needsReviewVal === "1";
    }
    {
        const hasAlertVal = formData.get(`${"hasAlert"}`);
        obj.hasAlert = hasAlertVal === "true" || hasAlertVal === "on" || hasAlertVal === "1";
    }
    obj.accountType = formData.get(`${"accountType"}`) ?? "";
    obj.subtype = formData.get(`${"subtype"}`) ?? "";
    {
        const isTaxExemptVal = formData.get(`${"isTaxExempt"}`);
        obj.isTaxExempt = isTaxExemptVal === "true" || isTaxExemptVal === "on" || isTaxExemptVal === "1";
    }
    obj.paymentTerms = formData.get(`${"paymentTerms"}`) ?? "";
    obj.tags = formData.getAll(`${"tags"}`) as Array<string>;
    obj.dateAdded = formData.get(`${"dateAdded"}`) ?? "";
    return toExit("accountDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Account;
    readonly errors: AccountErrors;
    readonly tainted: AccountTainted;
    readonly fields: AccountFieldControllers;
    validate(): Exit<Account, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Account>): void;
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
 }; export function accountCreateForm(overrides: Partial<Account>): AccountGigaform {}
let data = $state({
    ...accountDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as AccountErrors);
let tainted = $state<$MfPh3>({} as AccountTainted);
const fields = {} as AccountFieldControllers;
fields.id = {
    label: `${"id"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.taxRate = {
    label: `${"taxRate"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.site = {
    label: `${"site"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.salesRep = {
    label: `${"salesRep"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.orders = {
    label: `${"orders"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.activity = {
    label: `${"activity"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.customFields = {
    label: `${"customFields"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.accountName = {
    label: `${"accountName"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.sector = {
    label: `${"sector"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.memo = {
    label: `${"memo"}`,
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
fields.email = {
    label: `${"email"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.leadSource = {
    label: `${"leadSource"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.colors = {
    label: `${"colors"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.needsReview = {
    label: `${"needsReview"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.hasAlert = {
    label: `${"hasAlert"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.accountType = {
    label: `${"accountType"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.subtype = {
    label: `${"subtype"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.isTaxExempt = {
    label: `${"isTaxExempt"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.paymentTerms = {
    label: `${"paymentTerms"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.tags = {
    label: `${"tags"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.dateAdded = {
    label: `${"dateAdded"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<Account, Array<{
    field: string;
    message: string;
}>> {
    return toExit("accountDeserialize(data)");
    data = {
        ...accountDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Account = {
  defaultValue: accountDefaultValue,
  serialize: accountSerialize,
  serializeWithContext: accountSerializeWithContext,
  deserialize: accountDeserialize,
  deserializeWithContext: accountDeserializeWithContext,
  validateFields: accountValidateFields,
  hasShape: accountHasShape,
  is: accountIs,
  fromFormData: accountFromFormData,
  createForm: accountCreateForm
} as const;