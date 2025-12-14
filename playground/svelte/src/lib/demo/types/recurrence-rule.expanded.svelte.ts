import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import { Interval } from './interval.svelte';
import { RecurrenceEnd } from './recurrence-end.svelte';

export interface RecurrenceRule {
    interval: Interval;
    recurrenceBegins: string;
    recurrenceEnds: RecurrenceEnd | null;
    cancelledInstances: string[] | null;
    additionalInstances: string[] | null;
}

export function defaultValueRecurrenceRule(): RecurrenceRule {
    return {
        interval: Interval.defaultValue(),
        recurrenceBegins: '',
        recurrenceEnds: null,
        cancelledInstances: null,
        additionalInstances: null
    } as RecurrenceRule;
}

export function toStringifiedJSONRecurrenceRule(value: RecurrenceRule): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeRecurrenceRule(value, ctx));
}
export function toObjectRecurrenceRule(value: RecurrenceRule): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeRecurrenceRule(value, ctx);
}
export function __serializeRecurrenceRule(
    value: RecurrenceRule,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'RecurrenceRule', __id };
    result['interval'] =
        typeof (value.interval as any)?.__serialize === 'function'
            ? (value.interval as any).__serialize(ctx)
            : value.interval;
    result['recurrenceBegins'] = value.recurrenceBegins;
    if (value.recurrenceEnds !== null) {
        result['recurrenceEnds'] =
            typeof (value.recurrenceEnds as any)?.__serialize === 'function'
                ? (value.recurrenceEnds as any).__serialize(ctx)
                : value.recurrenceEnds;
    } else {
        result['recurrenceEnds'] = null;
    }
    if (value.cancelledInstances !== null) {
        result['cancelledInstances'] =
            typeof (value.cancelledInstances as any)?.__serialize === 'function'
                ? (value.cancelledInstances as any).__serialize(ctx)
                : value.cancelledInstances;
    } else {
        result['cancelledInstances'] = null;
    }
    if (value.additionalInstances !== null) {
        result['additionalInstances'] =
            typeof (value.additionalInstances as any)?.__serialize === 'function'
                ? (value.additionalInstances as any).__serialize(ctx)
                : value.additionalInstances;
    } else {
        result['additionalInstances'] = null;
    }
    return result;
}

export function fromStringifiedJSONRecurrenceRule(
    json: string,
    opts?: DeserializeOptions
): Result<RecurrenceRule, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectRecurrenceRule(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectRecurrenceRule(
    obj: unknown,
    opts?: DeserializeOptions
): Result<RecurrenceRule, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeRecurrenceRule(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'RecurrenceRule.fromObject: root cannot be a forward reference'
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
export function __deserializeRecurrenceRule(
    value: any,
    ctx: DeserializeContext
): RecurrenceRule | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'RecurrenceRule.__deserialize: expected an object' }
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
        throw new DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_interval = obj['interval'] as Interval;
        {
            const __result = Interval.__deserialize(__raw_interval, ctx);
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
            const __result = RecurrenceEnd.__deserialize(__raw_recurrenceEnds, ctx);
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
        throw new DeserializeError(errors);
    }
    return instance as RecurrenceRule;
}
export function validateFieldRecurrenceRule<K extends keyof RecurrenceRule>(
    field: K,
    value: RecurrenceRule[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsRecurrenceRule(
    partial: Partial<RecurrenceRule>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeRecurrenceRule(obj: unknown): boolean {
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
export function isRecurrenceRule(obj: unknown): obj is RecurrenceRule {
    if (!hasShapeRecurrenceRule(obj)) {
        return false;
    }
    const result = fromObjectRecurrenceRule(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsRecurrenceRule = {
    _errors: Option<Array<string>>;
    interval: Option<Array<string>>;
    recurrenceBegins: Option<Array<string>>;
    recurrenceEnds: Option<Array<string>>;
    cancelledInstances: Option<Array<string>>;
    additionalInstances: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedRecurrenceRule = {
    interval: Option<boolean>;
    recurrenceBegins: Option<boolean>;
    recurrenceEnds: Option<boolean>;
    cancelledInstances: Option<boolean>;
    additionalInstances: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersRecurrenceRule {
    readonly interval: FieldController<Interval>;
    readonly recurrenceBegins: FieldController<string>;
    readonly recurrenceEnds: FieldController<RecurrenceEnd | null>;
    readonly cancelledInstances: FieldController<string[] | null>;
    readonly additionalInstances: FieldController<string[] | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformRecurrenceRule {
    readonly data: RecurrenceRule;
    readonly errors: ErrorsRecurrenceRule;
    readonly tainted: TaintedRecurrenceRule;
    readonly fields: FieldControllersRecurrenceRule;
    validate(): Result<RecurrenceRule, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<RecurrenceRule>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormRecurrenceRule(
    overrides?: Partial<RecurrenceRule>
): GigaformRecurrenceRule {
    let data = $state({ ...RecurrenceRule.defaultValue(), ...overrides });
    let errors = $state<ErrorsRecurrenceRule>({
        _errors: Option.none(),
        interval: Option.none(),
        recurrenceBegins: Option.none(),
        recurrenceEnds: Option.none(),
        cancelledInstances: Option.none(),
        additionalInstances: Option.none()
    });
    let tainted = $state<TaintedRecurrenceRule>({
        interval: Option.none(),
        recurrenceBegins: Option.none(),
        recurrenceEnds: Option.none(),
        cancelledInstances: Option.none(),
        additionalInstances: Option.none()
    });
    const fields: FieldControllersRecurrenceRule = {
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
                const fieldErrors = RecurrenceRule.validateField('interval', data.interval);
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
                const fieldErrors = RecurrenceRule.validateField(
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
                const fieldErrors = RecurrenceRule.validateField(
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
                const fieldErrors = RecurrenceRule.validateField(
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
                const fieldErrors = RecurrenceRule.validateField(
                    'additionalInstances',
                    data.additionalInstances
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<RecurrenceRule, Array<{ field: string; message: string }>> {
        return RecurrenceRule.fromObject(data);
    }
    function reset(newOverrides?: Partial<RecurrenceRule>): void {
        data = { ...RecurrenceRule.defaultValue(), ...newOverrides };
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function fromFormDataRecurrenceRule(
    formData: FormData
): Result<RecurrenceRule, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        // Collect nested object fields with prefix "interval."
        const intervalObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('interval.')) {
                const fieldName = key.slice('interval.'.length);
                // Handle deeper nesting by splitting on dots
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
    return RecurrenceRule.fromStringifiedJSON(JSON.stringify(obj));
}
