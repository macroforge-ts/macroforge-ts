import { accountNameDefaultValue } from './account-name.svelte';
import { emailDefaultValue } from './email.svelte';
import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { accountNameSerializeWithContext } from './account-name.svelte';
import { emailSerializeWithContext } from './email.svelte';
import { leadStageSerializeWithContext } from './lead-stage.svelte';
import { nextStepSerializeWithContext } from './next-step.svelte';
import { phoneNumberSerializeWithContext } from './phone-number.svelte';
import { prioritySerializeWithContext } from './priority.svelte';
import { sectorSerializeWithContext } from './sector.svelte';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { accountNameDeserializeWithContext } from './account-name.svelte';
import { emailDeserializeWithContext } from './email.svelte';
import { leadStageDeserializeWithContext } from './lead-stage.svelte';
import { nextStepDeserializeWithContext } from './next-step.svelte';
import { priorityDeserializeWithContext } from './priority.svelte';
import { sectorDeserializeWithContext } from './sector.svelte';
import { Result } from '@playground/macro/gigaform';
import { Option } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
import type { ArrayFieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { PersonName } from './person-name.svelte';
import type { Site } from './site.svelte';
import type { PhoneNumber } from './phone-number.svelte';
import type { Represents } from './represents.svelte';
import type { Payment } from './payment.svelte';
import type { CompanyName } from './company-name.svelte';
import type { Account } from './account.svelte';
import type { Custom } from './custom.svelte';
import type { TaxRate } from './tax-rate.svelte';
import type { Company } from './company.svelte';
import type { Email } from './email.svelte';
import type { Sector } from './sector.svelte';
import type { Status } from './status.svelte';
import type { NextStep } from './next-step.svelte';
import type { LeadStage } from './lead-stage.svelte';
import type { AccountName } from './account-name.svelte';
import type { Priority } from './priority.svelte';

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

    phones: PhoneNumber[];

    email: Email;

    leadSource: string | null;

    site: string | Site;

    memo: string;

    needsReview: boolean;

    hasAlert: boolean;

    salesRep: Represents[] | null;

    color: string | null;

    accountType: string;

    subtype: string;

    isTaxExempt: boolean;

    paymentTerms: string;

    tags: string[];

    customFields: [string, string][];
}

export function leadDefaultValue(): Lead {
    return {
        id: '',
        number: null,
        accepted: false,
        probability: 0,
        priority: 'Medium',
        dueDate: null,
        closeDate: null,
        value: 0,
        stage: 'Open',
        status: '',
        description: null,
        nextStep: 'InitialContact',
        favorite: false,
        dateAdded: null,
        taxRate: null,
        sector: 'Residential',
        leadName: accountNameDefaultValue(),
        phones: [],
        email: emailDefaultValue(),
        leadSource: null,
        site: '',
        memo: '',
        needsReview: false,
        hasAlert: false,
        salesRep: null,
        color: null,
        accountType: '',
        subtype: '',
        isTaxExempt: false,
        paymentTerms: '',
        tags: [],
        customFields: []
    } as Lead;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function leadSerialize(
    value: Lead
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(leadSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function leadSerializeWithContext(
    value: Lead,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Lead', __id };
    result['id'] = value.id;
    result['number'] = value.number;
    result['accepted'] = value.accepted;
    result['probability'] = value.probability;
    result['priority'] = prioritySerializeWithContext(value.priority, ctx);
    result['dueDate'] = value.dueDate;
    result['closeDate'] = value.closeDate;
    result['value'] = value.value;
    result['stage'] = leadStageSerializeWithContext(value.stage, ctx);
    result['status'] = value.status;
    result['description'] = value.description;
    result['nextStep'] = nextStepSerializeWithContext(value.nextStep, ctx);
    result['favorite'] = value.favorite;
    result['dateAdded'] = value.dateAdded;
    if (value.taxRate !== null) {
        result['taxRate'] = value.taxRate;
    } else {
        result['taxRate'] = null;
    }
    result['sector'] = sectorSerializeWithContext(value.sector, ctx);
    result['leadName'] = accountNameSerializeWithContext(value.leadName, ctx);
    result['phones'] = value.phones.map((item) => phoneNumberSerializeWithContext(item, ctx));
    result['email'] = emailSerializeWithContext(value.email, ctx);
    result['leadSource'] = value.leadSource;
    result['site'] = value.site;
    result['memo'] = value.memo;
    result['needsReview'] = value.needsReview;
    result['hasAlert'] = value.hasAlert;
    if (value.salesRep !== null) {
        result['salesRep'] = value.salesRep;
    } else {
        result['salesRep'] = null;
    }
    result['color'] = value.color;
    result['accountType'] = value.accountType;
    result['subtype'] = value.subtype;
    result['isTaxExempt'] = value.isTaxExempt;
    result['paymentTerms'] = value.paymentTerms;
    result['tags'] = value.tags;
    result['customFields'] = value.customFields;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function leadDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Lead> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = leadDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                { field: '_root', message: 'Lead.deserialize: root cannot be a forward reference' }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return __mf_exitSucceed(resultOrRef);
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return __mf_exitFail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return __mf_exitFail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function leadDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Lead | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Lead.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('number' in obj)) {
        errors.push({ field: 'number', message: 'missing required field' });
    }
    if (!('accepted' in obj)) {
        errors.push({ field: 'accepted', message: 'missing required field' });
    }
    if (!('probability' in obj)) {
        errors.push({ field: 'probability', message: 'missing required field' });
    }
    if (!('priority' in obj)) {
        errors.push({ field: 'priority', message: 'missing required field' });
    }
    if (!('dueDate' in obj)) {
        errors.push({ field: 'dueDate', message: 'missing required field' });
    }
    if (!('closeDate' in obj)) {
        errors.push({ field: 'closeDate', message: 'missing required field' });
    }
    if (!('value' in obj)) {
        errors.push({ field: 'value', message: 'missing required field' });
    }
    if (!('stage' in obj)) {
        errors.push({ field: 'stage', message: 'missing required field' });
    }
    if (!('status' in obj)) {
        errors.push({ field: 'status', message: 'missing required field' });
    }
    if (!('description' in obj)) {
        errors.push({ field: 'description', message: 'missing required field' });
    }
    if (!('nextStep' in obj)) {
        errors.push({ field: 'nextStep', message: 'missing required field' });
    }
    if (!('favorite' in obj)) {
        errors.push({ field: 'favorite', message: 'missing required field' });
    }
    if (!('dateAdded' in obj)) {
        errors.push({ field: 'dateAdded', message: 'missing required field' });
    }
    if (!('taxRate' in obj)) {
        errors.push({ field: 'taxRate', message: 'missing required field' });
    }
    if (!('sector' in obj)) {
        errors.push({ field: 'sector', message: 'missing required field' });
    }
    if (!('leadName' in obj)) {
        errors.push({ field: 'leadName', message: 'missing required field' });
    }
    if (!('phones' in obj)) {
        errors.push({ field: 'phones', message: 'missing required field' });
    }
    if (!('email' in obj)) {
        errors.push({ field: 'email', message: 'missing required field' });
    }
    if (!('leadSource' in obj)) {
        errors.push({ field: 'leadSource', message: 'missing required field' });
    }
    if (!('site' in obj)) {
        errors.push({ field: 'site', message: 'missing required field' });
    }
    if (!('memo' in obj)) {
        errors.push({ field: 'memo', message: 'missing required field' });
    }
    if (!('needsReview' in obj)) {
        errors.push({ field: 'needsReview', message: 'missing required field' });
    }
    if (!('hasAlert' in obj)) {
        errors.push({ field: 'hasAlert', message: 'missing required field' });
    }
    if (!('salesRep' in obj)) {
        errors.push({ field: 'salesRep', message: 'missing required field' });
    }
    if (!('color' in obj)) {
        errors.push({ field: 'color', message: 'missing required field' });
    }
    if (!('accountType' in obj)) {
        errors.push({ field: 'accountType', message: 'missing required field' });
    }
    if (!('subtype' in obj)) {
        errors.push({ field: 'subtype', message: 'missing required field' });
    }
    if (!('isTaxExempt' in obj)) {
        errors.push({ field: 'isTaxExempt', message: 'missing required field' });
    }
    if (!('paymentTerms' in obj)) {
        errors.push({ field: 'paymentTerms', message: 'missing required field' });
    }
    if (!('tags' in obj)) {
        errors.push({ field: 'tags', message: 'missing required field' });
    }
    if (!('customFields' in obj)) {
        errors.push({ field: 'customFields', message: 'missing required field' });
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
        const __raw_id = obj['id'] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_number = obj['number'] as number | null;
        instance.number = __raw_number;
    }
    {
        const __raw_accepted = obj['accepted'] as boolean;
        instance.accepted = __raw_accepted;
    }
    {
        const __raw_probability = obj['probability'] as number;
        instance.probability = __raw_probability;
    }
    {
        const __raw_priority = obj['priority'] as Priority;
        {
            const __result = priorityDeserializeWithContext(__raw_priority, ctx);
            ctx.assignOrDefer(instance, 'priority', __result);
        }
    }
    {
        const __raw_dueDate = obj['dueDate'] as string | null;
        instance.dueDate = __raw_dueDate;
    }
    {
        const __raw_closeDate = obj['closeDate'] as string | null;
        instance.closeDate = __raw_closeDate;
    }
    {
        const __raw_value = obj['value'] as number;
        instance.value = __raw_value;
    }
    {
        const __raw_stage = obj['stage'] as LeadStage;
        {
            const __result = leadStageDeserializeWithContext(__raw_stage, ctx);
            ctx.assignOrDefer(instance, 'stage', __result);
        }
    }
    {
        const __raw_status = obj['status'] as string;
        if (__raw_status.length === 0) {
            errors.push({ field: 'status', message: 'must not be empty' });
        }
        instance.status = __raw_status;
    }
    {
        const __raw_description = obj['description'] as string | null;
        instance.description = __raw_description;
    }
    {
        const __raw_nextStep = obj['nextStep'] as NextStep;
        {
            const __result = nextStepDeserializeWithContext(__raw_nextStep, ctx);
            ctx.assignOrDefer(instance, 'nextStep', __result);
        }
    }
    {
        const __raw_favorite = obj['favorite'] as boolean;
        instance.favorite = __raw_favorite;
    }
    {
        const __raw_dateAdded = obj['dateAdded'] as string | null;
        instance.dateAdded = __raw_dateAdded;
    }
    {
        const __raw_taxRate = obj['taxRate'] as (string | TaxRate) | null;
        if (__raw_taxRate === null) {
            instance.taxRate = null;
        } else {
            instance.taxRate = __raw_taxRate;
        }
    }
    {
        const __raw_sector = obj['sector'] as Sector;
        {
            const __result = sectorDeserializeWithContext(__raw_sector, ctx);
            ctx.assignOrDefer(instance, 'sector', __result);
        }
    }
    {
        const __raw_leadName = obj['leadName'] as AccountName;
        {
            const __result = accountNameDeserializeWithContext(__raw_leadName, ctx);
            ctx.assignOrDefer(instance, 'leadName', __result);
        }
    }
    {
        const __raw_phones = obj['phones'] as PhoneNumber[];
        if (Array.isArray(__raw_phones)) {
            instance.phones = __raw_phones as PhoneNumber[];
        }
    }
    {
        const __raw_email = obj['email'] as Email;
        {
            const __result = emailDeserializeWithContext(__raw_email, ctx);
            ctx.assignOrDefer(instance, 'email', __result);
        }
    }
    {
        const __raw_leadSource = obj['leadSource'] as string | null;
        instance.leadSource = __raw_leadSource;
    }
    {
        const __raw_site = obj['site'] as string | Site;
        instance.site = __raw_site;
    }
    {
        const __raw_memo = obj['memo'] as string;
        if (__raw_memo.length === 0) {
            errors.push({ field: 'memo', message: 'must not be empty' });
        }
        instance.memo = __raw_memo;
    }
    {
        const __raw_needsReview = obj['needsReview'] as boolean;
        instance.needsReview = __raw_needsReview;
    }
    {
        const __raw_hasAlert = obj['hasAlert'] as boolean;
        instance.hasAlert = __raw_hasAlert;
    }
    {
        const __raw_salesRep = obj['salesRep'] as Represents[] | null;
        if (__raw_salesRep === null) {
            instance.salesRep = null;
        } else {
            instance.salesRep = __raw_salesRep;
        }
    }
    {
        const __raw_color = obj['color'] as string | null;
        instance.color = __raw_color;
    }
    {
        const __raw_accountType = obj['accountType'] as string;
        if (__raw_accountType.length === 0) {
            errors.push({ field: 'accountType', message: 'must not be empty' });
        }
        instance.accountType = __raw_accountType;
    }
    {
        const __raw_subtype = obj['subtype'] as string;
        if (__raw_subtype.length === 0) {
            errors.push({ field: 'subtype', message: 'must not be empty' });
        }
        instance.subtype = __raw_subtype;
    }
    {
        const __raw_isTaxExempt = obj['isTaxExempt'] as boolean;
        instance.isTaxExempt = __raw_isTaxExempt;
    }
    {
        const __raw_paymentTerms = obj['paymentTerms'] as string;
        if (__raw_paymentTerms.length === 0) {
            errors.push({ field: 'paymentTerms', message: 'must not be empty' });
        }
        instance.paymentTerms = __raw_paymentTerms;
    }
    {
        const __raw_tags = obj['tags'] as string[];
        if (Array.isArray(__raw_tags)) {
            instance.tags = __raw_tags as string[];
        }
    }
    {
        const __raw_customFields = obj['customFields'] as [string, string][];
        if (Array.isArray(__raw_customFields)) {
            instance.customFields = __raw_customFields as [string, string][];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Lead;
}
export function leadValidateField<K extends keyof Lead>(
    field: K,
    value: Lead[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'status': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'status', message: 'must not be empty' });
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
        case 'accountType': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'accountType', message: 'must not be empty' });
            }
            break;
        }
        case 'subtype': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'subtype', message: 'must not be empty' });
            }
            break;
        }
        case 'paymentTerms': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'paymentTerms', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function leadValidateFields(
    partial: Partial<Lead>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('status' in partial && partial.status !== undefined) {
        const __val = partial.status as string;
        if (__val.length === 0) {
            errors.push({ field: 'status', message: 'must not be empty' });
        }
    }
    if ('memo' in partial && partial.memo !== undefined) {
        const __val = partial.memo as string;
        if (__val.length === 0) {
            errors.push({ field: 'memo', message: 'must not be empty' });
        }
    }
    if ('accountType' in partial && partial.accountType !== undefined) {
        const __val = partial.accountType as string;
        if (__val.length === 0) {
            errors.push({ field: 'accountType', message: 'must not be empty' });
        }
    }
    if ('subtype' in partial && partial.subtype !== undefined) {
        const __val = partial.subtype as string;
        if (__val.length === 0) {
            errors.push({ field: 'subtype', message: 'must not be empty' });
        }
    }
    if ('paymentTerms' in partial && partial.paymentTerms !== undefined) {
        const __val = partial.paymentTerms as string;
        if (__val.length === 0) {
            errors.push({ field: 'paymentTerms', message: 'must not be empty' });
        }
    }
    return errors;
}
export function leadHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'id' in o &&
        'number' in o &&
        'accepted' in o &&
        'probability' in o &&
        'priority' in o &&
        'dueDate' in o &&
        'closeDate' in o &&
        'value' in o &&
        'stage' in o &&
        'status' in o &&
        'description' in o &&
        'nextStep' in o &&
        'favorite' in o &&
        'dateAdded' in o &&
        'taxRate' in o &&
        'sector' in o &&
        'leadName' in o &&
        'phones' in o &&
        'email' in o &&
        'leadSource' in o &&
        'site' in o &&
        'memo' in o &&
        'needsReview' in o &&
        'hasAlert' in o &&
        'salesRep' in o &&
        'color' in o &&
        'accountType' in o &&
        'subtype' in o &&
        'isTaxExempt' in o &&
        'paymentTerms' in o &&
        'tags' in o &&
        'customFields' in o
    );
}
export function leadIs(obj: unknown): obj is Lead {
    if (!leadHasShape(obj)) {
        return false;
    }
    const result = leadDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type LeadErrors = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    number: Option<Array<string>>;
    accepted: Option<Array<string>>;
    probability: Option<Array<string>>;
    priority: Option<Array<string>>;
    dueDate: Option<Array<string>>;
    closeDate: Option<Array<string>>;
    value: Option<Array<string>>;
    stage: Option<Array<string>>;
    status: Option<Array<string>>;
    description: Option<Array<string>>;
    nextStep: Option<Array<string>>;
    favorite: Option<Array<string>>;
    dateAdded: Option<Array<string>>;
    taxRate: Option<Array<string>>;
    sector: Option<Array<string>>;
    leadName: Option<Array<string>>;
    phones: Option<Array<string>>;
    email: Option<Array<string>>;
    leadSource: Option<Array<string>>;
    site: Option<Array<string>>;
    memo: Option<Array<string>>;
    needsReview: Option<Array<string>>;
    hasAlert: Option<Array<string>>;
    salesRep: Option<Array<string>>;
    color: Option<Array<string>>;
    accountType: Option<Array<string>>;
    subtype: Option<Array<string>>;
    isTaxExempt: Option<Array<string>>;
    paymentTerms: Option<Array<string>>;
    tags: Option<Array<string>>;
    customFields: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type LeadTainted = {
    id: Option<boolean>;
    number: Option<boolean>;
    accepted: Option<boolean>;
    probability: Option<boolean>;
    priority: Option<boolean>;
    dueDate: Option<boolean>;
    closeDate: Option<boolean>;
    value: Option<boolean>;
    stage: Option<boolean>;
    status: Option<boolean>;
    description: Option<boolean>;
    nextStep: Option<boolean>;
    favorite: Option<boolean>;
    dateAdded: Option<boolean>;
    taxRate: Option<boolean>;
    sector: Option<boolean>;
    leadName: Option<boolean>;
    phones: Option<boolean>;
    email: Option<boolean>;
    leadSource: Option<boolean>;
    site: Option<boolean>;
    memo: Option<boolean>;
    needsReview: Option<boolean>;
    hasAlert: Option<boolean>;
    salesRep: Option<boolean>;
    color: Option<boolean>;
    accountType: Option<boolean>;
    subtype: Option<boolean>;
    isTaxExempt: Option<boolean>;
    paymentTerms: Option<boolean>;
    tags: Option<boolean>;
    customFields: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface LeadFieldControllers {
    readonly id: FieldController<string>;
    readonly number: FieldController<number | null>;
    readonly accepted: FieldController<boolean>;
    readonly probability: FieldController<number>;
    readonly priority: FieldController<Priority>;
    readonly dueDate: FieldController<string | null>;
    readonly closeDate: FieldController<string | null>;
    readonly value: FieldController<number>;
    readonly stage: FieldController<LeadStage>;
    readonly status: FieldController<string>;
    readonly description: FieldController<string | null>;
    readonly nextStep: FieldController<NextStep>;
    readonly favorite: FieldController<boolean>;
    readonly dateAdded: FieldController<string | null>;
    readonly taxRate: FieldController<(string | TaxRate) | null>;
    readonly sector: FieldController<Sector>;
    readonly leadName: FieldController<AccountName>;
    readonly phones: ArrayFieldController<PhoneNumber>;
    readonly email: FieldController<Email>;
    readonly leadSource: FieldController<string | null>;
    readonly site: FieldController<string | Site>;
    readonly memo: FieldController<string>;
    readonly needsReview: FieldController<boolean>;
    readonly hasAlert: FieldController<boolean>;
    readonly salesRep: FieldController<Represents[] | null>;
    readonly color: FieldController<string | null>;
    readonly accountType: FieldController<string>;
    readonly subtype: FieldController<string>;
    readonly isTaxExempt: FieldController<boolean>;
    readonly paymentTerms: FieldController<string>;
    readonly tags: ArrayFieldController<string>;
    readonly customFields: ArrayFieldController<[string, string]>;
} /** Gigaform instance containing reactive state and field controllers */
export interface LeadGigaform {
    readonly data: Lead;
    readonly errors: LeadErrors;
    readonly tainted: LeadTainted;
    readonly fields: LeadFieldControllers;
    validate(): Result<Lead, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Lead>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function leadCreateForm(overrides?: Partial<Lead>): LeadGigaform {
    let data = $state({ ...leadDefaultValue(), ...overrides });
    let errors = $state<LeadErrors>({
        _errors: Option.none(),
        id: Option.none(),
        number: Option.none(),
        accepted: Option.none(),
        probability: Option.none(),
        priority: Option.none(),
        dueDate: Option.none(),
        closeDate: Option.none(),
        value: Option.none(),
        stage: Option.none(),
        status: Option.none(),
        description: Option.none(),
        nextStep: Option.none(),
        favorite: Option.none(),
        dateAdded: Option.none(),
        taxRate: Option.none(),
        sector: Option.none(),
        leadName: Option.none(),
        phones: Option.none(),
        email: Option.none(),
        leadSource: Option.none(),
        site: Option.none(),
        memo: Option.none(),
        needsReview: Option.none(),
        hasAlert: Option.none(),
        salesRep: Option.none(),
        color: Option.none(),
        accountType: Option.none(),
        subtype: Option.none(),
        isTaxExempt: Option.none(),
        paymentTerms: Option.none(),
        tags: Option.none(),
        customFields: Option.none()
    });
    let tainted = $state<LeadTainted>({
        id: Option.none(),
        number: Option.none(),
        accepted: Option.none(),
        probability: Option.none(),
        priority: Option.none(),
        dueDate: Option.none(),
        closeDate: Option.none(),
        value: Option.none(),
        stage: Option.none(),
        status: Option.none(),
        description: Option.none(),
        nextStep: Option.none(),
        favorite: Option.none(),
        dateAdded: Option.none(),
        taxRate: Option.none(),
        sector: Option.none(),
        leadName: Option.none(),
        phones: Option.none(),
        email: Option.none(),
        leadSource: Option.none(),
        site: Option.none(),
        memo: Option.none(),
        needsReview: Option.none(),
        hasAlert: Option.none(),
        salesRep: Option.none(),
        color: Option.none(),
        accountType: Option.none(),
        subtype: Option.none(),
        isTaxExempt: Option.none(),
        paymentTerms: Option.none(),
        tags: Option.none(),
        customFields: Option.none()
    });
    const fields: LeadFieldControllers = {
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
                const fieldErrors = leadValidateField('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        number: {
            path: ['number'] as const,
            name: 'number',
            constraints: { required: true },
            get: () => data.number,
            set: (value: number | null) => {
                data.number = value;
            },
            transform: (value: number | null): number | null => value,
            getError: () => errors.number,
            setError: (value: Option<Array<string>>) => {
                errors.number = value;
            },
            getTainted: () => tainted.number,
            setTainted: (value: Option<boolean>) => {
                tainted.number = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('number', data.number);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        accepted: {
            path: ['accepted'] as const,
            name: 'accepted',
            constraints: { required: true },
            get: () => data.accepted,
            set: (value: boolean) => {
                data.accepted = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.accepted,
            setError: (value: Option<Array<string>>) => {
                errors.accepted = value;
            },
            getTainted: () => tainted.accepted,
            setTainted: (value: Option<boolean>) => {
                tainted.accepted = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('accepted', data.accepted);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        probability: {
            path: ['probability'] as const,
            name: 'probability',
            constraints: { required: true },
            label: 'Probability',
            get: () => data.probability,
            set: (value: number) => {
                data.probability = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.probability,
            setError: (value: Option<Array<string>>) => {
                errors.probability = value;
            },
            getTainted: () => tainted.probability,
            setTainted: (value: Option<boolean>) => {
                tainted.probability = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('probability', data.probability);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        priority: {
            path: ['priority'] as const,
            name: 'priority',
            constraints: { required: true },
            label: 'Priority',
            get: () => data.priority,
            set: (value: Priority) => {
                data.priority = value;
            },
            transform: (value: Priority): Priority => value,
            getError: () => errors.priority,
            setError: (value: Option<Array<string>>) => {
                errors.priority = value;
            },
            getTainted: () => tainted.priority,
            setTainted: (value: Option<boolean>) => {
                tainted.priority = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('priority', data.priority);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        dueDate: {
            path: ['dueDate'] as const,
            name: 'dueDate',
            constraints: { required: true },
            label: 'Due Date',
            get: () => data.dueDate,
            set: (value: string | null) => {
                data.dueDate = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.dueDate,
            setError: (value: Option<Array<string>>) => {
                errors.dueDate = value;
            },
            getTainted: () => tainted.dueDate,
            setTainted: (value: Option<boolean>) => {
                tainted.dueDate = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('dueDate', data.dueDate);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        closeDate: {
            path: ['closeDate'] as const,
            name: 'closeDate',
            constraints: { required: true },
            label: 'Close Date',
            get: () => data.closeDate,
            set: (value: string | null) => {
                data.closeDate = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.closeDate,
            setError: (value: Option<Array<string>>) => {
                errors.closeDate = value;
            },
            getTainted: () => tainted.closeDate,
            setTainted: (value: Option<boolean>) => {
                tainted.closeDate = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('closeDate', data.closeDate);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        value: {
            path: ['value'] as const,
            name: 'value',
            constraints: { required: true },
            label: 'Value',
            get: () => data.value,
            set: (value: number) => {
                data.value = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.value,
            setError: (value: Option<Array<string>>) => {
                errors.value = value;
            },
            getTainted: () => tainted.value,
            setTainted: (value: Option<boolean>) => {
                tainted.value = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('value', data.value);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        stage: {
            path: ['stage'] as const,
            name: 'stage',
            constraints: { required: true },
            label: 'Stage',
            get: () => data.stage,
            set: (value: LeadStage) => {
                data.stage = value;
            },
            transform: (value: LeadStage): LeadStage => value,
            getError: () => errors.stage,
            setError: (value: Option<Array<string>>) => {
                errors.stage = value;
            },
            getTainted: () => tainted.stage,
            setTainted: (value: Option<boolean>) => {
                tainted.stage = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('stage', data.stage);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        status: {
            path: ['status'] as const,
            name: 'status',
            constraints: { required: true },
            label: 'Status',
            get: () => data.status,
            set: (value: string) => {
                data.status = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.status,
            setError: (value: Option<Array<string>>) => {
                errors.status = value;
            },
            getTainted: () => tainted.status,
            setTainted: (value: Option<boolean>) => {
                tainted.status = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('status', data.status);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        description: {
            path: ['description'] as const,
            name: 'description',
            constraints: { required: true },
            label: 'Description',
            get: () => data.description,
            set: (value: string | null) => {
                data.description = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.description,
            setError: (value: Option<Array<string>>) => {
                errors.description = value;
            },
            getTainted: () => tainted.description,
            setTainted: (value: Option<boolean>) => {
                tainted.description = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('description', data.description);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        nextStep: {
            path: ['nextStep'] as const,
            name: 'nextStep',
            constraints: { required: true },
            get: () => data.nextStep,
            set: (value: NextStep) => {
                data.nextStep = value;
            },
            transform: (value: NextStep): NextStep => value,
            getError: () => errors.nextStep,
            setError: (value: Option<Array<string>>) => {
                errors.nextStep = value;
            },
            getTainted: () => tainted.nextStep,
            setTainted: (value: Option<boolean>) => {
                tainted.nextStep = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('nextStep', data.nextStep);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        favorite: {
            path: ['favorite'] as const,
            name: 'favorite',
            constraints: { required: true },
            label: 'Favorite',
            get: () => data.favorite,
            set: (value: boolean) => {
                data.favorite = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.favorite,
            setError: (value: Option<Array<string>>) => {
                errors.favorite = value;
            },
            getTainted: () => tainted.favorite,
            setTainted: (value: Option<boolean>) => {
                tainted.favorite = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('favorite', data.favorite);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        dateAdded: {
            path: ['dateAdded'] as const,
            name: 'dateAdded',
            constraints: { required: true },
            get: () => data.dateAdded,
            set: (value: string | null) => {
                data.dateAdded = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.dateAdded,
            setError: (value: Option<Array<string>>) => {
                errors.dateAdded = value;
            },
            getTainted: () => tainted.dateAdded,
            setTainted: (value: Option<boolean>) => {
                tainted.dateAdded = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('dateAdded', data.dateAdded);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        taxRate: {
            path: ['taxRate'] as const,
            name: 'taxRate',
            constraints: { required: true },
            label: 'Tax Rate',
            get: () => data.taxRate,
            set: (value: (string | TaxRate) | null) => {
                data.taxRate = value;
            },
            transform: (value: (string | TaxRate) | null): (string | TaxRate) | null => value,
            getError: () => errors.taxRate,
            setError: (value: Option<Array<string>>) => {
                errors.taxRate = value;
            },
            getTainted: () => tainted.taxRate,
            setTainted: (value: Option<boolean>) => {
                tainted.taxRate = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('taxRate', data.taxRate);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        sector: {
            path: ['sector'] as const,
            name: 'sector',
            constraints: { required: true },
            label: 'Sector',
            get: () => data.sector,
            set: (value: Sector) => {
                data.sector = value;
            },
            transform: (value: Sector): Sector => value,
            getError: () => errors.sector,
            setError: (value: Option<Array<string>>) => {
                errors.sector = value;
            },
            getTainted: () => tainted.sector,
            setTainted: (value: Option<boolean>) => {
                tainted.sector = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('sector', data.sector);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        leadName: {
            path: ['leadName'] as const,
            name: 'leadName',
            constraints: { required: true },
            get: () => data.leadName,
            set: (value: AccountName) => {
                data.leadName = value;
            },
            transform: (value: AccountName): AccountName => value,
            getError: () => errors.leadName,
            setError: (value: Option<Array<string>>) => {
                errors.leadName = value;
            },
            getTainted: () => tainted.leadName,
            setTainted: (value: Option<boolean>) => {
                tainted.leadName = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('leadName', data.leadName);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        phones: {
            path: ['phones'] as const,
            name: 'phones',
            constraints: { required: true },
            get: () => data.phones,
            set: (value: PhoneNumber[]) => {
                data.phones = value;
            },
            transform: (value: PhoneNumber[]): PhoneNumber[] => value,
            getError: () => errors.phones,
            setError: (value: Option<Array<string>>) => {
                errors.phones = value;
            },
            getTainted: () => tainted.phones,
            setTainted: (value: Option<boolean>) => {
                tainted.phones = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('phones', data.phones);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['phones', index] as const,
                name: `phones.${index}`,
                constraints: { required: true },
                get: () => data.phones[index]!,
                set: (value: PhoneNumber) => {
                    data.phones[index] = value;
                },
                transform: (value: PhoneNumber): PhoneNumber => value,
                getError: () => errors.phones,
                setError: (value: Option<Array<string>>) => {
                    errors.phones = value;
                },
                getTainted: () => tainted.phones,
                setTainted: (value: Option<boolean>) => {
                    tainted.phones = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: PhoneNumber) => {
                data.phones.push(item);
            },
            remove: (index: number) => {
                data.phones.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.phones[a]!;
                data.phones[a] = data.phones[b]!;
                data.phones[b] = tmp;
            }
        },
        email: {
            path: ['email'] as const,
            name: 'email',
            constraints: { required: true },
            label: 'Email',
            get: () => data.email,
            set: (value: Email) => {
                data.email = value;
            },
            transform: (value: Email): Email => value,
            getError: () => errors.email,
            setError: (value: Option<Array<string>>) => {
                errors.email = value;
            },
            getTainted: () => tainted.email,
            setTainted: (value: Option<boolean>) => {
                tainted.email = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('email', data.email);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        leadSource: {
            path: ['leadSource'] as const,
            name: 'leadSource',
            constraints: { required: true },
            label: 'Lead Source',
            get: () => data.leadSource,
            set: (value: string | null) => {
                data.leadSource = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.leadSource,
            setError: (value: Option<Array<string>>) => {
                errors.leadSource = value;
            },
            getTainted: () => tainted.leadSource,
            setTainted: (value: Option<boolean>) => {
                tainted.leadSource = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('leadSource', data.leadSource);
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
                const fieldErrors = leadValidateField('site', data.site);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
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
                const fieldErrors = leadValidateField('memo', data.memo);
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
                const fieldErrors = leadValidateField('needsReview', data.needsReview);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        hasAlert: {
            path: ['hasAlert'] as const,
            name: 'hasAlert',
            constraints: { required: true },
            label: 'Has Alert',
            get: () => data.hasAlert,
            set: (value: boolean) => {
                data.hasAlert = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.hasAlert,
            setError: (value: Option<Array<string>>) => {
                errors.hasAlert = value;
            },
            getTainted: () => tainted.hasAlert,
            setTainted: (value: Option<boolean>) => {
                tainted.hasAlert = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('hasAlert', data.hasAlert);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        salesRep: {
            path: ['salesRep'] as const,
            name: 'salesRep',
            constraints: { required: true },
            label: 'Sales Rep',
            get: () => data.salesRep,
            set: (value: Represents[] | null) => {
                data.salesRep = value;
            },
            transform: (value: Represents[] | null): Represents[] | null => value,
            getError: () => errors.salesRep,
            setError: (value: Option<Array<string>>) => {
                errors.salesRep = value;
            },
            getTainted: () => tainted.salesRep,
            setTainted: (value: Option<boolean>) => {
                tainted.salesRep = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('salesRep', data.salesRep);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        color: {
            path: ['color'] as const,
            name: 'color',
            constraints: { required: true },
            get: () => data.color,
            set: (value: string | null) => {
                data.color = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.color,
            setError: (value: Option<Array<string>>) => {
                errors.color = value;
            },
            getTainted: () => tainted.color,
            setTainted: (value: Option<boolean>) => {
                tainted.color = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('color', data.color);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        accountType: {
            path: ['accountType'] as const,
            name: 'accountType',
            constraints: { required: true },
            label: 'Account Type',
            get: () => data.accountType,
            set: (value: string) => {
                data.accountType = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.accountType,
            setError: (value: Option<Array<string>>) => {
                errors.accountType = value;
            },
            getTainted: () => tainted.accountType,
            setTainted: (value: Option<boolean>) => {
                tainted.accountType = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('accountType', data.accountType);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        subtype: {
            path: ['subtype'] as const,
            name: 'subtype',
            constraints: { required: true },
            label: 'Subtype',
            get: () => data.subtype,
            set: (value: string) => {
                data.subtype = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.subtype,
            setError: (value: Option<Array<string>>) => {
                errors.subtype = value;
            },
            getTainted: () => tainted.subtype,
            setTainted: (value: Option<boolean>) => {
                tainted.subtype = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('subtype', data.subtype);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        isTaxExempt: {
            path: ['isTaxExempt'] as const,
            name: 'isTaxExempt',
            constraints: { required: true },
            label: 'Tax Exempt',
            get: () => data.isTaxExempt,
            set: (value: boolean) => {
                data.isTaxExempt = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.isTaxExempt,
            setError: (value: Option<Array<string>>) => {
                errors.isTaxExempt = value;
            },
            getTainted: () => tainted.isTaxExempt,
            setTainted: (value: Option<boolean>) => {
                tainted.isTaxExempt = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('isTaxExempt', data.isTaxExempt);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        paymentTerms: {
            path: ['paymentTerms'] as const,
            name: 'paymentTerms',
            constraints: { required: true },
            label: 'Payment Terms',
            get: () => data.paymentTerms,
            set: (value: string) => {
                data.paymentTerms = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.paymentTerms,
            setError: (value: Option<Array<string>>) => {
                errors.paymentTerms = value;
            },
            getTainted: () => tainted.paymentTerms,
            setTainted: (value: Option<boolean>) => {
                tainted.paymentTerms = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('paymentTerms', data.paymentTerms);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        tags: {
            path: ['tags'] as const,
            name: 'tags',
            constraints: { required: true },
            label: 'Tags',
            get: () => data.tags,
            set: (value: string[]) => {
                data.tags = value;
            },
            transform: (value: string[]): string[] => value,
            getError: () => errors.tags,
            setError: (value: Option<Array<string>>) => {
                errors.tags = value;
            },
            getTainted: () => tainted.tags,
            setTainted: (value: Option<boolean>) => {
                tainted.tags = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('tags', data.tags);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['tags', index] as const,
                name: `tags.${index}`,
                constraints: { required: true },
                get: () => data.tags[index]!,
                set: (value: string) => {
                    data.tags[index] = value;
                },
                transform: (value: string): string => value,
                getError: () => errors.tags,
                setError: (value: Option<Array<string>>) => {
                    errors.tags = value;
                },
                getTainted: () => tainted.tags,
                setTainted: (value: Option<boolean>) => {
                    tainted.tags = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: string) => {
                data.tags.push(item);
            },
            remove: (index: number) => {
                data.tags.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.tags[a]!;
                data.tags[a] = data.tags[b]!;
                data.tags[b] = tmp;
            }
        },
        customFields: {
            path: ['customFields'] as const,
            name: 'customFields',
            constraints: { required: true },
            get: () => data.customFields,
            set: (value: [string, string][]) => {
                data.customFields = value;
            },
            transform: (value: [string, string][]): [string, string][] => value,
            getError: () => errors.customFields,
            setError: (value: Option<Array<string>>) => {
                errors.customFields = value;
            },
            getTainted: () => tainted.customFields,
            setTainted: (value: Option<boolean>) => {
                tainted.customFields = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = leadValidateField('customFields', data.customFields);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['customFields', index] as const,
                name: `customFields.${index}`,
                constraints: { required: true },
                get: () => data.customFields[index]!,
                set: (value: [string, string]) => {
                    data.customFields[index] = value;
                },
                transform: (value: [string, string]): [string, string] => value,
                getError: () => errors.customFields,
                setError: (value: Option<Array<string>>) => {
                    errors.customFields = value;
                },
                getTainted: () => tainted.customFields,
                setTainted: (value: Option<boolean>) => {
                    tainted.customFields = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: [string, string]) => {
                data.customFields.push(item);
            },
            remove: (index: number) => {
                data.customFields.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.customFields[a]!;
                data.customFields[a] = data.customFields[b]!;
                data.customFields[b] = tmp;
            }
        }
    };
    function validate(): Result<Lead, Array<{ field: string; message: string }>> {
        return leadDeserialize(data);
    }
    function reset(newOverrides?: Partial<Lead>): void {
        data = { ...leadDefaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            id: Option.none(),
            number: Option.none(),
            accepted: Option.none(),
            probability: Option.none(),
            priority: Option.none(),
            dueDate: Option.none(),
            closeDate: Option.none(),
            value: Option.none(),
            stage: Option.none(),
            status: Option.none(),
            description: Option.none(),
            nextStep: Option.none(),
            favorite: Option.none(),
            dateAdded: Option.none(),
            taxRate: Option.none(),
            sector: Option.none(),
            leadName: Option.none(),
            phones: Option.none(),
            email: Option.none(),
            leadSource: Option.none(),
            site: Option.none(),
            memo: Option.none(),
            needsReview: Option.none(),
            hasAlert: Option.none(),
            salesRep: Option.none(),
            color: Option.none(),
            accountType: Option.none(),
            subtype: Option.none(),
            isTaxExempt: Option.none(),
            paymentTerms: Option.none(),
            tags: Option.none(),
            customFields: Option.none()
        };
        tainted = {
            id: Option.none(),
            number: Option.none(),
            accepted: Option.none(),
            probability: Option.none(),
            priority: Option.none(),
            dueDate: Option.none(),
            closeDate: Option.none(),
            value: Option.none(),
            stage: Option.none(),
            status: Option.none(),
            description: Option.none(),
            nextStep: Option.none(),
            favorite: Option.none(),
            dateAdded: Option.none(),
            taxRate: Option.none(),
            sector: Option.none(),
            leadName: Option.none(),
            phones: Option.none(),
            email: Option.none(),
            leadSource: Option.none(),
            site: Option.none(),
            memo: Option.none(),
            needsReview: Option.none(),
            hasAlert: Option.none(),
            salesRep: Option.none(),
            color: Option.none(),
            accountType: Option.none(),
            subtype: Option.none(),
            isTaxExempt: Option.none(),
            paymentTerms: Option.none(),
            tags: Option.none(),
            customFields: Option.none()
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */
export function leadFromFormData(
    formData: FormData
): Result<Lead, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    {
        const numberStr = formData.get('number');
        obj.number = numberStr ? parseFloat(numberStr as string) : 0;
        if (obj.number !== undefined && isNaN(obj.number as number)) obj.number = 0;
    }
    {
        const acceptedVal = formData.get('accepted');
        obj.accepted = acceptedVal === 'true' || acceptedVal === 'on' || acceptedVal === '1';
    }
    {
        const probabilityStr = formData.get('probability');
        obj.probability = probabilityStr ? parseFloat(probabilityStr as string) : 0;
        if (obj.probability !== undefined && isNaN(obj.probability as number)) obj.probability = 0;
    }
    {
        const priorityObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('priority.')) {
                const fieldName = key.slice('priority.'.length);
                const parts = fieldName.split('.');
                let current = priorityObj;
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
        obj.priority = priorityObj;
    }
    obj.dueDate = formData.get('dueDate') ?? '';
    obj.closeDate = formData.get('closeDate') ?? '';
    {
        const valueStr = formData.get('value');
        obj.value = valueStr ? parseFloat(valueStr as string) : 0;
        if (obj.value !== undefined && isNaN(obj.value as number)) obj.value = 0;
    }
    {
        const stageObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('stage.')) {
                const fieldName = key.slice('stage.'.length);
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
    obj.status = formData.get('status') ?? '';
    obj.description = formData.get('description') ?? '';
    {
        const nextStepObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('nextStep.')) {
                const fieldName = key.slice('nextStep.'.length);
                const parts = fieldName.split('.');
                let current = nextStepObj;
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
        obj.nextStep = nextStepObj;
    }
    {
        const favoriteVal = formData.get('favorite');
        obj.favorite = favoriteVal === 'true' || favoriteVal === 'on' || favoriteVal === '1';
    }
    obj.dateAdded = formData.get('dateAdded') ?? '';
    obj.taxRate = formData.get('taxRate') ?? '';
    {
        const sectorObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('sector.')) {
                const fieldName = key.slice('sector.'.length);
                const parts = fieldName.split('.');
                let current = sectorObj;
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
        obj.sector = sectorObj;
    }
    {
        const leadNameObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('leadName.')) {
                const fieldName = key.slice('leadName.'.length);
                const parts = fieldName.split('.');
                let current = leadNameObj;
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
        obj.leadName = leadNameObj;
    }
    {
        const phonesItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('phones.' + idx + '.') || idx === 0) {
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('phones.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('phones.' + idx + '.')) {
                        const fieldName = key.slice('phones.'.length + String(idx).length + 1);
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
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('email.')) {
                const fieldName = key.slice('email.'.length);
                const parts = fieldName.split('.');
                let current = emailObj;
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
        obj.email = emailObj;
    }
    obj.leadSource = formData.get('leadSource') ?? '';
    obj.site = formData.get('site') ?? '';
    obj.memo = formData.get('memo') ?? '';
    {
        const needsReviewVal = formData.get('needsReview');
        obj.needsReview =
            needsReviewVal === 'true' || needsReviewVal === 'on' || needsReviewVal === '1';
    }
    {
        const hasAlertVal = formData.get('hasAlert');
        obj.hasAlert = hasAlertVal === 'true' || hasAlertVal === 'on' || hasAlertVal === '1';
    }
    obj.salesRep = formData.get('salesRep') ?? '';
    obj.color = formData.get('color') ?? '';
    obj.accountType = formData.get('accountType') ?? '';
    obj.subtype = formData.get('subtype') ?? '';
    {
        const isTaxExemptVal = formData.get('isTaxExempt');
        obj.isTaxExempt =
            isTaxExemptVal === 'true' || isTaxExemptVal === 'on' || isTaxExemptVal === '1';
    }
    obj.paymentTerms = formData.get('paymentTerms') ?? '';
    obj.tags = formData.getAll('tags') as Array<string>;
    {
        const customFieldsItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('customFields.' + idx + '.') || idx === 0) {
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('customFields.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('customFields.' + idx + '.')) {
                        const fieldName = key.slice(
                            'customFields.'.length + String(idx).length + 1
                        );
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
    return leadDeserialize(obj);
}

export const Lead = {
    defaultValue: leadDefaultValue,
    serialize: leadSerialize,
    serializeWithContext: leadSerializeWithContext,
    deserialize: leadDeserialize,
    deserializeWithContext: leadDeserializeWithContext,
    validateFields: leadValidateFields,
    hasShape: leadHasShape,
    is: leadIs,
    createForm: leadCreateForm,
    fromFormData: leadFromFormData
} as const;
