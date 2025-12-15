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

export function defaultValuePaid(): Paid {
    return { amount: null, currency: null, paymentMethod: null } as Paid;
}

export function toStringifiedJSONPaid(value: Paid): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializePaid(value, ctx));
}
export function toObjectPaid(value: Paid): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializePaid(value, ctx);
}
export function __serializePaid(value: Paid, ctx: SerializeContext): Record<string, unknown> {
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

export function fromStringifiedJSONPaid(
    json: string,
    opts?: DeserializeOptions
): Result<Paid, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectPaid(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectPaid(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Paid, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializePaid(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Paid.fromObject: root cannot be a forward reference' }
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
export function __deserializePaid(value: any, ctx: DeserializeContext): Paid | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Paid.__deserialize: expected an object' }
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
export function validateFieldPaid<K extends keyof Paid>(
    field: K,
    value: Paid[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsPaid(
    partial: Partial<Paid>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapePaid(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'amount' in o && 'currency' in o && 'paymentMethod' in o;
}
export function isPaid(obj: unknown): obj is Paid {
    if (!hasShapePaid(obj)) {
        return false;
    }
    const result = fromObjectPaid(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsPaid = {
    _errors: Option<Array<string>>;
    amount: Option<Array<string>>;
    currency: Option<Array<string>>;
    paymentMethod: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedPaid = {
    amount: Option<boolean>;
    currency: Option<boolean>;
    paymentMethod: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersPaid {
    readonly amount: FieldController<number | null>;
    readonly currency: FieldController<string | null>;
    readonly paymentMethod: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformPaid {
    readonly data: Paid;
    readonly errors: ErrorsPaid;
    readonly tainted: TaintedPaid;
    readonly fields: FieldControllersPaid;
    validate(): Result<Paid, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Paid>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormPaid(overrides?: Partial<Paid>): GigaformPaid {
    let data = $state({ ...defaultValuePaid(), ...overrides });
    let errors = $state<ErrorsPaid>({
        _errors: Option.none(),
        amount: Option.none(),
        currency: Option.none(),
        paymentMethod: Option.none()
    });
    let tainted = $state<TaintedPaid>({
        amount: Option.none(),
        currency: Option.none(),
        paymentMethod: Option.none()
    });
    const fields: FieldControllersPaid = {
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
                const fieldErrors = validateFieldPaid('amount', data.amount);
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
                const fieldErrors = validateFieldPaid('currency', data.currency);
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
                const fieldErrors = validateFieldPaid('paymentMethod', data.paymentMethod);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Paid, Array<{ field: string; message: string }>> {
        return fromObjectPaid(data);
    }
    function reset(newOverrides?: Partial<Paid>): void {
        data = { ...defaultValuePaid(), ...newOverrides };
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
export function fromFormDataPaid(
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
    return fromStringifiedJSONPaid(JSON.stringify(obj));
}
