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

export type Priority = /** @default */ 'Medium' | 'High' | 'Low';

export function priorityDefaultValue(): Priority {
    return 'Medium';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function prioritySerialize(
    value: Priority
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(prioritySerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function prioritySerializeWithContext(value: Priority, ctx: SerializeContext): unknown {
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
    opts?: DeserializeOptions
): Exit.Exit<Array<{ field: string; message: string }>, Priority> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = priorityDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Exit.fail([
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
export function priorityDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): Priority | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Priority | PendingRef;
    }
    const allowedValues = ['Medium', 'High', 'Low'] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
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
    validate(): Exit<Array<{ field: string; message: string }>, Priority>;
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
    function validate(): Exit<Array<{ field: string; message: string }>, Priority> {
        return toExit(priorityDeserialize(data));
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
): Exit<Array<{ field: string; message: string }>, Priority> {
    const discriminant = formData.get('_value') as 'Medium' | 'High' | 'Low' | null;
    if (!discriminant) {
        return toExit({
            success: false,
            errors: [{ field: '_value', message: 'Missing discriminant field' }]
        });
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Medium') {
    } else if (discriminant === 'High') {
    } else if (discriminant === 'Low') {
    }
    return toExit(priorityDeserialize(obj));
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
