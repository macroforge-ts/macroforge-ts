import { accountNameDefaultValue } from './account-name.svelte';
import { colorsDefaultValue } from './colors.svelte';
import { emailDefaultValue } from './email.svelte';
import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { accountNameSerializeWithContext } from './account-name.svelte';
import { colorsSerializeWithContext } from './colors.svelte';
import { didSerializeWithContext } from './did.svelte';
import { emailSerializeWithContext } from './email.svelte';
import { orderedSerializeWithContext } from './ordered.svelte';
import { phoneNumberSerializeWithContext } from './phone-number.svelte';
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
import { colorsDeserializeWithContext } from './colors.svelte';
import { emailDeserializeWithContext } from './email.svelte';
import { sectorDeserializeWithContext } from './sector.svelte';
import { Result } from 'macroforge/reexports';
import { Option } from 'macroforge/reexports';
import type { FieldController } from '@playground/macro/gigaform';
import type { ArrayFieldController } from '@playground/macro/gigaform';
import type { Did } from './did.svelte';
import type { PersonName } from './person-name.svelte';
import type { Site } from './site.svelte';
import type { PhoneNumber } from './phone-number.svelte';
import type { Represents } from './represents.svelte';
import type { Payment } from './payment.svelte';
import type { CompanyName } from './company-name.svelte';
import type { Custom } from './custom.svelte';
import type { Colors } from './colors.svelte';
import type { TaxRate } from './tax-rate.svelte';
import type { Lead } from './lead.svelte';
import type { Company } from './company.svelte';
import type { Ordered } from './ordered.svelte';
import type { Email } from './email.svelte';
import type { Sector } from './sector.svelte';
import type { AccountName } from './account-name.svelte';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Account {
    id: string;

    taxRate: string | TaxRate;

    site: string | Site;

    salesRep: Represents[] | null;

    orders: Ordered[];

    activity: Did[];

    customFields: [string, string][];

    accountName: AccountName;

    sector: Sector;

    memo: string | null;

    phones: PhoneNumber[];

    email: Email;

    leadSource: string;

    colors: Colors;

    needsReview: boolean;

    hasAlert: boolean;

    accountType: string;

    subtype: string;

    isTaxExempt: boolean;

    paymentTerms: string;

    tags: string[];

    dateAdded: string;
}

export function accountDefaultValue(): Account {
    return {
        id: '',
        taxRate: '',
        site: '',
        salesRep: null,
        orders: [],
        activity: [],
        customFields: [],
        accountName: accountNameDefaultValue(),
        sector: 'Residential',
        memo: null,
        phones: [],
        email: emailDefaultValue(),
        leadSource: '',
        colors: colorsDefaultValue(),
        needsReview: false,
        hasAlert: false,
        accountType: '',
        subtype: '',
        isTaxExempt: false,
        paymentTerms: '',
        tags: [],
        dateAdded: ''
    } as Account;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function accountSerialize(
    value: Account
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(accountSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function accountSerializeWithContext(
    value: Account,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Account', __id };
    result['id'] = value.id;
    result['taxRate'] = value.taxRate;
    result['site'] = value.site;
    if (value.salesRep !== null) {
        result['salesRep'] = value.salesRep;
    } else {
        result['salesRep'] = null;
    }
    result['orders'] = value.orders.map((item) => orderedSerializeWithContext(item, ctx));
    result['activity'] = value.activity.map((item) => didSerializeWithContext(item, ctx));
    result['customFields'] = value.customFields;
    result['accountName'] = accountNameSerializeWithContext(value.accountName, ctx);
    result['sector'] = sectorSerializeWithContext(value.sector, ctx);
    result['memo'] = value.memo;
    result['phones'] = value.phones.map((item) => phoneNumberSerializeWithContext(item, ctx));
    result['email'] = emailSerializeWithContext(value.email, ctx);
    result['leadSource'] = value.leadSource;
    result['colors'] = colorsSerializeWithContext(value.colors, ctx);
    result['needsReview'] = value.needsReview;
    result['hasAlert'] = value.hasAlert;
    result['accountType'] = value.accountType;
    result['subtype'] = value.subtype;
    result['isTaxExempt'] = value.isTaxExempt;
    result['paymentTerms'] = value.paymentTerms;
    result['tags'] = value.tags;
    result['dateAdded'] = value.dateAdded;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function accountDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Account> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = accountDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'Account.deserialize: root cannot be a forward reference'
                }
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
export function accountDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Account | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Account.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('taxRate' in obj)) {
        errors.push({ field: 'taxRate', message: 'missing required field' });
    }
    if (!('site' in obj)) {
        errors.push({ field: 'site', message: 'missing required field' });
    }
    if (!('salesRep' in obj)) {
        errors.push({ field: 'salesRep', message: 'missing required field' });
    }
    if (!('orders' in obj)) {
        errors.push({ field: 'orders', message: 'missing required field' });
    }
    if (!('activity' in obj)) {
        errors.push({ field: 'activity', message: 'missing required field' });
    }
    if (!('customFields' in obj)) {
        errors.push({ field: 'customFields', message: 'missing required field' });
    }
    if (!('accountName' in obj)) {
        errors.push({ field: 'accountName', message: 'missing required field' });
    }
    if (!('sector' in obj)) {
        errors.push({ field: 'sector', message: 'missing required field' });
    }
    if (!('memo' in obj)) {
        errors.push({ field: 'memo', message: 'missing required field' });
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
    if (!('colors' in obj)) {
        errors.push({ field: 'colors', message: 'missing required field' });
    }
    if (!('needsReview' in obj)) {
        errors.push({ field: 'needsReview', message: 'missing required field' });
    }
    if (!('hasAlert' in obj)) {
        errors.push({ field: 'hasAlert', message: 'missing required field' });
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
    if (!('dateAdded' in obj)) {
        errors.push({ field: 'dateAdded', message: 'missing required field' });
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
        const __raw_taxRate = obj['taxRate'] as string | TaxRate;
        instance.taxRate = __raw_taxRate;
    }
    {
        const __raw_site = obj['site'] as string | Site;
        instance.site = __raw_site;
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
        const __raw_orders = obj['orders'] as Ordered[];
        if (Array.isArray(__raw_orders)) {
            instance.orders = __raw_orders as Ordered[];
        }
    }
    {
        const __raw_activity = obj['activity'] as Did[];
        if (Array.isArray(__raw_activity)) {
            instance.activity = __raw_activity as Did[];
        }
    }
    {
        const __raw_customFields = obj['customFields'] as [string, string][];
        if (Array.isArray(__raw_customFields)) {
            instance.customFields = __raw_customFields as [string, string][];
        }
    }
    {
        const __raw_accountName = obj['accountName'] as AccountName;
        {
            const __result = accountNameDeserializeWithContext(__raw_accountName, ctx);
            ctx.assignOrDefer(instance, 'accountName', __result);
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
        const __raw_memo = obj['memo'] as string | null;
        instance.memo = __raw_memo;
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
        const __raw_leadSource = obj['leadSource'] as string;
        if (__raw_leadSource.length === 0) {
            errors.push({ field: 'leadSource', message: 'must not be empty' });
        }
        instance.leadSource = __raw_leadSource;
    }
    {
        const __raw_colors = obj['colors'] as Colors;
        {
            const __result = colorsDeserializeWithContext(__raw_colors, ctx);
            ctx.assignOrDefer(instance, 'colors', __result);
        }
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
        const __raw_dateAdded = obj['dateAdded'] as string;
        instance.dateAdded = __raw_dateAdded;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Account;
}
export function accountValidateField<K extends keyof Account>(
    field: K,
    value: Account[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'leadSource': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'leadSource', message: 'must not be empty' });
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
export function accountValidateFields(
    partial: Partial<Account>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('leadSource' in partial && partial.leadSource !== undefined) {
        const __val = partial.leadSource as string;
        if (__val.length === 0) {
            errors.push({ field: 'leadSource', message: 'must not be empty' });
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
export function accountHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'id' in o &&
        'taxRate' in o &&
        'site' in o &&
        'salesRep' in o &&
        'orders' in o &&
        'activity' in o &&
        'customFields' in o &&
        'accountName' in o &&
        'sector' in o &&
        'memo' in o &&
        'phones' in o &&
        'email' in o &&
        'leadSource' in o &&
        'colors' in o &&
        'needsReview' in o &&
        'hasAlert' in o &&
        'accountType' in o &&
        'subtype' in o &&
        'isTaxExempt' in o &&
        'paymentTerms' in o &&
        'tags' in o &&
        'dateAdded' in o
    );
}
export function accountIs(obj: unknown): obj is Account {
    if (!accountHasShape(obj)) {
        return false;
    }
    const result = accountDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type AccountErrors = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    taxRate: Option<Array<string>>;
    site: Option<Array<string>>;
    salesRep: Option<Array<string>>;
    orders: Option<Array<string>>;
    activity: Option<Array<string>>;
    customFields: Option<Array<string>>;
    accountName: Option<Array<string>>;
    sector: Option<Array<string>>;
    memo: Option<Array<string>>;
    phones: Option<Array<string>>;
    email: Option<Array<string>>;
    leadSource: Option<Array<string>>;
    colors: Option<Array<string>>;
    needsReview: Option<Array<string>>;
    hasAlert: Option<Array<string>>;
    accountType: Option<Array<string>>;
    subtype: Option<Array<string>>;
    isTaxExempt: Option<Array<string>>;
    paymentTerms: Option<Array<string>>;
    tags: Option<Array<string>>;
    dateAdded: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type AccountTainted = {
    id: Option<boolean>;
    taxRate: Option<boolean>;
    site: Option<boolean>;
    salesRep: Option<boolean>;
    orders: Option<boolean>;
    activity: Option<boolean>;
    customFields: Option<boolean>;
    accountName: Option<boolean>;
    sector: Option<boolean>;
    memo: Option<boolean>;
    phones: Option<boolean>;
    email: Option<boolean>;
    leadSource: Option<boolean>;
    colors: Option<boolean>;
    needsReview: Option<boolean>;
    hasAlert: Option<boolean>;
    accountType: Option<boolean>;
    subtype: Option<boolean>;
    isTaxExempt: Option<boolean>;
    paymentTerms: Option<boolean>;
    tags: Option<boolean>;
    dateAdded: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface AccountFieldControllers {
    readonly id: FieldController<string>;
    readonly taxRate: FieldController<string | TaxRate>;
    readonly site: FieldController<string | Site>;
    readonly salesRep: FieldController<Represents[] | null>;
    readonly orders: ArrayFieldController<Ordered>;
    readonly activity: ArrayFieldController<Did>;
    readonly customFields: ArrayFieldController<[string, string]>;
    readonly accountName: FieldController<AccountName>;
    readonly sector: FieldController<Sector>;
    readonly memo: FieldController<string | null>;
    readonly phones: ArrayFieldController<PhoneNumber>;
    readonly email: FieldController<Email>;
    readonly leadSource: FieldController<string>;
    readonly colors: FieldController<Colors>;
    readonly needsReview: FieldController<boolean>;
    readonly hasAlert: FieldController<boolean>;
    readonly accountType: FieldController<string>;
    readonly subtype: FieldController<string>;
    readonly isTaxExempt: FieldController<boolean>;
    readonly paymentTerms: FieldController<string>;
    readonly tags: ArrayFieldController<string>;
    readonly dateAdded: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface AccountGigaform {
    readonly data: Account;
    readonly errors: AccountErrors;
    readonly tainted: AccountTainted;
    readonly fields: AccountFieldControllers;
    validate(): Result<Account, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Account>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function accountCreateForm(overrides?: Partial<Account>): AccountGigaform {
    let data = $state({ ...accountDefaultValue(), ...overrides });
    let errors = $state<AccountErrors>({
        _errors: Option.none(),
        id: Option.none(),
        taxRate: Option.none(),
        site: Option.none(),
        salesRep: Option.none(),
        orders: Option.none(),
        activity: Option.none(),
        customFields: Option.none(),
        accountName: Option.none(),
        sector: Option.none(),
        memo: Option.none(),
        phones: Option.none(),
        email: Option.none(),
        leadSource: Option.none(),
        colors: Option.none(),
        needsReview: Option.none(),
        hasAlert: Option.none(),
        accountType: Option.none(),
        subtype: Option.none(),
        isTaxExempt: Option.none(),
        paymentTerms: Option.none(),
        tags: Option.none(),
        dateAdded: Option.none()
    });
    let tainted = $state<AccountTainted>({
        id: Option.none(),
        taxRate: Option.none(),
        site: Option.none(),
        salesRep: Option.none(),
        orders: Option.none(),
        activity: Option.none(),
        customFields: Option.none(),
        accountName: Option.none(),
        sector: Option.none(),
        memo: Option.none(),
        phones: Option.none(),
        email: Option.none(),
        leadSource: Option.none(),
        colors: Option.none(),
        needsReview: Option.none(),
        hasAlert: Option.none(),
        accountType: Option.none(),
        subtype: Option.none(),
        isTaxExempt: Option.none(),
        paymentTerms: Option.none(),
        tags: Option.none(),
        dateAdded: Option.none()
    });
    const fields: AccountFieldControllers = {
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
                const fieldErrors = accountValidateField('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        taxRate: {
            path: ['taxRate'] as const,
            name: 'taxRate',
            constraints: { required: true },
            label: 'Tax Rate',
            get: () => data.taxRate,
            set: (value: string | TaxRate) => {
                data.taxRate = value;
            },
            transform: (value: string | TaxRate): string | TaxRate => value,
            getError: () => errors.taxRate,
            setError: (value: Option<Array<string>>) => {
                errors.taxRate = value;
            },
            getTainted: () => tainted.taxRate,
            setTainted: (value: Option<boolean>) => {
                tainted.taxRate = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = accountValidateField('taxRate', data.taxRate);
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
                const fieldErrors = accountValidateField('site', data.site);
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
                const fieldErrors = accountValidateField('salesRep', data.salesRep);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        orders: {
            path: ['orders'] as const,
            name: 'orders',
            constraints: { required: true },
            get: () => data.orders,
            set: (value: Ordered[]) => {
                data.orders = value;
            },
            transform: (value: Ordered[]): Ordered[] => value,
            getError: () => errors.orders,
            setError: (value: Option<Array<string>>) => {
                errors.orders = value;
            },
            getTainted: () => tainted.orders,
            setTainted: (value: Option<boolean>) => {
                tainted.orders = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = accountValidateField('orders', data.orders);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['orders', index] as const,
                name: `orders.${index}`,
                constraints: { required: true },
                get: () => data.orders[index]!,
                set: (value: Ordered) => {
                    data.orders[index] = value;
                },
                transform: (value: Ordered): Ordered => value,
                getError: () => errors.orders,
                setError: (value: Option<Array<string>>) => {
                    errors.orders = value;
                },
                getTainted: () => tainted.orders,
                setTainted: (value: Option<boolean>) => {
                    tainted.orders = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: Ordered) => {
                data.orders.push(item);
            },
            remove: (index: number) => {
                data.orders.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.orders[a]!;
                data.orders[a] = data.orders[b]!;
                data.orders[b] = tmp;
            }
        },
        activity: {
            path: ['activity'] as const,
            name: 'activity',
            constraints: { required: true },
            get: () => data.activity,
            set: (value: Did[]) => {
                data.activity = value;
            },
            transform: (value: Did[]): Did[] => value,
            getError: () => errors.activity,
            setError: (value: Option<Array<string>>) => {
                errors.activity = value;
            },
            getTainted: () => tainted.activity,
            setTainted: (value: Option<boolean>) => {
                tainted.activity = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = accountValidateField('activity', data.activity);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['activity', index] as const,
                name: `activity.${index}`,
                constraints: { required: true },
                get: () => data.activity[index]!,
                set: (value: Did) => {
                    data.activity[index] = value;
                },
                transform: (value: Did): Did => value,
                getError: () => errors.activity,
                setError: (value: Option<Array<string>>) => {
                    errors.activity = value;
                },
                getTainted: () => tainted.activity,
                setTainted: (value: Option<boolean>) => {
                    tainted.activity = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: Did) => {
                data.activity.push(item);
            },
            remove: (index: number) => {
                data.activity.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.activity[a]!;
                data.activity[a] = data.activity[b]!;
                data.activity[b] = tmp;
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
                const fieldErrors = accountValidateField('customFields', data.customFields);
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
        },
        accountName: {
            path: ['accountName'] as const,
            name: 'accountName',
            constraints: { required: true },
            get: () => data.accountName,
            set: (value: AccountName) => {
                data.accountName = value;
            },
            transform: (value: AccountName): AccountName => value,
            getError: () => errors.accountName,
            setError: (value: Option<Array<string>>) => {
                errors.accountName = value;
            },
            getTainted: () => tainted.accountName,
            setTainted: (value: Option<boolean>) => {
                tainted.accountName = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = accountValidateField('accountName', data.accountName);
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
                const fieldErrors = accountValidateField('sector', data.sector);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        memo: {
            path: ['memo'] as const,
            name: 'memo',
            constraints: { required: true },
            label: 'Memo',
            get: () => data.memo,
            set: (value: string | null) => {
                data.memo = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.memo,
            setError: (value: Option<Array<string>>) => {
                errors.memo = value;
            },
            getTainted: () => tainted.memo,
            setTainted: (value: Option<boolean>) => {
                tainted.memo = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = accountValidateField('memo', data.memo);
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
                const fieldErrors = accountValidateField('phones', data.phones);
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
                const fieldErrors = accountValidateField('email', data.email);
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
                const fieldErrors = accountValidateField('leadSource', data.leadSource);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        colors: {
            path: ['colors'] as const,
            name: 'colors',
            constraints: { required: true },
            get: () => data.colors,
            set: (value: Colors) => {
                data.colors = value;
            },
            transform: (value: Colors): Colors => value,
            getError: () => errors.colors,
            setError: (value: Option<Array<string>>) => {
                errors.colors = value;
            },
            getTainted: () => tainted.colors,
            setTainted: (value: Option<boolean>) => {
                tainted.colors = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = accountValidateField('colors', data.colors);
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
                const fieldErrors = accountValidateField('needsReview', data.needsReview);
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
                const fieldErrors = accountValidateField('hasAlert', data.hasAlert);
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
                const fieldErrors = accountValidateField('accountType', data.accountType);
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
                const fieldErrors = accountValidateField('subtype', data.subtype);
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
                const fieldErrors = accountValidateField('isTaxExempt', data.isTaxExempt);
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
                const fieldErrors = accountValidateField('paymentTerms', data.paymentTerms);
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
                const fieldErrors = accountValidateField('tags', data.tags);
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
        dateAdded: {
            path: ['dateAdded'] as const,
            name: 'dateAdded',
            constraints: { required: true },
            get: () => data.dateAdded,
            set: (value: string) => {
                data.dateAdded = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.dateAdded,
            setError: (value: Option<Array<string>>) => {
                errors.dateAdded = value;
            },
            getTainted: () => tainted.dateAdded,
            setTainted: (value: Option<boolean>) => {
                tainted.dateAdded = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = accountValidateField('dateAdded', data.dateAdded);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Account, Array<{ field: string; message: string }>> {
        return accountDeserialize(data);
    }
    function reset(newOverrides?: Partial<Account>): void {
        data = { ...accountDefaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            id: Option.none(),
            taxRate: Option.none(),
            site: Option.none(),
            salesRep: Option.none(),
            orders: Option.none(),
            activity: Option.none(),
            customFields: Option.none(),
            accountName: Option.none(),
            sector: Option.none(),
            memo: Option.none(),
            phones: Option.none(),
            email: Option.none(),
            leadSource: Option.none(),
            colors: Option.none(),
            needsReview: Option.none(),
            hasAlert: Option.none(),
            accountType: Option.none(),
            subtype: Option.none(),
            isTaxExempt: Option.none(),
            paymentTerms: Option.none(),
            tags: Option.none(),
            dateAdded: Option.none()
        };
        tainted = {
            id: Option.none(),
            taxRate: Option.none(),
            site: Option.none(),
            salesRep: Option.none(),
            orders: Option.none(),
            activity: Option.none(),
            customFields: Option.none(),
            accountName: Option.none(),
            sector: Option.none(),
            memo: Option.none(),
            phones: Option.none(),
            email: Option.none(),
            leadSource: Option.none(),
            colors: Option.none(),
            needsReview: Option.none(),
            hasAlert: Option.none(),
            accountType: Option.none(),
            subtype: Option.none(),
            isTaxExempt: Option.none(),
            paymentTerms: Option.none(),
            tags: Option.none(),
            dateAdded: Option.none()
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
export function accountFromFormData(
    formData: FormData
): Result<Account, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.taxRate = formData.get('taxRate') ?? '';
    obj.site = formData.get('site') ?? '';
    obj.salesRep = formData.get('salesRep') ?? '';
    {
        const ordersItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('orders.' + idx + '.') || idx === 0) {
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('orders.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('orders.' + idx + '.')) {
                        const fieldName = key.slice('orders.'.length + String(idx).length + 1);
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
        while (formData.has('activity.' + idx + '.') || idx === 0) {
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('activity.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('activity.' + idx + '.')) {
                        const fieldName = key.slice('activity.'.length + String(idx).length + 1);
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
    {
        const accountNameObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('accountName.')) {
                const fieldName = key.slice('accountName.'.length);
                const parts = fieldName.split('.');
                let current = accountNameObj;
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
        obj.accountName = accountNameObj;
    }
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
    obj.memo = formData.get('memo') ?? '';
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
    {
        const colorsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('colors.')) {
                const fieldName = key.slice('colors.'.length);
                const parts = fieldName.split('.');
                let current = colorsObj;
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
        obj.colors = colorsObj;
    }
    {
        const needsReviewVal = formData.get('needsReview');
        obj.needsReview =
            needsReviewVal === 'true' || needsReviewVal === 'on' || needsReviewVal === '1';
    }
    {
        const hasAlertVal = formData.get('hasAlert');
        obj.hasAlert = hasAlertVal === 'true' || hasAlertVal === 'on' || hasAlertVal === '1';
    }
    obj.accountType = formData.get('accountType') ?? '';
    obj.subtype = formData.get('subtype') ?? '';
    {
        const isTaxExemptVal = formData.get('isTaxExempt');
        obj.isTaxExempt =
            isTaxExemptVal === 'true' || isTaxExemptVal === 'on' || isTaxExemptVal === '1';
    }
    obj.paymentTerms = formData.get('paymentTerms') ?? '';
    obj.tags = formData.getAll('tags') as Array<string>;
    obj.dateAdded = formData.get('dateAdded') ?? '';
    return accountDeserialize(obj);
}

export const Account = {
    defaultValue: accountDefaultValue,
    serialize: accountSerialize,
    serializeWithContext: accountSerializeWithContext,
    deserialize: accountDeserialize,
    deserializeWithContext: accountDeserializeWithContext,
    validateFields: accountValidateFields,
    hasShape: accountHasShape,
    is: accountIs,
    createForm: accountCreateForm,
    fromFormData: accountFromFormData
} as const;
