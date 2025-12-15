import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface YearlyRecurrenceRule {
    quantityOfYears: number;
}

export function defaultValueYearlyRecurrenceRule(): YearlyRecurrenceRule {
    return { quantityOfYears: 0 } as YearlyRecurrenceRule;
}

export function toStringifiedJSONYearlyRecurrenceRule(value: YearlyRecurrenceRule): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeYearlyRecurrenceRule(value, ctx));
}
export function toObjectYearlyRecurrenceRule(value: YearlyRecurrenceRule): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeYearlyRecurrenceRule(value, ctx);
}
export function __serializeYearlyRecurrenceRule(
    value: YearlyRecurrenceRule,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'YearlyRecurrenceRule', __id };
    result['quantityOfYears'] = value.quantityOfYears;
    return result;
}

export function fromStringifiedJSONYearlyRecurrenceRule(
    json: string,
    opts?: DeserializeOptions
): Result<YearlyRecurrenceRule, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectYearlyRecurrenceRule(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectYearlyRecurrenceRule(
    obj: unknown,
    opts?: DeserializeOptions
): Result<YearlyRecurrenceRule, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeYearlyRecurrenceRule(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'YearlyRecurrenceRule.fromObject: root cannot be a forward reference'
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
export function __deserializeYearlyRecurrenceRule(
    value: any,
    ctx: DeserializeContext
): YearlyRecurrenceRule | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'YearlyRecurrenceRule.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('quantityOfYears' in obj)) {
        errors.push({ field: 'quantityOfYears', message: 'missing required field' });
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
        const __raw_quantityOfYears = obj['quantityOfYears'] as number;
        instance.quantityOfYears = __raw_quantityOfYears;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as YearlyRecurrenceRule;
}
export function validateFieldYearlyRecurrenceRule<K extends keyof YearlyRecurrenceRule>(
    field: K,
    value: YearlyRecurrenceRule[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsYearlyRecurrenceRule(
    partial: Partial<YearlyRecurrenceRule>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeYearlyRecurrenceRule(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'quantityOfYears' in o;
}
export function isYearlyRecurrenceRule(obj: unknown): obj is YearlyRecurrenceRule {
    if (!hasShapeYearlyRecurrenceRule(obj)) {
        return false;
    }
    const result = fromObjectYearlyRecurrenceRule(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsYearlyRecurrenceRule = {
    _errors: Option<Array<string>>;
    quantityOfYears: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedYearlyRecurrenceRule = {
    quantityOfYears: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersYearlyRecurrenceRule {
    readonly quantityOfYears: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformYearlyRecurrenceRule {
    readonly data: YearlyRecurrenceRule;
    readonly errors: ErrorsYearlyRecurrenceRule;
    readonly tainted: TaintedYearlyRecurrenceRule;
    readonly fields: FieldControllersYearlyRecurrenceRule;
    validate(): Result<YearlyRecurrenceRule, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<YearlyRecurrenceRule>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormYearlyRecurrenceRule(
    overrides?: Partial<YearlyRecurrenceRule>
): GigaformYearlyRecurrenceRule {
    let data = $state({ ...defaultValueYearlyRecurrenceRule(), ...overrides });
    let errors = $state<ErrorsYearlyRecurrenceRule>({
        _errors: Option.none(),
        quantityOfYears: Option.none()
    });
    let tainted = $state<TaintedYearlyRecurrenceRule>({ quantityOfYears: Option.none() });
    const fields: FieldControllersYearlyRecurrenceRule = {
        quantityOfYears: {
            path: ['quantityOfYears'] as const,
            name: 'quantityOfYears',
            constraints: { required: true },

            get: () => data.quantityOfYears,
            set: (value: number) => {
                data.quantityOfYears = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.quantityOfYears,
            setError: (value: Option<Array<string>>) => {
                errors.quantityOfYears = value;
            },
            getTainted: () => tainted.quantityOfYears,
            setTainted: (value: Option<boolean>) => {
                tainted.quantityOfYears = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldYearlyRecurrenceRule(
                    'quantityOfYears',
                    data.quantityOfYears
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<YearlyRecurrenceRule, Array<{ field: string; message: string }>> {
        return fromObjectYearlyRecurrenceRule(data);
    }
    function reset(newOverrides?: Partial<YearlyRecurrenceRule>): void {
        data = { ...defaultValueYearlyRecurrenceRule(), ...newOverrides };
        errors = { _errors: Option.none(), quantityOfYears: Option.none() };
        tainted = { quantityOfYears: Option.none() };
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
export function fromFormDataYearlyRecurrenceRule(
    formData: FormData
): Result<YearlyRecurrenceRule, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const quantityOfYearsStr = formData.get('quantityOfYears');
        obj.quantityOfYears = quantityOfYearsStr ? parseFloat(quantityOfYearsStr as string) : 0;
        if (obj.quantityOfYears !== undefined && isNaN(obj.quantityOfYears as number))
            obj.quantityOfYears = 0;
    }
    return fromStringifiedJSONYearlyRecurrenceRule(JSON.stringify(obj));
}
