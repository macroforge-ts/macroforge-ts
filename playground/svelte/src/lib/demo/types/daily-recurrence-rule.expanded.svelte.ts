import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
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
    const ctx = SerializeContext.create();
    return JSON.stringify(dailyRecurrenceRuleSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function dailyRecurrenceRuleSerializeWithContext(
    value: DailyRecurrenceRule,
    ctx: SerializeContext
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
    opts?: DeserializeOptions
): Result<DailyRecurrenceRule, Array<{ field: string; message: string }>> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = dailyRecurrenceRuleDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'DailyRecurrenceRule.deserialize: root cannot be a forward reference'
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
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function dailyRecurrenceRuleDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): DailyRecurrenceRule | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
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
        throw new DeserializeError(errors);
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
        throw new DeserializeError(errors);
    }
    return instance as DailyRecurrenceRule;
}
export function dailyRecurrenceRuleValidateField<K extends keyof DailyRecurrenceRule>(
    field: K,
    value: DailyRecurrenceRule[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function dailyRecurrenceRuleValidateFields(
    partial: Partial<DailyRecurrenceRule>
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
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type DailyRecurrenceRuleErrors = {
    _errors: Option<Array<string>>;
    quantityOfDays: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type DailyRecurrenceRuleTainted = {
    quantityOfDays: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface DailyRecurrenceRuleFieldControllers {
    readonly quantityOfDays: FieldController<number>;
} /** Gigaform instance containing reactive state and field controllers */
export interface DailyRecurrenceRuleGigaform {
    readonly data: DailyRecurrenceRule;
    readonly errors: DailyRecurrenceRuleErrors;
    readonly tainted: DailyRecurrenceRuleTainted;
    readonly fields: DailyRecurrenceRuleFieldControllers;
    validate(): Result<DailyRecurrenceRule, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<DailyRecurrenceRule>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function dailyRecurrenceRuleCreateForm(
    overrides?: Partial<DailyRecurrenceRule>
): DailyRecurrenceRuleGigaform {
    let data = $state({ ...dailyRecurrenceRuleDefaultValue(), ...overrides });
    let errors = $state<DailyRecurrenceRuleErrors>({
        _errors: Option.none(),
        quantityOfDays: Option.none()
    });
    let tainted = $state<DailyRecurrenceRuleTainted>({ quantityOfDays: Option.none() });
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
            setError: (value: Option<Array<string>>) => {
                errors.quantityOfDays = value;
            },
            getTainted: () => tainted.quantityOfDays,
            setTainted: (value: Option<boolean>) => {
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
    function validate(): Result<DailyRecurrenceRule, Array<{ field: string; message: string }>> {
        return dailyRecurrenceRuleFromObject(data);
    }
    function reset(newOverrides?: Partial<DailyRecurrenceRule>): void {
        data = { ...dailyRecurrenceRuleDefaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), quantityOfDays: Option.none() };
        tainted = { quantityOfDays: Option.none() };
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
export function dailyRecurrenceRuleFromFormData(
    formData: FormData
): Result<DailyRecurrenceRule, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const quantityOfDaysStr = formData.get('quantityOfDays');
        obj.quantityOfDays = quantityOfDaysStr ? parseFloat(quantityOfDaysStr as string) : 0;
        if (obj.quantityOfDays !== undefined && isNaN(obj.quantityOfDays as number))
            obj.quantityOfDays = 0;
    }
    return dailyRecurrenceRuleFromStringifiedJSON(JSON.stringify(obj));
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
