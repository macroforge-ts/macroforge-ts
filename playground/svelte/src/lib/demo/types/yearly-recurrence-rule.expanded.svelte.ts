import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { Result } from 'macroforge/reexports';
import { Option } from 'macroforge/reexports';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface YearlyRecurrenceRule {
    quantityOfYears: number;
}

export function yearlyRecurrenceRuleDefaultValue(): YearlyRecurrenceRule {
    return { quantityOfYears: 0 } as YearlyRecurrenceRule;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function yearlyRecurrenceRuleSerialize(
    value: YearlyRecurrenceRule
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(yearlyRecurrenceRuleSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function yearlyRecurrenceRuleSerializeWithContext(
    value: YearlyRecurrenceRule,
    ctx: __mf_SerializeContext
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

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function yearlyRecurrenceRuleDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, YearlyRecurrenceRule> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = yearlyRecurrenceRuleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'YearlyRecurrenceRule.deserialize: root cannot be a forward reference'
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
export function yearlyRecurrenceRuleDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): YearlyRecurrenceRule | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message: 'YearlyRecurrenceRule.deserializeWithContext: expected an object'
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('quantityOfYears' in obj)) {
        errors.push({ field: 'quantityOfYears', message: 'missing required field' });
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
        const __raw_quantityOfYears = obj['quantityOfYears'] as number;
        instance.quantityOfYears = __raw_quantityOfYears;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as YearlyRecurrenceRule;
}
export function yearlyRecurrenceRuleValidateField<K extends keyof YearlyRecurrenceRule>(
    field: K,
    value: YearlyRecurrenceRule[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function yearlyRecurrenceRuleValidateFields(
    partial: Partial<YearlyRecurrenceRule>
): Array<{ field: string; message: string }> {
    return [];
}
export function yearlyRecurrenceRuleHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'quantityOfYears' in o;
}
export function yearlyRecurrenceRuleIs(obj: unknown): obj is YearlyRecurrenceRule {
    if (!yearlyRecurrenceRuleHasShape(obj)) {
        return false;
    }
    const result = yearlyRecurrenceRuleDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type YearlyRecurrenceRuleErrors = {
    _errors: Option<Array<string>>;
    quantityOfYears: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type YearlyRecurrenceRuleTainted = {
    quantityOfYears: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface YearlyRecurrenceRuleFieldControllers {
    readonly quantityOfYears: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface YearlyRecurrenceRuleGigaform {
    readonly data: YearlyRecurrenceRule;
    readonly errors: YearlyRecurrenceRuleErrors;
    readonly tainted: YearlyRecurrenceRuleTainted;
    readonly fields: YearlyRecurrenceRuleFieldControllers;
    validate(): Result<YearlyRecurrenceRule, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<YearlyRecurrenceRule>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function yearlyRecurrenceRuleCreateForm(
    overrides?: Partial<YearlyRecurrenceRule>
): YearlyRecurrenceRuleGigaform {
    let data = $state({ ...yearlyRecurrenceRuleDefaultValue(), ...overrides });
    let errors = $state<YearlyRecurrenceRuleErrors>({
        _errors: Option.none(),
        quantityOfYears: Option.none()
    });
    let tainted = $state<YearlyRecurrenceRuleTainted>({ quantityOfYears: Option.none() });
    const fields: YearlyRecurrenceRuleFieldControllers = {
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
                const fieldErrors = yearlyRecurrenceRuleValidateField(
                    'quantityOfYears',
                    data.quantityOfYears
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<YearlyRecurrenceRule, Array<{ field: string; message: string }>> {
        return yearlyRecurrenceRuleDeserialize(data);
    }
    function reset(newOverrides?: Partial<YearlyRecurrenceRule>): void {
        data = { ...yearlyRecurrenceRuleDefaultValue(), ...newOverrides };
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */
export function yearlyRecurrenceRuleFromFormData(
    formData: FormData
): Result<YearlyRecurrenceRule, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const quantityOfYearsStr = formData.get('quantityOfYears');
        obj.quantityOfYears = quantityOfYearsStr ? parseFloat(quantityOfYearsStr as string) : 0;
        if (obj.quantityOfYears !== undefined && isNaN(obj.quantityOfYears as number))
            obj.quantityOfYears = 0;
    }
    return yearlyRecurrenceRuleDeserialize(obj);
}

export const YearlyRecurrenceRule = {
    defaultValue: yearlyRecurrenceRuleDefaultValue,
    serialize: yearlyRecurrenceRuleSerialize,
    serializeWithContext: yearlyRecurrenceRuleSerializeWithContext,
    deserialize: yearlyRecurrenceRuleDeserialize,
    deserializeWithContext: yearlyRecurrenceRuleDeserializeWithContext,
    validateFields: yearlyRecurrenceRuleValidateFields,
    hasShape: yearlyRecurrenceRuleHasShape,
    is: yearlyRecurrenceRuleIs,
    createForm: yearlyRecurrenceRuleCreateForm,
    fromFormData: yearlyRecurrenceRuleFromFormData
} as const;
