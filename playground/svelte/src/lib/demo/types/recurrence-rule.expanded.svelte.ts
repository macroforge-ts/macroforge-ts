import { intervalDefaultValue } from './interval.svelte';
import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { intervalSerializeWithContext } from './interval.svelte';
import { recurrenceEndSerializeWithContext } from './recurrence-end.svelte';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { intervalDeserializeWithContext } from './interval.svelte';
import { recurrenceEndDeserializeWithContext } from './recurrence-end.svelte';
import { Result } from '@playground/macro/gigaform';
import { Option } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { Interval } from './interval.svelte';
import type { RecurrenceEnd } from './recurrence-end.svelte';

export interface RecurrenceRule {
    interval: Interval;
    recurrenceBegins: string;
    recurrenceEnds: RecurrenceEnd | null;
    cancelledInstances: string[] | null;
    additionalInstances: string[] | null;
}

export function recurrenceRuleDefaultValue(): RecurrenceRule {
    return {
        interval: intervalDefaultValue(),
        recurrenceBegins: '',
        recurrenceEnds: null,
        cancelledInstances: null,
        additionalInstances: null
    } as RecurrenceRule;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function recurrenceRuleSerialize(
    value: RecurrenceRule
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(recurrenceRuleSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function recurrenceRuleSerializeWithContext(
    value: RecurrenceRule,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'RecurrenceRule', __id };
    result['interval'] = intervalSerializeWithContext(value.interval, ctx);
    result['recurrenceBegins'] = value.recurrenceBegins;
    if (value.recurrenceEnds !== null) {
        result['recurrenceEnds'] = recurrenceEndSerializeWithContext(value.recurrenceEnds, ctx);
    } else {
        result['recurrenceEnds'] = null;
    }
    if (value.cancelledInstances !== null) {
        result['cancelledInstances'] = value.cancelledInstances;
    } else {
        result['cancelledInstances'] = null;
    }
    if (value.additionalInstances !== null) {
        result['additionalInstances'] = value.additionalInstances;
    } else {
        result['additionalInstances'] = null;
    }
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function recurrenceRuleDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, RecurrenceRule> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = recurrenceRuleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'RecurrenceRule.deserialize: root cannot be a forward reference'
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
export function recurrenceRuleDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): RecurrenceRule | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'RecurrenceRule.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('interval' in obj)) {
        errors.push({ field: 'interval', message: 'missing required field' });
    }
    if (!('recurrenceBegins' in obj)) {
        errors.push({ field: 'recurrenceBegins', message: 'missing required field' });
    }
    if (!('recurrenceEnds' in obj)) {
        errors.push({ field: 'recurrenceEnds', message: 'missing required field' });
    }
    if (!('cancelledInstances' in obj)) {
        errors.push({ field: 'cancelledInstances', message: 'missing required field' });
    }
    if (!('additionalInstances' in obj)) {
        errors.push({ field: 'additionalInstances', message: 'missing required field' });
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
        const __raw_interval = obj['interval'] as Interval;
        {
            const __result = intervalDeserializeWithContext(__raw_interval, ctx);
            ctx.assignOrDefer(instance, 'interval', __result);
        }
    }
    {
        const __raw_recurrenceBegins = obj['recurrenceBegins'] as string;
        instance.recurrenceBegins = __raw_recurrenceBegins;
    }
    {
        const __raw_recurrenceEnds = obj['recurrenceEnds'] as RecurrenceEnd | null;
        if (__raw_recurrenceEnds === null) {
            instance.recurrenceEnds = null;
        } else {
            const __result = recurrenceEndDeserializeWithContext(__raw_recurrenceEnds, ctx);
            ctx.assignOrDefer(instance, 'recurrenceEnds', __result);
        }
    }
    {
        const __raw_cancelledInstances = obj['cancelledInstances'] as string[] | null;
        if (__raw_cancelledInstances === null) {
            instance.cancelledInstances = null;
        } else {
            instance.cancelledInstances = __raw_cancelledInstances;
        }
    }
    {
        const __raw_additionalInstances = obj['additionalInstances'] as string[] | null;
        if (__raw_additionalInstances === null) {
            instance.additionalInstances = null;
        } else {
            instance.additionalInstances = __raw_additionalInstances;
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as RecurrenceRule;
}
export function recurrenceRuleValidateField<K extends keyof RecurrenceRule>(
    field: K,
    value: RecurrenceRule[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function recurrenceRuleValidateFields(
    partial: Partial<RecurrenceRule>
): Array<{ field: string; message: string }> {
    return [];
}
export function recurrenceRuleHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'interval' in o &&
        'recurrenceBegins' in o &&
        'recurrenceEnds' in o &&
        'cancelledInstances' in o &&
        'additionalInstances' in o
    );
}
export function recurrenceRuleIs(obj: unknown): obj is RecurrenceRule {
    if (!recurrenceRuleHasShape(obj)) {
        return false;
    }
    const result = recurrenceRuleDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type RecurrenceRuleErrors = {
    _errors: Option<Array<string>>;
    interval: Option<Array<string>>;
    recurrenceBegins: Option<Array<string>>;
    recurrenceEnds: Option<Array<string>>;
    cancelledInstances: Option<Array<string>>;
    additionalInstances: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type RecurrenceRuleTainted = {
    interval: Option<boolean>;
    recurrenceBegins: Option<boolean>;
    recurrenceEnds: Option<boolean>;
    cancelledInstances: Option<boolean>;
    additionalInstances: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface RecurrenceRuleFieldControllers {
    readonly interval: FieldController<Interval>;
    readonly recurrenceBegins: FieldController<string>;
    readonly recurrenceEnds: FieldController<RecurrenceEnd | null>;
    readonly cancelledInstances: FieldController<string[] | null>;
    readonly additionalInstances: FieldController<string[] | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface RecurrenceRuleGigaform {
    readonly data: RecurrenceRule;
    readonly errors: RecurrenceRuleErrors;
    readonly tainted: RecurrenceRuleTainted;
    readonly fields: RecurrenceRuleFieldControllers;
    validate(): Result<RecurrenceRule, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<RecurrenceRule>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function recurrenceRuleCreateForm(
    overrides?: Partial<RecurrenceRule>
): RecurrenceRuleGigaform {
    let data = $state({ ...recurrenceRuleDefaultValue(), ...overrides });
    let errors = $state<RecurrenceRuleErrors>({
        _errors: Option.none(),
        interval: Option.none(),
        recurrenceBegins: Option.none(),
        recurrenceEnds: Option.none(),
        cancelledInstances: Option.none(),
        additionalInstances: Option.none()
    });
    let tainted = $state<RecurrenceRuleTainted>({
        interval: Option.none(),
        recurrenceBegins: Option.none(),
        recurrenceEnds: Option.none(),
        cancelledInstances: Option.none(),
        additionalInstances: Option.none()
    });
    const fields: RecurrenceRuleFieldControllers = {
        interval: {
            path: ['interval'] as const,
            name: 'interval',
            constraints: { required: true },
            get: () => data.interval,
            set: (value: Interval) => {
                data.interval = value;
            },
            transform: (value: Interval): Interval => value,
            getError: () => errors.interval,
            setError: (value: Option<Array<string>>) => {
                errors.interval = value;
            },
            getTainted: () => tainted.interval,
            setTainted: (value: Option<boolean>) => {
                tainted.interval = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = recurrenceRuleValidateField('interval', data.interval);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        recurrenceBegins: {
            path: ['recurrenceBegins'] as const,
            name: 'recurrenceBegins',
            constraints: { required: true },
            get: () => data.recurrenceBegins,
            set: (value: string) => {
                data.recurrenceBegins = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.recurrenceBegins,
            setError: (value: Option<Array<string>>) => {
                errors.recurrenceBegins = value;
            },
            getTainted: () => tainted.recurrenceBegins,
            setTainted: (value: Option<boolean>) => {
                tainted.recurrenceBegins = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = recurrenceRuleValidateField(
                    'recurrenceBegins',
                    data.recurrenceBegins
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        recurrenceEnds: {
            path: ['recurrenceEnds'] as const,
            name: 'recurrenceEnds',
            constraints: { required: true },
            get: () => data.recurrenceEnds,
            set: (value: RecurrenceEnd | null) => {
                data.recurrenceEnds = value;
            },
            transform: (value: RecurrenceEnd | null): RecurrenceEnd | null => value,
            getError: () => errors.recurrenceEnds,
            setError: (value: Option<Array<string>>) => {
                errors.recurrenceEnds = value;
            },
            getTainted: () => tainted.recurrenceEnds,
            setTainted: (value: Option<boolean>) => {
                tainted.recurrenceEnds = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = recurrenceRuleValidateField(
                    'recurrenceEnds',
                    data.recurrenceEnds
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        cancelledInstances: {
            path: ['cancelledInstances'] as const,
            name: 'cancelledInstances',
            constraints: { required: true },
            get: () => data.cancelledInstances,
            set: (value: string[] | null) => {
                data.cancelledInstances = value;
            },
            transform: (value: string[] | null): string[] | null => value,
            getError: () => errors.cancelledInstances,
            setError: (value: Option<Array<string>>) => {
                errors.cancelledInstances = value;
            },
            getTainted: () => tainted.cancelledInstances,
            setTainted: (value: Option<boolean>) => {
                tainted.cancelledInstances = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = recurrenceRuleValidateField(
                    'cancelledInstances',
                    data.cancelledInstances
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        additionalInstances: {
            path: ['additionalInstances'] as const,
            name: 'additionalInstances',
            constraints: { required: true },
            get: () => data.additionalInstances,
            set: (value: string[] | null) => {
                data.additionalInstances = value;
            },
            transform: (value: string[] | null): string[] | null => value,
            getError: () => errors.additionalInstances,
            setError: (value: Option<Array<string>>) => {
                errors.additionalInstances = value;
            },
            getTainted: () => tainted.additionalInstances,
            setTainted: (value: Option<boolean>) => {
                tainted.additionalInstances = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = recurrenceRuleValidateField(
                    'additionalInstances',
                    data.additionalInstances
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<RecurrenceRule, Array<{ field: string; message: string }>> {
        return recurrenceRuleDeserialize(data);
    }
    function reset(newOverrides?: Partial<RecurrenceRule>): void {
        data = { ...recurrenceRuleDefaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            interval: Option.none(),
            recurrenceBegins: Option.none(),
            recurrenceEnds: Option.none(),
            cancelledInstances: Option.none(),
            additionalInstances: Option.none()
        };
        tainted = {
            interval: Option.none(),
            recurrenceBegins: Option.none(),
            recurrenceEnds: Option.none(),
            cancelledInstances: Option.none(),
            additionalInstances: Option.none()
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
export function recurrenceRuleFromFormData(
    formData: FormData
): Result<RecurrenceRule, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const intervalObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('interval.')) {
                const fieldName = key.slice('interval.'.length);
                const parts = fieldName.split('.');
                let current = intervalObj;
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
        obj.interval = intervalObj;
    }
    obj.recurrenceBegins = formData.get('recurrenceBegins') ?? '';
    obj.recurrenceEnds = formData.get('recurrenceEnds') ?? '';
    obj.cancelledInstances = formData.get('cancelledInstances') ?? '';
    obj.additionalInstances = formData.get('additionalInstances') ?? '';
    return recurrenceRuleDeserialize(obj);
}

export const RecurrenceRule = {
    defaultValue: recurrenceRuleDefaultValue,
    serialize: recurrenceRuleSerialize,
    serializeWithContext: recurrenceRuleSerializeWithContext,
    deserialize: recurrenceRuleDeserialize,
    deserializeWithContext: recurrenceRuleDeserializeWithContext,
    validateFields: recurrenceRuleValidateFields,
    hasShape: recurrenceRuleHasShape,
    is: recurrenceRuleIs,
    createForm: recurrenceRuleCreateForm,
    fromFormData: recurrenceRuleFromFormData
} as const;
