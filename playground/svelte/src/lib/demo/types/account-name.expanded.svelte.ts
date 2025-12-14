import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import { PersonName } from './person-name.svelte';
import { CompanyName } from './company-name.svelte';
import { Company } from './company.svelte';

export type AccountName = /** @default */ CompanyName | PersonName;

export function defaultValueAccountName(): AccountName {
    return CompanyName.defaultValue();
}

export function toStringifiedJSONAccountName(value: AccountName): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeAccountName(value, ctx));
}
export function toObjectAccountName(value: AccountName): unknown {
    const ctx = SerializeContext.create();
    return __serializeAccountName(value, ctx);
}
export function __serializeAccountName(value: AccountName, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONAccountName(
    json: string,
    opts?: DeserializeOptions
): Result<AccountName, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectAccountName(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectAccountName(
    obj: unknown,
    opts?: DeserializeOptions
): Result<AccountName, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeAccountName(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'AccountName.fromObject: root cannot be a forward reference'
                }
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
export function __deserializeAccountName(
    value: any,
    ctx: DeserializeContext
): AccountName | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as AccountName | PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new DeserializeError([
            { field: '_root', message: 'AccountName.__deserialize: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new DeserializeError([
            {
                field: '_root',
                message: 'AccountName.__deserialize: missing __type field for union dispatch'
            }
        ]);
    }
    if (__typeName === 'CompanyName') {
        if (typeof (CompanyName as any)?.__deserialize === 'function') {
            return (CompanyName as any).__deserialize(value, ctx) as AccountName;
        }
        return value as AccountName;
    }
    if (__typeName === 'PersonName') {
        if (typeof (PersonName as any)?.__deserialize === 'function') {
            return (PersonName as any).__deserialize(value, ctx) as AccountName;
        }
        return value as AccountName;
    }
    throw new DeserializeError([
        {
            field: '_root',
            message:
                'AccountName.__deserialize: unknown type "' +
                __typeName +
                '". Expected one of: CompanyName, PersonName'
        }
    ]);
}
export function isAccountName(value: unknown): value is AccountName {
    if (typeof value !== 'object' || value === null) {
        return false;
    }
    const __typeName = (value as any).__type;
    return __typeName === 'CompanyName' || __typeName === 'PersonName';
}

/** Per-variant error types */ export type CompanyNameErrorsAccountName = {
    _errors: Option<Array<string>>;
};
export type PersonNameErrorsAccountName = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type CompanyNameTaintedAccountName = {};
export type PersonNameTaintedAccountName = {}; /** Union error type */
export type ErrorsAccountName =
    | ({ _type: 'CompanyName' } & CompanyNameErrorsAccountName)
    | ({ _type: 'PersonName' } & PersonNameErrorsAccountName); /** Union tainted type */
export type TaintedAccountName =
    | ({ _type: 'CompanyName' } & CompanyNameTaintedAccountName)
    | ({
          _type: 'PersonName';
      } & PersonNameTaintedAccountName); /** Per-variant field controller types */
export interface CompanyNameFieldControllersAccountName {}
export interface PersonNameFieldControllersAccountName {} /** Union Gigaform interface with variant switching */
export interface GigaformAccountName {
    readonly currentVariant: 'CompanyName' | 'PersonName';
    readonly data: AccountName;
    readonly errors: ErrorsAccountName;
    readonly tainted: TaintedAccountName;
    readonly variants: VariantFieldsAccountName;
    switchVariant(variant: 'CompanyName' | 'PersonName'): void;
    validate(): Result<AccountName, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<AccountName>): void;
} /** Variant fields container */
export interface VariantFieldsAccountName {
    readonly CompanyName: { readonly fields: CompanyNameFieldControllersAccountName };
    readonly PersonName: { readonly fields: PersonNameFieldControllersAccountName };
} /** Gets default value for a specific variant */
function getDefaultForVariantAccountName(variant: string): AccountName {
    switch (variant) {
        case 'CompanyName':
            return CompanyName.defaultValue() as AccountName;
        case 'PersonName':
            return PersonName.defaultValue() as AccountName;
        default:
            return CompanyName.defaultValue() as AccountName;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormAccountName(initial?: AccountName): GigaformAccountName {
    const initialVariant: 'CompanyName' | 'PersonName' = 'CompanyName';
    let currentVariant = $state<'CompanyName' | 'PersonName'>(initialVariant);
    let data = $state<AccountName>(initial ?? getDefaultForVariantAccountName(initialVariant));
    let errors = $state<ErrorsAccountName>({} as ErrorsAccountName);
    let tainted = $state<TaintedAccountName>({} as TaintedAccountName);
    const variants: VariantFieldsAccountName = {
        CompanyName: {
            fields: {} as CompanyNameFieldControllersAccountName
        },
        PersonName: {
            fields: {} as PersonNameFieldControllersAccountName
        }
    };
    function switchVariant(variant: 'CompanyName' | 'PersonName'): void {
        currentVariant = variant;
        data = getDefaultForVariantAccountName(variant);
        errors = {} as ErrorsAccountName;
        tainted = {} as TaintedAccountName;
    }
    function validate(): Result<AccountName, Array<{ field: string; message: string }>> {
        return AccountName.fromObject(data);
    }
    function reset(overrides?: Partial<AccountName>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantAccountName(currentVariant);
        errors = {} as ErrorsAccountName;
        tainted = {} as TaintedAccountName;
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
export function fromFormDataAccountName(
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
    return AccountName.fromStringifiedJSON(JSON.stringify(obj));
}
