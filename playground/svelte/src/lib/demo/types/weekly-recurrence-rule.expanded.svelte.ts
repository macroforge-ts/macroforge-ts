import { SerializeContext } from 'macroforge/serde';
import { weekdaySerializeWithContext } from './weekday.svelte';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
import type { ArrayFieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { Weekday } from './weekday.svelte';

export interface WeeklyRecurrenceRule {
    quantityOfWeeks: number;
    weekdays: Weekday[];
}

export function weeklyRecurrenceRuleDefaultValue(): WeeklyRecurrenceRule {
    return { quantityOfWeeks: 0, weekdays: [] } as WeeklyRecurrenceRule;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function weeklyRecurrenceRuleSerialize(
    value: WeeklyRecurrenceRule
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(weeklyRecurrenceRuleSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function weeklyRecurrenceRuleSerializeWithContext(
    value: WeeklyRecurrenceRule,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'WeeklyRecurrenceRule', __id };
    result['quantityOfWeeks'] = value.quantityOfWeeks;
    result['weekdays'] = value.weekdays.map((item) => weekdaySerializeWithContext(item, ctx));
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function weeklyRecurrenceRuleDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Result<WeeklyRecurrenceRule, Array<{ field: string; message: string }>> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = weeklyRecurrenceRuleDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'WeeklyRecurrenceRule.deserialize: root cannot be a forward reference'
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
export function weeklyRecurrenceRuleDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): WeeklyRecurrenceRule | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message: 'WeeklyRecurrenceRule.deserializeWithContext: expected an object'
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('quantityOfWeeks' in obj)) {
        errors.push({ field: 'quantityOfWeeks', message: 'missing required field' });
    }
    if (!('weekdays' in obj)) {
        errors.push({ field: 'weekdays', message: 'missing required field' });
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
        const __raw_quantityOfWeeks = obj['quantityOfWeeks'] as number;
        instance.quantityOfWeeks = __raw_quantityOfWeeks;
    }
    {
        const __raw_weekdays = obj['weekdays'] as Weekday[];
        if (Array.isArray(__raw_weekdays)) {
            instance.weekdays = __raw_weekdays as Weekday[];
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as WeeklyRecurrenceRule;
}
export function weeklyRecurrenceRuleValidateField<K extends keyof WeeklyRecurrenceRule>(
    field: K,
    value: WeeklyRecurrenceRule[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function weeklyRecurrenceRuleValidateFields(
    partial: Partial<WeeklyRecurrenceRule>
): Array<{ field: string; message: string }> {
    return [];
}
export function weeklyRecurrenceRuleHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'quantityOfWeeks' in o && 'weekdays' in o;
}
export function weeklyRecurrenceRuleIs(obj: unknown): obj is WeeklyRecurrenceRule {
    if (!weeklyRecurrenceRuleHasShape(obj)) {
        return false;
    }
    const result = weeklyRecurrenceRuleDeserialize(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type WeeklyRecurrenceRuleErrors = {
    _errors: Option<Array<string>>;
    quantityOfWeeks: Option<Array<string>>;
    weekdays: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type WeeklyRecurrenceRuleTainted = {
    quantityOfWeeks: Option<boolean>;
    weekdays: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface WeeklyRecurrenceRuleFieldControllers {
    readonly quantityOfWeeks: FieldController<number>;
    readonly weekdays: ArrayFieldController<Weekday>;
} /** Gigaform instance containing reactive state and field controllers */
export interface WeeklyRecurrenceRuleGigaform {
    readonly data: WeeklyRecurrenceRule;
    readonly errors: WeeklyRecurrenceRuleErrors;
    readonly tainted: WeeklyRecurrenceRuleTainted;
    readonly fields: WeeklyRecurrenceRuleFieldControllers;
    validate(): Result<WeeklyRecurrenceRule, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<WeeklyRecurrenceRule>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function weeklyRecurrenceRuleCreateForm(
    overrides?: Partial<WeeklyRecurrenceRule>
): WeeklyRecurrenceRuleGigaform {
    let data = $state({ ...weeklyRecurrenceRuleDefaultValue(), ...overrides });
    let errors = $state<WeeklyRecurrenceRuleErrors>({
        _errors: Option.none(),
        quantityOfWeeks: Option.none(),
        weekdays: Option.none()
    });
    let tainted = $state<WeeklyRecurrenceRuleTainted>({
        quantityOfWeeks: Option.none(),
        weekdays: Option.none()
    });
    const fields: WeeklyRecurrenceRuleFieldControllers = {
        quantityOfWeeks: {
            path: ['quantityOfWeeks'] as const,
            name: 'quantityOfWeeks',
            constraints: { required: true },

            get: () => data.quantityOfWeeks,
            set: (value: number) => {
                data.quantityOfWeeks = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.quantityOfWeeks,
            setError: (value: Option<Array<string>>) => {
                errors.quantityOfWeeks = value;
            },
            getTainted: () => tainted.quantityOfWeeks,
            setTainted: (value: Option<boolean>) => {
                tainted.quantityOfWeeks = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = weeklyRecurrenceRuleValidateField(
                    'quantityOfWeeks',
                    data.quantityOfWeeks
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        weekdays: {
            path: ['weekdays'] as const,
            name: 'weekdays',
            constraints: { required: true },

            get: () => data.weekdays,
            set: (value: Weekday[]) => {
                data.weekdays = value;
            },
            transform: (value: Weekday[]): Weekday[] => value,
            getError: () => errors.weekdays,
            setError: (value: Option<Array<string>>) => {
                errors.weekdays = value;
            },
            getTainted: () => tainted.weekdays,
            setTainted: (value: Option<boolean>) => {
                tainted.weekdays = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = weeklyRecurrenceRuleValidateField('weekdays', data.weekdays);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['weekdays', index] as const,
                name: `weekdays.${index}`,
                constraints: { required: true },
                get: () => data.weekdays[index]!,
                set: (value: Weekday) => {
                    data.weekdays[index] = value;
                },
                transform: (value: Weekday): Weekday => value,
                getError: () => errors.weekdays,
                setError: (value: Option<Array<string>>) => {
                    errors.weekdays = value;
                },
                getTainted: () => tainted.weekdays,
                setTainted: (value: Option<boolean>) => {
                    tainted.weekdays = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: Weekday) => {
                data.weekdays.push(item);
            },
            remove: (index: number) => {
                data.weekdays.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.weekdays[a]!;
                data.weekdays[a] = data.weekdays[b]!;
                data.weekdays[b] = tmp;
            }
        }
    };
    function validate(): Result<WeeklyRecurrenceRule, Array<{ field: string; message: string }>> {
        return weeklyRecurrenceRuleFromObject(data);
    }
    function reset(newOverrides?: Partial<WeeklyRecurrenceRule>): void {
        data = { ...weeklyRecurrenceRuleDefaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            quantityOfWeeks: Option.none(),
            weekdays: Option.none()
        };
        tainted = { quantityOfWeeks: Option.none(), weekdays: Option.none() };
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
export function weeklyRecurrenceRuleFromFormData(
    formData: FormData
): Result<WeeklyRecurrenceRule, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const quantityOfWeeksStr = formData.get('quantityOfWeeks');
        obj.quantityOfWeeks = quantityOfWeeksStr ? parseFloat(quantityOfWeeksStr as string) : 0;
        if (obj.quantityOfWeeks !== undefined && isNaN(obj.quantityOfWeeks as number))
            obj.quantityOfWeeks = 0;
    }
    {
        // Collect array items from indexed form fields
        const weekdaysItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('weekdays.' + idx + '.') || idx === 0) {
            // Check if any field with this index exists
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('weekdays.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('weekdays.' + idx + '.')) {
                        const fieldName = key.slice('weekdays.'.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                weekdaysItems.push(item);
            }
            idx++;
            if (idx > 1000) break; // Safety limit
        }
        obj.weekdays = weekdaysItems;
    }
    return weeklyRecurrenceRuleFromStringifiedJSON(JSON.stringify(obj));
}

export const WeeklyRecurrenceRule = {
    defaultValue: weeklyRecurrenceRuleDefaultValue,
    serialize: weeklyRecurrenceRuleSerialize,
    serializeWithContext: weeklyRecurrenceRuleSerializeWithContext,
    deserialize: weeklyRecurrenceRuleDeserialize,
    deserializeWithContext: weeklyRecurrenceRuleDeserializeWithContext,
    validateFields: weeklyRecurrenceRuleValidateFields,
    hasShape: weeklyRecurrenceRuleHasShape,
    is: weeklyRecurrenceRuleIs,
    createForm: weeklyRecurrenceRuleCreateForm,
    fromFormData: weeklyRecurrenceRuleFromFormData
} as const;
