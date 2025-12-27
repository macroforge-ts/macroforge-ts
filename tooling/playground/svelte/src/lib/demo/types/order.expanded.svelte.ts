import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { billedItemSerializeWithContext } from "./billed-item.svelte";
import { orderStageSerializeWithContext } from "./order-stage.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { orderStageDeserializeWithContext } from "./order-stage.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
import type { ArrayFieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { DateTime } from 'effect';
import type { Option } from 'effect/Option';
import type { Account } from './account.svelte';
import type { Appointment } from './appointment.svelte';
import type { BilledItem } from './billed-item.svelte';
import type { Employee } from './employee.svelte';
import type { OrderStage } from './order-stage.svelte';
import type { Package } from './package.svelte';
import type { Payment } from './payment.svelte';
import type { Promotion } from './promotion.svelte';
import type { Site } from './site.svelte';


export interface Order {
    
    id: string;
    
    
    account: string | Account;
    
    
    stage: OrderStage;
    
    number: number;
    
    payments: Array<string | Payment>;
    
    
    opportunity: string;
    
    
    reference: string;
    
    
    leadSource: string;
    
    
    salesRep: string | Employee;
    
    
    group: string;
    
    
    subgroup: string;
    
    isPosted: boolean;
    
    needsReview: boolean;
    
    
    actionItem: string;
    
    upsale: number;
    
    dateCreated: DateTime.DateTime;
    
    
    appointment: string | Appointment;
    
    lastTechs: Array<string | Employee>;
    
    package: Array<string | Package> | null;
    
    promotion: Array<string | Promotion> | null;
    
    balance: number;
    
    due: DateTime.DateTime;
    
    total: number;
    
    
    site: string | Site;
    
    billedItems: Array<BilledItem>;
    
    memo: Option<string>;
    
    discount: number;
    
    tip: number;
    
    commissions: Array<number>;
}

export function orderDefaultValue(): Order {
    return {
        id: "",
        account: "",
        stage: "Estimate",
        number: 0,
        payments: [],
        opportunity: "",
        reference: "",
        leadSource: "",
        salesRep: "",
        group: "",
        subgroup: "",
        isPosted: false,
        needsReview: false,
        actionItem: "",
        upsale: 0,
        dateCreated: dateTime.dateTimeDefaultValue(),
        appointment: "",
        lastTechs: [],
        package: null,
        promotion: null,
        balance: 0,
        due: dateTime.dateTimeDefaultValue(),
        total: 0,
        site: "",
        billedItems: [],
        memo: optionDefaultValue<string>(),
        discount: 0,
        tip: 0,
        commissions: []
    } as Order;
}

export function orderSerialize(value: Order): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(orderSerializeWithContext(value, ctx));
}
export function orderSerializeWithContext(value: Order, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Order"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"account"}`] = value.account;
    result[`${"stage"}`] = orderStageSerializeWithContext(value.stage, ctx);
    result[`${"number"}`] = value.number;
    result[`${"payments"}`] = value.payments;
    result[`${"opportunity"}`] = value.opportunity;
    result[`${"reference"}`] = value.reference;
    result[`${"leadSource"}`] = value.leadSource;
    result[`${"salesRep"}`] = value.salesRep;
    result[`${"group"}`] = value.group;
    result[`${"subgroup"}`] = value.subgroup;
    result[`${"isPosted"}`] = value.isPosted;
    result[`${"needsReview"}`] = value.needsReview;
    result[`${"actionItem"}`] = value.actionItem;
    result[`${"upsale"}`] = value.upsale;
    result[`${"dateCreated"}`] = dateTime.dateTimeSerializeWithContext(value.dateCreated, ctx);
    result[`${"appointment"}`] = value.appointment;
    result[`${"lastTechs"}`] = value.lastTechs;
    if (value.package !== null) {
        result[`${"package"}`] = value.package;
    }
    if (value.promotion !== null) {
        result[`${"promotion"}`] = value.promotion;
    }
    result[`${"balance"}`] = value.balance;
    result[`${"due"}`] = dateTime.dateTimeSerializeWithContext(value.due, ctx);
    result[`${"total"}`] = value.total;
    result[`${"site"}`] = value.site;
    result[`${"billedItems"}`] = value.billedItems.map((item)=>billedItemSerializeWithContext(item, ctx));
    result[`${"memo"}`] = option<string>SerializeWithContext(value.memo, ctx);
    result[`${"discount"}`] = value.discount;
    result[`${"tip"}`] = value.tip;
    result[`${"commissions"}`] = value.commissions;
    return result;
}

export function orderDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Order } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = orderDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Order.deserialize: root cannot be a forward reference"
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
export function orderDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Order | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Order"}.deserializeWithContext: expected an object`
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
    if (!(`${"account"}` in obj)) {
        errors.push({
            field: `${"account"}`,
            message: "missing required field"
        });
    }
    if (!(`${"stage"}` in obj)) {
        errors.push({
            field: `${"stage"}`,
            message: "missing required field"
        });
    }
    if (!(`${"number"}` in obj)) {
        errors.push({
            field: `${"number"}`,
            message: "missing required field"
        });
    }
    if (!(`${"payments"}` in obj)) {
        errors.push({
            field: `${"payments"}`,
            message: "missing required field"
        });
    }
    if (!(`${"opportunity"}` in obj)) {
        errors.push({
            field: `${"opportunity"}`,
            message: "missing required field"
        });
    }
    if (!(`${"reference"}` in obj)) {
        errors.push({
            field: `${"reference"}`,
            message: "missing required field"
        });
    }
    if (!(`${"leadSource"}` in obj)) {
        errors.push({
            field: `${"leadSource"}`,
            message: "missing required field"
        });
    }
    if (!(`${"salesRep"}` in obj)) {
        errors.push({
            field: `${"salesRep"}`,
            message: "missing required field"
        });
    }
    if (!(`${"group"}` in obj)) {
        errors.push({
            field: `${"group"}`,
            message: "missing required field"
        });
    }
    if (!(`${"subgroup"}` in obj)) {
        errors.push({
            field: `${"subgroup"}`,
            message: "missing required field"
        });
    }
    if (!(`${"isPosted"}` in obj)) {
        errors.push({
            field: `${"isPosted"}`,
            message: "missing required field"
        });
    }
    if (!(`${"needsReview"}` in obj)) {
        errors.push({
            field: `${"needsReview"}`,
            message: "missing required field"
        });
    }
    if (!(`${"actionItem"}` in obj)) {
        errors.push({
            field: `${"actionItem"}`,
            message: "missing required field"
        });
    }
    if (!(`${"upsale"}` in obj)) {
        errors.push({
            field: `${"upsale"}`,
            message: "missing required field"
        });
    }
    if (!(`${"dateCreated"}` in obj)) {
        errors.push({
            field: `${"dateCreated"}`,
            message: "missing required field"
        });
    }
    if (!(`${"appointment"}` in obj)) {
        errors.push({
            field: `${"appointment"}`,
            message: "missing required field"
        });
    }
    if (!(`${"lastTechs"}` in obj)) {
        errors.push({
            field: `${"lastTechs"}`,
            message: "missing required field"
        });
    }
    if (!(`${"package"}` in obj)) {
        errors.push({
            field: `${"package"}`,
            message: "missing required field"
        });
    }
    if (!(`${"promotion"}` in obj)) {
        errors.push({
            field: `${"promotion"}`,
            message: "missing required field"
        });
    }
    if (!(`${"balance"}` in obj)) {
        errors.push({
            field: `${"balance"}`,
            message: "missing required field"
        });
    }
    if (!(`${"due"}` in obj)) {
        errors.push({
            field: `${"due"}`,
            message: "missing required field"
        });
    }
    if (!(`${"total"}` in obj)) {
        errors.push({
            field: `${"total"}`,
            message: "missing required field"
        });
    }
    if (!(`${"site"}` in obj)) {
        errors.push({
            field: `${"site"}`,
            message: "missing required field"
        });
    }
    if (!(`${"billedItems"}` in obj)) {
        errors.push({
            field: `${"billedItems"}`,
            message: "missing required field"
        });
    }
    if (!(`${"memo"}` in obj)) {
        errors.push({
            field: `${"memo"}`,
            message: "missing required field"
        });
    }
    if (!(`${"discount"}` in obj)) {
        errors.push({
            field: `${"discount"}`,
            message: "missing required field"
        });
    }
    if (!(`${"tip"}` in obj)) {
        errors.push({
            field: `${"tip"}`,
            message: "missing required field"
        });
    }
    if (!(`${"commissions"}` in obj)) {
        errors.push({
            field: `${"commissions"}`,
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
        const __raw_account = obj[`${"account"}`] as string | Account;
        instance.account = __raw_account;
    }
    {
        const __raw_stage = obj[`${"stage"}`] as OrderStage;
        {
            const __result = orderStageDeserializeWithContext(__raw_stage, ctx);
            ctx.assignOrDefer(instance, `${"stage"}`, __result);
        }
    }
    {
        const __raw_number = obj[`${"number"}`] as number;
        instance.number = __raw_number;
    }
    {
        const __raw_payments = obj[`${"payments"}`] as Array<string | Payment>;
        if (Array.isArray(__raw_payments)) {
            instance.payments = __raw_payments as string | Payment[];
        }
    }
    {
        const __raw_opportunity = obj[`${"opportunity"}`] as string;
        if (__raw_opportunity.trim().length === 0) {
            errors.push({
                field: "opportunity",
                message: "Order.opportunity must not be empty"
            });
        }
        instance.opportunity = __raw_opportunity;
    }
    {
        const __raw_reference = obj[`${"reference"}`] as string;
        if (__raw_reference.trim().length === 0) {
            errors.push({
                field: "reference",
                message: "Order.reference must not be empty"
            });
        }
        instance.reference = __raw_reference;
    }
    {
        const __raw_leadSource = obj[`${"leadSource"}`] as string;
        if (__raw_leadSource.trim().length === 0) {
            errors.push({
                field: "leadSource",
                message: "Order.leadSource must not be empty"
            });
        }
        instance.leadSource = __raw_leadSource;
    }
    {
        const __raw_salesRep = obj[`${"salesRep"}`] as string | Employee;
        instance.salesRep = __raw_salesRep;
    }
    {
        const __raw_group = obj[`${"group"}`] as string;
        if (__raw_group.trim().length === 0) {
            errors.push({
                field: "group",
                message: "Order.group must not be empty"
            });
        }
        instance.group = __raw_group;
    }
    {
        const __raw_subgroup = obj[`${"subgroup"}`] as string;
        if (__raw_subgroup.trim().length === 0) {
            errors.push({
                field: "subgroup",
                message: "Order.subgroup must not be empty"
            });
        }
        instance.subgroup = __raw_subgroup;
    }
    {
        const __raw_isPosted = obj[`${"isPosted"}`] as boolean;
        instance.isPosted = __raw_isPosted;
    }
    {
        const __raw_needsReview = obj[`${"needsReview"}`] as boolean;
        instance.needsReview = __raw_needsReview;
    }
    {
        const __raw_actionItem = obj[`${"actionItem"}`] as string;
        if (__raw_actionItem.trim().length === 0) {
            errors.push({
                field: "actionItem",
                message: "Order.actionItem must not be empty"
            });
        }
        instance.actionItem = __raw_actionItem;
    }
    {
        const __raw_upsale = obj[`${"upsale"}`] as number;
        instance.upsale = __raw_upsale;
    }
    {
        const __raw_dateCreated = obj[`${"dateCreated"}`] as DateTime.DateTime;
        {
            const __result = dateTime.dateTimeDeserializeWithContext(__raw_dateCreated, ctx);
            ctx.assignOrDefer(instance, `${"dateCreated"}`, __result);
        }
    }
    {
        const __raw_appointment = obj[`${"appointment"}`] as string | Appointment;
        instance.appointment = __raw_appointment;
    }
    {
        const __raw_lastTechs = obj[`${"lastTechs"}`] as Array<string | Employee>;
        if (Array.isArray(__raw_lastTechs)) {
            instance.lastTechs = __raw_lastTechs as string | Employee[];
        }
    }
    {
        const __raw_package = obj[`${"package"}`] as Array<string | Package> | null;
        if (__raw_package === null) {
            instance.package = null;
        } else {
            instance.package = __raw_package;
        }
    }
    {
        const __raw_promotion = obj[`${"promotion"}`] as Array<string | Promotion> | null;
        if (__raw_promotion === null) {
            instance.promotion = null;
        } else {
            instance.promotion = __raw_promotion;
        }
    }
    {
        const __raw_balance = obj[`${"balance"}`] as number;
        instance.balance = __raw_balance;
    }
    {
        const __raw_due = obj[`${"due"}`] as DateTime.DateTime;
        {
            const __result = dateTime.dateTimeDeserializeWithContext(__raw_due, ctx);
            ctx.assignOrDefer(instance, `${"due"}`, __result);
        }
    }
    {
        const __raw_total = obj[`${"total"}`] as number;
        instance.total = __raw_total;
    }
    {
        const __raw_site = obj[`${"site"}`] as string | Site;
        instance.site = __raw_site;
    }
    {
        const __raw_billedItems = obj[`${"billedItems"}`] as Array<BilledItem>;
        if (Array.isArray(__raw_billedItems)) {
            instance.billedItems = __raw_billedItems as BilledItem[];
        }
    }
    {
        const __raw_memo = obj[`${"memo"}`] as Option<string>;
        {
            const __result = option<string>DeserializeWithContext(__raw_memo, ctx);
            ctx.assignOrDefer(instance, `${"memo"}`, __result);
        }
    }
    {
        const __raw_discount = obj[`${"discount"}`] as number;
        instance.discount = __raw_discount;
    }
    {
        const __raw_tip = obj[`${"tip"}`] as number;
        instance.tip = __raw_tip;
    }
    {
        const __raw_commissions = obj[`${"commissions"}`] as Array<number>;
        if (Array.isArray(__raw_commissions)) {
            instance.commissions = __raw_commissions as number[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Order;
}
export function orderValidateField<K extends keyof Order>(_field: K, _value: Order[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"opportunity"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "opportunity",
                message: "Order.opportunity must not be empty"
            });
        }
    }
    if (_field === `${"reference"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "reference",
                message: "Order.reference must not be empty"
            });
        }
    }
    if (_field === `${"leadSource"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "leadSource",
                message: "Order.leadSource must not be empty"
            });
        }
    }
    if (_field === `${"group"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "group",
                message: "Order.group must not be empty"
            });
        }
    }
    if (_field === `${"subgroup"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "subgroup",
                message: "Order.subgroup must not be empty"
            });
        }
    }
    if (_field === `${"actionItem"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "actionItem",
                message: "Order.actionItem must not be empty"
            });
        }
    }
    return errors;
}
export function orderValidateFields(_partial: Partial<Order>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"opportunity"}` in _partial && _partial.opportunity !== undefined) {
        const __val = _partial.opportunity as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "opportunity",
                message: "Order.opportunity must not be empty"
            });
        }
    }
    if (`${"reference"}` in _partial && _partial.reference !== undefined) {
        const __val = _partial.reference as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "reference",
                message: "Order.reference must not be empty"
            });
        }
    }
    if (`${"leadSource"}` in _partial && _partial.leadSource !== undefined) {
        const __val = _partial.leadSource as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "leadSource",
                message: "Order.leadSource must not be empty"
            });
        }
    }
    if (`${"group"}` in _partial && _partial.group !== undefined) {
        const __val = _partial.group as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "group",
                message: "Order.group must not be empty"
            });
        }
    }
    if (`${"subgroup"}` in _partial && _partial.subgroup !== undefined) {
        const __val = _partial.subgroup as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "subgroup",
                message: "Order.subgroup must not be empty"
            });
        }
    }
    if (`${"actionItem"}` in _partial && _partial.actionItem !== undefined) {
        const __val = _partial.actionItem as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "actionItem",
                message: "Order.actionItem must not be empty"
            });
        }
    }
    return errors;
}
export function orderHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "account" in o && "stage" in o && "number" in o && "payments" in o && "opportunity" in o && "reference" in o && "leadSource" in o && "salesRep" in o && "group" in o && "subgroup" in o && "isPosted" in o && "needsReview" in o && "actionItem" in o && "upsale" in o && "dateCreated" in o && "appointment" in o && "lastTechs" in o && "package" in o && "promotion" in o && "balance" in o && "due" in o && "total" in o && "site" in o && "billedItems" in o && "memo" in o && "discount" in o && "tip" in o && "commissions" in o';
}
export function orderIs(obj: unknown): obj is Order {
    if (!orderHasShape(obj)) {
        return false;
    }
    const result = orderDeserialize(obj);
    return result.success;
}

export function orderFromFormData(formData: FormData): Exit<Order, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get(`${"id"}`) ?? "";
    obj.account = formData.get(`${"account"}`) ?? "";
    {
        const stageObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"stage"}.`)) {
                const fieldName = key.slice(`${"stage"}.`.length);
                const parts = fieldName.split(".");
                let current = stageObj;
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
        obj.stage = stageObj;
    }
    {
        const numberStr = formData.get(`${"number"}`);
        obj.number = numberStr ? parseFloat(numberStr as string) : $MfPh5;
        if (obj.number !== undefined && isNaN(obj.number as number)) obj.number = "0";
    }
    obj.payments = formData.getAll(`${"payments"}`) as Array<string>;
    obj.opportunity = formData.get(`${"opportunity"}`) ?? "";
    obj.reference = formData.get(`${"reference"}`) ?? "";
    obj.leadSource = formData.get(`${"leadSource"}`) ?? "";
    obj.salesRep = formData.get(`${"salesRep"}`) ?? "";
    obj.group = formData.get(`${"group"}`) ?? "";
    obj.subgroup = formData.get(`${"subgroup"}`) ?? "";
    {
        const isPostedVal = formData.get(`${"isPosted"}`);
        obj.isPosted = isPostedVal === "true" || isPostedVal === "on" || isPostedVal === "1";
    }
    {
        const needsReviewVal = formData.get(`${"needsReview"}`);
        obj.needsReview = needsReviewVal === "true" || needsReviewVal === "on" || needsReviewVal === "1";
    }
    obj.actionItem = formData.get(`${"actionItem"}`) ?? "";
    {
        const upsaleStr = formData.get(`${"upsale"}`);
        obj.upsale = upsaleStr ? parseFloat(upsaleStr as string) : $MfPh5;
        if (obj.upsale !== undefined && isNaN(obj.upsale as number)) obj.upsale = "0";
    }
    {
        const dateCreatedObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"dateCreated"}.`)) {
                const fieldName = key.slice(`${"dateCreated"}.`.length);
                const parts = fieldName.split(".");
                let current = dateCreatedObj;
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
        obj.dateCreated = dateCreatedObj;
    }
    obj.appointment = formData.get(`${"appointment"}`) ?? "";
    obj.lastTechs = formData.getAll(`${"lastTechs"}`) as Array<string>;
    obj.package = formData.get(`${"package"}`) ?? "";
    obj.promotion = formData.get(`${"promotion"}`) ?? "";
    {
        const balanceStr = formData.get(`${"balance"}`);
        obj.balance = balanceStr ? parseFloat(balanceStr as string) : $MfPh5;
        if (obj.balance !== undefined && isNaN(obj.balance as number)) obj.balance = "0";
    }
    {
        const dueObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"due"}.`)) {
                const fieldName = key.slice(`${"due"}.`.length);
                const parts = fieldName.split(".");
                let current = dueObj;
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
        obj.due = dueObj;
    }
    {
        const totalStr = formData.get(`${"total"}`);
        obj.total = totalStr ? parseFloat(totalStr as string) : $MfPh5;
        if (obj.total !== undefined && isNaN(obj.total as number)) obj.total = "0";
    }
    obj.site = formData.get(`${"site"}`) ?? "";
    {
        const billedItemsItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while(formData.has(`${"billedItems"}.` + idx + ".") || idx === 0){
            const hasAny = Array.from(formData.keys()).some((k)=>k.startsWith(`${"billedItems"}.` + idx + "."));
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())){
                    if (key.startsWith(`${"billedItems"}.` + idx + ".")) {
                        const fieldName = key.slice(`${"billedItems"}.`.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                billedItemsItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.billedItems = billedItemsItems;
    }
    {
        const memoObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"memo"}.`)) {
                const fieldName = key.slice(`${"memo"}.`.length);
                const parts = fieldName.split(".");
                let current = memoObj;
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
        obj.memo = memoObj;
    }
    {
        const discountStr = formData.get(`${"discount"}`);
        obj.discount = discountStr ? parseFloat(discountStr as string) : $MfPh5;
        if (obj.discount !== undefined && isNaN(obj.discount as number)) obj.discount = "0";
    }
    {
        const tipStr = formData.get(`${"tip"}`);
        obj.tip = tipStr ? parseFloat(tipStr as string) : $MfPh5;
        if (obj.tip !== undefined && isNaN(obj.tip as number)) obj.tip = "0";
    }
    obj.commissions = formData.getAll(`${"commissions"}`).map((v)=>parseFloat(v as string)).filter((n)=>!isNaN(n));
    return toExit("orderDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Order;
    readonly errors: OrderErrors;
    readonly tainted: OrderTainted;
    readonly fields: OrderFieldControllers;
    validate(): Exit<Order, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Order>): void;
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
 }; export function orderCreateForm(overrides: Partial<Order>): OrderGigaform {}
let data = $state({
    ...orderDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as OrderErrors);
let tainted = $state<$MfPh3>({} as OrderTainted);
const fields = {} as OrderFieldControllers;
fields.id = {
    label: `${"id"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.account = {
    label: `${"account"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.stage = {
    label: `${"stage"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.number = {
    label: `${"number"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.payments = {
    label: `${"payments"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.opportunity = {
    label: `${"opportunity"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.reference = {
    label: `${"reference"}`,
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
fields.salesRep = {
    label: `${"salesRep"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.group = {
    label: `${"group"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.subgroup = {
    label: `${"subgroup"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.isPosted = {
    label: `${"isPosted"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.needsReview = {
    label: `${"needsReview"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.actionItem = {
    label: `${"actionItem"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.upsale = {
    label: `${"upsale"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.dateCreated = {
    label: `${"dateCreated"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.appointment = {
    label: `${"appointment"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.lastTechs = {
    label: `${"lastTechs"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.package = {
    label: `${"package"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.promotion = {
    label: `${"promotion"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.balance = {
    label: `${"balance"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.due = {
    label: `${"due"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.total = {
    label: `${"total"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.site = {
    label: `${"site"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.billedItems = {
    label: `${"billedItems"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.memo = {
    label: `${"memo"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.discount = {
    label: `${"discount"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.tip = {
    label: `${"tip"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.commissions = {
    label: `${"commissions"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
function validate(): Exit<Order, Array<{
    field: string;
    message: string;
}>> {
    return toExit("orderDeserialize(data)");
    data = {
        ...orderDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Order = {
  defaultValue: orderDefaultValue,
  serialize: orderSerialize,
  serializeWithContext: orderSerializeWithContext,
  deserialize: orderDeserialize,
  deserializeWithContext: orderDeserializeWithContext,
  validateFields: orderValidateFields,
  hasShape: orderHasShape,
  is: orderIs,
  fromFormData: orderFromFormData,
  createForm: orderCreateForm
} as const;