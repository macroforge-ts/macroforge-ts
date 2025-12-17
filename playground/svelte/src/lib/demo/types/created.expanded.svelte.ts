import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Created {
    initialData: string | null;
}

export function createdDefaultValue(): Created {
    return { initialData: null } as Created;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function createdSerialize(
    value: Created
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(createdSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function createdSerializeWithContext(
    value: Created,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Created', __id };
    result['initialData'] = value.initialData;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function createdDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Result<Created, Array<{ field: string; message: string }>> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = createdDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Created.deserialize: root cannot be a forward reference'
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
export function createdDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): Created | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Created.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('initialData' in obj)) {
        errors.push({ field: 'initialData', message: 'missing required field' });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_initialData = obj['initialData'] as string | null;
        instance.initialData = __raw_initialData;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Created;
}
export function createdValidateField<K extends keyof Created>(
    field: K,
    value: Created[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function createdValidateFields(
    partial: Partial<Created>
): Array<{ field: string; message: string }> {
    return [];
}
export function createdHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'initialData' in o;
}
export function createdIs(obj: unknown): obj is Created {
    if (!createdHasShape(obj)) {
        return false;
    }
    const result = createdDeserialize(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type CreatedErrors = {
    _errors: Option<Array<string>>;
    initialData: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type CreatedTainted = {
    initialData: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface CreatedFieldControllers {
    readonly initialData: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface CreatedGigaform {
    readonly data: Created;
    readonly errors: CreatedErrors;
    readonly tainted: CreatedTainted;
    readonly fields: CreatedFieldControllers;
    validate(): Result<Created, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Created>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createdCreateForm(overrides?: Partial<Created>): CreatedGigaform {
    let data = $state({ ...createdDefaultValue(), ...overrides });
    let errors = $state<CreatedErrors>({ _errors: Option.none(), initialData: Option.none() });
    let tainted = $state<CreatedTainted>({ initialData: Option.none() });
    const fields: CreatedFieldControllers = {
        initialData: {
            path: ['initialData'] as const,
            name: 'initialData',
            constraints: { required: true },

            get: () => data.initialData,
            set: (value: string | null) => {
                data.initialData = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.initialData,
            setError: (value: Option<Array<string>>) => {
                errors.initialData = value;
            },
            getTainted: () => tainted.initialData,
            setTainted: (value: Option<boolean>) => {
                tainted.initialData = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = createdValidateField('initialData', data.initialData);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Created, Array<{ field: string; message: string }>> {
        return createdFromObject(data);
    }
    function reset(newOverrides?: Partial<Created>): void {
        data = { ...createdDefaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), initialData: Option.none() };
        tainted = { initialData: Option.none() };
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function createdFromFormData(
    formData: FormData
): Result<Created, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.initialData = formData.get('initialData') ?? '';
    return createdFromStringifiedJSON(JSON.stringify(obj));
}

export const Created = {
    defaultValue: createdDefaultValue,
    serialize: createdSerialize,
    serializeWithContext: createdSerializeWithContext,
    deserialize: createdDeserialize,
    deserializeWithContext: createdDeserializeWithContext,
    validateFields: createdValidateFields,
    hasShape: createdHasShape,
    is: createdIs,
    createForm: createdCreateForm,
    fromFormData: createdFromFormData
} as const;
