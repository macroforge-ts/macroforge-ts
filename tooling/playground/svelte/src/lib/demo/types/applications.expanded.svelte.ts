import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option as __gf_Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';

export type Applications =
    | /** @default */ 'Sales'
    | 'Accounting'
    | 'Errand'
    | 'HumanResources'
    | 'Logistics'
    | 'Marketing'
    | 'Website';

export function applicationsDefaultValue(): Applications {
    return 'Sales';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function applicationsSerialize(
    value: Applications
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(applicationsSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function applicationsSerializeWithContext(
    value: Applications,
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
@returns Result containing the deserialized value or validation errors */ export function applicationsDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: Applications }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = applicationsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: '_root',
                        message: 'Applications.deserialize: root cannot be a forward reference'
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
export function applicationsDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Applications | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Applications | __mf_PendingRef;
    }
    const allowedValues = [
        'Sales',
        'Accounting',
        'Errand',
        'HumanResources',
        'Logistics',
        'Marketing',
        'Website'
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for Applications: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as Applications;
}
export function applicationsIs(value: unknown): value is Applications {
    const allowedValues = [
        'Sales',
        'Accounting',
        'Errand',
        'HumanResources',
        'Logistics',
        'Marketing',
        'Website'
    ] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type ApplicationsSalesErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ApplicationsAccountingErrors = { _errors: __gf_Option<Array<string>> };
export type ApplicationsErrandErrors = { _errors: __gf_Option<Array<string>> };
export type ApplicationsHumanResourcesErrors = { _errors: __gf_Option<Array<string>> };
export type ApplicationsLogisticsErrors = { _errors: __gf_Option<Array<string>> };
export type ApplicationsMarketingErrors = { _errors: __gf_Option<Array<string>> };
export type ApplicationsWebsiteErrors = {
    _errors: __gf_Option<Array<string>>;
}; /** Per-variant tainted types */
export type ApplicationsSalesTainted = {};
export type ApplicationsAccountingTainted = {};
export type ApplicationsErrandTainted = {};
export type ApplicationsHumanResourcesTainted = {};
export type ApplicationsLogisticsTainted = {};
export type ApplicationsMarketingTainted = {};
export type ApplicationsWebsiteTainted = {}; /** Union error type */
export type ApplicationsErrors =
    | ({ _value: 'Sales' } & ApplicationsSalesErrors)
    | ({ _value: 'Accounting' } & ApplicationsAccountingErrors)
    | ({ _value: 'Errand' } & ApplicationsErrandErrors)
    | ({ _value: 'HumanResources' } & ApplicationsHumanResourcesErrors)
    | ({ _value: 'Logistics' } & ApplicationsLogisticsErrors)
    | ({ _value: 'Marketing' } & ApplicationsMarketingErrors)
    | ({ _value: 'Website' } & ApplicationsWebsiteErrors); /** Union tainted type */
export type ApplicationsTainted =
    | ({ _value: 'Sales' } & ApplicationsSalesTainted)
    | ({ _value: 'Accounting' } & ApplicationsAccountingTainted)
    | ({ _value: 'Errand' } & ApplicationsErrandTainted)
    | ({ _value: 'HumanResources' } & ApplicationsHumanResourcesTainted)
    | ({ _value: 'Logistics' } & ApplicationsLogisticsTainted)
    | ({ _value: 'Marketing' } & ApplicationsMarketingTainted)
    | ({
          _value: 'Website';
      } & ApplicationsWebsiteTainted); /** Per-variant field controller types */
export interface ApplicationsSalesFieldControllers {}
export interface ApplicationsAccountingFieldControllers {}
export interface ApplicationsErrandFieldControllers {}
export interface ApplicationsHumanResourcesFieldControllers {}
export interface ApplicationsLogisticsFieldControllers {}
export interface ApplicationsMarketingFieldControllers {}
export interface ApplicationsWebsiteFieldControllers {} /** Union Gigaform interface with variant switching */
export interface ApplicationsGigaform {
    readonly currentVariant:
        | 'Sales'
        | 'Accounting'
        | 'Errand'
        | 'HumanResources'
        | 'Logistics'
        | 'Marketing'
        | 'Website';
    readonly data: Applications;
    readonly errors: ApplicationsErrors;
    readonly tainted: ApplicationsTainted;
    readonly variants: ApplicationsVariantFields;
    switchVariant(
        variant:
            | 'Sales'
            | 'Accounting'
            | 'Errand'
            | 'HumanResources'
            | 'Logistics'
            | 'Marketing'
            | 'Website'
    ): void;
    validate(): Exit<Applications, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Applications>): void;
} /** Variant fields container */
export interface ApplicationsVariantFields {
    readonly Sales: { readonly fields: ApplicationsSalesFieldControllers };
    readonly Accounting: { readonly fields: ApplicationsAccountingFieldControllers };
    readonly Errand: { readonly fields: ApplicationsErrandFieldControllers };
    readonly HumanResources: { readonly fields: ApplicationsHumanResourcesFieldControllers };
    readonly Logistics: { readonly fields: ApplicationsLogisticsFieldControllers };
    readonly Marketing: { readonly fields: ApplicationsMarketingFieldControllers };
    readonly Website: { readonly fields: ApplicationsWebsiteFieldControllers };
} /** Gets default value for a specific variant */
function applicationsGetDefaultForVariant(variant: string): Applications {
    switch (variant) {
        case 'Sales':
            return 'Sales' as Applications;
        case 'Accounting':
            return 'Accounting' as Applications;
        case 'Errand':
            return 'Errand' as Applications;
        case 'HumanResources':
            return 'HumanResources' as Applications;
        case 'Logistics':
            return 'Logistics' as Applications;
        case 'Marketing':
            return 'Marketing' as Applications;
        case 'Website':
            return 'Website' as Applications;
        default:
            return 'Sales' as Applications;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function applicationsCreateForm(initial?: Applications): ApplicationsGigaform {
    const initialVariant:
        | 'Sales'
        | 'Accounting'
        | 'Errand'
        | 'HumanResources'
        | 'Logistics'
        | 'Marketing'
        | 'Website' =
        (initial as
            | 'Sales'
            | 'Accounting'
            | 'Errand'
            | 'HumanResources'
            | 'Logistics'
            | 'Marketing'
            | 'Website') ?? 'Sales';
    let currentVariant = $state<
        'Sales' | 'Accounting' | 'Errand' | 'HumanResources' | 'Logistics' | 'Marketing' | 'Website'
    >(initialVariant);
    let data = $state<Applications>(initial ?? applicationsGetDefaultForVariant(initialVariant));
    let errors = $state<ApplicationsErrors>({} as ApplicationsErrors);
    let tainted = $state<ApplicationsTainted>({} as ApplicationsTainted);
    const variants: ApplicationsVariantFields = {
        Sales: { fields: {} as ApplicationsSalesFieldControllers },
        Accounting: { fields: {} as ApplicationsAccountingFieldControllers },
        Errand: { fields: {} as ApplicationsErrandFieldControllers },
        HumanResources: { fields: {} as ApplicationsHumanResourcesFieldControllers },
        Logistics: { fields: {} as ApplicationsLogisticsFieldControllers },
        Marketing: { fields: {} as ApplicationsMarketingFieldControllers },
        Website: { fields: {} as ApplicationsWebsiteFieldControllers }
    };
    function switchVariant(
        variant:
            | 'Sales'
            | 'Accounting'
            | 'Errand'
            | 'HumanResources'
            | 'Logistics'
            | 'Marketing'
            | 'Website'
    ): void {
        currentVariant = variant;
        data = applicationsGetDefaultForVariant(variant);
        errors = {} as ApplicationsErrors;
        tainted = {} as ApplicationsTainted;
    }
    function validate(): Exit<Applications, Array<{ field: string; message: string }>> {
        return toExit(applicationsDeserialize(data));
    }
    function reset(overrides?: Partial<Applications>): void {
        data = overrides
            ? (overrides as typeof data)
            : applicationsGetDefaultForVariant(currentVariant);
        errors = {} as ApplicationsErrors;
        tainted = {} as ApplicationsTainted;
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
export function applicationsFromFormData(
    formData: FormData
): Exit<Applications, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as
        | 'Sales'
        | 'Accounting'
        | 'Errand'
        | 'HumanResources'
        | 'Logistics'
        | 'Marketing'
        | 'Website'
        | null;
    if (!discriminant) {
        return toExit({
            success: false,
            errors: [{ field: '_value', message: 'Missing discriminant field' }]
        });
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Sales') {
    } else if (discriminant === 'Accounting') {
    } else if (discriminant === 'Errand') {
    } else if (discriminant === 'HumanResources') {
    } else if (discriminant === 'Logistics') {
    } else if (discriminant === 'Marketing') {
    } else if (discriminant === 'Website') {
    }
    return toExit(applicationsDeserialize(obj));
}

export const Applications = {
    defaultValue: applicationsDefaultValue,
    serialize: applicationsSerialize,
    serializeWithContext: applicationsSerializeWithContext,
    deserialize: applicationsDeserialize,
    deserializeWithContext: applicationsDeserializeWithContext,
    is: applicationsIs,
    createForm: applicationsCreateForm,
    fromFormData: applicationsFromFormData
} as const;
