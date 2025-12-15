import { defaultValueDataPath } from './data-path.svelte';
import { SerializeContext } from 'macroforge/serde';
import { __serializeDataPath } from './data-path.svelte';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { __deserializeDataPath } from './data-path.svelte';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { DataPath } from './data-path.svelte';

export interface ColumnConfig {
    heading: string;
    dataPath: DataPath;
}

export function defaultValueColumnConfig(): ColumnConfig {
    return { heading: '', dataPath: defaultValueDataPath() } as ColumnConfig;
}

export function toStringifiedJSONColumnConfig(value: ColumnConfig): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeColumnConfig(value, ctx));
}
export function toObjectColumnConfig(value: ColumnConfig): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeColumnConfig(value, ctx);
}
export function __serializeColumnConfig(
    value: ColumnConfig,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'ColumnConfig', __id };
    result['heading'] = value.heading;
    result['dataPath'] = __serializeDataPath(value.dataPath, ctx);
    return result;
}

export function fromStringifiedJSONColumnConfig(
    json: string,
    opts?: DeserializeOptions
): Result<ColumnConfig, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectColumnConfig(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectColumnConfig(
    obj: unknown,
    opts?: DeserializeOptions
): Result<ColumnConfig, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeColumnConfig(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'ColumnConfig.fromObject: root cannot be a forward reference'
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
export function __deserializeColumnConfig(
    value: any,
    ctx: DeserializeContext
): ColumnConfig | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'ColumnConfig.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('heading' in obj)) {
        errors.push({ field: 'heading', message: 'missing required field' });
    }
    if (!('dataPath' in obj)) {
        errors.push({ field: 'dataPath', message: 'missing required field' });
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
        const __raw_heading = obj['heading'] as string;
        if (__raw_heading.length === 0) {
            errors.push({ field: 'heading', message: 'must not be empty' });
        }
        instance.heading = __raw_heading;
    }
    {
        const __raw_dataPath = obj['dataPath'] as DataPath;
        {
            const __result = __deserializeDataPath(__raw_dataPath, ctx);
            ctx.assignOrDefer(instance, 'dataPath', __result);
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as ColumnConfig;
}
export function validateFieldColumnConfig<K extends keyof ColumnConfig>(
    field: K,
    value: ColumnConfig[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'heading': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'heading', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function validateFieldsColumnConfig(
    partial: Partial<ColumnConfig>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('heading' in partial && partial.heading !== undefined) {
        const __val = partial.heading as string;
        if (__val.length === 0) {
            errors.push({ field: 'heading', message: 'must not be empty' });
        }
    }
    return errors;
}
export function hasShapeColumnConfig(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'heading' in o && 'dataPath' in o;
}
export function isColumnConfig(obj: unknown): obj is ColumnConfig {
    if (!hasShapeColumnConfig(obj)) {
        return false;
    }
    const result = fromObjectColumnConfig(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsColumnConfig = {
    _errors: Option<Array<string>>;
    heading: Option<Array<string>>;
    dataPath: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedColumnConfig = {
    heading: Option<boolean>;
    dataPath: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersColumnConfig {
    readonly heading: FieldController<string>;
    readonly dataPath: FieldController<DataPath>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformColumnConfig {
    readonly data: ColumnConfig;
    readonly errors: ErrorsColumnConfig;
    readonly tainted: TaintedColumnConfig;
    readonly fields: FieldControllersColumnConfig;
    validate(): Result<ColumnConfig, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<ColumnConfig>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormColumnConfig(overrides?: Partial<ColumnConfig>): GigaformColumnConfig {
    let data = $state({ ...defaultValueColumnConfig(), ...overrides });
    let errors = $state<ErrorsColumnConfig>({
        _errors: Option.none(),
        heading: Option.none(),
        dataPath: Option.none()
    });
    let tainted = $state<TaintedColumnConfig>({ heading: Option.none(), dataPath: Option.none() });
    const fields: FieldControllersColumnConfig = {
        heading: {
            path: ['heading'] as const,
            name: 'heading',
            constraints: { required: true },

            get: () => data.heading,
            set: (value: string) => {
                data.heading = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.heading,
            setError: (value: Option<Array<string>>) => {
                errors.heading = value;
            },
            getTainted: () => tainted.heading,
            setTainted: (value: Option<boolean>) => {
                tainted.heading = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldColumnConfig('heading', data.heading);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        dataPath: {
            path: ['dataPath'] as const,
            name: 'dataPath',
            constraints: { required: true },

            get: () => data.dataPath,
            set: (value: DataPath) => {
                data.dataPath = value;
            },
            transform: (value: DataPath): DataPath => value,
            getError: () => errors.dataPath,
            setError: (value: Option<Array<string>>) => {
                errors.dataPath = value;
            },
            getTainted: () => tainted.dataPath,
            setTainted: (value: Option<boolean>) => {
                tainted.dataPath = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldColumnConfig('dataPath', data.dataPath);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<ColumnConfig, Array<{ field: string; message: string }>> {
        return fromObjectColumnConfig(data);
    }
    function reset(newOverrides?: Partial<ColumnConfig>): void {
        data = { ...defaultValueColumnConfig(), ...newOverrides };
        errors = { _errors: Option.none(), heading: Option.none(), dataPath: Option.none() };
        tainted = { heading: Option.none(), dataPath: Option.none() };
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
export function fromFormDataColumnConfig(
    formData: FormData
): Result<ColumnConfig, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.heading = formData.get('heading') ?? '';
    {
        // Collect nested object fields with prefix "dataPath."
        const dataPathObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('dataPath.')) {
                const fieldName = key.slice('dataPath.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = dataPathObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.dataPath = dataPathObj;
    }
    return fromStringifiedJSONColumnConfig(JSON.stringify(obj));
}
