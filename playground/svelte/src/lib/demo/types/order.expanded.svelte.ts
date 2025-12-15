import { SerializeContext } from 'macroforge/serde';
import { __serializeBilledItem } from './billed-item.svelte';
import { __serializeOrderStage } from './order-stage.svelte';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { __deserializeOrderStage } from './order-stage.svelte';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
import type { ArrayFieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

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

export interface Order {
    id: string;

    account: string | Account;

    stage: OrderStage;

    number: number;

    payments: (string | Payment)[];

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

    dateCreated: string;

    appointment: string | Appointment;

    lastTechs: (string | Employee)[];

    package: (string | Package)[] | null;

    promotion: (string | Promotion)[] | null;

    balance: number;

    due: string;

    total: number;

    site: string | Site;

    billedItems: BilledItem[];

    memo: string;

    discount: number;

    tip: number;

    commissions: number[];
}

export function defaultValueOrder(): Order {
    return {
        id: '',
        account: '',
        stage: 'Estimate',
        number: 0,
        payments: [],
        opportunity: '',
        reference: '',
        leadSource: '',
        salesRep: '',
        group: '',
        subgroup: '',
        isPosted: false,
        needsReview: false,
        actionItem: '',
        upsale: 0,
        dateCreated: '',
        appointment: '',
        lastTechs: [],
        package: null,
        promotion: null,
        balance: 0,
        due: '',
        total: 0,
        site: '',
        billedItems: [],
        memo: '',
        discount: 0,
        tip: 0,
        commissions: []
    } as Order;
}

export function toStringifiedJSONOrder(value: Order): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeOrder(value, ctx));
}
export function toObjectOrder(value: Order): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeOrder(value, ctx);
}
export function __serializeOrder(value: Order, ctx: SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Order', __id };
    result['id'] = value.id;
    result['account'] = value.account;
    result['stage'] = __serializeOrderStage(value.stage, ctx);
    result['number'] = value.number;
    result['payments'] = value.payments;
    result['opportunity'] = value.opportunity;
    result['reference'] = value.reference;
    result['leadSource'] = value.leadSource;
    result['salesRep'] = value.salesRep;
    result['group'] = value.group;
    result['subgroup'] = value.subgroup;
    result['isPosted'] = value.isPosted;
    result['needsReview'] = value.needsReview;
    result['actionItem'] = value.actionItem;
    result['upsale'] = value.upsale;
    result['dateCreated'] = value.dateCreated;
    result['appointment'] = value.appointment;
    result['lastTechs'] = value.lastTechs;
    if (value.package !== null) {
        result['package'] = value.package;
    } else {
        result['package'] = null;
    }
    if (value.promotion !== null) {
        result['promotion'] = value.promotion;
    } else {
        result['promotion'] = null;
    }
    result['balance'] = value.balance;
    result['due'] = value.due;
    result['total'] = value.total;
    result['site'] = value.site;
    result['billedItems'] = value.billedItems.map((item) => __serializeBilledItem(item, ctx));
    result['memo'] = value.memo;
    result['discount'] = value.discount;
    result['tip'] = value.tip;
    result['commissions'] = value.commissions;
    return result;
}

export function fromStringifiedJSONOrder(
    json: string,
    opts?: DeserializeOptions
): Result<Order, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectOrder(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectOrder(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Order, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeOrder(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Order.fromObject: root cannot be a forward reference' }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function __deserializeOrder(value: any, ctx: DeserializeContext): Order | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Order.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('account' in obj)) {
        errors.push({ field: 'account', message: 'missing required field' });
    }
    if (!('stage' in obj)) {
        errors.push({ field: 'stage', message: 'missing required field' });
    }
    if (!('number' in obj)) {
        errors.push({ field: 'number', message: 'missing required field' });
    }
    if (!('payments' in obj)) {
        errors.push({ field: 'payments', message: 'missing required field' });
    }
    if (!('opportunity' in obj)) {
        errors.push({ field: 'opportunity', message: 'missing required field' });
    }
    if (!('reference' in obj)) {
        errors.push({ field: 'reference', message: 'missing required field' });
    }
    if (!('leadSource' in obj)) {
        errors.push({ field: 'leadSource', message: 'missing required field' });
    }
    if (!('salesRep' in obj)) {
        errors.push({ field: 'salesRep', message: 'missing required field' });
    }
    if (!('group' in obj)) {
        errors.push({ field: 'group', message: 'missing required field' });
    }
    if (!('subgroup' in obj)) {
        errors.push({ field: 'subgroup', message: 'missing required field' });
    }
    if (!('isPosted' in obj)) {
        errors.push({ field: 'isPosted', message: 'missing required field' });
    }
    if (!('needsReview' in obj)) {
        errors.push({ field: 'needsReview', message: 'missing required field' });
    }
    if (!('actionItem' in obj)) {
        errors.push({ field: 'actionItem', message: 'missing required field' });
    }
    if (!('upsale' in obj)) {
        errors.push({ field: 'upsale', message: 'missing required field' });
    }
    if (!('dateCreated' in obj)) {
        errors.push({ field: 'dateCreated', message: 'missing required field' });
    }
    if (!('appointment' in obj)) {
        errors.push({ field: 'appointment', message: 'missing required field' });
    }
    if (!('lastTechs' in obj)) {
        errors.push({ field: 'lastTechs', message: 'missing required field' });
    }
    if (!('package' in obj)) {
        errors.push({ field: 'package', message: 'missing required field' });
    }
    if (!('promotion' in obj)) {
        errors.push({ field: 'promotion', message: 'missing required field' });
    }
    if (!('balance' in obj)) {
        errors.push({ field: 'balance', message: 'missing required field' });
    }
    if (!('due' in obj)) {
        errors.push({ field: 'due', message: 'missing required field' });
    }
    if (!('total' in obj)) {
        errors.push({ field: 'total', message: 'missing required field' });
    }
    if (!('site' in obj)) {
        errors.push({ field: 'site', message: 'missing required field' });
    }
    if (!('billedItems' in obj)) {
        errors.push({ field: 'billedItems', message: 'missing required field' });
    }
    if (!('memo' in obj)) {
        errors.push({ field: 'memo', message: 'missing required field' });
    }
    if (!('discount' in obj)) {
        errors.push({ field: 'discount', message: 'missing required field' });
    }
    if (!('tip' in obj)) {
        errors.push({ field: 'tip', message: 'missing required field' });
    }
    if (!('commissions' in obj)) {
        errors.push({ field: 'commissions', message: 'missing required field' });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_id = obj['id'] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_account = obj['account'] as string | Account;
        instance.account = __raw_account;
    }
    {
        const __raw_stage = obj['stage'] as OrderStage;
        {
            const __result = __deserializeOrderStage(__raw_stage, ctx);
            ctx.assignOrDefer(instance, 'stage', __result);
        }
    }
    {
        const __raw_number = obj['number'] as number;
        instance.number = __raw_number;
    }
    {
        const __raw_payments = obj['payments'] as (string | Payment)[];
        if (Array.isArray(__raw_payments)) {
            instance.payments = __raw_payments as (string | Payment)[];
        }
    }
    {
        const __raw_opportunity = obj['opportunity'] as string;
        if (__raw_opportunity.length === 0) {
            errors.push({ field: 'opportunity', message: 'must not be empty' });
        }
        instance.opportunity = __raw_opportunity;
    }
    {
        const __raw_reference = obj['reference'] as string;
        if (__raw_reference.length === 0) {
            errors.push({ field: 'reference', message: 'must not be empty' });
        }
        instance.reference = __raw_reference;
    }
    {
        const __raw_leadSource = obj['leadSource'] as string;
        if (__raw_leadSource.length === 0) {
            errors.push({ field: 'leadSource', message: 'must not be empty' });
        }
        instance.leadSource = __raw_leadSource;
    }
    {
        const __raw_salesRep = obj['salesRep'] as string | Employee;
        instance.salesRep = __raw_salesRep;
    }
    {
        const __raw_group = obj['group'] as string;
        if (__raw_group.length === 0) {
            errors.push({ field: 'group', message: 'must not be empty' });
        }
        instance.group = __raw_group;
    }
    {
        const __raw_subgroup = obj['subgroup'] as string;
        if (__raw_subgroup.length === 0) {
            errors.push({ field: 'subgroup', message: 'must not be empty' });
        }
        instance.subgroup = __raw_subgroup;
    }
    {
        const __raw_isPosted = obj['isPosted'] as boolean;
        instance.isPosted = __raw_isPosted;
    }
    {
        const __raw_needsReview = obj['needsReview'] as boolean;
        instance.needsReview = __raw_needsReview;
    }
    {
        const __raw_actionItem = obj['actionItem'] as string;
        if (__raw_actionItem.length === 0) {
            errors.push({ field: 'actionItem', message: 'must not be empty' });
        }
        instance.actionItem = __raw_actionItem;
    }
    {
        const __raw_upsale = obj['upsale'] as number;
        instance.upsale = __raw_upsale;
    }
    {
        const __raw_dateCreated = obj['dateCreated'] as string;
        instance.dateCreated = __raw_dateCreated;
    }
    {
        const __raw_appointment = obj['appointment'] as string | Appointment;
        instance.appointment = __raw_appointment;
    }
    {
        const __raw_lastTechs = obj['lastTechs'] as (string | Employee)[];
        if (Array.isArray(__raw_lastTechs)) {
            instance.lastTechs = __raw_lastTechs as (string | Employee)[];
        }
    }
    {
        const __raw_package = obj['package'] as (string | Package)[] | null;
        if (__raw_package === null) {
            instance.package = null;
        } else {
            instance.package = __raw_package;
        }
    }
    {
        const __raw_promotion = obj['promotion'] as (string | Promotion)[] | null;
        if (__raw_promotion === null) {
            instance.promotion = null;
        } else {
            instance.promotion = __raw_promotion;
        }
    }
    {
        const __raw_balance = obj['balance'] as number;
        instance.balance = __raw_balance;
    }
    {
        const __raw_due = obj['due'] as string;
        instance.due = __raw_due;
    }
    {
        const __raw_total = obj['total'] as number;
        instance.total = __raw_total;
    }
    {
        const __raw_site = obj['site'] as string | Site;
        instance.site = __raw_site;
    }
    {
        const __raw_billedItems = obj['billedItems'] as BilledItem[];
        if (Array.isArray(__raw_billedItems)) {
            instance.billedItems = __raw_billedItems as BilledItem[];
        }
    }
    {
        const __raw_memo = obj['memo'] as string;
        if (__raw_memo.length === 0) {
            errors.push({ field: 'memo', message: 'must not be empty' });
        }
        instance.memo = __raw_memo;
    }
    {
        const __raw_discount = obj['discount'] as number;
        instance.discount = __raw_discount;
    }
    {
        const __raw_tip = obj['tip'] as number;
        instance.tip = __raw_tip;
    }
    {
        const __raw_commissions = obj['commissions'] as number[];
        if (Array.isArray(__raw_commissions)) {
            instance.commissions = __raw_commissions as number[];
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Order;
}
export function validateFieldOrder<K extends keyof Order>(
    field: K,
    value: Order[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'opportunity': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'opportunity', message: 'must not be empty' });
            }
            break;
        }
        case 'reference': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'reference', message: 'must not be empty' });
            }
            break;
        }
        case 'leadSource': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'leadSource', message: 'must not be empty' });
            }
            break;
        }
        case 'group': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'group', message: 'must not be empty' });
            }
            break;
        }
        case 'subgroup': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'subgroup', message: 'must not be empty' });
            }
            break;
        }
        case 'actionItem': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'actionItem', message: 'must not be empty' });
            }
            break;
        }
        case 'memo': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'memo', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function validateFieldsOrder(
    partial: Partial<Order>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('opportunity' in partial && partial.opportunity !== undefined) {
        const __val = partial.opportunity as string;
        if (__val.length === 0) {
            errors.push({ field: 'opportunity', message: 'must not be empty' });
        }
    }
    if ('reference' in partial && partial.reference !== undefined) {
        const __val = partial.reference as string;
        if (__val.length === 0) {
            errors.push({ field: 'reference', message: 'must not be empty' });
        }
    }
    if ('leadSource' in partial && partial.leadSource !== undefined) {
        const __val = partial.leadSource as string;
        if (__val.length === 0) {
            errors.push({ field: 'leadSource', message: 'must not be empty' });
        }
    }
    if ('group' in partial && partial.group !== undefined) {
        const __val = partial.group as string;
        if (__val.length === 0) {
            errors.push({ field: 'group', message: 'must not be empty' });
        }
    }
    if ('subgroup' in partial && partial.subgroup !== undefined) {
        const __val = partial.subgroup as string;
        if (__val.length === 0) {
            errors.push({ field: 'subgroup', message: 'must not be empty' });
        }
    }
    if ('actionItem' in partial && partial.actionItem !== undefined) {
        const __val = partial.actionItem as string;
        if (__val.length === 0) {
            errors.push({ field: 'actionItem', message: 'must not be empty' });
        }
    }
    if ('memo' in partial && partial.memo !== undefined) {
        const __val = partial.memo as string;
        if (__val.length === 0) {
            errors.push({ field: 'memo', message: 'must not be empty' });
        }
    }
    return errors;
}
export function hasShapeOrder(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'id' in o &&
        'account' in o &&
        'stage' in o &&
        'number' in o &&
        'payments' in o &&
        'opportunity' in o &&
        'reference' in o &&
        'leadSource' in o &&
        'salesRep' in o &&
        'group' in o &&
        'subgroup' in o &&
        'isPosted' in o &&
        'needsReview' in o &&
        'actionItem' in o &&
        'upsale' in o &&
        'dateCreated' in o &&
        'appointment' in o &&
        'lastTechs' in o &&
        'package' in o &&
        'promotion' in o &&
        'balance' in o &&
        'due' in o &&
        'total' in o &&
        'site' in o &&
        'billedItems' in o &&
        'memo' in o &&
        'discount' in o &&
        'tip' in o &&
        'commissions' in o
    );
}
export function isOrder(obj: unknown): obj is Order {
    if (!hasShapeOrder(obj)) {
        return false;
    }
    const result = fromObjectOrder(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsOrder = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    account: Option<Array<string>>;
    stage: Option<Array<string>>;
    number: Option<Array<string>>;
    payments: Option<Array<string>>;
    opportunity: Option<Array<string>>;
    reference: Option<Array<string>>;
    leadSource: Option<Array<string>>;
    salesRep: Option<Array<string>>;
    group: Option<Array<string>>;
    subgroup: Option<Array<string>>;
    isPosted: Option<Array<string>>;
    needsReview: Option<Array<string>>;
    actionItem: Option<Array<string>>;
    upsale: Option<Array<string>>;
    dateCreated: Option<Array<string>>;
    appointment: Option<Array<string>>;
    lastTechs: Option<Array<string>>;
    package: Option<Array<string>>;
    promotion: Option<Array<string>>;
    balance: Option<Array<string>>;
    due: Option<Array<string>>;
    total: Option<Array<string>>;
    site: Option<Array<string>>;
    billedItems: Option<Array<string>>;
    memo: Option<Array<string>>;
    discount: Option<Array<string>>;
    tip: Option<Array<string>>;
    commissions: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedOrder = {
    id: Option<boolean>;
    account: Option<boolean>;
    stage: Option<boolean>;
    number: Option<boolean>;
    payments: Option<boolean>;
    opportunity: Option<boolean>;
    reference: Option<boolean>;
    leadSource: Option<boolean>;
    salesRep: Option<boolean>;
    group: Option<boolean>;
    subgroup: Option<boolean>;
    isPosted: Option<boolean>;
    needsReview: Option<boolean>;
    actionItem: Option<boolean>;
    upsale: Option<boolean>;
    dateCreated: Option<boolean>;
    appointment: Option<boolean>;
    lastTechs: Option<boolean>;
    package: Option<boolean>;
    promotion: Option<boolean>;
    balance: Option<boolean>;
    due: Option<boolean>;
    total: Option<boolean>;
    site: Option<boolean>;
    billedItems: Option<boolean>;
    memo: Option<boolean>;
    discount: Option<boolean>;
    tip: Option<boolean>;
    commissions: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersOrder {
    readonly id: FieldController<string>;
    readonly account: FieldController<string | Account>;
    readonly stage: FieldController<OrderStage>;
    readonly number: FieldController<number>;
    readonly payments: ArrayFieldController<string | Payment>;
    readonly opportunity: FieldController<string>;
    readonly reference: FieldController<string>;
    readonly leadSource: FieldController<string>;
    readonly salesRep: FieldController<string | Employee>;
    readonly group: FieldController<string>;
    readonly subgroup: FieldController<string>;
    readonly isPosted: FieldController<boolean>;
    readonly needsReview: FieldController<boolean>;
    readonly actionItem: FieldController<string>;
    readonly upsale: FieldController<number>;
    readonly dateCreated: FieldController<string>;
    readonly appointment: FieldController<string | Appointment>;
    readonly lastTechs: ArrayFieldController<string | Employee>;
    readonly package: FieldController<(string | Package)[] | null>;
    readonly promotion: FieldController<(string | Promotion)[] | null>;
    readonly balance: FieldController<number>;
    readonly due: FieldController<string>;
    readonly total: FieldController<number>;
    readonly site: FieldController<string | Site>;
    readonly billedItems: ArrayFieldController<BilledItem>;
    readonly memo: FieldController<string>;
    readonly discount: FieldController<number>;
    readonly tip: FieldController<number>;
    readonly commissions: ArrayFieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformOrder {
    readonly data: Order;
    readonly errors: ErrorsOrder;
    readonly tainted: TaintedOrder;
    readonly fields: FieldControllersOrder;
    validate(): Result<Order, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Order>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormOrder(overrides?: Partial<Order>): GigaformOrder {
    let data = $state({ ...defaultValueOrder(), ...overrides });
    let errors = $state<ErrorsOrder>({
        _errors: Option.none(),
        id: Option.none(),
        account: Option.none(),
        stage: Option.none(),
        number: Option.none(),
        payments: Option.none(),
        opportunity: Option.none(),
        reference: Option.none(),
        leadSource: Option.none(),
        salesRep: Option.none(),
        group: Option.none(),
        subgroup: Option.none(),
        isPosted: Option.none(),
        needsReview: Option.none(),
        actionItem: Option.none(),
        upsale: Option.none(),
        dateCreated: Option.none(),
        appointment: Option.none(),
        lastTechs: Option.none(),
        package: Option.none(),
        promotion: Option.none(),
        balance: Option.none(),
        due: Option.none(),
        total: Option.none(),
        site: Option.none(),
        billedItems: Option.none(),
        memo: Option.none(),
        discount: Option.none(),
        tip: Option.none(),
        commissions: Option.none()
    });
    let tainted = $state<TaintedOrder>({
        id: Option.none(),
        account: Option.none(),
        stage: Option.none(),
        number: Option.none(),
        payments: Option.none(),
        opportunity: Option.none(),
        reference: Option.none(),
        leadSource: Option.none(),
        salesRep: Option.none(),
        group: Option.none(),
        subgroup: Option.none(),
        isPosted: Option.none(),
        needsReview: Option.none(),
        actionItem: Option.none(),
        upsale: Option.none(),
        dateCreated: Option.none(),
        appointment: Option.none(),
        lastTechs: Option.none(),
        package: Option.none(),
        promotion: Option.none(),
        balance: Option.none(),
        due: Option.none(),
        total: Option.none(),
        site: Option.none(),
        billedItems: Option.none(),
        memo: Option.none(),
        discount: Option.none(),
        tip: Option.none(),
        commissions: Option.none()
    });
    const fields: FieldControllersOrder = {
        id: {
            path: ['id'] as const,
            name: 'id',
            constraints: { required: true },

            get: () => data.id,
            set: (value: string) => {
                data.id = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.id,
            setError: (value: Option<Array<string>>) => {
                errors.id = value;
            },
            getTainted: () => tainted.id,
            setTainted: (value: Option<boolean>) => {
                tainted.id = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        account: {
            path: ['account'] as const,
            name: 'account',
            constraints: { required: true },
            label: 'Account',
            get: () => data.account,
            set: (value: string | Account) => {
                data.account = value;
            },
            transform: (value: string | Account): string | Account => value,
            getError: () => errors.account,
            setError: (value: Option<Array<string>>) => {
                errors.account = value;
            },
            getTainted: () => tainted.account,
            setTainted: (value: Option<boolean>) => {
                tainted.account = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('account', data.account);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        stage: {
            path: ['stage'] as const,
            name: 'stage',
            constraints: { required: true },
            label: 'Stage',
            get: () => data.stage,
            set: (value: OrderStage) => {
                data.stage = value;
            },
            transform: (value: OrderStage): OrderStage => value,
            getError: () => errors.stage,
            setError: (value: Option<Array<string>>) => {
                errors.stage = value;
            },
            getTainted: () => tainted.stage,
            setTainted: (value: Option<boolean>) => {
                tainted.stage = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('stage', data.stage);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        number: {
            path: ['number'] as const,
            name: 'number',
            constraints: { required: true },

            get: () => data.number,
            set: (value: number) => {
                data.number = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.number,
            setError: (value: Option<Array<string>>) => {
                errors.number = value;
            },
            getTainted: () => tainted.number,
            setTainted: (value: Option<boolean>) => {
                tainted.number = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('number', data.number);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        payments: {
            path: ['payments'] as const,
            name: 'payments',
            constraints: { required: true },

            get: () => data.payments,
            set: (value: (string | Payment)[]) => {
                data.payments = value;
            },
            transform: (value: (string | Payment)[]): (string | Payment)[] => value,
            getError: () => errors.payments,
            setError: (value: Option<Array<string>>) => {
                errors.payments = value;
            },
            getTainted: () => tainted.payments,
            setTainted: (value: Option<boolean>) => {
                tainted.payments = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('payments', data.payments);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['payments', index] as const,
                name: `payments.${index}`,
                constraints: { required: true },
                get: () => data.payments[index]!,
                set: (value: string | Payment) => {
                    data.payments[index] = value;
                },
                transform: (value: string | Payment): string | Payment => value,
                getError: () => errors.payments,
                setError: (value: Option<Array<string>>) => {
                    errors.payments = value;
                },
                getTainted: () => tainted.payments,
                setTainted: (value: Option<boolean>) => {
                    tainted.payments = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: string | Payment) => {
                data.payments.push(item);
            },
            remove: (index: number) => {
                data.payments.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.payments[a]!;
                data.payments[a] = data.payments[b]!;
                data.payments[b] = tmp;
            }
        },
        opportunity: {
            path: ['opportunity'] as const,
            name: 'opportunity',
            constraints: { required: true },
            label: 'Opportunity',
            get: () => data.opportunity,
            set: (value: string) => {
                data.opportunity = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.opportunity,
            setError: (value: Option<Array<string>>) => {
                errors.opportunity = value;
            },
            getTainted: () => tainted.opportunity,
            setTainted: (value: Option<boolean>) => {
                tainted.opportunity = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('opportunity', data.opportunity);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        reference: {
            path: ['reference'] as const,
            name: 'reference',
            constraints: { required: true },
            label: 'Reference',
            get: () => data.reference,
            set: (value: string) => {
                data.reference = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.reference,
            setError: (value: Option<Array<string>>) => {
                errors.reference = value;
            },
            getTainted: () => tainted.reference,
            setTainted: (value: Option<boolean>) => {
                tainted.reference = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('reference', data.reference);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        leadSource: {
            path: ['leadSource'] as const,
            name: 'leadSource',
            constraints: { required: true },
            label: 'Lead Source',
            get: () => data.leadSource,
            set: (value: string) => {
                data.leadSource = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.leadSource,
            setError: (value: Option<Array<string>>) => {
                errors.leadSource = value;
            },
            getTainted: () => tainted.leadSource,
            setTainted: (value: Option<boolean>) => {
                tainted.leadSource = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('leadSource', data.leadSource);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        salesRep: {
            path: ['salesRep'] as const,
            name: 'salesRep',
            constraints: { required: true },
            label: 'Sales Rep',
            get: () => data.salesRep,
            set: (value: string | Employee) => {
                data.salesRep = value;
            },
            transform: (value: string | Employee): string | Employee => value,
            getError: () => errors.salesRep,
            setError: (value: Option<Array<string>>) => {
                errors.salesRep = value;
            },
            getTainted: () => tainted.salesRep,
            setTainted: (value: Option<boolean>) => {
                tainted.salesRep = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('salesRep', data.salesRep);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        group: {
            path: ['group'] as const,
            name: 'group',
            constraints: { required: true },
            label: 'Group',
            get: () => data.group,
            set: (value: string) => {
                data.group = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.group,
            setError: (value: Option<Array<string>>) => {
                errors.group = value;
            },
            getTainted: () => tainted.group,
            setTainted: (value: Option<boolean>) => {
                tainted.group = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('group', data.group);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        subgroup: {
            path: ['subgroup'] as const,
            name: 'subgroup',
            constraints: { required: true },
            label: 'Subgroup',
            get: () => data.subgroup,
            set: (value: string) => {
                data.subgroup = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.subgroup,
            setError: (value: Option<Array<string>>) => {
                errors.subgroup = value;
            },
            getTainted: () => tainted.subgroup,
            setTainted: (value: Option<boolean>) => {
                tainted.subgroup = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('subgroup', data.subgroup);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        isPosted: {
            path: ['isPosted'] as const,
            name: 'isPosted',
            constraints: { required: true },
            label: 'Posted',
            get: () => data.isPosted,
            set: (value: boolean) => {
                data.isPosted = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.isPosted,
            setError: (value: Option<Array<string>>) => {
                errors.isPosted = value;
            },
            getTainted: () => tainted.isPosted,
            setTainted: (value: Option<boolean>) => {
                tainted.isPosted = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('isPosted', data.isPosted);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        needsReview: {
            path: ['needsReview'] as const,
            name: 'needsReview',
            constraints: { required: true },
            label: 'Needs Review',
            get: () => data.needsReview,
            set: (value: boolean) => {
                data.needsReview = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.needsReview,
            setError: (value: Option<Array<string>>) => {
                errors.needsReview = value;
            },
            getTainted: () => tainted.needsReview,
            setTainted: (value: Option<boolean>) => {
                tainted.needsReview = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('needsReview', data.needsReview);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        actionItem: {
            path: ['actionItem'] as const,
            name: 'actionItem',
            constraints: { required: true },
            label: 'Action Item',
            get: () => data.actionItem,
            set: (value: string) => {
                data.actionItem = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.actionItem,
            setError: (value: Option<Array<string>>) => {
                errors.actionItem = value;
            },
            getTainted: () => tainted.actionItem,
            setTainted: (value: Option<boolean>) => {
                tainted.actionItem = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('actionItem', data.actionItem);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        upsale: {
            path: ['upsale'] as const,
            name: 'upsale',
            constraints: { required: true },

            get: () => data.upsale,
            set: (value: number) => {
                data.upsale = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.upsale,
            setError: (value: Option<Array<string>>) => {
                errors.upsale = value;
            },
            getTainted: () => tainted.upsale,
            setTainted: (value: Option<boolean>) => {
                tainted.upsale = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('upsale', data.upsale);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        dateCreated: {
            path: ['dateCreated'] as const,
            name: 'dateCreated',
            constraints: { required: true },

            get: () => data.dateCreated,
            set: (value: string) => {
                data.dateCreated = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.dateCreated,
            setError: (value: Option<Array<string>>) => {
                errors.dateCreated = value;
            },
            getTainted: () => tainted.dateCreated,
            setTainted: (value: Option<boolean>) => {
                tainted.dateCreated = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('dateCreated', data.dateCreated);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        appointment: {
            path: ['appointment'] as const,
            name: 'appointment',
            constraints: { required: true },
            label: 'Appointment',
            get: () => data.appointment,
            set: (value: string | Appointment) => {
                data.appointment = value;
            },
            transform: (value: string | Appointment): string | Appointment => value,
            getError: () => errors.appointment,
            setError: (value: Option<Array<string>>) => {
                errors.appointment = value;
            },
            getTainted: () => tainted.appointment,
            setTainted: (value: Option<boolean>) => {
                tainted.appointment = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('appointment', data.appointment);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        lastTechs: {
            path: ['lastTechs'] as const,
            name: 'lastTechs',
            constraints: { required: true },
            label: 'Technicians',
            get: () => data.lastTechs,
            set: (value: (string | Employee)[]) => {
                data.lastTechs = value;
            },
            transform: (value: (string | Employee)[]): (string | Employee)[] => value,
            getError: () => errors.lastTechs,
            setError: (value: Option<Array<string>>) => {
                errors.lastTechs = value;
            },
            getTainted: () => tainted.lastTechs,
            setTainted: (value: Option<boolean>) => {
                tainted.lastTechs = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('lastTechs', data.lastTechs);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['lastTechs', index] as const,
                name: `lastTechs.${index}`,
                constraints: { required: true },
                get: () => data.lastTechs[index]!,
                set: (value: string | Employee) => {
                    data.lastTechs[index] = value;
                },
                transform: (value: string | Employee): string | Employee => value,
                getError: () => errors.lastTechs,
                setError: (value: Option<Array<string>>) => {
                    errors.lastTechs = value;
                },
                getTainted: () => tainted.lastTechs,
                setTainted: (value: Option<boolean>) => {
                    tainted.lastTechs = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: string | Employee) => {
                data.lastTechs.push(item);
            },
            remove: (index: number) => {
                data.lastTechs.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.lastTechs[a]!;
                data.lastTechs[a] = data.lastTechs[b]!;
                data.lastTechs[b] = tmp;
            }
        },
        package: {
            path: ['package'] as const,
            name: 'package',
            constraints: { required: true },

            get: () => data.package,
            set: (value: (string | Package)[] | null) => {
                data.package = value;
            },
            transform: (value: (string | Package)[] | null): (string | Package)[] | null => value,
            getError: () => errors.package,
            setError: (value: Option<Array<string>>) => {
                errors.package = value;
            },
            getTainted: () => tainted.package,
            setTainted: (value: Option<boolean>) => {
                tainted.package = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('package', data.package);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        promotion: {
            path: ['promotion'] as const,
            name: 'promotion',
            constraints: { required: true },

            get: () => data.promotion,
            set: (value: (string | Promotion)[] | null) => {
                data.promotion = value;
            },
            transform: (value: (string | Promotion)[] | null): (string | Promotion)[] | null =>
                value,
            getError: () => errors.promotion,
            setError: (value: Option<Array<string>>) => {
                errors.promotion = value;
            },
            getTainted: () => tainted.promotion,
            setTainted: (value: Option<boolean>) => {
                tainted.promotion = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('promotion', data.promotion);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        balance: {
            path: ['balance'] as const,
            name: 'balance',
            constraints: { required: true },

            get: () => data.balance,
            set: (value: number) => {
                data.balance = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.balance,
            setError: (value: Option<Array<string>>) => {
                errors.balance = value;
            },
            getTainted: () => tainted.balance,
            setTainted: (value: Option<boolean>) => {
                tainted.balance = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('balance', data.balance);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        due: {
            path: ['due'] as const,
            name: 'due',
            constraints: { required: true },
            label: 'Due',
            get: () => data.due,
            set: (value: string) => {
                data.due = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.due,
            setError: (value: Option<Array<string>>) => {
                errors.due = value;
            },
            getTainted: () => tainted.due,
            setTainted: (value: Option<boolean>) => {
                tainted.due = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('due', data.due);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        total: {
            path: ['total'] as const,
            name: 'total',
            constraints: { required: true },

            get: () => data.total,
            set: (value: number) => {
                data.total = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.total,
            setError: (value: Option<Array<string>>) => {
                errors.total = value;
            },
            getTainted: () => tainted.total,
            setTainted: (value: Option<boolean>) => {
                tainted.total = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('total', data.total);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        site: {
            path: ['site'] as const,
            name: 'site',
            constraints: { required: true },
            label: 'Site',
            get: () => data.site,
            set: (value: string | Site) => {
                data.site = value;
            },
            transform: (value: string | Site): string | Site => value,
            getError: () => errors.site,
            setError: (value: Option<Array<string>>) => {
                errors.site = value;
            },
            getTainted: () => tainted.site,
            setTainted: (value: Option<boolean>) => {
                tainted.site = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('site', data.site);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        billedItems: {
            path: ['billedItems'] as const,
            name: 'billedItems',
            constraints: { required: true },

            get: () => data.billedItems,
            set: (value: BilledItem[]) => {
                data.billedItems = value;
            },
            transform: (value: BilledItem[]): BilledItem[] => value,
            getError: () => errors.billedItems,
            setError: (value: Option<Array<string>>) => {
                errors.billedItems = value;
            },
            getTainted: () => tainted.billedItems,
            setTainted: (value: Option<boolean>) => {
                tainted.billedItems = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('billedItems', data.billedItems);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['billedItems', index] as const,
                name: `billedItems.${index}`,
                constraints: { required: true },
                get: () => data.billedItems[index]!,
                set: (value: BilledItem) => {
                    data.billedItems[index] = value;
                },
                transform: (value: BilledItem): BilledItem => value,
                getError: () => errors.billedItems,
                setError: (value: Option<Array<string>>) => {
                    errors.billedItems = value;
                },
                getTainted: () => tainted.billedItems,
                setTainted: (value: Option<boolean>) => {
                    tainted.billedItems = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: BilledItem) => {
                data.billedItems.push(item);
            },
            remove: (index: number) => {
                data.billedItems.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.billedItems[a]!;
                data.billedItems[a] = data.billedItems[b]!;
                data.billedItems[b] = tmp;
            }
        },
        memo: {
            path: ['memo'] as const,
            name: 'memo',
            constraints: { required: true },
            label: 'Memo',
            get: () => data.memo,
            set: (value: string) => {
                data.memo = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.memo,
            setError: (value: Option<Array<string>>) => {
                errors.memo = value;
            },
            getTainted: () => tainted.memo,
            setTainted: (value: Option<boolean>) => {
                tainted.memo = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('memo', data.memo);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        discount: {
            path: ['discount'] as const,
            name: 'discount',
            constraints: { required: true },

            get: () => data.discount,
            set: (value: number) => {
                data.discount = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.discount,
            setError: (value: Option<Array<string>>) => {
                errors.discount = value;
            },
            getTainted: () => tainted.discount,
            setTainted: (value: Option<boolean>) => {
                tainted.discount = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('discount', data.discount);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        tip: {
            path: ['tip'] as const,
            name: 'tip',
            constraints: { required: true },

            get: () => data.tip,
            set: (value: number) => {
                data.tip = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.tip,
            setError: (value: Option<Array<string>>) => {
                errors.tip = value;
            },
            getTainted: () => tainted.tip,
            setTainted: (value: Option<boolean>) => {
                tainted.tip = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('tip', data.tip);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        commissions: {
            path: ['commissions'] as const,
            name: 'commissions',
            constraints: { required: true },

            get: () => data.commissions,
            set: (value: number[]) => {
                data.commissions = value;
            },
            transform: (value: number[]): number[] => value,
            getError: () => errors.commissions,
            setError: (value: Option<Array<string>>) => {
                errors.commissions = value;
            },
            getTainted: () => tainted.commissions,
            setTainted: (value: Option<boolean>) => {
                tainted.commissions = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOrder('commissions', data.commissions);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['commissions', index] as const,
                name: `commissions.${index}`,
                constraints: { required: true },
                get: () => data.commissions[index]!,
                set: (value: number) => {
                    data.commissions[index] = value;
                },
                transform: (value: number): number => value,
                getError: () => errors.commissions,
                setError: (value: Option<Array<string>>) => {
                    errors.commissions = value;
                },
                getTainted: () => tainted.commissions,
                setTainted: (value: Option<boolean>) => {
                    tainted.commissions = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: number) => {
                data.commissions.push(item);
            },
            remove: (index: number) => {
                data.commissions.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.commissions[a]!;
                data.commissions[a] = data.commissions[b]!;
                data.commissions[b] = tmp;
            }
        }
    };
    function validate(): Result<Order, Array<{ field: string; message: string }>> {
        return fromObjectOrder(data);
    }
    function reset(newOverrides?: Partial<Order>): void {
        data = { ...defaultValueOrder(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            id: Option.none(),
            account: Option.none(),
            stage: Option.none(),
            number: Option.none(),
            payments: Option.none(),
            opportunity: Option.none(),
            reference: Option.none(),
            leadSource: Option.none(),
            salesRep: Option.none(),
            group: Option.none(),
            subgroup: Option.none(),
            isPosted: Option.none(),
            needsReview: Option.none(),
            actionItem: Option.none(),
            upsale: Option.none(),
            dateCreated: Option.none(),
            appointment: Option.none(),
            lastTechs: Option.none(),
            package: Option.none(),
            promotion: Option.none(),
            balance: Option.none(),
            due: Option.none(),
            total: Option.none(),
            site: Option.none(),
            billedItems: Option.none(),
            memo: Option.none(),
            discount: Option.none(),
            tip: Option.none(),
            commissions: Option.none()
        };
        tainted = {
            id: Option.none(),
            account: Option.none(),
            stage: Option.none(),
            number: Option.none(),
            payments: Option.none(),
            opportunity: Option.none(),
            reference: Option.none(),
            leadSource: Option.none(),
            salesRep: Option.none(),
            group: Option.none(),
            subgroup: Option.none(),
            isPosted: Option.none(),
            needsReview: Option.none(),
            actionItem: Option.none(),
            upsale: Option.none(),
            dateCreated: Option.none(),
            appointment: Option.none(),
            lastTechs: Option.none(),
            package: Option.none(),
            promotion: Option.none(),
            balance: Option.none(),
            due: Option.none(),
            total: Option.none(),
            site: Option.none(),
            billedItems: Option.none(),
            memo: Option.none(),
            discount: Option.none(),
            tip: Option.none(),
            commissions: Option.none()
        };
    }
    return {
        get data() {
            return data;
        },
        set data(v) {
            data = v;
        },
        get errors() {
            return errors;
        },
        set errors(v) {
            errors = v;
        },
        get tainted() {
            return tainted;
        },
        set tainted(v) {
            tainted = v;
        },
        fields,
        validate,
        reset
    };
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function fromFormDataOrder(
    formData: FormData
): Result<Order, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.account = formData.get('account') ?? '';
    {
        // Collect nested object fields with prefix "stage."
        const stageObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('stage.')) {
                const fieldName = key.slice('stage.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = stageObj;
                for (let i = 0; i < parts.length - 1; i++) {
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
        const numberStr = formData.get('number');
        obj.number = numberStr ? parseFloat(numberStr as string) : 0;
        if (obj.number !== undefined && isNaN(obj.number as number)) obj.number = 0;
    }
    {
        // Collect array items from indexed form fields
        const paymentsItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('payments.' + idx + '.') || idx === 0) {
            // Check if any field with this index exists
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('payments.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('payments.' + idx + '.')) {
                        const fieldName = key.slice('payments.'.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                paymentsItems.push(item);
            }
            idx++;
            if (idx > 1000) break; // Safety limit
        }
        obj.payments = paymentsItems;
    }
    obj.opportunity = formData.get('opportunity') ?? '';
    obj.reference = formData.get('reference') ?? '';
    obj.leadSource = formData.get('leadSource') ?? '';
    obj.salesRep = formData.get('salesRep') ?? '';
    obj.group = formData.get('group') ?? '';
    obj.subgroup = formData.get('subgroup') ?? '';
    {
        const isPostedVal = formData.get('isPosted');
        obj.isPosted = isPostedVal === 'true' || isPostedVal === 'on' || isPostedVal === '1';
    }
    {
        const needsReviewVal = formData.get('needsReview');
        obj.needsReview =
            needsReviewVal === 'true' || needsReviewVal === 'on' || needsReviewVal === '1';
    }
    obj.actionItem = formData.get('actionItem') ?? '';
    {
        const upsaleStr = formData.get('upsale');
        obj.upsale = upsaleStr ? parseFloat(upsaleStr as string) : 0;
        if (obj.upsale !== undefined && isNaN(obj.upsale as number)) obj.upsale = 0;
    }
    obj.dateCreated = formData.get('dateCreated') ?? '';
    obj.appointment = formData.get('appointment') ?? '';
    {
        // Collect array items from indexed form fields
        const lastTechsItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('lastTechs.' + idx + '.') || idx === 0) {
            // Check if any field with this index exists
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('lastTechs.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('lastTechs.' + idx + '.')) {
                        const fieldName = key.slice('lastTechs.'.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                lastTechsItems.push(item);
            }
            idx++;
            if (idx > 1000) break; // Safety limit
        }
        obj.lastTechs = lastTechsItems;
    }
    obj.package = formData.get('package') ?? '';
    obj.promotion = formData.get('promotion') ?? '';
    {
        const balanceStr = formData.get('balance');
        obj.balance = balanceStr ? parseFloat(balanceStr as string) : 0;
        if (obj.balance !== undefined && isNaN(obj.balance as number)) obj.balance = 0;
    }
    obj.due = formData.get('due') ?? '';
    {
        const totalStr = formData.get('total');
        obj.total = totalStr ? parseFloat(totalStr as string) : 0;
        if (obj.total !== undefined && isNaN(obj.total as number)) obj.total = 0;
    }
    obj.site = formData.get('site') ?? '';
    {
        // Collect array items from indexed form fields
        const billedItemsItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('billedItems.' + idx + '.') || idx === 0) {
            // Check if any field with this index exists
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('billedItems.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('billedItems.' + idx + '.')) {
                        const fieldName = key.slice('billedItems.'.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                billedItemsItems.push(item);
            }
            idx++;
            if (idx > 1000) break; // Safety limit
        }
        obj.billedItems = billedItemsItems;
    }
    obj.memo = formData.get('memo') ?? '';
    {
        const discountStr = formData.get('discount');
        obj.discount = discountStr ? parseFloat(discountStr as string) : 0;
        if (obj.discount !== undefined && isNaN(obj.discount as number)) obj.discount = 0;
    }
    {
        const tipStr = formData.get('tip');
        obj.tip = tipStr ? parseFloat(tipStr as string) : 0;
        if (obj.tip !== undefined && isNaN(obj.tip as number)) obj.tip = 0;
    }
    obj.commissions = formData
        .getAll('commissions')
        .map((v) => parseFloat(v as string))
        .filter((n) => !isNaN(n));
    return fromStringifiedJSONOrder(JSON.stringify(obj));
}
