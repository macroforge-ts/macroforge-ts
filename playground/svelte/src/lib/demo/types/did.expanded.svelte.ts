import { activityTypeDefaultValue } from './activity-type.svelte';
import { SerializeContext } from 'macroforge/serde';
import { activityTypeSerializeWithContext } from './activity-type.svelte';
import { Exit } from 'macroforge/utils/effect';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { activityTypeDeserializeWithContext } from './activity-type.svelte';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { Target } from './target.svelte';
import type { ActivityType } from './activity-type.svelte';
import type { Actor } from './actor.svelte';

export interface Did {
    in: string | Actor;

    out: string | Target;
    id: string;
    activityType: ActivityType;
    createdAt: string;
    metadata: string | null;
}

export function didDefaultValue(): Did {
    return {
        in: '',
        out: '',
        id: '',
        activityType: activityTypeDefaultValue(),
        createdAt: '',
        metadata: null
    } as Did;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function didSerialize(
    value: Did
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(didSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function didSerializeWithContext(
    value: Did,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Did', __id };
    result['in'] = value.in;
    result['out'] = value.out;
    result['id'] = value.id;
    result['activityType'] = activityTypeSerializeWithContext(value.activityType, ctx);
    result['createdAt'] = value.createdAt;
    result['metadata'] = value.metadata;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function didDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Exit.Exit<Array<{ field: string; message: string }>, Did> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = didDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Exit.fail([
                { field: '_root', message: 'Did.deserialize: root cannot be a forward reference' }
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
export function didDeserializeWithContext(value: any, ctx: DeserializeContext): Did | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Did.deserializeWithContext: expected an object' }
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
            const __result = activityTypeDeserializeWithContext(__raw_activityType, ctx);
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
export function didValidateField<K extends keyof Did>(
    field: K,
    value: Did[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function didValidateFields(
    partial: Partial<Did>
): Array<{ field: string; message: string }> {
    return [];
}
export function didHasShape(obj: unknown): boolean {
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
export function didIs(obj: unknown): obj is Did {
    if (!didHasShape(obj)) {
        return false;
    }
    const result = didDeserialize(obj);
    return Exit.isSuccess(result);
}

/** Nested error structure matching the data shape */ export type DidErrors = {
    _errors: Option<Array<string>>;
    in: Option<Array<string>>;
    out: Option<Array<string>>;
    id: Option<Array<string>>;
    activityType: Option<Array<string>>;
    createdAt: Option<Array<string>>;
    metadata: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type DidTainted = {
    in: Option<boolean>;
    out: Option<boolean>;
    id: Option<boolean>;
    activityType: Option<boolean>;
    createdAt: Option<boolean>;
    metadata: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface DidFieldControllers {
    readonly in: FieldController<string | Actor>;
    readonly out: FieldController<string | Target>;
    readonly id: FieldController<string>;
    readonly activityType: FieldController<ActivityType>;
    readonly createdAt: FieldController<string>;
    readonly metadata: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface DidGigaform {
    readonly data: Did;
    readonly errors: DidErrors;
    readonly tainted: DidTainted;
    readonly fields: DidFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, Did>;
    reset(overrides?: Partial<Did>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function didCreateForm(overrides?: Partial<Did>): DidGigaform {
    let data = $state({ ...didDefaultValue(), ...overrides });
    let errors = $state<DidErrors>({
        _errors: optionNone(),
        in: optionNone(),
        out: optionNone(),
        id: optionNone(),
        activityType: optionNone(),
        createdAt: optionNone(),
        metadata: optionNone()
    });
    let tainted = $state<DidTainted>({
        in: optionNone(),
        out: optionNone(),
        id: optionNone(),
        activityType: optionNone(),
        createdAt: optionNone(),
        metadata: optionNone()
    });
    const fields: DidFieldControllers = {
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
                const fieldErrors = didValidateField('in', data.in);
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
                const fieldErrors = didValidateField('out', data.out);
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
                const fieldErrors = didValidateField('id', data.id);
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
                const fieldErrors = didValidateField('activityType', data.activityType);
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
                const fieldErrors = didValidateField('createdAt', data.createdAt);
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
                const fieldErrors = didValidateField('metadata', data.metadata);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, Did> {
        return toExit(didDeserialize(data));
    }
    function reset(newOverrides?: Partial<Did>): void {
        data = { ...didDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            in: optionNone(),
            out: optionNone(),
            id: optionNone(),
            activityType: optionNone(),
            createdAt: optionNone(),
            metadata: optionNone()
        };
        tainted = {
            in: optionNone(),
            out: optionNone(),
            id: optionNone(),
            activityType: optionNone(),
            createdAt: optionNone(),
            metadata: optionNone()
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */
export function didFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, Did> {
    const obj: Record<string, unknown> = {};
    obj.in = formData.get('in') ?? '';
    obj.out = formData.get('out') ?? '';
    obj.id = formData.get('id') ?? '';
    {
        const activityTypeObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('activityType.')) {
                const fieldName = key.slice('activityType.'.length);
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
    return toExit(didDeserialize(obj));
}

export const Did = {
    defaultValue: didDefaultValue,
    serialize: didSerialize,
    serializeWithContext: didSerializeWithContext,
    deserialize: didDeserialize,
    deserializeWithContext: didDeserializeWithContext,
    validateFields: didValidateFields,
    hasShape: didHasShape,
    is: didIs,
    createForm: didCreateForm,
    fromFormData: didFromFormData
} as const;
