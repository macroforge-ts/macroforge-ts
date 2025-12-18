import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { Result } from '@playground/macro/gigaform';
import { Option } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Viewed {
    durationSeconds: number | null;
    source: string | null;
}

export function viewedDefaultValue(): Viewed {
    return { durationSeconds: null, source: null } as Viewed;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function viewedSerialize(
    value: Viewed
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(viewedSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function viewedSerializeWithContext(
    value: Viewed,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Viewed', __id };
    result['durationSeconds'] = value.durationSeconds;
    result['source'] = value.source;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function viewedDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Viewed> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = viewedDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'Viewed.deserialize: root cannot be a forward reference'
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
export function viewedDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Viewed | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Viewed.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('durationSeconds' in obj)) {
        errors.push({ field: 'durationSeconds', message: 'missing required field' });
    }
    if (!('source' in obj)) {
        errors.push({ field: 'source', message: 'missing required field' });
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
        const __raw_durationSeconds = obj['durationSeconds'] as number | null;
        instance.durationSeconds = __raw_durationSeconds;
    }
    {
        const __raw_source = obj['source'] as string | null;
        instance.source = __raw_source;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Viewed;
}
export function viewedValidateField<K extends keyof Viewed>(
    field: K,
    value: Viewed[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function viewedValidateFields(
    partial: Partial<Viewed>
): Array<{ field: string; message: string }> {
    return [];
}
export function viewedHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'durationSeconds' in o && 'source' in o;
}
export function viewedIs(obj: unknown): obj is Viewed {
    if (!viewedHasShape(obj)) {
        return false;
    }
    const result = viewedDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type ViewedErrors = {
    _errors: Option<Array<string>>;
    durationSeconds: Option<Array<string>>;
    source: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type ViewedTainted = {
    durationSeconds: Option<boolean>;
    source: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface ViewedFieldControllers {
    readonly durationSeconds: FieldController<number | null>;
    readonly source: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface ViewedGigaform {
    readonly data: Viewed;
    readonly errors: ViewedErrors;
    readonly tainted: ViewedTainted;
    readonly fields: ViewedFieldControllers;
    validate(): Result<Viewed, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Viewed>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function viewedCreateForm(overrides?: Partial<Viewed>): ViewedGigaform {
    let data = $state({ ...viewedDefaultValue(), ...overrides });
    let errors = $state<ViewedErrors>({
        _errors: Option.none(),
        durationSeconds: Option.none(),
        source: Option.none()
    });
    let tainted = $state<ViewedTainted>({ durationSeconds: Option.none(), source: Option.none() });
    const fields: ViewedFieldControllers = {
        durationSeconds: {
            path: ['durationSeconds'] as const,
            name: 'durationSeconds',
            constraints: { required: true },
            get: () => data.durationSeconds,
            set: (value: number | null) => {
                data.durationSeconds = value;
            },
            transform: (value: number | null): number | null => value,
            getError: () => errors.durationSeconds,
            setError: (value: Option<Array<string>>) => {
                errors.durationSeconds = value;
            },
            getTainted: () => tainted.durationSeconds,
            setTainted: (value: Option<boolean>) => {
                tainted.durationSeconds = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = viewedValidateField('durationSeconds', data.durationSeconds);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        source: {
            path: ['source'] as const,
            name: 'source',
            constraints: { required: true },
            get: () => data.source,
            set: (value: string | null) => {
                data.source = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.source,
            setError: (value: Option<Array<string>>) => {
                errors.source = value;
            },
            getTainted: () => tainted.source,
            setTainted: (value: Option<boolean>) => {
                tainted.source = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = viewedValidateField('source', data.source);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Viewed, Array<{ field: string; message: string }>> {
        return viewedDeserialize(data);
    }
    function reset(newOverrides?: Partial<Viewed>): void {
        data = { ...viewedDefaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), durationSeconds: Option.none(), source: Option.none() };
        tainted = { durationSeconds: Option.none(), source: Option.none() };
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
export function viewedFromFormData(
    formData: FormData
): Result<Viewed, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const durationSecondsStr = formData.get('durationSeconds');
        obj.durationSeconds = durationSecondsStr ? parseFloat(durationSecondsStr as string) : 0;
        if (obj.durationSeconds !== undefined && isNaN(obj.durationSeconds as number))
            obj.durationSeconds = 0;
    }
    obj.source = formData.get('source') ?? '';
    return viewedDeserialize(obj);
}

export const Viewed = {
    defaultValue: viewedDefaultValue,
    serialize: viewedSerialize,
    serializeWithContext: viewedSerializeWithContext,
    deserialize: viewedDeserialize,
    deserializeWithContext: viewedDeserializeWithContext,
    validateFields: viewedValidateFields,
    hasShape: viewedHasShape,
    is: viewedIs,
    createForm: viewedCreateForm,
    fromFormData: viewedFromFormData
} as const;
