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

import { DirectionHue } from './direction-hue.svelte';

export interface Custom {
    mappings: DirectionHue[];
}

export function defaultValueCustom(): Custom {
    return { mappings: [] } as Custom;
}

export function toStringifiedJSONCustom(value: Custom): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeCustom(value, ctx));
}
export function toObjectCustom(value: Custom): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeCustom(value, ctx);
}
export function __serializeCustom(value: Custom, ctx: SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Custom', __id };
    result['mappings'] = value.mappings.map((item: any) =>
        typeof item?.__serialize === 'function' ? item.__serialize(ctx) : item
    );
    return result;
}

export function fromStringifiedJSONCustom(
    json: string,
    opts?: DeserializeOptions
): Result<Custom, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectCustom(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectCustom(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Custom, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeCustom(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Custom.fromObject: root cannot be a forward reference' }
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
export function __deserializeCustom(value: any, ctx: DeserializeContext): Custom | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Custom.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('mappings' in obj)) {
        errors.push({ field: 'mappings', message: 'missing required field' });
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
        const __raw_mappings = obj['mappings'] as DirectionHue[];
        if (Array.isArray(__raw_mappings)) {
            instance.mappings = __raw_mappings as DirectionHue[];
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Custom;
}
export function validateFieldCustom<K extends keyof Custom>(
    field: K,
    value: Custom[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsCustom(
    partial: Partial<Custom>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeCustom(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'mappings' in o;
}
export function isCustom(obj: unknown): obj is Custom {
    if (!hasShapeCustom(obj)) {
        return false;
    }
    const result = fromObjectCustom(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsCustom = {
    _errors: Option<Array<string>>;
    mappings: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedCustom = {
    mappings: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersCustom {
    readonly mappings: ArrayFieldController<DirectionHue>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformCustom {
    readonly data: Custom;
    readonly errors: ErrorsCustom;
    readonly tainted: TaintedCustom;
    readonly fields: FieldControllersCustom;
    validate(): Result<Custom, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Custom>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormCustom(overrides?: Partial<Custom>): GigaformCustom {
    let data = $state({ ...Custom.defaultValue(), ...overrides });
    let errors = $state<ErrorsCustom>({ _errors: Option.none(), mappings: Option.none() });
    let tainted = $state<TaintedCustom>({ mappings: Option.none() });
    const fields: FieldControllersCustom = {
        mappings: {
            path: ['mappings'] as const,
            name: 'mappings',
            constraints: { required: true },

            get: () => data.mappings,
            set: (value: DirectionHue[]) => {
                data.mappings = value;
            },
            transform: (value: DirectionHue[]): DirectionHue[] => value,
            getError: () => errors.mappings,
            setError: (value: Option<Array<string>>) => {
                errors.mappings = value;
            },
            getTainted: () => tainted.mappings,
            setTainted: (value: Option<boolean>) => {
                tainted.mappings = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = Custom.validateField('mappings', data.mappings);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['mappings', index] as const,
                name: `mappings.${index}`,
                constraints: { required: true },
                get: () => data.mappings[index]!,
                set: (value: DirectionHue) => {
                    data.mappings[index] = value;
                },
                transform: (value: DirectionHue): DirectionHue => value,
                getError: () => errors.mappings,
                setError: (value: Option<Array<string>>) => {
                    errors.mappings = value;
                },
                getTainted: () => tainted.mappings,
                setTainted: (value: Option<boolean>) => {
                    tainted.mappings = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: DirectionHue) => {
                data.mappings.push(item);
            },
            remove: (index: number) => {
                data.mappings.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.mappings[a]!;
                data.mappings[a] = data.mappings[b]!;
                data.mappings[b] = tmp;
            }
        }
    };
    function validate(): Result<Custom, Array<{ field: string; message: string }>> {
        return Custom.fromObject(data);
    }
    function reset(newOverrides?: Partial<Custom>): void {
        data = { ...Custom.defaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), mappings: Option.none() };
        tainted = { mappings: Option.none() };
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
export function fromFormDataCustom(
    formData: FormData
): Result<Custom, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        // Collect array items from indexed form fields
        const mappingsItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('mappings.' + idx + '.') || idx === 0) {
            // Check if any field with this index exists
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('mappings.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('mappings.' + idx + '.')) {
                        const fieldName = key.slice('mappings.'.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                mappingsItems.push(item);
            }
            idx++;
            if (idx > 1000) break; // Safety limit
        }
        obj.mappings = mappingsItems;
    }
    return Custom.fromStringifiedJSON(JSON.stringify(obj));
}
