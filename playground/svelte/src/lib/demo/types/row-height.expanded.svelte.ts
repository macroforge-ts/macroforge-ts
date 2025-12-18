import { SerializeContext } from 'macroforge/serde';
import { Exit } from 'macroforge/utils/effect';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';

export type RowHeight = 'ExtraSmall' | 'Small' | /** @default */ 'Medium' | 'Large';

export function rowHeightDefaultValue(): RowHeight {
    return 'Medium';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function rowHeightSerialize(
    value: RowHeight
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(rowHeightSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function rowHeightSerializeWithContext(value: RowHeight, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function rowHeightDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Exit.Exit<Array<{ field: string; message: string }>, RowHeight> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = rowHeightDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Exit.fail([
                {
                    field: '_root',
                    message: 'RowHeight.deserialize: root cannot be a forward reference'
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Exit.succeed(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Exit.fail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Exit.fail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function rowHeightDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): RowHeight | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as RowHeight | PendingRef;
    }
    const allowedValues = ['ExtraSmall', 'Small', 'Medium', 'Large'] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for RowHeight: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as RowHeight;
}
export function rowHeightIs(value: unknown): value is RowHeight {
    const allowedValues = ['ExtraSmall', 'Small', 'Medium', 'Large'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type RowHeightExtraSmallErrors = {
    _errors: Option<Array<string>>;
};
export type RowHeightSmallErrors = { _errors: Option<Array<string>> };
export type RowHeightMediumErrors = { _errors: Option<Array<string>> };
export type RowHeightLargeErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type RowHeightExtraSmallTainted = {};
export type RowHeightSmallTainted = {};
export type RowHeightMediumTainted = {};
export type RowHeightLargeTainted = {}; /** Union error type */
export type RowHeightErrors =
    | ({ _value: 'ExtraSmall' } & RowHeightExtraSmallErrors)
    | ({ _value: 'Small' } & RowHeightSmallErrors)
    | ({ _value: 'Medium' } & RowHeightMediumErrors)
    | ({ _value: 'Large' } & RowHeightLargeErrors); /** Union tainted type */
export type RowHeightTainted =
    | ({ _value: 'ExtraSmall' } & RowHeightExtraSmallTainted)
    | ({ _value: 'Small' } & RowHeightSmallTainted)
    | ({ _value: 'Medium' } & RowHeightMediumTainted)
    | ({ _value: 'Large' } & RowHeightLargeTainted); /** Per-variant field controller types */
export interface RowHeightExtraSmallFieldControllers {}
export interface RowHeightSmallFieldControllers {}
export interface RowHeightMediumFieldControllers {}
export interface RowHeightLargeFieldControllers {} /** Union Gigaform interface with variant switching */
export interface RowHeightGigaform {
    readonly currentVariant: 'ExtraSmall' | 'Small' | 'Medium' | 'Large';
    readonly data: RowHeight;
    readonly errors: RowHeightErrors;
    readonly tainted: RowHeightTainted;
    readonly variants: RowHeightVariantFields;
    switchVariant(variant: 'ExtraSmall' | 'Small' | 'Medium' | 'Large'): void;
    validate(): Exit<Array<{ field: string; message: string }>, RowHeight>;
    reset(overrides?: Partial<RowHeight>): void;
} /** Variant fields container */
export interface RowHeightVariantFields {
    readonly ExtraSmall: { readonly fields: RowHeightExtraSmallFieldControllers };
    readonly Small: { readonly fields: RowHeightSmallFieldControllers };
    readonly Medium: { readonly fields: RowHeightMediumFieldControllers };
    readonly Large: { readonly fields: RowHeightLargeFieldControllers };
} /** Gets default value for a specific variant */
function rowHeightGetDefaultForVariant(variant: string): RowHeight {
    switch (variant) {
        case 'ExtraSmall':
            return 'ExtraSmall' as RowHeight;
        case 'Small':
            return 'Small' as RowHeight;
        case 'Medium':
            return 'Medium' as RowHeight;
        case 'Large':
            return 'Large' as RowHeight;
        default:
            return 'ExtraSmall' as RowHeight;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function rowHeightCreateForm(initial?: RowHeight): RowHeightGigaform {
    const initialVariant: 'ExtraSmall' | 'Small' | 'Medium' | 'Large' =
        (initial as 'ExtraSmall' | 'Small' | 'Medium' | 'Large') ?? 'ExtraSmall';
    let currentVariant = $state<'ExtraSmall' | 'Small' | 'Medium' | 'Large'>(initialVariant);
    let data = $state<RowHeight>(initial ?? rowHeightGetDefaultForVariant(initialVariant));
    let errors = $state<RowHeightErrors>({} as RowHeightErrors);
    let tainted = $state<RowHeightTainted>({} as RowHeightTainted);
    const variants: RowHeightVariantFields = {
        ExtraSmall: { fields: {} as RowHeightExtraSmallFieldControllers },
        Small: { fields: {} as RowHeightSmallFieldControllers },
        Medium: { fields: {} as RowHeightMediumFieldControllers },
        Large: { fields: {} as RowHeightLargeFieldControllers }
    };
    function switchVariant(variant: 'ExtraSmall' | 'Small' | 'Medium' | 'Large'): void {
        currentVariant = variant;
        data = rowHeightGetDefaultForVariant(variant);
        errors = {} as RowHeightErrors;
        tainted = {} as RowHeightTainted;
    }
    function validate(): Exit<Array<{ field: string; message: string }>, RowHeight> {
        return toExit(rowHeightDeserialize(data));
    }
    function reset(overrides?: Partial<RowHeight>): void {
        data = overrides
            ? (overrides as typeof data)
            : rowHeightGetDefaultForVariant(currentVariant);
        errors = {} as RowHeightErrors;
        tainted = {} as RowHeightTainted;
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
export function rowHeightFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, RowHeight> {
    const discriminant = formData.get('_value') as
        | 'ExtraSmall'
        | 'Small'
        | 'Medium'
        | 'Large'
        | null;
    if (!discriminant) {
        return toExit({
            success: false,
            errors: [{ field: '_value', message: 'Missing discriminant field' }]
        });
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'ExtraSmall') {
    } else if (discriminant === 'Small') {
    } else if (discriminant === 'Medium') {
    } else if (discriminant === 'Large') {
    }
    return toExit(rowHeightDeserialize(obj));
}

export const RowHeight = {
    defaultValue: rowHeightDefaultValue,
    serialize: rowHeightSerialize,
    serializeWithContext: rowHeightSerializeWithContext,
    deserialize: rowHeightDeserialize,
    deserializeWithContext: rowHeightDeserializeWithContext,
    is: rowHeightIs,
    createForm: rowHeightCreateForm,
    fromFormData: rowHeightFromFormData
} as const;
