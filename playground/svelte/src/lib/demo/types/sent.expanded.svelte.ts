import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Sent {
    recipient: string | null;
    method: string | null;
}

export function defaultValueSent(): Sent {
    return { recipient: null, method: null } as Sent;
}

export function toStringifiedJSONSent(value: Sent): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeSent(value, ctx));
}
export function toObjectSent(value: Sent): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeSent(value, ctx);
}
export function __serializeSent(value: Sent, ctx: SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Sent', __id };
    result['recipient'] = value.recipient;
    result['method'] = value.method;
    return result;
}

export function fromStringifiedJSONSent(
    json: string,
    opts?: DeserializeOptions
): Result<Sent, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectSent(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectSent(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Sent, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeSent(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Sent.fromObject: root cannot be a forward reference' }
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
export function __deserializeSent(value: any, ctx: DeserializeContext): Sent | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Sent.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('recipient' in obj)) {
        errors.push({ field: 'recipient', message: 'missing required field' });
    }
    if (!('method' in obj)) {
        errors.push({ field: 'method', message: 'missing required field' });
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
        const __raw_recipient = obj['recipient'] as string | null;
        instance.recipient = __raw_recipient;
    }
    {
        const __raw_method = obj['method'] as string | null;
        instance.method = __raw_method;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Sent;
}
export function validateFieldSent<K extends keyof Sent>(
    field: K,
    value: Sent[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsSent(
    partial: Partial<Sent>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeSent(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'recipient' in o && 'method' in o;
}
export function isSent(obj: unknown): obj is Sent {
    if (!hasShapeSent(obj)) {
        return false;
    }
    const result = fromObjectSent(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsSent = {
    _errors: Option<Array<string>>;
    recipient: Option<Array<string>>;
    method: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedSent = {
    recipient: Option<boolean>;
    method: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersSent {
    readonly recipient: FieldController<string | null>;
    readonly method: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformSent {
    readonly data: Sent;
    readonly errors: ErrorsSent;
    readonly tainted: TaintedSent;
    readonly fields: FieldControllersSent;
    validate(): Result<Sent, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Sent>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormSent(overrides?: Partial<Sent>): GigaformSent {
    let data = $state({ ...defaultValueSent(), ...overrides });
    let errors = $state<ErrorsSent>({
        _errors: Option.none(),
        recipient: Option.none(),
        method: Option.none()
    });
    let tainted = $state<TaintedSent>({ recipient: Option.none(), method: Option.none() });
    const fields: FieldControllersSent = {
        recipient: {
            path: ['recipient'] as const,
            name: 'recipient',
            constraints: { required: true },

            get: () => data.recipient,
            set: (value: string | null) => {
                data.recipient = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.recipient,
            setError: (value: Option<Array<string>>) => {
                errors.recipient = value;
            },
            getTainted: () => tainted.recipient,
            setTainted: (value: Option<boolean>) => {
                tainted.recipient = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldSent('recipient', data.recipient);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        method: {
            path: ['method'] as const,
            name: 'method',
            constraints: { required: true },

            get: () => data.method,
            set: (value: string | null) => {
                data.method = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.method,
            setError: (value: Option<Array<string>>) => {
                errors.method = value;
            },
            getTainted: () => tainted.method,
            setTainted: (value: Option<boolean>) => {
                tainted.method = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldSent('method', data.method);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Sent, Array<{ field: string; message: string }>> {
        return fromObjectSent(data);
    }
    function reset(newOverrides?: Partial<Sent>): void {
        data = { ...defaultValueSent(), ...newOverrides };
        errors = { _errors: Option.none(), recipient: Option.none(), method: Option.none() };
        tainted = { recipient: Option.none(), method: Option.none() };
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
export function fromFormDataSent(
    formData: FormData
): Result<Sent, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.recipient = formData.get('recipient') ?? '';
    obj.method = formData.get('method') ?? '';
    return fromStringifiedJSONSent(JSON.stringify(obj));
}
