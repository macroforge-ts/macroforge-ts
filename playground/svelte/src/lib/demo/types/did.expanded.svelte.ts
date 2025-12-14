import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import { Target } from './target.svelte';
import { ActivityType } from './activity-type.svelte';
import { Actor } from './actor.svelte';

export interface Did {
    in: string | Actor;

    out: string | Target;
    id: string;
    activityType: ActivityType;
    createdAt: string;
    metadata: string | null;
}

export function defaultValueDid(): Did {
    return {
        in: '',
        out: '',
        id: '',
        activityType: ActivityType.defaultValue(),
        createdAt: '',
        metadata: null
    } as Did;
}

export function toStringifiedJSONDid(value: Did): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeDid(value, ctx));
}
export function toObjectDid(value: Did): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeDid(value, ctx);
}
export function __serializeDid(value: Did, ctx: SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Did', __id };
    result['in'] = value.in;
    result['out'] = value.out;
    result['id'] = value.id;
    result['activityType'] =
        typeof (value.activityType as any)?.__serialize === 'function'
            ? (value.activityType as any).__serialize(ctx)
            : value.activityType;
    result['createdAt'] = value.createdAt;
    result['metadata'] = value.metadata;
    return result;
}

export function fromStringifiedJSONDid(
    json: string,
    opts?: DeserializeOptions
): Result<Did, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectDid(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectDid(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Did, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeDid(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Did.fromObject: root cannot be a forward reference' }
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
export function __deserializeDid(value: any, ctx: DeserializeContext): Did | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Did.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('in' in obj)) {
        errors.push({ field: 'in', message: 'missing required field' });
    }
    if (!('out' in obj)) {
        errors.push({ field: 'out', message: 'missing required field' });
    }
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('activityType' in obj)) {
        errors.push({ field: 'activityType', message: 'missing required field' });
    }
    if (!('createdAt' in obj)) {
        errors.push({ field: 'createdAt', message: 'missing required field' });
    }
    if (!('metadata' in obj)) {
        errors.push({ field: 'metadata', message: 'missing required field' });
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
        const __raw_in = obj['in'] as string | Actor;
        instance.in = __raw_in;
    }
    {
        const __raw_out = obj['out'] as string | Target;
        instance.out = __raw_out;
    }
    {
        const __raw_id = obj['id'] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_activityType = obj['activityType'] as ActivityType;
        {
            const __result = ActivityType.__deserialize(__raw_activityType, ctx);
            ctx.assignOrDefer(instance, 'activityType', __result);
        }
    }
    {
        const __raw_createdAt = obj['createdAt'] as string;
        instance.createdAt = __raw_createdAt;
    }
    {
        const __raw_metadata = obj['metadata'] as string | null;
        instance.metadata = __raw_metadata;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Did;
}
export function validateFieldDid<K extends keyof Did>(
    field: K,
    value: Did[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsDid(
    partial: Partial<Did>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeDid(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'in' in o &&
        'out' in o &&
        'id' in o &&
        'activityType' in o &&
        'createdAt' in o &&
        'metadata' in o
    );
}
export function isDid(obj: unknown): obj is Did {
    if (!hasShapeDid(obj)) {
        return false;
    }
    const result = fromObjectDid(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsDid = {
    _errors: Option<Array<string>>;
    in: Option<Array<string>>;
    out: Option<Array<string>>;
    id: Option<Array<string>>;
    activityType: Option<Array<string>>;
    createdAt: Option<Array<string>>;
    metadata: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedDid = {
    in: Option<boolean>;
    out: Option<boolean>;
    id: Option<boolean>;
    activityType: Option<boolean>;
    createdAt: Option<boolean>;
    metadata: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersDid {
    readonly in: FieldController<string | Actor>;
    readonly out: FieldController<string | Target>;
    readonly id: FieldController<string>;
    readonly activityType: FieldController<ActivityType>;
    readonly createdAt: FieldController<string>;
    readonly metadata: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformDid {
    readonly data: Did;
    readonly errors: ErrorsDid;
    readonly tainted: TaintedDid;
    readonly fields: FieldControllersDid;
    validate(): Result<Did, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Did>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormDid(overrides?: Partial<Did>): GigaformDid {
    let data = $state({ ...Did.defaultValue(), ...overrides });
    let errors = $state<ErrorsDid>({
        _errors: Option.none(),
        in: Option.none(),
        out: Option.none(),
        id: Option.none(),
        activityType: Option.none(),
        createdAt: Option.none(),
        metadata: Option.none()
    });
    let tainted = $state<TaintedDid>({
        in: Option.none(),
        out: Option.none(),
        id: Option.none(),
        activityType: Option.none(),
        createdAt: Option.none(),
        metadata: Option.none()
    });
    const fields: FieldControllersDid = {
        in: {
            path: ['in'] as const,
            name: 'in',
            constraints: { required: true },

            get: () => data.in,
            set: (value: string | Actor) => {
                data.in = value;
            },
            transform: (value: string | Actor): string | Actor => value,
            getError: () => errors.in,
            setError: (value: Option<Array<string>>) => {
                errors.in = value;
            },
            getTainted: () => tainted.in,
            setTainted: (value: Option<boolean>) => {
                tainted.in = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = Did.validateField('in', data.in);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        out: {
            path: ['out'] as const,
            name: 'out',
            constraints: { required: true },

            get: () => data.out,
            set: (value: string | Target) => {
                data.out = value;
            },
            transform: (value: string | Target): string | Target => value,
            getError: () => errors.out,
            setError: (value: Option<Array<string>>) => {
                errors.out = value;
            },
            getTainted: () => tainted.out,
            setTainted: (value: Option<boolean>) => {
                tainted.out = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = Did.validateField('out', data.out);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        id: {
            path: ['id'] as const,
            name: 'id',
            constraints: { required: true },

            get: () => data.id,
            set: (value: string) => {
                data.id = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.id,
            setError: (value: Option<Array<string>>) => {
                errors.id = value;
            },
            getTainted: () => tainted.id,
            setTainted: (value: Option<boolean>) => {
                tainted.id = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = Did.validateField('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        activityType: {
            path: ['activityType'] as const,
            name: 'activityType',
            constraints: { required: true },

            get: () => data.activityType,
            set: (value: ActivityType) => {
                data.activityType = value;
            },
            transform: (value: ActivityType): ActivityType => value,
            getError: () => errors.activityType,
            setError: (value: Option<Array<string>>) => {
                errors.activityType = value;
            },
            getTainted: () => tainted.activityType,
            setTainted: (value: Option<boolean>) => {
                tainted.activityType = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = Did.validateField('activityType', data.activityType);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        createdAt: {
            path: ['createdAt'] as const,
            name: 'createdAt',
            constraints: { required: true },

            get: () => data.createdAt,
            set: (value: string) => {
                data.createdAt = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.createdAt,
            setError: (value: Option<Array<string>>) => {
                errors.createdAt = value;
            },
            getTainted: () => tainted.createdAt,
            setTainted: (value: Option<boolean>) => {
                tainted.createdAt = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = Did.validateField('createdAt', data.createdAt);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        metadata: {
            path: ['metadata'] as const,
            name: 'metadata',
            constraints: { required: true },

            get: () => data.metadata,
            set: (value: string | null) => {
                data.metadata = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.metadata,
            setError: (value: Option<Array<string>>) => {
                errors.metadata = value;
            },
            getTainted: () => tainted.metadata,
            setTainted: (value: Option<boolean>) => {
                tainted.metadata = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = Did.validateField('metadata', data.metadata);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Did, Array<{ field: string; message: string }>> {
        return Did.fromObject(data);
    }
    function reset(newOverrides?: Partial<Did>): void {
        data = { ...Did.defaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            in: Option.none(),
            out: Option.none(),
            id: Option.none(),
            activityType: Option.none(),
            createdAt: Option.none(),
            metadata: Option.none()
        };
        tainted = {
            in: Option.none(),
            out: Option.none(),
            id: Option.none(),
            activityType: Option.none(),
            createdAt: Option.none(),
            metadata: Option.none()
        };
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
export function fromFormDataDid(
    formData: FormData
): Result<Did, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.in = formData.get('in') ?? '';
    obj.out = formData.get('out') ?? '';
    obj.id = formData.get('id') ?? '';
    {
        // Collect nested object fields with prefix "activityType."
        const activityTypeObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('activityType.')) {
                const fieldName = key.slice('activityType.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = activityTypeObj;
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
        obj.activityType = activityTypeObj;
    }
    obj.createdAt = formData.get('createdAt') ?? '';
    obj.metadata = formData.get('metadata') ?? '';
    return Did.fromStringifiedJSON(JSON.stringify(obj));
}
