import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { Result } from 'macroforge/reexports';
import { Option } from 'macroforge/reexports';
import type { FieldController } from '@playground/macro/gigaform';

export type Priority = /** @default */ 'Medium' | 'High' | 'Low';

export function priorityDefaultValue(): Priority {
    return 'Medium';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function prioritySerialize(
    value: Priority
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(prioritySerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function prioritySerializeWithContext(value: Priority, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function priorityDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Priority> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = priorityDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'Priority.deserialize: root cannot be a forward reference'
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
export function priorityDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Priority | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Priority | __mf_PendingRef;
    }
    const allowedValues = ['Medium', 'High', 'Low'] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for Priority: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as Priority;
}
export function priorityIs(value: unknown): value is Priority {
    const allowedValues = ['Medium', 'High', 'Low'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type PriorityMediumErrors = {
    _errors: Option<Array<string>>;
};
export type PriorityHighErrors = { _errors: Option<Array<string>> };
export type PriorityLowErrors = { _errors: Option<Array<string>> }; /** Per-variant tainted types */
export type PriorityMediumTainted = {};
export type PriorityHighTainted = {};
export type PriorityLowTainted = {}; /** Union error type */
export type PriorityErrors =
    | ({ _value: 'Medium' } & PriorityMediumErrors)
    | ({ _value: 'High' } & PriorityHighErrors)
    | ({ _value: 'Low' } & PriorityLowErrors); /** Union tainted type */
export type PriorityTainted =
    | ({ _value: 'Medium' } & PriorityMediumTainted)
    | ({ _value: 'High' } & PriorityHighTainted)
    | ({ _value: 'Low' } & PriorityLowTainted); /** Per-variant field controller types */
export interface PriorityMediumFieldControllers {}
export interface PriorityHighFieldControllers {}
export interface PriorityLowFieldControllers {} /** Union Gigaform interface with variant switching */
export interface PriorityGigaform {
    readonly currentVariant: 'Medium' | 'High' | 'Low';
    readonly data: Priority;
    readonly errors: PriorityErrors;
    readonly tainted: PriorityTainted;
    readonly variants: PriorityVariantFields;
    switchVariant(variant: 'Medium' | 'High' | 'Low'): void;
    validate(): Result<Priority, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Priority>): void;
} /** Variant fields container */
export interface PriorityVariantFields {
    readonly Medium: { readonly fields: PriorityMediumFieldControllers };
    readonly High: { readonly fields: PriorityHighFieldControllers };
    readonly Low: { readonly fields: PriorityLowFieldControllers };
} /** Gets default value for a specific variant */
function priorityGetDefaultForVariant(variant: string): Priority {
    switch (variant) {
        case 'Medium':
            return 'Medium' as Priority;
        case 'High':
            return 'High' as Priority;
        case 'Low':
            return 'Low' as Priority;
        default:
            return 'Medium' as Priority;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function priorityCreateForm(initial?: Priority): PriorityGigaform {
    const initialVariant: 'Medium' | 'High' | 'Low' =
        (initial as 'Medium' | 'High' | 'Low') ?? 'Medium';
    let currentVariant = $state<'Medium' | 'High' | 'Low'>(initialVariant);
    let data = $state<Priority>(initial ?? priorityGetDefaultForVariant(initialVariant));
    let errors = $state<PriorityErrors>({} as PriorityErrors);
    let tainted = $state<PriorityTainted>({} as PriorityTainted);
    const variants: PriorityVariantFields = {
        Medium: { fields: {} as PriorityMediumFieldControllers },
        High: { fields: {} as PriorityHighFieldControllers },
        Low: { fields: {} as PriorityLowFieldControllers }
    };
    function switchVariant(variant: 'Medium' | 'High' | 'Low'): void {
        currentVariant = variant;
        data = priorityGetDefaultForVariant(variant);
        errors = {} as PriorityErrors;
        tainted = {} as PriorityTainted;
    }
    function validate(): Result<Priority, Array<{ field: string; message: string }>> {
        return priorityDeserialize(data);
    }
    function reset(overrides?: Partial<Priority>): void {
        data = overrides
            ? (overrides as typeof data)
            : priorityGetDefaultForVariant(currentVariant);
        errors = {} as PriorityErrors;
        tainted = {} as PriorityTainted;
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
export function priorityFromFormData(
    formData: FormData
): Result<Priority, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as 'Medium' | 'High' | 'Low' | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Medium') {
    } else if (discriminant === 'High') {
    } else if (discriminant === 'Low') {
    }
    return priorityDeserialize(obj);
}

export const Priority = {
    defaultValue: priorityDefaultValue,
    serialize: prioritySerialize,
    serializeWithContext: prioritySerializeWithContext,
    deserialize: priorityDeserialize,
    deserializeWithContext: priorityDeserializeWithContext,
    is: priorityIs,
    createForm: priorityCreateForm,
    fromFormData: priorityFromFormData
} as const;
