import { SerializeContext } from 'macroforge/serde';
import { Exit } from 'macroforge/utils/effect';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Promotion {
    id: string;
    date: string;
}

export function promotionDefaultValue(): Promotion {
    return { id: '', date: '' } as Promotion;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function promotionSerialize(
    value: Promotion
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(promotionSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function promotionSerializeWithContext(
    value: Promotion,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Promotion', __id };
    result['id'] = value.id;
    result['date'] = value.date;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function promotionDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Exit.Exit<Array<{ field: string; message: string }>, Promotion> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = promotionDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Exit.fail([
                {
                    field: '_root',
                    message: 'Promotion.deserialize: root cannot be a forward reference'
                }
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
export function promotionDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): Promotion | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Promotion.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('date' in obj)) {
        errors.push({ field: 'date', message: 'missing required field' });
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
        const __raw_id = obj['id'] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_date = obj['date'] as string;
        instance.date = __raw_date;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Promotion;
}
export function promotionValidateField<K extends keyof Promotion>(
    field: K,
    value: Promotion[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function promotionValidateFields(
    partial: Partial<Promotion>
): Array<{ field: string; message: string }> {
    return [];
}
export function promotionHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'id' in o && 'date' in o;
}
export function promotionIs(obj: unknown): obj is Promotion {
    if (!promotionHasShape(obj)) {
        return false;
    }
    const result = promotionDeserialize(obj);
    return Exit.isSuccess(result);
}

/** Nested error structure matching the data shape */ export type PromotionErrors = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    date: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type PromotionTainted = {
    id: Option<boolean>;
    date: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface PromotionFieldControllers {
    readonly id: FieldController<string>;
    readonly date: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface PromotionGigaform {
    readonly data: Promotion;
    readonly errors: PromotionErrors;
    readonly tainted: PromotionTainted;
    readonly fields: PromotionFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, Promotion>;
    reset(overrides?: Partial<Promotion>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function promotionCreateForm(overrides?: Partial<Promotion>): PromotionGigaform {
    let data = $state({ ...promotionDefaultValue(), ...overrides });
    let errors = $state<PromotionErrors>({
        _errors: optionNone(),
        id: optionNone(),
        date: optionNone()
    });
    let tainted = $state<PromotionTainted>({ id: optionNone(), date: optionNone() });
    const fields: PromotionFieldControllers = {
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
                const fieldErrors = promotionValidateField('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        date: {
            path: ['date'] as const,
            name: 'date',
            constraints: { required: true },
            get: () => data.date,
            set: (value: string) => {
                data.date = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.date,
            setError: (value: Option<Array<string>>) => {
                errors.date = value;
            },
            getTainted: () => tainted.date,
            setTainted: (value: Option<boolean>) => {
                tainted.date = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = promotionValidateField('date', data.date);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, Promotion> {
        return toExit(promotionDeserialize(data));
    }
    function reset(newOverrides?: Partial<Promotion>): void {
        data = { ...promotionDefaultValue(), ...newOverrides };
        errors = { _errors: optionNone(), id: optionNone(), date: optionNone() };
        tainted = { id: optionNone(), date: optionNone() };
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
export function promotionFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, Promotion> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.date = formData.get('date') ?? '';
    return toExit(promotionDeserialize(obj));
}

export const Promotion = {
    defaultValue: promotionDefaultValue,
    serialize: promotionSerialize,
    serializeWithContext: promotionSerializeWithContext,
    deserialize: promotionDeserialize,
    deserializeWithContext: promotionDeserializeWithContext,
    validateFields: promotionValidateFields,
    hasShape: promotionHasShape,
    is: promotionIs,
    createForm: promotionCreateForm,
    fromFormData: promotionFromFormData
} as const;
