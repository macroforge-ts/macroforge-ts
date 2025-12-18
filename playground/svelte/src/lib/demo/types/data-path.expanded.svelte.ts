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
import type { ArrayFieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface DataPath {
    path: string[];
    formatter: string | null;
}

export function dataPathDefaultValue(): DataPath {
    return { path: [], formatter: null } as DataPath;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function dataPathSerialize(
    value: DataPath
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(dataPathSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function dataPathSerializeWithContext(
    value: DataPath,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'DataPath', __id };
    result['path'] = value.path;
    result['formatter'] = value.formatter;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function dataPathDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, DataPath> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = dataPathDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'DataPath.deserialize: root cannot be a forward reference'
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
export function dataPathDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): DataPath | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'DataPath.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('path' in obj)) {
        errors.push({ field: 'path', message: 'missing required field' });
    }
    if (!('formatter' in obj)) {
        errors.push({ field: 'formatter', message: 'missing required field' });
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
        const __raw_path = obj['path'] as string[];
        if (Array.isArray(__raw_path)) {
            instance.path = __raw_path as string[];
        }
    }
    {
        const __raw_formatter = obj['formatter'] as string | null;
        instance.formatter = __raw_formatter;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as DataPath;
}
export function dataPathValidateField<K extends keyof DataPath>(
    field: K,
    value: DataPath[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function dataPathValidateFields(
    partial: Partial<DataPath>
): Array<{ field: string; message: string }> {
    return [];
}
export function dataPathHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'path' in o && 'formatter' in o;
}
export function dataPathIs(obj: unknown): obj is DataPath {
    if (!dataPathHasShape(obj)) {
        return false;
    }
    const result = dataPathDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type DataPathErrors = {
    _errors: Option<Array<string>>;
    path: Option<Array<string>>;
    formatter: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type DataPathTainted = {
    path: Option<boolean>;
    formatter: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface DataPathFieldControllers {
    readonly path: ArrayFieldController<string>;
    readonly formatter: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface DataPathGigaform {
    readonly data: DataPath;
    readonly errors: DataPathErrors;
    readonly tainted: DataPathTainted;
    readonly fields: DataPathFieldControllers;
    validate(): Result<DataPath, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<DataPath>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function dataPathCreateForm(overrides?: Partial<DataPath>): DataPathGigaform {
    let data = $state({ ...dataPathDefaultValue(), ...overrides });
    let errors = $state<DataPathErrors>({
        _errors: Option.none(),
        path: Option.none(),
        formatter: Option.none()
    });
    let tainted = $state<DataPathTainted>({ path: Option.none(), formatter: Option.none() });
    const fields: DataPathFieldControllers = {
        path: {
            path: ['path'] as const,
            name: 'path',
            constraints: { required: true },
            get: () => data.path,
            set: (value: string[]) => {
                data.path = value;
            },
            transform: (value: string[]): string[] => value,
            getError: () => errors.path,
            setError: (value: Option<Array<string>>) => {
                errors.path = value;
            },
            getTainted: () => tainted.path,
            setTainted: (value: Option<boolean>) => {
                tainted.path = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = dataPathValidateField('path', data.path);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['path', index] as const,
                name: `path.${index}`,
                constraints: { required: true },
                get: () => data.path[index]!,
                set: (value: string) => {
                    data.path[index] = value;
                },
                transform: (value: string): string => value,
                getError: () => errors.path,
                setError: (value: Option<Array<string>>) => {
                    errors.path = value;
                },
                getTainted: () => tainted.path,
                setTainted: (value: Option<boolean>) => {
                    tainted.path = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: string) => {
                data.path.push(item);
            },
            remove: (index: number) => {
                data.path.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.path[a]!;
                data.path[a] = data.path[b]!;
                data.path[b] = tmp;
            }
        },
        formatter: {
            path: ['formatter'] as const,
            name: 'formatter',
            constraints: { required: true },
            get: () => data.formatter,
            set: (value: string | null) => {
                data.formatter = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.formatter,
            setError: (value: Option<Array<string>>) => {
                errors.formatter = value;
            },
            getTainted: () => tainted.formatter,
            setTainted: (value: Option<boolean>) => {
                tainted.formatter = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = dataPathValidateField('formatter', data.formatter);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<DataPath, Array<{ field: string; message: string }>> {
        return dataPathDeserialize(data);
    }
    function reset(newOverrides?: Partial<DataPath>): void {
        data = { ...dataPathDefaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), path: Option.none(), formatter: Option.none() };
        tainted = { path: Option.none(), formatter: Option.none() };
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
export function dataPathFromFormData(
    formData: FormData
): Result<DataPath, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.path = formData.getAll('path') as Array<string>;
    obj.formatter = formData.get('formatter') ?? '';
    return dataPathDeserialize(obj);
}

export const DataPath = {
    defaultValue: dataPathDefaultValue,
    serialize: dataPathSerialize,
    serializeWithContext: dataPathSerializeWithContext,
    deserialize: dataPathDeserialize,
    deserializeWithContext: dataPathDeserializeWithContext,
    validateFields: dataPathValidateFields,
    hasShape: dataPathHasShape,
    is: dataPathIs,
    createForm: dataPathCreateForm,
    fromFormData: dataPathFromFormData
} as const;
