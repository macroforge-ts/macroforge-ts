import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Paid {
    amount: number | null;
    currency: string | null;
    paymentMethod: string | null;
}

export function paidDefaultValue(): Paid {
    return { amount: null, currency: null, paymentMethod: null } as Paid;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function paidSerialize(
    value: Paid
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(paidSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function paidSerializeWithContext(
    value: Paid,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Paid', __id };
    result['amount'] = value.amount;
    result['currency'] = value.currency;
    result['paymentMethod'] = value.paymentMethod;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function paidDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Result<Paid, Array<{ field: string; message: string }>> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = paidDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Paid.deserialize: root cannot be a forward reference' }
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
export function paidDeserializeWithContext(value: any, ctx: DeserializeContext): Paid | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Paid.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('amount' in obj)) {
        errors.push({ field: 'amount', message: 'missing required field' });
    }
    if (!('currency' in obj)) {
        errors.push({ field: 'currency', message: 'missing required field' });
    }
    if (!('paymentMethod' in obj)) {
        errors.push({ field: 'paymentMethod', message: 'missing required field' });
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
        const __raw_amount = obj['amount'] as number | null;
        instance.amount = __raw_amount;
    }
    {
        const __raw_currency = obj['currency'] as string | null;
        instance.currency = __raw_currency;
    }
    {
        const __raw_paymentMethod = obj['paymentMethod'] as string | null;
        instance.paymentMethod = __raw_paymentMethod;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Paid;
}
export function paidValidateField<K extends keyof Paid>(
    field: K,
    value: Paid[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function paidValidateFields(
    partial: Partial<Paid>
): Array<{ field: string; message: string }> {
    return [];
}
export function paidHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'amount' in o && 'currency' in o && 'paymentMethod' in o;
}
export function paidIs(obj: unknown): obj is Paid {
    if (!paidHasShape(obj)) {
        return false;
    }
    const result = paidDeserialize(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type PaidErrors = {
    _errors: Option<Array<string>>;
    amount: Option<Array<string>>;
    currency: Option<Array<string>>;
    paymentMethod: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type PaidTainted = {
    amount: Option<boolean>;
    currency: Option<boolean>;
    paymentMethod: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface PaidFieldControllers {
    readonly amount: FieldController<number | null>;
    readonly currency: FieldController<string | null>;
    readonly paymentMethod: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface PaidGigaform {
    readonly data: Paid;
    readonly errors: PaidErrors;
    readonly tainted: PaidTainted;
    readonly fields: PaidFieldControllers;
    validate(): Result<Paid, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Paid>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function paidCreateForm(overrides?: Partial<Paid>): PaidGigaform {
    let data = $state({ ...paidDefaultValue(), ...overrides });
    let errors = $state<PaidErrors>({
        _errors: Option.none(),
        amount: Option.none(),
        currency: Option.none(),
        paymentMethod: Option.none()
    });
    let tainted = $state<PaidTainted>({
        amount: Option.none(),
        currency: Option.none(),
        paymentMethod: Option.none()
    });
    const fields: PaidFieldControllers = {
        amount: {
            path: ['amount'] as const,
            name: 'amount',
            constraints: { required: true },

            get: () => data.amount,
            set: (value: number | null) => {
                data.amount = value;
            },
            transform: (value: number | null): number | null => value,
            getError: () => errors.amount,
            setError: (value: Option<Array<string>>) => {
                errors.amount = value;
            },
            getTainted: () => tainted.amount,
            setTainted: (value: Option<boolean>) => {
                tainted.amount = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = paidValidateField('amount', data.amount);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        currency: {
            path: ['currency'] as const,
            name: 'currency',
            constraints: { required: true },

            get: () => data.currency,
            set: (value: string | null) => {
                data.currency = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.currency,
            setError: (value: Option<Array<string>>) => {
                errors.currency = value;
            },
            getTainted: () => tainted.currency,
            setTainted: (value: Option<boolean>) => {
                tainted.currency = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = paidValidateField('currency', data.currency);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        paymentMethod: {
            path: ['paymentMethod'] as const,
            name: 'paymentMethod',
            constraints: { required: true },

            get: () => data.paymentMethod,
            set: (value: string | null) => {
                data.paymentMethod = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.paymentMethod,
            setError: (value: Option<Array<string>>) => {
                errors.paymentMethod = value;
            },
            getTainted: () => tainted.paymentMethod,
            setTainted: (value: Option<boolean>) => {
                tainted.paymentMethod = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = paidValidateField('paymentMethod', data.paymentMethod);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Paid, Array<{ field: string; message: string }>> {
        return paidFromObject(data);
    }
    function reset(newOverrides?: Partial<Paid>): void {
        data = { ...paidDefaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            amount: Option.none(),
            currency: Option.none(),
            paymentMethod: Option.none()
        };
        tainted = { amount: Option.none(), currency: Option.none(), paymentMethod: Option.none() };
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
export function paidFromFormData(
    formData: FormData
): Result<Paid, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const amountStr = formData.get('amount');
        obj.amount = amountStr ? parseFloat(amountStr as string) : 0;
        if (obj.amount !== undefined && isNaN(obj.amount as number)) obj.amount = 0;
    }
    obj.currency = formData.get('currency') ?? '';
    obj.paymentMethod = formData.get('paymentMethod') ?? '';
    return paidFromStringifiedJSON(JSON.stringify(obj));
}

export const Paid = {
    defaultValue: paidDefaultValue,
    serialize: paidSerialize,
    serializeWithContext: paidSerializeWithContext,
    deserialize: paidDeserialize,
    deserializeWithContext: paidDeserializeWithContext,
    validateFields: paidValidateFields,
    hasShape: paidHasShape,
    is: paidIs,
    createForm: paidCreateForm,
    fromFormData: paidFromFormData
} as const;
