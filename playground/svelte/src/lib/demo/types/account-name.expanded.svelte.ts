import { companyNameDefaultValue } from './company-name.svelte';
import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { companyNameDeserializeWithContext } from './company-name.svelte';
import { personNameDeserializeWithContext } from './person-name.svelte';
import { Result } from 'macroforge/reexports';
import { Option } from 'macroforge/reexports';
import type { FieldController } from '@playground/macro/gigaform';
import { personNameDefaultValue } from './person-name.svelte';
/** import macro {Gigaform} from "@playground/macro"; */

import type { PersonName } from './person-name.svelte';
import type { CompanyName } from './company-name.svelte';
import type { Company } from './company.svelte';

export type AccountName = /** @default */ CompanyName | PersonName;

export function accountNameDefaultValue(): AccountName {
    return companyNameDefaultValue();
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function accountNameSerialize(
    value: AccountName
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(accountNameSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function accountNameSerializeWithContext(
    value: AccountName,
    ctx: __mf_SerializeContext
): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function accountNameDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, AccountName> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = accountNameDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'AccountName.deserialize: root cannot be a forward reference'
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
export function accountNameDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): AccountName | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as AccountName | __mf_PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'AccountName.deserializeWithContext: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message:
                    'AccountName.deserializeWithContext: missing __type field for union dispatch'
            }
        ]);
    }
    if (__typeName === 'CompanyName') {
        return companyNameDeserializeWithContext(value, ctx) as AccountName;
    }
    if (__typeName === 'PersonName') {
        return personNameDeserializeWithContext(value, ctx) as AccountName;
    }
    throw new __mf_DeserializeError([
        {
            field: '_root',
            message:
                'AccountName.deserializeWithContext: unknown type "' +
                __typeName +
                '". Expected one of: CompanyName, PersonName'
        }
    ]);
}
export function accountNameIs(value: unknown): value is AccountName {
    if (typeof value !== 'object' || value === null) {
        return false;
    }
    const __typeName = (value as any).__type;
    return __typeName === 'CompanyName' || __typeName === 'PersonName';
}

/** Per-variant error types */ export type AccountNameCompanyNameErrors = {
    _errors: Option<Array<string>>;
};
export type AccountNamePersonNameErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type AccountNameCompanyNameTainted = {};
export type AccountNamePersonNameTainted = {}; /** Union error type */
export type AccountNameErrors =
    | ({ _type: 'CompanyName' } & AccountNameCompanyNameErrors)
    | ({ _type: 'PersonName' } & AccountNamePersonNameErrors); /** Union tainted type */
export type AccountNameTainted =
    | ({ _type: 'CompanyName' } & AccountNameCompanyNameTainted)
    | ({
          _type: 'PersonName';
      } & AccountNamePersonNameTainted); /** Per-variant field controller types */
export interface AccountNameCompanyNameFieldControllers {}
export interface AccountNamePersonNameFieldControllers {} /** Union Gigaform interface with variant switching */
export interface AccountNameGigaform {
    readonly currentVariant: 'CompanyName' | 'PersonName';
    readonly data: AccountName;
    readonly errors: AccountNameErrors;
    readonly tainted: AccountNameTainted;
    readonly variants: AccountNameVariantFields;
    switchVariant(variant: 'CompanyName' | 'PersonName'): void;
    validate(): Result<AccountName, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<AccountName>): void;
} /** Variant fields container */
export interface AccountNameVariantFields {
    readonly CompanyName: { readonly fields: AccountNameCompanyNameFieldControllers };
    readonly PersonName: { readonly fields: AccountNamePersonNameFieldControllers };
} /** Gets default value for a specific variant */
function accountNameGetDefaultForVariant(variant: string): AccountName {
    switch (variant) {
        case 'CompanyName':
            return companyNameDefaultValue() as AccountName;
        case 'PersonName':
            return personNameDefaultValue() as AccountName;
        default:
            return companyNameDefaultValue() as AccountName;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function accountNameCreateForm(initial?: AccountName): AccountNameGigaform {
    const initialVariant: 'CompanyName' | 'PersonName' = 'CompanyName';
    let currentVariant = $state<'CompanyName' | 'PersonName'>(initialVariant);
    let data = $state<AccountName>(initial ?? accountNameGetDefaultForVariant(initialVariant));
    let errors = $state<AccountNameErrors>({} as AccountNameErrors);
    let tainted = $state<AccountNameTainted>({} as AccountNameTainted);
    const variants: AccountNameVariantFields = {
        CompanyName: { fields: {} as AccountNameCompanyNameFieldControllers },
        PersonName: { fields: {} as AccountNamePersonNameFieldControllers }
    };
    function switchVariant(variant: 'CompanyName' | 'PersonName'): void {
        currentVariant = variant;
        data = accountNameGetDefaultForVariant(variant);
        errors = {} as AccountNameErrors;
        tainted = {} as AccountNameTainted;
    }
    function validate(): Result<AccountName, Array<{ field: string; message: string }>> {
        return accountNameDeserialize(data);
    }
    function reset(overrides?: Partial<AccountName>): void {
        data = overrides
            ? (overrides as typeof data)
            : accountNameGetDefaultForVariant(currentVariant);
        errors = {} as AccountNameErrors;
        tainted = {} as AccountNameTainted;
    }
    return {
        get currentVariant() {
            return currentVariant;
        },
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
        variants,
        switchVariant,
        validate,
        reset
    };
} /** Parses FormData for union type, determining variant from discriminant field */
export function accountNameFromFormData(
    formData: FormData
): Result<AccountName, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_type') as 'CompanyName' | 'PersonName' | null;
    if (!discriminant) {
        return Result.err([{ field: '_type', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._type = discriminant;
    if (discriminant === 'CompanyName') {
    } else if (discriminant === 'PersonName') {
    }
    return accountNameDeserialize(obj);
}

export const AccountName = {
    defaultValue: accountNameDefaultValue,
    serialize: accountNameSerialize,
    serializeWithContext: accountNameSerializeWithContext,
    deserialize: accountNameDeserialize,
    deserializeWithContext: accountNameDeserializeWithContext,
    is: accountNameIs,
    createForm: accountNameCreateForm,
    fromFormData: accountNameFromFormData
} as const;
