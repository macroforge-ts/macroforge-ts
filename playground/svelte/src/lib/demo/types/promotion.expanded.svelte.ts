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
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(promotionSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function promotionSerializeWithContext(
    value: Promotion,
    ctx: __mf_SerializeContext
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
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Promotion> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = promotionDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
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
export function promotionDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Promotion | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
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
        throw new __mf_DeserializeError(errors);
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
        throw new __mf_DeserializeError(errors);
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
    return __mf_exitIsSuccess(result);
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
    validate(): Result<Promotion, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Promotion>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function promotionCreateForm(overrides?: Partial<Promotion>): PromotionGigaform {
    let data = $state({ ...promotionDefaultValue(), ...overrides });
    let errors = $state<PromotionErrors>({
        _errors: Option.none(),
        id: Option.none(),
        date: Option.none()
    });
    let tainted = $state<PromotionTainted>({ id: Option.none(), date: Option.none() });
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
    function validate(): Result<Promotion, Array<{ field: string; message: string }>> {
        return promotionDeserialize(data);
    }
    function reset(newOverrides?: Partial<Promotion>): void {
        data = { ...promotionDefaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), id: Option.none(), date: Option.none() };
        tainted = { id: Option.none(), date: Option.none() };
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
): Result<Promotion, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.date = formData.get('date') ?? '';
    return promotionDeserialize(obj);
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
