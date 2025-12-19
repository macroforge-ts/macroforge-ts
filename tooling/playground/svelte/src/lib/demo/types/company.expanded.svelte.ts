import { colorsConfigDefaultValue } from './colors-config.svelte';
import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { colorsConfigSerializeWithContext } from './colors-config.svelte';
import { phoneNumberSerializeWithContext } from './phone-number.svelte';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { colorsConfigDeserializeWithContext } from './colors-config.svelte';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option as __gf_Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
import type { ArrayFieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { Site } from './site.svelte';
import type { PhoneNumber } from './phone-number.svelte';
import type { TaxRate } from './tax-rate.svelte';
import type { ColorsConfig } from './colors-config.svelte';

export interface Company {
    id: string;

    legalName: string;

    headquarters: string | Site;
    phones: PhoneNumber[];

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
        id: '',
        legalName: '',
        headquarters: '',
        phones: [],
        fax: '',
        email: '',
        website: '',
        taxId: '',
        referenceNumber: 0,
        postalCodeLookup: '',
        timeZone: '',
        defaultTax: '',
        defaultTaxLocation: '',
        defaultAreaCode: 0,
        defaultAccountType: '',
        lookupFormatting: '',
        accountNameFormat: '',
        merchantServiceProvider: null,
        dateDisplayStyle: '',
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

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function companySerialize(
    value: Company
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(companySerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function companySerializeWithContext(
    value: Company,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Company', __id };
    result['id'] = value.id;
    result['legalName'] = value.legalName;
    result['headquarters'] = value.headquarters;
    result['phones'] = value.phones.map((item) => phoneNumberSerializeWithContext(item, ctx));
    result['fax'] = value.fax;
    result['email'] = value.email;
    result['website'] = value.website;
    result['taxId'] = value.taxId;
    result['referenceNumber'] = value.referenceNumber;
    result['postalCodeLookup'] = value.postalCodeLookup;
    result['timeZone'] = value.timeZone;
    result['defaultTax'] = value.defaultTax;
    result['defaultTaxLocation'] = value.defaultTaxLocation;
    result['defaultAreaCode'] = value.defaultAreaCode;
    result['defaultAccountType'] = value.defaultAccountType;
    result['lookupFormatting'] = value.lookupFormatting;
    result['accountNameFormat'] = value.accountNameFormat;
    result['merchantServiceProvider'] = value.merchantServiceProvider;
    result['dateDisplayStyle'] = value.dateDisplayStyle;
    result['hasAutoCommission'] = value.hasAutoCommission;
    result['hasAutoDaylightSavings'] = value.hasAutoDaylightSavings;
    result['hasAutoFmsTracking'] = value.hasAutoFmsTracking;
    result['hasNotifications'] = value.hasNotifications;
    result['hasRequiredLeadSource'] = value.hasRequiredLeadSource;
    result['hasRequiredEmail'] = value.hasRequiredEmail;
    result['hasSortServiceItemsAlphabetically'] = value.hasSortServiceItemsAlphabetically;
    result['hasAttachOrderToAppointmentEmails'] = value.hasAttachOrderToAppointmentEmails;
    result['scheduleInterval'] = value.scheduleInterval;
    result['colorsConfig'] = colorsConfigSerializeWithContext(value.colorsConfig, ctx);
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function companyDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: Company }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = companyDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: '_root',
                        message: 'Company.deserialize: root cannot be a forward reference'
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return { success: true, value: resultOrRef };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return { success: false, errors: e.errors };
        }
        const message = e instanceof Error ? e.message : String(e);
        return { success: false, errors: [{ field: '_root', message }] };
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function companyDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Company | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Company.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('legalName' in obj)) {
        errors.push({ field: 'legalName', message: 'missing required field' });
    }
    if (!('headquarters' in obj)) {
        errors.push({ field: 'headquarters', message: 'missing required field' });
    }
    if (!('phones' in obj)) {
        errors.push({ field: 'phones', message: 'missing required field' });
    }
    if (!('fax' in obj)) {
        errors.push({ field: 'fax', message: 'missing required field' });
    }
    if (!('email' in obj)) {
        errors.push({ field: 'email', message: 'missing required field' });
    }
    if (!('website' in obj)) {
        errors.push({ field: 'website', message: 'missing required field' });
    }
    if (!('taxId' in obj)) {
        errors.push({ field: 'taxId', message: 'missing required field' });
    }
    if (!('referenceNumber' in obj)) {
        errors.push({ field: 'referenceNumber', message: 'missing required field' });
    }
    if (!('postalCodeLookup' in obj)) {
        errors.push({ field: 'postalCodeLookup', message: 'missing required field' });
    }
    if (!('timeZone' in obj)) {
        errors.push({ field: 'timeZone', message: 'missing required field' });
    }
    if (!('defaultTax' in obj)) {
        errors.push({ field: 'defaultTax', message: 'missing required field' });
    }
    if (!('defaultTaxLocation' in obj)) {
        errors.push({ field: 'defaultTaxLocation', message: 'missing required field' });
    }
    if (!('defaultAreaCode' in obj)) {
        errors.push({ field: 'defaultAreaCode', message: 'missing required field' });
    }
    if (!('defaultAccountType' in obj)) {
        errors.push({ field: 'defaultAccountType', message: 'missing required field' });
    }
    if (!('lookupFormatting' in obj)) {
        errors.push({ field: 'lookupFormatting', message: 'missing required field' });
    }
    if (!('accountNameFormat' in obj)) {
        errors.push({ field: 'accountNameFormat', message: 'missing required field' });
    }
    if (!('merchantServiceProvider' in obj)) {
        errors.push({ field: 'merchantServiceProvider', message: 'missing required field' });
    }
    if (!('dateDisplayStyle' in obj)) {
        errors.push({ field: 'dateDisplayStyle', message: 'missing required field' });
    }
    if (!('hasAutoCommission' in obj)) {
        errors.push({ field: 'hasAutoCommission', message: 'missing required field' });
    }
    if (!('hasAutoDaylightSavings' in obj)) {
        errors.push({ field: 'hasAutoDaylightSavings', message: 'missing required field' });
    }
    if (!('hasAutoFmsTracking' in obj)) {
        errors.push({ field: 'hasAutoFmsTracking', message: 'missing required field' });
    }
    if (!('hasNotifications' in obj)) {
        errors.push({ field: 'hasNotifications', message: 'missing required field' });
    }
    if (!('hasRequiredLeadSource' in obj)) {
        errors.push({ field: 'hasRequiredLeadSource', message: 'missing required field' });
    }
    if (!('hasRequiredEmail' in obj)) {
        errors.push({ field: 'hasRequiredEmail', message: 'missing required field' });
    }
    if (!('hasSortServiceItemsAlphabetically' in obj)) {
        errors.push({
            field: 'hasSortServiceItemsAlphabetically',
            message: 'missing required field'
        });
    }
    if (!('hasAttachOrderToAppointmentEmails' in obj)) {
        errors.push({
            field: 'hasAttachOrderToAppointmentEmails',
            message: 'missing required field'
        });
    }
    if (!('scheduleInterval' in obj)) {
        errors.push({ field: 'scheduleInterval', message: 'missing required field' });
    }
    if (!('colorsConfig' in obj)) {
        errors.push({ field: 'colorsConfig', message: 'missing required field' });
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
        const __raw_legalName = obj['legalName'] as string;
        if (__raw_legalName.length === 0) {
            errors.push({ field: 'legalName', message: 'must not be empty' });
        }
        instance.legalName = __raw_legalName;
    }
    {
        const __raw_headquarters = obj['headquarters'] as string | Site;
        instance.headquarters = __raw_headquarters;
    }
    {
        const __raw_phones = obj['phones'] as PhoneNumber[];
        if (Array.isArray(__raw_phones)) {
            instance.phones = __raw_phones as PhoneNumber[];
        }
    }
    {
        const __raw_fax = obj['fax'] as string;
        if (__raw_fax.length === 0) {
            errors.push({ field: 'fax', message: 'must not be empty' });
        }
        instance.fax = __raw_fax;
    }
    {
        const __raw_email = obj['email'] as string;
        if (__raw_email.length === 0) {
            errors.push({ field: 'email', message: 'must not be empty' });
        }
        instance.email = __raw_email;
    }
    {
        const __raw_website = obj['website'] as string;
        if (__raw_website.length === 0) {
            errors.push({ field: 'website', message: 'must not be empty' });
        }
        instance.website = __raw_website;
    }
    {
        const __raw_taxId = obj['taxId'] as string;
        if (__raw_taxId.length === 0) {
            errors.push({ field: 'taxId', message: 'must not be empty' });
        }
        instance.taxId = __raw_taxId;
    }
    {
        const __raw_referenceNumber = obj['referenceNumber'] as number;
        instance.referenceNumber = __raw_referenceNumber;
    }
    {
        const __raw_postalCodeLookup = obj['postalCodeLookup'] as string;
        if (__raw_postalCodeLookup.length === 0) {
            errors.push({ field: 'postalCodeLookup', message: 'must not be empty' });
        }
        instance.postalCodeLookup = __raw_postalCodeLookup;
    }
    {
        const __raw_timeZone = obj['timeZone'] as string;
        instance.timeZone = __raw_timeZone;
    }
    {
        const __raw_defaultTax = obj['defaultTax'] as string | TaxRate;
        instance.defaultTax = __raw_defaultTax;
    }
    {
        const __raw_defaultTaxLocation = obj['defaultTaxLocation'] as string;
        if (__raw_defaultTaxLocation.length === 0) {
            errors.push({ field: 'defaultTaxLocation', message: 'must not be empty' });
        }
        instance.defaultTaxLocation = __raw_defaultTaxLocation;
    }
    {
        const __raw_defaultAreaCode = obj['defaultAreaCode'] as number;
        instance.defaultAreaCode = __raw_defaultAreaCode;
    }
    {
        const __raw_defaultAccountType = obj['defaultAccountType'] as string;
        if (__raw_defaultAccountType.length === 0) {
            errors.push({ field: 'defaultAccountType', message: 'must not be empty' });
        }
        instance.defaultAccountType = __raw_defaultAccountType;
    }
    {
        const __raw_lookupFormatting = obj['lookupFormatting'] as string;
        if (__raw_lookupFormatting.length === 0) {
            errors.push({ field: 'lookupFormatting', message: 'must not be empty' });
        }
        instance.lookupFormatting = __raw_lookupFormatting;
    }
    {
        const __raw_accountNameFormat = obj['accountNameFormat'] as string;
        if (__raw_accountNameFormat.length === 0) {
            errors.push({ field: 'accountNameFormat', message: 'must not be empty' });
        }
        instance.accountNameFormat = __raw_accountNameFormat;
    }
    {
        const __raw_merchantServiceProvider = obj['merchantServiceProvider'] as string | null;
        instance.merchantServiceProvider = __raw_merchantServiceProvider;
    }
    {
        const __raw_dateDisplayStyle = obj['dateDisplayStyle'] as string;
        if (__raw_dateDisplayStyle.length === 0) {
            errors.push({ field: 'dateDisplayStyle', message: 'must not be empty' });
        }
        instance.dateDisplayStyle = __raw_dateDisplayStyle;
    }
    {
        const __raw_hasAutoCommission = obj['hasAutoCommission'] as boolean;
        instance.hasAutoCommission = __raw_hasAutoCommission;
    }
    {
        const __raw_hasAutoDaylightSavings = obj['hasAutoDaylightSavings'] as boolean;
        instance.hasAutoDaylightSavings = __raw_hasAutoDaylightSavings;
    }
    {
        const __raw_hasAutoFmsTracking = obj['hasAutoFmsTracking'] as boolean;
        instance.hasAutoFmsTracking = __raw_hasAutoFmsTracking;
    }
    {
        const __raw_hasNotifications = obj['hasNotifications'] as boolean;
        instance.hasNotifications = __raw_hasNotifications;
    }
    {
        const __raw_hasRequiredLeadSource = obj['hasRequiredLeadSource'] as boolean;
        instance.hasRequiredLeadSource = __raw_hasRequiredLeadSource;
    }
    {
        const __raw_hasRequiredEmail = obj['hasRequiredEmail'] as boolean;
        instance.hasRequiredEmail = __raw_hasRequiredEmail;
    }
    {
        const __raw_hasSortServiceItemsAlphabetically = obj[
            'hasSortServiceItemsAlphabetically'
        ] as boolean;
        instance.hasSortServiceItemsAlphabetically = __raw_hasSortServiceItemsAlphabetically;
    }
    {
        const __raw_hasAttachOrderToAppointmentEmails = obj[
            'hasAttachOrderToAppointmentEmails'
        ] as boolean;
        instance.hasAttachOrderToAppointmentEmails = __raw_hasAttachOrderToAppointmentEmails;
    }
    {
        const __raw_scheduleInterval = obj['scheduleInterval'] as number;
        instance.scheduleInterval = __raw_scheduleInterval;
    }
    {
        const __raw_colorsConfig = obj['colorsConfig'] as ColorsConfig;
        {
            const __result = colorsConfigDeserializeWithContext(__raw_colorsConfig, ctx);
            ctx.assignOrDefer(instance, 'colorsConfig', __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Company;
}
export function companyValidateField<K extends keyof Company>(
    _field: K,
    _value: Company[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (_field) {
        case 'legalName': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'legalName', message: 'must not be empty' });
            }
            break;
        }
        case 'fax': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'fax', message: 'must not be empty' });
            }
            break;
        }
        case 'email': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'email', message: 'must not be empty' });
            }
            break;
        }
        case 'website': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'website', message: 'must not be empty' });
            }
            break;
        }
        case 'taxId': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'taxId', message: 'must not be empty' });
            }
            break;
        }
        case 'postalCodeLookup': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'postalCodeLookup', message: 'must not be empty' });
            }
            break;
        }
        case 'defaultTaxLocation': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'defaultTaxLocation', message: 'must not be empty' });
            }
            break;
        }
        case 'defaultAccountType': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'defaultAccountType', message: 'must not be empty' });
            }
            break;
        }
        case 'lookupFormatting': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'lookupFormatting', message: 'must not be empty' });
            }
            break;
        }
        case 'accountNameFormat': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'accountNameFormat', message: 'must not be empty' });
            }
            break;
        }
        case 'dateDisplayStyle': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'dateDisplayStyle', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function companyValidateFields(
    _partial: Partial<Company>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('legalName' in _partial && _partial.legalName !== undefined) {
        const __val = _partial.legalName as string;
        if (__val.length === 0) {
            errors.push({ field: 'legalName', message: 'must not be empty' });
        }
    }
    if ('fax' in _partial && _partial.fax !== undefined) {
        const __val = _partial.fax as string;
        if (__val.length === 0) {
            errors.push({ field: 'fax', message: 'must not be empty' });
        }
    }
    if ('email' in _partial && _partial.email !== undefined) {
        const __val = _partial.email as string;
        if (__val.length === 0) {
            errors.push({ field: 'email', message: 'must not be empty' });
        }
    }
    if ('website' in _partial && _partial.website !== undefined) {
        const __val = _partial.website as string;
        if (__val.length === 0) {
            errors.push({ field: 'website', message: 'must not be empty' });
        }
    }
    if ('taxId' in _partial && _partial.taxId !== undefined) {
        const __val = _partial.taxId as string;
        if (__val.length === 0) {
            errors.push({ field: 'taxId', message: 'must not be empty' });
        }
    }
    if ('postalCodeLookup' in _partial && _partial.postalCodeLookup !== undefined) {
        const __val = _partial.postalCodeLookup as string;
        if (__val.length === 0) {
            errors.push({ field: 'postalCodeLookup', message: 'must not be empty' });
        }
    }
    if ('defaultTaxLocation' in _partial && _partial.defaultTaxLocation !== undefined) {
        const __val = _partial.defaultTaxLocation as string;
        if (__val.length === 0) {
            errors.push({ field: 'defaultTaxLocation', message: 'must not be empty' });
        }
    }
    if ('defaultAccountType' in _partial && _partial.defaultAccountType !== undefined) {
        const __val = _partial.defaultAccountType as string;
        if (__val.length === 0) {
            errors.push({ field: 'defaultAccountType', message: 'must not be empty' });
        }
    }
    if ('lookupFormatting' in _partial && _partial.lookupFormatting !== undefined) {
        const __val = _partial.lookupFormatting as string;
        if (__val.length === 0) {
            errors.push({ field: 'lookupFormatting', message: 'must not be empty' });
        }
    }
    if ('accountNameFormat' in _partial && _partial.accountNameFormat !== undefined) {
        const __val = _partial.accountNameFormat as string;
        if (__val.length === 0) {
            errors.push({ field: 'accountNameFormat', message: 'must not be empty' });
        }
    }
    if ('dateDisplayStyle' in _partial && _partial.dateDisplayStyle !== undefined) {
        const __val = _partial.dateDisplayStyle as string;
        if (__val.length === 0) {
            errors.push({ field: 'dateDisplayStyle', message: 'must not be empty' });
        }
    }
    return errors;
}
export function companyHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'id' in o &&
        'legalName' in o &&
        'headquarters' in o &&
        'phones' in o &&
        'fax' in o &&
        'email' in o &&
        'website' in o &&
        'taxId' in o &&
        'referenceNumber' in o &&
        'postalCodeLookup' in o &&
        'timeZone' in o &&
        'defaultTax' in o &&
        'defaultTaxLocation' in o &&
        'defaultAreaCode' in o &&
        'defaultAccountType' in o &&
        'lookupFormatting' in o &&
        'accountNameFormat' in o &&
        'merchantServiceProvider' in o &&
        'dateDisplayStyle' in o &&
        'hasAutoCommission' in o &&
        'hasAutoDaylightSavings' in o &&
        'hasAutoFmsTracking' in o &&
        'hasNotifications' in o &&
        'hasRequiredLeadSource' in o &&
        'hasRequiredEmail' in o &&
        'hasSortServiceItemsAlphabetically' in o &&
        'hasAttachOrderToAppointmentEmails' in o &&
        'scheduleInterval' in o &&
        'colorsConfig' in o
    );
}
export function companyIs(obj: unknown): obj is Company {
    if (!companyHasShape(obj)) {
        return false;
    }
    const result = companyDeserialize(obj);
    return result.success;
}

/** Nested error structure matching the data shape */ export type CompanyErrors = {
    _errors: __gf_Option<Array<string>>;
    id: __gf_Option<Array<string>>;
    legalName: __gf_Option<Array<string>>;
    headquarters: __gf_Option<Array<string>>;
    phones: __gf_Option<Array<string>>;
    fax: __gf_Option<Array<string>>;
    email: __gf_Option<Array<string>>;
    website: __gf_Option<Array<string>>;
    taxId: __gf_Option<Array<string>>;
    referenceNumber: __gf_Option<Array<string>>;
    postalCodeLookup: __gf_Option<Array<string>>;
    timeZone: __gf_Option<Array<string>>;
    defaultTax: __gf_Option<Array<string>>;
    defaultTaxLocation: __gf_Option<Array<string>>;
    defaultAreaCode: __gf_Option<Array<string>>;
    defaultAccountType: __gf_Option<Array<string>>;
    lookupFormatting: __gf_Option<Array<string>>;
    accountNameFormat: __gf_Option<Array<string>>;
    merchantServiceProvider: __gf_Option<Array<string>>;
    dateDisplayStyle: __gf_Option<Array<string>>;
    hasAutoCommission: __gf_Option<Array<string>>;
    hasAutoDaylightSavings: __gf_Option<Array<string>>;
    hasAutoFmsTracking: __gf_Option<Array<string>>;
    hasNotifications: __gf_Option<Array<string>>;
    hasRequiredLeadSource: __gf_Option<Array<string>>;
    hasRequiredEmail: __gf_Option<Array<string>>;
    hasSortServiceItemsAlphabetically: __gf_Option<Array<string>>;
    hasAttachOrderToAppointmentEmails: __gf_Option<Array<string>>;
    scheduleInterval: __gf_Option<Array<string>>;
    colorsConfig: __gf_Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type CompanyTainted = {
    id: __gf_Option<boolean>;
    legalName: __gf_Option<boolean>;
    headquarters: __gf_Option<boolean>;
    phones: __gf_Option<boolean>;
    fax: __gf_Option<boolean>;
    email: __gf_Option<boolean>;
    website: __gf_Option<boolean>;
    taxId: __gf_Option<boolean>;
    referenceNumber: __gf_Option<boolean>;
    postalCodeLookup: __gf_Option<boolean>;
    timeZone: __gf_Option<boolean>;
    defaultTax: __gf_Option<boolean>;
    defaultTaxLocation: __gf_Option<boolean>;
    defaultAreaCode: __gf_Option<boolean>;
    defaultAccountType: __gf_Option<boolean>;
    lookupFormatting: __gf_Option<boolean>;
    accountNameFormat: __gf_Option<boolean>;
    merchantServiceProvider: __gf_Option<boolean>;
    dateDisplayStyle: __gf_Option<boolean>;
    hasAutoCommission: __gf_Option<boolean>;
    hasAutoDaylightSavings: __gf_Option<boolean>;
    hasAutoFmsTracking: __gf_Option<boolean>;
    hasNotifications: __gf_Option<boolean>;
    hasRequiredLeadSource: __gf_Option<boolean>;
    hasRequiredEmail: __gf_Option<boolean>;
    hasSortServiceItemsAlphabetically: __gf_Option<boolean>;
    hasAttachOrderToAppointmentEmails: __gf_Option<boolean>;
    scheduleInterval: __gf_Option<boolean>;
    colorsConfig: __gf_Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface CompanyFieldControllers {
    readonly id: FieldController<string>;
    readonly legalName: FieldController<string>;
    readonly headquarters: FieldController<string | Site>;
    readonly phones: ArrayFieldController<PhoneNumber>;
    readonly fax: FieldController<string>;
    readonly email: FieldController<string>;
    readonly website: FieldController<string>;
    readonly taxId: FieldController<string>;
    readonly referenceNumber: FieldController<number>;
    readonly postalCodeLookup: FieldController<string>;
    readonly timeZone: FieldController<string>;
    readonly defaultTax: FieldController<string | TaxRate>;
    readonly defaultTaxLocation: FieldController<string>;
    readonly defaultAreaCode: FieldController<number>;
    readonly defaultAccountType: FieldController<string>;
    readonly lookupFormatting: FieldController<string>;
    readonly accountNameFormat: FieldController<string>;
    readonly merchantServiceProvider: FieldController<string | null>;
    readonly dateDisplayStyle: FieldController<string>;
    readonly hasAutoCommission: FieldController<boolean>;
    readonly hasAutoDaylightSavings: FieldController<boolean>;
    readonly hasAutoFmsTracking: FieldController<boolean>;
    readonly hasNotifications: FieldController<boolean>;
    readonly hasRequiredLeadSource: FieldController<boolean>;
    readonly hasRequiredEmail: FieldController<boolean>;
    readonly hasSortServiceItemsAlphabetically: FieldController<boolean>;
    readonly hasAttachOrderToAppointmentEmails: FieldController<boolean>;
    readonly scheduleInterval: FieldController<number>;
    readonly colorsConfig: FieldController<ColorsConfig>;
} /** Gigaform instance containing reactive state and field controllers */
export interface CompanyGigaform {
    readonly data: Company;
    readonly errors: CompanyErrors;
    readonly tainted: CompanyTainted;
    readonly fields: CompanyFieldControllers;
    validate(): Exit<Company, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Company>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function companyCreateForm(overrides?: Partial<Company>): CompanyGigaform {
    let data = $state({ ...companyDefaultValue(), ...overrides });
    let errors = $state<CompanyErrors>({
        _errors: optionNone(),
        id: optionNone(),
        legalName: optionNone(),
        headquarters: optionNone(),
        phones: optionNone(),
        fax: optionNone(),
        email: optionNone(),
        website: optionNone(),
        taxId: optionNone(),
        referenceNumber: optionNone(),
        postalCodeLookup: optionNone(),
        timeZone: optionNone(),
        defaultTax: optionNone(),
        defaultTaxLocation: optionNone(),
        defaultAreaCode: optionNone(),
        defaultAccountType: optionNone(),
        lookupFormatting: optionNone(),
        accountNameFormat: optionNone(),
        merchantServiceProvider: optionNone(),
        dateDisplayStyle: optionNone(),
        hasAutoCommission: optionNone(),
        hasAutoDaylightSavings: optionNone(),
        hasAutoFmsTracking: optionNone(),
        hasNotifications: optionNone(),
        hasRequiredLeadSource: optionNone(),
        hasRequiredEmail: optionNone(),
        hasSortServiceItemsAlphabetically: optionNone(),
        hasAttachOrderToAppointmentEmails: optionNone(),
        scheduleInterval: optionNone(),
        colorsConfig: optionNone()
    });
    let tainted = $state<CompanyTainted>({
        id: optionNone(),
        legalName: optionNone(),
        headquarters: optionNone(),
        phones: optionNone(),
        fax: optionNone(),
        email: optionNone(),
        website: optionNone(),
        taxId: optionNone(),
        referenceNumber: optionNone(),
        postalCodeLookup: optionNone(),
        timeZone: optionNone(),
        defaultTax: optionNone(),
        defaultTaxLocation: optionNone(),
        defaultAreaCode: optionNone(),
        defaultAccountType: optionNone(),
        lookupFormatting: optionNone(),
        accountNameFormat: optionNone(),
        merchantServiceProvider: optionNone(),
        dateDisplayStyle: optionNone(),
        hasAutoCommission: optionNone(),
        hasAutoDaylightSavings: optionNone(),
        hasAutoFmsTracking: optionNone(),
        hasNotifications: optionNone(),
        hasRequiredLeadSource: optionNone(),
        hasRequiredEmail: optionNone(),
        hasSortServiceItemsAlphabetically: optionNone(),
        hasAttachOrderToAppointmentEmails: optionNone(),
        scheduleInterval: optionNone(),
        colorsConfig: optionNone()
    });
    const fields: CompanyFieldControllers = {
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
            setError: (value: __gf_Option<Array<string>>) => {
                errors.id = value;
            },
            getTainted: () => tainted.id,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.id = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        legalName: {
            path: ['legalName'] as const,
            name: 'legalName',
            constraints: { required: true },
            get: () => data.legalName,
            set: (value: string) => {
                data.legalName = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.legalName,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.legalName = value;
            },
            getTainted: () => tainted.legalName,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.legalName = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('legalName', data.legalName);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        headquarters: {
            path: ['headquarters'] as const,
            name: 'headquarters',
            constraints: { required: true },
            get: () => data.headquarters,
            set: (value: string | Site) => {
                data.headquarters = value;
            },
            transform: (value: string | Site): string | Site => value,
            getError: () => errors.headquarters,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.headquarters = value;
            },
            getTainted: () => tainted.headquarters,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.headquarters = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('headquarters', data.headquarters);
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
            setError: (value: __gf_Option<Array<string>>) => {
                errors.phones = value;
            },
            getTainted: () => tainted.phones,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.phones = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('phones', data.phones);
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
                setError: (value: __gf_Option<Array<string>>) => {
                    errors.phones = value;
                },
                getTainted: () => tainted.phones,
                setTainted: (value: __gf_Option<boolean>) => {
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
        fax: {
            path: ['fax'] as const,
            name: 'fax',
            constraints: { required: true },
            get: () => data.fax,
            set: (value: string) => {
                data.fax = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.fax,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.fax = value;
            },
            getTainted: () => tainted.fax,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.fax = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('fax', data.fax);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        email: {
            path: ['email'] as const,
            name: 'email',
            constraints: { required: true },
            get: () => data.email,
            set: (value: string) => {
                data.email = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.email,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.email = value;
            },
            getTainted: () => tainted.email,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.email = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('email', data.email);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        website: {
            path: ['website'] as const,
            name: 'website',
            constraints: { required: true },
            get: () => data.website,
            set: (value: string) => {
                data.website = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.website,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.website = value;
            },
            getTainted: () => tainted.website,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.website = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('website', data.website);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        taxId: {
            path: ['taxId'] as const,
            name: 'taxId',
            constraints: { required: true },
            get: () => data.taxId,
            set: (value: string) => {
                data.taxId = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.taxId,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.taxId = value;
            },
            getTainted: () => tainted.taxId,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.taxId = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('taxId', data.taxId);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        referenceNumber: {
            path: ['referenceNumber'] as const,
            name: 'referenceNumber',
            constraints: { required: true },
            get: () => data.referenceNumber,
            set: (value: number) => {
                data.referenceNumber = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.referenceNumber,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.referenceNumber = value;
            },
            getTainted: () => tainted.referenceNumber,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.referenceNumber = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('referenceNumber', data.referenceNumber);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        postalCodeLookup: {
            path: ['postalCodeLookup'] as const,
            name: 'postalCodeLookup',
            constraints: { required: true },
            get: () => data.postalCodeLookup,
            set: (value: string) => {
                data.postalCodeLookup = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.postalCodeLookup,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.postalCodeLookup = value;
            },
            getTainted: () => tainted.postalCodeLookup,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.postalCodeLookup = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('postalCodeLookup', data.postalCodeLookup);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        timeZone: {
            path: ['timeZone'] as const,
            name: 'timeZone',
            constraints: { required: true },
            get: () => data.timeZone,
            set: (value: string) => {
                data.timeZone = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.timeZone,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.timeZone = value;
            },
            getTainted: () => tainted.timeZone,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.timeZone = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('timeZone', data.timeZone);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        defaultTax: {
            path: ['defaultTax'] as const,
            name: 'defaultTax',
            constraints: { required: true },
            get: () => data.defaultTax,
            set: (value: string | TaxRate) => {
                data.defaultTax = value;
            },
            transform: (value: string | TaxRate): string | TaxRate => value,
            getError: () => errors.defaultTax,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.defaultTax = value;
            },
            getTainted: () => tainted.defaultTax,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.defaultTax = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('defaultTax', data.defaultTax);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        defaultTaxLocation: {
            path: ['defaultTaxLocation'] as const,
            name: 'defaultTaxLocation',
            constraints: { required: true },
            get: () => data.defaultTaxLocation,
            set: (value: string) => {
                data.defaultTaxLocation = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.defaultTaxLocation,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.defaultTaxLocation = value;
            },
            getTainted: () => tainted.defaultTaxLocation,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.defaultTaxLocation = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField(
                    'defaultTaxLocation',
                    data.defaultTaxLocation
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        defaultAreaCode: {
            path: ['defaultAreaCode'] as const,
            name: 'defaultAreaCode',
            constraints: { required: true },
            get: () => data.defaultAreaCode,
            set: (value: number) => {
                data.defaultAreaCode = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.defaultAreaCode,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.defaultAreaCode = value;
            },
            getTainted: () => tainted.defaultAreaCode,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.defaultAreaCode = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('defaultAreaCode', data.defaultAreaCode);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        defaultAccountType: {
            path: ['defaultAccountType'] as const,
            name: 'defaultAccountType',
            constraints: { required: true },
            get: () => data.defaultAccountType,
            set: (value: string) => {
                data.defaultAccountType = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.defaultAccountType,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.defaultAccountType = value;
            },
            getTainted: () => tainted.defaultAccountType,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.defaultAccountType = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField(
                    'defaultAccountType',
                    data.defaultAccountType
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        lookupFormatting: {
            path: ['lookupFormatting'] as const,
            name: 'lookupFormatting',
            constraints: { required: true },
            get: () => data.lookupFormatting,
            set: (value: string) => {
                data.lookupFormatting = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.lookupFormatting,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.lookupFormatting = value;
            },
            getTainted: () => tainted.lookupFormatting,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.lookupFormatting = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('lookupFormatting', data.lookupFormatting);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        accountNameFormat: {
            path: ['accountNameFormat'] as const,
            name: 'accountNameFormat',
            constraints: { required: true },
            get: () => data.accountNameFormat,
            set: (value: string) => {
                data.accountNameFormat = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.accountNameFormat,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.accountNameFormat = value;
            },
            getTainted: () => tainted.accountNameFormat,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.accountNameFormat = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField(
                    'accountNameFormat',
                    data.accountNameFormat
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        merchantServiceProvider: {
            path: ['merchantServiceProvider'] as const,
            name: 'merchantServiceProvider',
            constraints: { required: true },
            get: () => data.merchantServiceProvider,
            set: (value: string | null) => {
                data.merchantServiceProvider = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.merchantServiceProvider,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.merchantServiceProvider = value;
            },
            getTainted: () => tainted.merchantServiceProvider,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.merchantServiceProvider = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField(
                    'merchantServiceProvider',
                    data.merchantServiceProvider
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        dateDisplayStyle: {
            path: ['dateDisplayStyle'] as const,
            name: 'dateDisplayStyle',
            constraints: { required: true },
            get: () => data.dateDisplayStyle,
            set: (value: string) => {
                data.dateDisplayStyle = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.dateDisplayStyle,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.dateDisplayStyle = value;
            },
            getTainted: () => tainted.dateDisplayStyle,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.dateDisplayStyle = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('dateDisplayStyle', data.dateDisplayStyle);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        hasAutoCommission: {
            path: ['hasAutoCommission'] as const,
            name: 'hasAutoCommission',
            constraints: { required: true },
            get: () => data.hasAutoCommission,
            set: (value: boolean) => {
                data.hasAutoCommission = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.hasAutoCommission,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.hasAutoCommission = value;
            },
            getTainted: () => tainted.hasAutoCommission,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.hasAutoCommission = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField(
                    'hasAutoCommission',
                    data.hasAutoCommission
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        hasAutoDaylightSavings: {
            path: ['hasAutoDaylightSavings'] as const,
            name: 'hasAutoDaylightSavings',
            constraints: { required: true },
            get: () => data.hasAutoDaylightSavings,
            set: (value: boolean) => {
                data.hasAutoDaylightSavings = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.hasAutoDaylightSavings,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.hasAutoDaylightSavings = value;
            },
            getTainted: () => tainted.hasAutoDaylightSavings,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.hasAutoDaylightSavings = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField(
                    'hasAutoDaylightSavings',
                    data.hasAutoDaylightSavings
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        hasAutoFmsTracking: {
            path: ['hasAutoFmsTracking'] as const,
            name: 'hasAutoFmsTracking',
            constraints: { required: true },
            get: () => data.hasAutoFmsTracking,
            set: (value: boolean) => {
                data.hasAutoFmsTracking = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.hasAutoFmsTracking,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.hasAutoFmsTracking = value;
            },
            getTainted: () => tainted.hasAutoFmsTracking,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.hasAutoFmsTracking = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField(
                    'hasAutoFmsTracking',
                    data.hasAutoFmsTracking
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        hasNotifications: {
            path: ['hasNotifications'] as const,
            name: 'hasNotifications',
            constraints: { required: true },
            get: () => data.hasNotifications,
            set: (value: boolean) => {
                data.hasNotifications = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.hasNotifications,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.hasNotifications = value;
            },
            getTainted: () => tainted.hasNotifications,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.hasNotifications = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('hasNotifications', data.hasNotifications);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        hasRequiredLeadSource: {
            path: ['hasRequiredLeadSource'] as const,
            name: 'hasRequiredLeadSource',
            constraints: { required: true },
            get: () => data.hasRequiredLeadSource,
            set: (value: boolean) => {
                data.hasRequiredLeadSource = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.hasRequiredLeadSource,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.hasRequiredLeadSource = value;
            },
            getTainted: () => tainted.hasRequiredLeadSource,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.hasRequiredLeadSource = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField(
                    'hasRequiredLeadSource',
                    data.hasRequiredLeadSource
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        hasRequiredEmail: {
            path: ['hasRequiredEmail'] as const,
            name: 'hasRequiredEmail',
            constraints: { required: true },
            get: () => data.hasRequiredEmail,
            set: (value: boolean) => {
                data.hasRequiredEmail = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.hasRequiredEmail,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.hasRequiredEmail = value;
            },
            getTainted: () => tainted.hasRequiredEmail,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.hasRequiredEmail = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('hasRequiredEmail', data.hasRequiredEmail);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        hasSortServiceItemsAlphabetically: {
            path: ['hasSortServiceItemsAlphabetically'] as const,
            name: 'hasSortServiceItemsAlphabetically',
            constraints: { required: true },
            get: () => data.hasSortServiceItemsAlphabetically,
            set: (value: boolean) => {
                data.hasSortServiceItemsAlphabetically = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.hasSortServiceItemsAlphabetically,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.hasSortServiceItemsAlphabetically = value;
            },
            getTainted: () => tainted.hasSortServiceItemsAlphabetically,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.hasSortServiceItemsAlphabetically = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField(
                    'hasSortServiceItemsAlphabetically',
                    data.hasSortServiceItemsAlphabetically
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        hasAttachOrderToAppointmentEmails: {
            path: ['hasAttachOrderToAppointmentEmails'] as const,
            name: 'hasAttachOrderToAppointmentEmails',
            constraints: { required: true },
            get: () => data.hasAttachOrderToAppointmentEmails,
            set: (value: boolean) => {
                data.hasAttachOrderToAppointmentEmails = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.hasAttachOrderToAppointmentEmails,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.hasAttachOrderToAppointmentEmails = value;
            },
            getTainted: () => tainted.hasAttachOrderToAppointmentEmails,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.hasAttachOrderToAppointmentEmails = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField(
                    'hasAttachOrderToAppointmentEmails',
                    data.hasAttachOrderToAppointmentEmails
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        scheduleInterval: {
            path: ['scheduleInterval'] as const,
            name: 'scheduleInterval',
            constraints: { required: true },
            get: () => data.scheduleInterval,
            set: (value: number) => {
                data.scheduleInterval = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.scheduleInterval,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.scheduleInterval = value;
            },
            getTainted: () => tainted.scheduleInterval,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.scheduleInterval = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('scheduleInterval', data.scheduleInterval);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        colorsConfig: {
            path: ['colorsConfig'] as const,
            name: 'colorsConfig',
            constraints: { required: true },
            get: () => data.colorsConfig,
            set: (value: ColorsConfig) => {
                data.colorsConfig = value;
            },
            transform: (value: ColorsConfig): ColorsConfig => value,
            getError: () => errors.colorsConfig,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.colorsConfig = value;
            },
            getTainted: () => tainted.colorsConfig,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.colorsConfig = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyValidateField('colorsConfig', data.colorsConfig);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Company, Array<{ field: string; message: string }>> {
        return toExit(companyDeserialize(data));
    }
    function reset(newOverrides?: Partial<Company>): void {
        data = { ...companyDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            id: optionNone(),
            legalName: optionNone(),
            headquarters: optionNone(),
            phones: optionNone(),
            fax: optionNone(),
            email: optionNone(),
            website: optionNone(),
            taxId: optionNone(),
            referenceNumber: optionNone(),
            postalCodeLookup: optionNone(),
            timeZone: optionNone(),
            defaultTax: optionNone(),
            defaultTaxLocation: optionNone(),
            defaultAreaCode: optionNone(),
            defaultAccountType: optionNone(),
            lookupFormatting: optionNone(),
            accountNameFormat: optionNone(),
            merchantServiceProvider: optionNone(),
            dateDisplayStyle: optionNone(),
            hasAutoCommission: optionNone(),
            hasAutoDaylightSavings: optionNone(),
            hasAutoFmsTracking: optionNone(),
            hasNotifications: optionNone(),
            hasRequiredLeadSource: optionNone(),
            hasRequiredEmail: optionNone(),
            hasSortServiceItemsAlphabetically: optionNone(),
            hasAttachOrderToAppointmentEmails: optionNone(),
            scheduleInterval: optionNone(),
            colorsConfig: optionNone()
        };
        tainted = {
            id: optionNone(),
            legalName: optionNone(),
            headquarters: optionNone(),
            phones: optionNone(),
            fax: optionNone(),
            email: optionNone(),
            website: optionNone(),
            taxId: optionNone(),
            referenceNumber: optionNone(),
            postalCodeLookup: optionNone(),
            timeZone: optionNone(),
            defaultTax: optionNone(),
            defaultTaxLocation: optionNone(),
            defaultAreaCode: optionNone(),
            defaultAccountType: optionNone(),
            lookupFormatting: optionNone(),
            accountNameFormat: optionNone(),
            merchantServiceProvider: optionNone(),
            dateDisplayStyle: optionNone(),
            hasAutoCommission: optionNone(),
            hasAutoDaylightSavings: optionNone(),
            hasAutoFmsTracking: optionNone(),
            hasNotifications: optionNone(),
            hasRequiredLeadSource: optionNone(),
            hasRequiredEmail: optionNone(),
            hasSortServiceItemsAlphabetically: optionNone(),
            hasAttachOrderToAppointmentEmails: optionNone(),
            scheduleInterval: optionNone(),
            colorsConfig: optionNone()
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
export function companyFromFormData(
    formData: FormData
): Exit<Company, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.legalName = formData.get('legalName') ?? '';
    obj.headquarters = formData.get('headquarters') ?? '';
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
    obj.fax = formData.get('fax') ?? '';
    obj.email = formData.get('email') ?? '';
    obj.website = formData.get('website') ?? '';
    obj.taxId = formData.get('taxId') ?? '';
    {
        const referenceNumberStr = formData.get('referenceNumber');
        obj.referenceNumber = referenceNumberStr ? parseFloat(referenceNumberStr as string) : 0;
        if (obj.referenceNumber !== undefined && isNaN(obj.referenceNumber as number))
            obj.referenceNumber = 0;
    }
    obj.postalCodeLookup = formData.get('postalCodeLookup') ?? '';
    obj.timeZone = formData.get('timeZone') ?? '';
    obj.defaultTax = formData.get('defaultTax') ?? '';
    obj.defaultTaxLocation = formData.get('defaultTaxLocation') ?? '';
    {
        const defaultAreaCodeStr = formData.get('defaultAreaCode');
        obj.defaultAreaCode = defaultAreaCodeStr ? parseFloat(defaultAreaCodeStr as string) : 0;
        if (obj.defaultAreaCode !== undefined && isNaN(obj.defaultAreaCode as number))
            obj.defaultAreaCode = 0;
    }
    obj.defaultAccountType = formData.get('defaultAccountType') ?? '';
    obj.lookupFormatting = formData.get('lookupFormatting') ?? '';
    obj.accountNameFormat = formData.get('accountNameFormat') ?? '';
    obj.merchantServiceProvider = formData.get('merchantServiceProvider') ?? '';
    obj.dateDisplayStyle = formData.get('dateDisplayStyle') ?? '';
    {
        const hasAutoCommissionVal = formData.get('hasAutoCommission');
        obj.hasAutoCommission =
            hasAutoCommissionVal === 'true' ||
            hasAutoCommissionVal === 'on' ||
            hasAutoCommissionVal === '1';
    }
    {
        const hasAutoDaylightSavingsVal = formData.get('hasAutoDaylightSavings');
        obj.hasAutoDaylightSavings =
            hasAutoDaylightSavingsVal === 'true' ||
            hasAutoDaylightSavingsVal === 'on' ||
            hasAutoDaylightSavingsVal === '1';
    }
    {
        const hasAutoFmsTrackingVal = formData.get('hasAutoFmsTracking');
        obj.hasAutoFmsTracking =
            hasAutoFmsTrackingVal === 'true' ||
            hasAutoFmsTrackingVal === 'on' ||
            hasAutoFmsTrackingVal === '1';
    }
    {
        const hasNotificationsVal = formData.get('hasNotifications');
        obj.hasNotifications =
            hasNotificationsVal === 'true' ||
            hasNotificationsVal === 'on' ||
            hasNotificationsVal === '1';
    }
    {
        const hasRequiredLeadSourceVal = formData.get('hasRequiredLeadSource');
        obj.hasRequiredLeadSource =
            hasRequiredLeadSourceVal === 'true' ||
            hasRequiredLeadSourceVal === 'on' ||
            hasRequiredLeadSourceVal === '1';
    }
    {
        const hasRequiredEmailVal = formData.get('hasRequiredEmail');
        obj.hasRequiredEmail =
            hasRequiredEmailVal === 'true' ||
            hasRequiredEmailVal === 'on' ||
            hasRequiredEmailVal === '1';
    }
    {
        const hasSortServiceItemsAlphabeticallyVal = formData.get(
            'hasSortServiceItemsAlphabetically'
        );
        obj.hasSortServiceItemsAlphabetically =
            hasSortServiceItemsAlphabeticallyVal === 'true' ||
            hasSortServiceItemsAlphabeticallyVal === 'on' ||
            hasSortServiceItemsAlphabeticallyVal === '1';
    }
    {
        const hasAttachOrderToAppointmentEmailsVal = formData.get(
            'hasAttachOrderToAppointmentEmails'
        );
        obj.hasAttachOrderToAppointmentEmails =
            hasAttachOrderToAppointmentEmailsVal === 'true' ||
            hasAttachOrderToAppointmentEmailsVal === 'on' ||
            hasAttachOrderToAppointmentEmailsVal === '1';
    }
    {
        const scheduleIntervalStr = formData.get('scheduleInterval');
        obj.scheduleInterval = scheduleIntervalStr ? parseFloat(scheduleIntervalStr as string) : 0;
        if (obj.scheduleInterval !== undefined && isNaN(obj.scheduleInterval as number))
            obj.scheduleInterval = 0;
    }
    {
        const colorsConfigObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('colorsConfig.')) {
                const fieldName = key.slice('colorsConfig.'.length);
                const parts = fieldName.split('.');
                let current = colorsConfigObj;
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
        obj.colorsConfig = colorsConfigObj;
    }
    return toExit(companyDeserialize(obj));
}

export const Company = {
    defaultValue: companyDefaultValue,
    serialize: companySerialize,
    serializeWithContext: companySerializeWithContext,
    deserialize: companyDeserialize,
    deserializeWithContext: companyDeserializeWithContext,
    validateFields: companyValidateFields,
    hasShape: companyHasShape,
    is: companyIs,
    createForm: companyCreateForm,
    fromFormData: companyFromFormData
} as const;
