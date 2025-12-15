import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
import type { ArrayFieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface DataPath {
    path: string[];
    formatter: string | null;
}

export function defaultValueDataPath(): DataPath {
    return { path: [], formatter: null } as DataPath;
}

export function toStringifiedJSONDataPath(value: DataPath): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeDataPath(value, ctx));
}
export function toObjectDataPath(value: DataPath): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeDataPath(value, ctx);
}
export function __serializeDataPath(
    value: DataPath,
    ctx: SerializeContext
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

export function fromStringifiedJSONDataPath(
    json: string,
    opts?: DeserializeOptions
): Result<DataPath, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectDataPath(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectDataPath(
    obj: unknown,
    opts?: DeserializeOptions
): Result<DataPath, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeDataPath(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'DataPath.fromObject: root cannot be a forward reference'
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
export function __deserializeDataPath(value: any, ctx: DeserializeContext): DataPath | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'DataPath.__deserialize: expected an object' }
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
        throw new DeserializeError(errors);
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
        throw new DeserializeError(errors);
    }
    return instance as DataPath;
}
export function validateFieldDataPath<K extends keyof DataPath>(
    field: K,
    value: DataPath[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsDataPath(
    partial: Partial<DataPath>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeDataPath(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'path' in o && 'formatter' in o;
}
export function isDataPath(obj: unknown): obj is DataPath {
    if (!hasShapeDataPath(obj)) {
        return false;
    }
    const result = fromObjectDataPath(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsDataPath = {
    _errors: Option<Array<string>>;
    path: Option<Array<string>>;
    formatter: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedDataPath = {
    path: Option<boolean>;
    formatter: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersDataPath {
    readonly path: ArrayFieldController<string>;
    readonly formatter: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformDataPath {
    readonly data: DataPath;
    readonly errors: ErrorsDataPath;
    readonly tainted: TaintedDataPath;
    readonly fields: FieldControllersDataPath;
    validate(): Result<DataPath, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<DataPath>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormDataPath(overrides?: Partial<DataPath>): GigaformDataPath {
    let data = $state({ ...defaultValueDataPath(), ...overrides });
    let errors = $state<ErrorsDataPath>({
        _errors: Option.none(),
        path: Option.none(),
        formatter: Option.none()
    });
    let tainted = $state<TaintedDataPath>({ path: Option.none(), formatter: Option.none() });
    const fields: FieldControllersDataPath = {
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
                const fieldErrors = validateFieldDataPath('path', data.path);
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
                const fieldErrors = validateFieldDataPath('formatter', data.formatter);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<DataPath, Array<{ field: string; message: string }>> {
        return fromObjectDataPath(data);
    }
    function reset(newOverrides?: Partial<DataPath>): void {
        data = { ...defaultValueDataPath(), ...newOverrides };
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function fromFormDataDataPath(
    formData: FormData
): Result<DataPath, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.path = formData.getAll('path') as Array<string>;
    obj.formatter = formData.get('formatter') ?? '';
    return fromStringifiedJSONDataPath(JSON.stringify(obj));
}
