import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface MonthlyRecurrenceRule {
    quantityOfMonths: number;
    day: number;

    name: string;
}

export function defaultValueMonthlyRecurrenceRule(): MonthlyRecurrenceRule {
    return { quantityOfMonths: 0, day: 0, name: '' } as MonthlyRecurrenceRule;
}

export function toStringifiedJSONMonthlyRecurrenceRule(value: MonthlyRecurrenceRule): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeMonthlyRecurrenceRule(value, ctx));
}
export function toObjectMonthlyRecurrenceRule(
    value: MonthlyRecurrenceRule
): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeMonthlyRecurrenceRule(value, ctx);
}
export function __serializeMonthlyRecurrenceRule(
    value: MonthlyRecurrenceRule,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'MonthlyRecurrenceRule', __id };
    result['quantityOfMonths'] = value.quantityOfMonths;
    result['day'] = value.day;
    result['name'] = value.name;
    return result;
}

export function fromStringifiedJSONMonthlyRecurrenceRule(
    json: string,
    opts?: DeserializeOptions
): Result<MonthlyRecurrenceRule, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectMonthlyRecurrenceRule(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectMonthlyRecurrenceRule(
    obj: unknown,
    opts?: DeserializeOptions
): Result<MonthlyRecurrenceRule, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeMonthlyRecurrenceRule(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'MonthlyRecurrenceRule.fromObject: root cannot be a forward reference'
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
export function __deserializeMonthlyRecurrenceRule(
    value: any,
    ctx: DeserializeContext
): MonthlyRecurrenceRule | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'MonthlyRecurrenceRule.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('quantityOfMonths' in obj)) {
        errors.push({ field: 'quantityOfMonths', message: 'missing required field' });
    }
    if (!('day' in obj)) {
        errors.push({ field: 'day', message: 'missing required field' });
    }
    if (!('name' in obj)) {
        errors.push({ field: 'name', message: 'missing required field' });
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
        const __raw_quantityOfMonths = obj['quantityOfMonths'] as number;
        instance.quantityOfMonths = __raw_quantityOfMonths;
    }
    {
        const __raw_day = obj['day'] as number;
        instance.day = __raw_day;
    }
    {
        const __raw_name = obj['name'] as string;
        if (__raw_name.length === 0) {
            errors.push({ field: 'name', message: 'must not be empty' });
        }
        instance.name = __raw_name;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as MonthlyRecurrenceRule;
}
export function validateFieldMonthlyRecurrenceRule<K extends keyof MonthlyRecurrenceRule>(
    field: K,
    value: MonthlyRecurrenceRule[K]
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
export function validateFieldsMonthlyRecurrenceRule(
    partial: Partial<MonthlyRecurrenceRule>
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
export function hasShapeMonthlyRecurrenceRule(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'quantityOfMonths' in o && 'day' in o && 'name' in o;
}
export function isMonthlyRecurrenceRule(obj: unknown): obj is MonthlyRecurrenceRule {
    if (!hasShapeMonthlyRecurrenceRule(obj)) {
        return false;
    }
    const result = fromObjectMonthlyRecurrenceRule(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsMonthlyRecurrenceRule = {
    _errors: Option<Array<string>>;
    quantityOfMonths: Option<Array<string>>;
    day: Option<Array<string>>;
    name: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedMonthlyRecurrenceRule = {
    quantityOfMonths: Option<boolean>;
    day: Option<boolean>;
    name: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersMonthlyRecurrenceRule {
    readonly quantityOfMonths: FieldController<number>;
    readonly day: FieldController<number>;
    readonly name: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformMonthlyRecurrenceRule {
    readonly data: MonthlyRecurrenceRule;
    readonly errors: ErrorsMonthlyRecurrenceRule;
    readonly tainted: TaintedMonthlyRecurrenceRule;
    readonly fields: FieldControllersMonthlyRecurrenceRule;
    validate(): Result<MonthlyRecurrenceRule, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<MonthlyRecurrenceRule>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormMonthlyRecurrenceRule(
    overrides?: Partial<MonthlyRecurrenceRule>
): GigaformMonthlyRecurrenceRule {
    let data = $state({ ...defaultValueMonthlyRecurrenceRule(), ...overrides });
    let errors = $state<ErrorsMonthlyRecurrenceRule>({
        _errors: Option.none(),
        quantityOfMonths: Option.none(),
        day: Option.none(),
        name: Option.none()
    });
    let tainted = $state<TaintedMonthlyRecurrenceRule>({
        quantityOfMonths: Option.none(),
        day: Option.none(),
        name: Option.none()
    });
    const fields: FieldControllersMonthlyRecurrenceRule = {
        quantityOfMonths: {
            path: ['quantityOfMonths'] as const,
            name: 'quantityOfMonths',
            constraints: { required: true },

            get: () => data.quantityOfMonths,
            set: (value: number) => {
                data.quantityOfMonths = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.quantityOfMonths,
            setError: (value: Option<Array<string>>) => {
                errors.quantityOfMonths = value;
            },
            getTainted: () => tainted.quantityOfMonths,
            setTainted: (value: Option<boolean>) => {
                tainted.quantityOfMonths = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldMonthlyRecurrenceRule(
                    'quantityOfMonths',
                    data.quantityOfMonths
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        day: {
            path: ['day'] as const,
            name: 'day',
            constraints: { required: true },

            get: () => data.day,
            set: (value: number) => {
                data.day = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.day,
            setError: (value: Option<Array<string>>) => {
                errors.day = value;
            },
            getTainted: () => tainted.day,
            setTainted: (value: Option<boolean>) => {
                tainted.day = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldMonthlyRecurrenceRule('day', data.day);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
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
                const fieldErrors = validateFieldMonthlyRecurrenceRule('name', data.name);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<MonthlyRecurrenceRule, Array<{ field: string; message: string }>> {
        return fromObjectMonthlyRecurrenceRule(data);
    }
    function reset(newOverrides?: Partial<MonthlyRecurrenceRule>): void {
        data = { ...defaultValueMonthlyRecurrenceRule(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            quantityOfMonths: Option.none(),
            day: Option.none(),
            name: Option.none()
        };
        tainted = { quantityOfMonths: Option.none(), day: Option.none(), name: Option.none() };
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
export function fromFormDataMonthlyRecurrenceRule(
    formData: FormData
): Result<MonthlyRecurrenceRule, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const quantityOfMonthsStr = formData.get('quantityOfMonths');
        obj.quantityOfMonths = quantityOfMonthsStr ? parseFloat(quantityOfMonthsStr as string) : 0;
        if (obj.quantityOfMonths !== undefined && isNaN(obj.quantityOfMonths as number))
            obj.quantityOfMonths = 0;
    }
    {
        const dayStr = formData.get('day');
        obj.day = dayStr ? parseFloat(dayStr as string) : 0;
        if (obj.day !== undefined && isNaN(obj.day as number)) obj.day = 0;
    }
    obj.name = formData.get('name') ?? '';
    return fromStringifiedJSONMonthlyRecurrenceRule(JSON.stringify(obj));
}
