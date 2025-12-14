import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Payment {
    id: string;
    date: string;
}

export function defaultValuePayment(): Payment {
    return { id: '', date: '' } as Payment;
}

export function toStringifiedJSONPayment(value: Payment): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializePayment(value, ctx));
}
export function toObjectPayment(value: Payment): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializePayment(value, ctx);
}
export function __serializePayment(value: Payment, ctx: SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Payment', __id };
    result['id'] = value.id;
    result['date'] = value.date;
    return result;
}

export function fromStringifiedJSONPayment(
    json: string,
    opts?: DeserializeOptions
): Result<Payment, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectPayment(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectPayment(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Payment, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializePayment(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Payment.fromObject: root cannot be a forward reference'
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
export function __deserializePayment(value: any, ctx: DeserializeContext): Payment | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Payment.__deserialize: expected an object' }
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
    return instance as Payment;
}
export function validateFieldPayment<K extends keyof Payment>(
    field: K,
    value: Payment[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsPayment(
    partial: Partial<Payment>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapePayment(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'id' in o && 'date' in o;
}
export function isPayment(obj: unknown): obj is Payment {
    if (!hasShapePayment(obj)) {
        return false;
    }
    const result = fromObjectPayment(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsPayment = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    date: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedPayment = {
    id: Option<boolean>;
    date: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersPayment {
    readonly id: FieldController<string>;
    readonly date: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformPayment {
    readonly data: Payment;
    readonly errors: ErrorsPayment;
    readonly tainted: TaintedPayment;
    readonly fields: FieldControllersPayment;
    validate(): Result<Payment, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Payment>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormPayment(overrides?: Partial<Payment>): GigaformPayment {
    let data = $state({ ...Payment.defaultValue(), ...overrides });
    let errors = $state<ErrorsPayment>({
        _errors: Option.none(),
        id: Option.none(),
        date: Option.none()
    });
    let tainted = $state<TaintedPayment>({ id: Option.none(), date: Option.none() });
    const fields: FieldControllersPayment = {
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
                const fieldErrors = Payment.validateField('id', data.id);
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
                const fieldErrors = Payment.validateField('date', data.date);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Payment, Array<{ field: string; message: string }>> {
        return Payment.fromObject(data);
    }
    function reset(newOverrides?: Partial<Payment>): void {
        data = { ...Payment.defaultValue(), ...newOverrides };
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function fromFormDataPayment(
    formData: FormData
): Result<Payment, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.date = formData.get('date') ?? '';
    return Payment.fromStringifiedJSON(JSON.stringify(obj));
}
