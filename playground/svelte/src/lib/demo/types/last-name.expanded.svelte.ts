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
/** import macro {Gigaform} from "@playground/macro"; */

export interface LastName {
    name: string;
}

export function lastNameDefaultValue(): LastName {
    return { name: '' } as LastName;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function lastNameSerialize(
    value: LastName
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(lastNameSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function lastNameSerializeWithContext(
    value: LastName,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'LastName', __id };
    result['name'] = value.name;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function lastNameDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, LastName> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = lastNameDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'LastName.deserialize: root cannot be a forward reference'
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
export function lastNameDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): LastName | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'LastName.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('name' in obj)) {
        errors.push({ field: 'name', message: 'missing required field' });
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
        const __raw_name = obj['name'] as string;
        if (__raw_name.length === 0) {
            errors.push({ field: 'name', message: 'must not be empty' });
        }
        instance.name = __raw_name;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as LastName;
}
export function lastNameValidateField<K extends keyof LastName>(
    field: K,
    value: LastName[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'name': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'name', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function lastNameValidateFields(
    partial: Partial<LastName>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('name' in partial && partial.name !== undefined) {
        const __val = partial.name as string;
        if (__val.length === 0) {
            errors.push({ field: 'name', message: 'must not be empty' });
        }
    }
    return errors;
}
export function lastNameHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'name' in o;
}
export function lastNameIs(obj: unknown): obj is LastName {
    if (!lastNameHasShape(obj)) {
        return false;
    }
    const result = lastNameDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type LastNameErrors = {
    _errors: Option<Array<string>>;
    name: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type LastNameTainted = {
    name: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface LastNameFieldControllers {
    readonly name: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface LastNameGigaform {
    readonly data: LastName;
    readonly errors: LastNameErrors;
    readonly tainted: LastNameTainted;
    readonly fields: LastNameFieldControllers;
    validate(): Result<LastName, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<LastName>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function lastNameCreateForm(overrides?: Partial<LastName>): LastNameGigaform {
    let data = $state({ ...lastNameDefaultValue(), ...overrides });
    let errors = $state<LastNameErrors>({ _errors: Option.none(), name: Option.none() });
    let tainted = $state<LastNameTainted>({ name: Option.none() });
    const fields: LastNameFieldControllers = {
        name: {
            path: ['name'] as const,
            name: 'name',
            constraints: { required: true },
            get: () => data.name,
            set: (value: string) => {
                data.name = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.name,
            setError: (value: Option<Array<string>>) => {
                errors.name = value;
            },
            getTainted: () => tainted.name,
            setTainted: (value: Option<boolean>) => {
                tainted.name = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = lastNameValidateField('name', data.name);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<LastName, Array<{ field: string; message: string }>> {
        return lastNameDeserialize(data);
    }
    function reset(newOverrides?: Partial<LastName>): void {
        data = { ...lastNameDefaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), name: Option.none() };
        tainted = { name: Option.none() };
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
export function lastNameFromFormData(
    formData: FormData
): Result<LastName, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.name = formData.get('name') ?? '';
    return lastNameDeserialize(obj);
}

export const LastName = {
    defaultValue: lastNameDefaultValue,
    serialize: lastNameSerialize,
    serializeWithContext: lastNameSerializeWithContext,
    deserialize: lastNameDeserialize,
    deserializeWithContext: lastNameDeserializeWithContext,
    validateFields: lastNameValidateFields,
    hasShape: lastNameHasShape,
    is: lastNameIs,
    createForm: lastNameCreateForm,
    fromFormData: lastNameFromFormData
} as const;
