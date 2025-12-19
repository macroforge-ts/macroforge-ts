import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option as __gf_Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface DailyRecurrenceRule {
    quantityOfDays: number;
}

export function dailyRecurrenceRuleDefaultValue(): DailyRecurrenceRule {
    return { quantityOfDays: 0 } as DailyRecurrenceRule;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function dailyRecurrenceRuleSerialize(
    value: DailyRecurrenceRule
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(dailyRecurrenceRuleSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function dailyRecurrenceRuleSerializeWithContext(
    value: DailyRecurrenceRule,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'DailyRecurrenceRule', __id };
    result['quantityOfDays'] = value.quantityOfDays;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function dailyRecurrenceRuleDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: DailyRecurrenceRule }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = dailyRecurrenceRuleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: '_root',
                        message:
                            'DailyRecurrenceRule.deserialize: root cannot be a forward reference'
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return { success: true, value: resultOrRef };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return { success: false, errors: e.errors };
        }
        const message = e instanceof Error ? e.message : String(e);
        return { success: false, errors: [{ field: '_root', message }] };
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function dailyRecurrenceRuleDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): DailyRecurrenceRule | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message: 'DailyRecurrenceRule.deserializeWithContext: expected an object'
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('quantityOfDays' in obj)) {
        errors.push({ field: 'quantityOfDays', message: 'missing required field' });
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
        const __raw_quantityOfDays = obj['quantityOfDays'] as number;
        instance.quantityOfDays = __raw_quantityOfDays;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as DailyRecurrenceRule;
}
export function dailyRecurrenceRuleValidateField<K extends keyof DailyRecurrenceRule>(
    _field: K,
    _value: DailyRecurrenceRule[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function dailyRecurrenceRuleValidateFields(
    _partial: Partial<DailyRecurrenceRule>
): Array<{ field: string; message: string }> {
    return [];
}
export function dailyRecurrenceRuleHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'quantityOfDays' in o;
}
export function dailyRecurrenceRuleIs(obj: unknown): obj is DailyRecurrenceRule {
    if (!dailyRecurrenceRuleHasShape(obj)) {
        return false;
    }
    const result = dailyRecurrenceRuleDeserialize(obj);
    return result.success;
}

/** Nested error structure matching the data shape */ export type DailyRecurrenceRuleErrors = {
    _errors: __gf_Option<Array<string>>;
    quantityOfDays: __gf_Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type DailyRecurrenceRuleTainted = {
    quantityOfDays: __gf_Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface DailyRecurrenceRuleFieldControllers {
    readonly quantityOfDays: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface DailyRecurrenceRuleGigaform {
    readonly data: DailyRecurrenceRule;
    readonly errors: DailyRecurrenceRuleErrors;
    readonly tainted: DailyRecurrenceRuleTainted;
    readonly fields: DailyRecurrenceRuleFieldControllers;
    validate(): Exit<DailyRecurrenceRule, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<DailyRecurrenceRule>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function dailyRecurrenceRuleCreateForm(
    overrides?: Partial<DailyRecurrenceRule>
): DailyRecurrenceRuleGigaform {
    let data = $state({ ...dailyRecurrenceRuleDefaultValue(), ...overrides });
    let errors = $state<DailyRecurrenceRuleErrors>({
        _errors: optionNone(),
        quantityOfDays: optionNone()
    });
    let tainted = $state<DailyRecurrenceRuleTainted>({ quantityOfDays: optionNone() });
    const fields: DailyRecurrenceRuleFieldControllers = {
        quantityOfDays: {
            path: ['quantityOfDays'] as const,
            name: 'quantityOfDays',
            constraints: { required: true },
            get: () => data.quantityOfDays,
            set: (value: number) => {
                data.quantityOfDays = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.quantityOfDays,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.quantityOfDays = value;
            },
            getTainted: () => tainted.quantityOfDays,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.quantityOfDays = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = dailyRecurrenceRuleValidateField(
                    'quantityOfDays',
                    data.quantityOfDays
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<DailyRecurrenceRule, Array<{ field: string; message: string }>> {
        return toExit(dailyRecurrenceRuleDeserialize(data));
    }
    function reset(newOverrides?: Partial<DailyRecurrenceRule>): void {
        data = { ...dailyRecurrenceRuleDefaultValue(), ...newOverrides };
        errors = { _errors: optionNone(), quantityOfDays: optionNone() };
        tainted = { quantityOfDays: optionNone() };
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
export function dailyRecurrenceRuleFromFormData(
    formData: FormData
): Exit<DailyRecurrenceRule, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const quantityOfDaysStr = formData.get('quantityOfDays');
        obj.quantityOfDays = quantityOfDaysStr ? parseFloat(quantityOfDaysStr as string) : 0;
        if (obj.quantityOfDays !== undefined && isNaN(obj.quantityOfDays as number))
            obj.quantityOfDays = 0;
    }
    return toExit(dailyRecurrenceRuleDeserialize(obj));
}

export const DailyRecurrenceRule = {
    defaultValue: dailyRecurrenceRuleDefaultValue,
    serialize: dailyRecurrenceRuleSerialize,
    serializeWithContext: dailyRecurrenceRuleSerializeWithContext,
    deserialize: dailyRecurrenceRuleDeserialize,
    deserializeWithContext: dailyRecurrenceRuleDeserializeWithContext,
    validateFields: dailyRecurrenceRuleValidateFields,
    hasShape: dailyRecurrenceRuleHasShape,
    is: dailyRecurrenceRuleIs,
    createForm: dailyRecurrenceRuleCreateForm,
    fromFormData: dailyRecurrenceRuleFromFormData
} as const;
