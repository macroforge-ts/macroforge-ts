import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface MonthlyRecurrenceRule {
    quantityOfMonths: number;
    day: number;

    name: string;
}

export function monthlyRecurrenceRuleDefaultValue(): MonthlyRecurrenceRule {
    return { quantityOfMonths: 0, day: 0, name: '' } as MonthlyRecurrenceRule;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function monthlyRecurrenceRuleSerialize(
    value: MonthlyRecurrenceRule
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(monthlyRecurrenceRuleSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function monthlyRecurrenceRuleSerializeWithContext(
    value: MonthlyRecurrenceRule,
    ctx: __mf_SerializeContext
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

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function monthlyRecurrenceRuleDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, MonthlyRecurrenceRule> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = monthlyRecurrenceRuleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'MonthlyRecurrenceRule.deserialize: root cannot be a forward reference'
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
export function monthlyRecurrenceRuleDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): MonthlyRecurrenceRule | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message: 'MonthlyRecurrenceRule.deserializeWithContext: expected an object'
            }
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
        throw new __mf_DeserializeError(errors);
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
        throw new __mf_DeserializeError(errors);
    }
    return instance as MonthlyRecurrenceRule;
}
export function monthlyRecurrenceRuleValidateField<K extends keyof MonthlyRecurrenceRule>(
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
export function monthlyRecurrenceRuleValidateFields(
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
export function monthlyRecurrenceRuleHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'quantityOfMonths' in o && 'day' in o && 'name' in o;
}
export function monthlyRecurrenceRuleIs(obj: unknown): obj is MonthlyRecurrenceRule {
    if (!monthlyRecurrenceRuleHasShape(obj)) {
        return false;
    }
    const result = monthlyRecurrenceRuleDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type MonthlyRecurrenceRuleErrors = {
    _errors: Option<Array<string>>;
    quantityOfMonths: Option<Array<string>>;
    day: Option<Array<string>>;
    name: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type MonthlyRecurrenceRuleTainted = {
    quantityOfMonths: Option<boolean>;
    day: Option<boolean>;
    name: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface MonthlyRecurrenceRuleFieldControllers {
    readonly quantityOfMonths: FieldController<number>;
    readonly day: FieldController<number>;
    readonly name: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface MonthlyRecurrenceRuleGigaform {
    readonly data: MonthlyRecurrenceRule;
    readonly errors: MonthlyRecurrenceRuleErrors;
    readonly tainted: MonthlyRecurrenceRuleTainted;
    readonly fields: MonthlyRecurrenceRuleFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, MonthlyRecurrenceRule>;
    reset(overrides?: Partial<MonthlyRecurrenceRule>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function monthlyRecurrenceRuleCreateForm(
    overrides?: Partial<MonthlyRecurrenceRule>
): MonthlyRecurrenceRuleGigaform {
    let data = $state({ ...monthlyRecurrenceRuleDefaultValue(), ...overrides });
    let errors = $state<MonthlyRecurrenceRuleErrors>({
        _errors: optionNone(),
        quantityOfMonths: optionNone(),
        day: optionNone(),
        name: optionNone()
    });
    let tainted = $state<MonthlyRecurrenceRuleTainted>({
        quantityOfMonths: optionNone(),
        day: optionNone(),
        name: optionNone()
    });
    const fields: MonthlyRecurrenceRuleFieldControllers = {
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
                const fieldErrors = monthlyRecurrenceRuleValidateField(
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
                const fieldErrors = monthlyRecurrenceRuleValidateField('day', data.day);
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
                const fieldErrors = monthlyRecurrenceRuleValidateField('name', data.name);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, MonthlyRecurrenceRule> {
        return monthlyRecurrenceRuleDeserialize(data);
    }
    function reset(newOverrides?: Partial<MonthlyRecurrenceRule>): void {
        data = { ...monthlyRecurrenceRuleDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            quantityOfMonths: optionNone(),
            day: optionNone(),
            name: optionNone()
        };
        tainted = { quantityOfMonths: optionNone(), day: optionNone(), name: optionNone() };
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
export function monthlyRecurrenceRuleFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, MonthlyRecurrenceRule> {
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
    return monthlyRecurrenceRuleDeserialize(obj);
}

export const MonthlyRecurrenceRule = {
    defaultValue: monthlyRecurrenceRuleDefaultValue,
    serialize: monthlyRecurrenceRuleSerialize,
    serializeWithContext: monthlyRecurrenceRuleSerializeWithContext,
    deserialize: monthlyRecurrenceRuleDeserialize,
    deserializeWithContext: monthlyRecurrenceRuleDeserializeWithContext,
    validateFields: monthlyRecurrenceRuleValidateFields,
    hasShape: monthlyRecurrenceRuleHasShape,
    is: monthlyRecurrenceRuleIs,
    createForm: monthlyRecurrenceRuleCreateForm,
    fromFormData: monthlyRecurrenceRuleFromFormData
} as const;
