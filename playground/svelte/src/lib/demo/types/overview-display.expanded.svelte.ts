import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { Table } from './table.svelte';

export type OverviewDisplay = /** @default */ 'Card' | 'Table';

export function overviewDisplayDefaultValue(): OverviewDisplay {
    return 'Card';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function overviewDisplaySerialize(
    value: OverviewDisplay
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(overviewDisplaySerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function overviewDisplaySerializeWithContext(
    value: OverviewDisplay,
    ctx: SerializeContext
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
@returns Result containing the deserialized value or validation errors */ export function overviewDisplayDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Result<OverviewDisplay, Array<{ field: string; message: string }>> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = overviewDisplayDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'OverviewDisplay.deserialize: root cannot be a forward reference'
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
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function overviewDisplayDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): OverviewDisplay | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as OverviewDisplay | PendingRef;
    }
    const allowedValues = ['Card', 'Table'] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for OverviewDisplay: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as OverviewDisplay;
}
export function overviewDisplayIs(value: unknown): value is OverviewDisplay {
    const allowedValues = ['Card', 'Table'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type OverviewDisplayCardErrors = {
    _errors: Option<Array<string>>;
};
export type OverviewDisplayTableErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type OverviewDisplayCardTainted = {};
export type OverviewDisplayTableTainted = {}; /** Union error type */
export type OverviewDisplayErrors =
    | ({ _value: 'Card' } & OverviewDisplayCardErrors)
    | ({ _value: 'Table' } & OverviewDisplayTableErrors); /** Union tainted type */
export type OverviewDisplayTainted =
    | ({ _value: 'Card' } & OverviewDisplayCardTainted)
    | ({ _value: 'Table' } & OverviewDisplayTableTainted); /** Per-variant field controller types */
export interface OverviewDisplayCardFieldControllers {}
export interface OverviewDisplayTableFieldControllers {} /** Union Gigaform interface with variant switching */
export interface OverviewDisplayGigaform {
    readonly currentVariant: 'Card' | 'Table';
    readonly data: OverviewDisplay;
    readonly errors: OverviewDisplayErrors;
    readonly tainted: OverviewDisplayTainted;
    readonly variants: OverviewDisplayVariantFields;
    switchVariant(variant: 'Card' | 'Table'): void;
    validate(): Result<OverviewDisplay, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<OverviewDisplay>): void;
} /** Variant fields container */
export interface OverviewDisplayVariantFields {
    readonly Card: { readonly fields: OverviewDisplayCardFieldControllers };
    readonly Table: { readonly fields: OverviewDisplayTableFieldControllers };
} /** Gets default value for a specific variant */
function overviewDisplayGetDefaultForVariant(variant: string): OverviewDisplay {
    switch (variant) {
        case 'Card':
            return 'Card' as OverviewDisplay;
        case 'Table':
            return 'Table' as OverviewDisplay;
        default:
            return 'Card' as OverviewDisplay;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function overviewDisplayCreateForm(initial?: OverviewDisplay): OverviewDisplayGigaform {
    const initialVariant: 'Card' | 'Table' = (initial as 'Card' | 'Table') ?? 'Card';
    let currentVariant = $state<'Card' | 'Table'>(initialVariant);
    let data = $state<OverviewDisplay>(
        initial ?? overviewDisplayGetDefaultForVariant(initialVariant)
    );
    let errors = $state<OverviewDisplayErrors>({} as OverviewDisplayErrors);
    let tainted = $state<OverviewDisplayTainted>({} as OverviewDisplayTainted);
    const variants: OverviewDisplayVariantFields = {
        Card: {
            fields: {} as OverviewDisplayCardFieldControllers
        },
        Table: {
            fields: {} as OverviewDisplayTableFieldControllers
        }
    };
    function switchVariant(variant: 'Card' | 'Table'): void {
        currentVariant = variant;
        data = overviewDisplayGetDefaultForVariant(variant);
        errors = {} as OverviewDisplayErrors;
        tainted = {} as OverviewDisplayTainted;
    }
    function validate(): Result<OverviewDisplay, Array<{ field: string; message: string }>> {
        return overviewDisplayFromObject(data);
    }
    function reset(overrides?: Partial<OverviewDisplay>): void {
        data = overrides
            ? (overrides as typeof data)
            : overviewDisplayGetDefaultForVariant(currentVariant);
        errors = {} as OverviewDisplayErrors;
        tainted = {} as OverviewDisplayTainted;
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
export function overviewDisplayFromFormData(
    formData: FormData
): Result<OverviewDisplay, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as 'Card' | 'Table' | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Card') {
    } else if (discriminant === 'Table') {
    }
    return overviewDisplayFromStringifiedJSON(JSON.stringify(obj));
}

export const OverviewDisplay = {
    defaultValue: overviewDisplayDefaultValue,
    serialize: overviewDisplaySerialize,
    serializeWithContext: overviewDisplaySerializeWithContext,
    deserialize: overviewDisplayDeserialize,
    deserializeWithContext: overviewDisplayDeserializeWithContext,
    is: overviewDisplayIs,
    createForm: overviewDisplayCreateForm,
    fromFormData: overviewDisplayFromFormData
} as const;
