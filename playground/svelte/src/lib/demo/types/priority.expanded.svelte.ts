import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

export type Priority = /** @default */ 'Medium' | 'High' | 'Low';

export function defaultValuePriority(): Priority {
    return 'Medium';
}

export function toStringifiedJSONPriority(value: Priority): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializePriority(value, ctx));
}
export function toObjectPriority(value: Priority): unknown {
    const ctx = SerializeContext.create();
    return __serializePriority(value, ctx);
}
export function __serializePriority(value: Priority, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONPriority(
    json: string,
    opts?: DeserializeOptions
): Result<Priority, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectPriority(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectPriority(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Priority, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializePriority(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Priority.fromObject: root cannot be a forward reference'
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
export function __deserializePriority(value: any, ctx: DeserializeContext): Priority | PendingRef {
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
export function isPriority(value: unknown): value is Priority {
    const allowedValues = ['Medium', 'High', 'Low'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type MediumErrorsPriority = {
    _errors: Option<Array<string>>;
};
export type HighErrorsPriority = { _errors: Option<Array<string>> };
export type LowErrorsPriority = { _errors: Option<Array<string>> }; /** Per-variant tainted types */
export type MediumTaintedPriority = {};
export type HighTaintedPriority = {};
export type LowTaintedPriority = {}; /** Union error type */
export type ErrorsPriority =
    | ({ _value: 'Medium' } & MediumErrorsPriority)
    | ({ _value: 'High' } & HighErrorsPriority)
    | ({ _value: 'Low' } & LowErrorsPriority); /** Union tainted type */
export type TaintedPriority =
    | ({ _value: 'Medium' } & MediumTaintedPriority)
    | ({ _value: 'High' } & HighTaintedPriority)
    | ({ _value: 'Low' } & LowTaintedPriority); /** Per-variant field controller types */
export interface MediumFieldControllersPriority {}
export interface HighFieldControllersPriority {}
export interface LowFieldControllersPriority {} /** Union Gigaform interface with variant switching */
export interface GigaformPriority {
    readonly currentVariant: 'Medium' | 'High' | 'Low';
    readonly data: Priority;
    readonly errors: ErrorsPriority;
    readonly tainted: TaintedPriority;
    readonly variants: VariantFieldsPriority;
    switchVariant(variant: 'Medium' | 'High' | 'Low'): void;
    validate(): Result<Priority, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Priority>): void;
} /** Variant fields container */
export interface VariantFieldsPriority {
    readonly Medium: { readonly fields: MediumFieldControllersPriority };
    readonly High: { readonly fields: HighFieldControllersPriority };
    readonly Low: { readonly fields: LowFieldControllersPriority };
} /** Gets default value for a specific variant */
function getDefaultForVariantPriority(variant: string): Priority {
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
export function createFormPriority(initial?: Priority): GigaformPriority {
    const initialVariant: 'Medium' | 'High' | 'Low' =
        (initial as 'Medium' | 'High' | 'Low') ?? 'Medium';
    let currentVariant = $state<'Medium' | 'High' | 'Low'>(initialVariant);
    let data = $state<Priority>(initial ?? getDefaultForVariantPriority(initialVariant));
    let errors = $state<ErrorsPriority>({} as ErrorsPriority);
    let tainted = $state<TaintedPriority>({} as TaintedPriority);
    const variants: VariantFieldsPriority = {
        Medium: {
            fields: {} as MediumFieldControllersPriority
        },
        High: {
            fields: {} as HighFieldControllersPriority
        },
        Low: {
            fields: {} as LowFieldControllersPriority
        }
    };
    function switchVariant(variant: 'Medium' | 'High' | 'Low'): void {
        currentVariant = variant;
        data = getDefaultForVariantPriority(variant);
        errors = {} as ErrorsPriority;
        tainted = {} as TaintedPriority;
    }
    function validate(): Result<Priority, Array<{ field: string; message: string }>> {
        return fromObjectPriority(data);
    }
    function reset(overrides?: Partial<Priority>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantPriority(currentVariant);
        errors = {} as ErrorsPriority;
        tainted = {} as TaintedPriority;
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
export function fromFormDataPriority(
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
    return fromStringifiedJSONPriority(JSON.stringify(obj));
}
