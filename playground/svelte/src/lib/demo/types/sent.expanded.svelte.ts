import { SerializeContext } from 'macroforge/serde';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Sent {
    recipient: string | null;
    method: string | null;
}

export function sentDefaultValue(): Sent {
    return { recipient: null, method: null } as Sent;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function sentSerialize(
    value: Sent
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(sentSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function sentSerializeWithContext(
    value: Sent,
    ctx: SerializeContext
): Record<string, unknown> {
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

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function sentDeserialize(
    input: unknown,
    opts?: DeserializeOptions
):
    | { success: true; value: Sent }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = sentDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: '_root',
                        message: 'Sent.deserialize: root cannot be a forward reference'
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
        if (e instanceof DeserializeError) {
            return { success: false, errors: e.errors };
        }
        const message = e instanceof Error ? e.message : String(e);
        return { success: false, errors: [{ field: '_root', message }] };
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function sentDeserializeWithContext(value: any, ctx: DeserializeContext): Sent | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Sent.deserializeWithContext: expected an object' }
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
export function sentValidateField<K extends keyof Sent>(
    field: K,
    value: Sent[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function sentValidateFields(
    partial: Partial<Sent>
): Array<{ field: string; message: string }> {
    return [];
}
export function sentHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'recipient' in o && 'method' in o;
}
export function sentIs(obj: unknown): obj is Sent {
    if (!sentHasShape(obj)) {
        return false;
    }
    const result = sentDeserialize(obj);
    return result.success;
}

/** Nested error structure matching the data shape */ export type SentErrors = {
    _errors: Option<Array<string>>;
    recipient: Option<Array<string>>;
    method: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type SentTainted = {
    recipient: Option<boolean>;
    method: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface SentFieldControllers {
    readonly recipient: FieldController<string | null>;
    readonly method: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface SentGigaform {
    readonly data: Sent;
    readonly errors: SentErrors;
    readonly tainted: SentTainted;
    readonly fields: SentFieldControllers;
    validate(): Result<Sent, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Sent>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function sentCreateForm(overrides?: Partial<Sent>): SentGigaform {
    let data = $state({ ...sentDefaultValue(), ...overrides });
    let errors = $state<SentErrors>({
        _errors: Option.none(),
        recipient: Option.none(),
        method: Option.none()
    });
    let tainted = $state<SentTainted>({ recipient: Option.none(), method: Option.none() });
    const fields: SentFieldControllers = {
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
                const fieldErrors = sentValidateField('recipient', data.recipient);
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
                const fieldErrors = sentValidateField('method', data.method);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Sent, Array<{ field: string; message: string }>> {
        return sentDeserialize(data);
    }
    function reset(newOverrides?: Partial<Sent>): void {
        data = { ...sentDefaultValue(), ...newOverrides };
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */
export function sentFromFormData(
    formData: FormData
): Result<Sent, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.recipient = formData.get('recipient') ?? '';
    obj.method = formData.get('method') ?? '';
    return sentDeserialize(obj);
}

export const Sent = {
    defaultValue: sentDefaultValue,
    serialize: sentSerialize,
    serializeWithContext: sentSerializeWithContext,
    deserialize: sentDeserialize,
    deserializeWithContext: sentDeserializeWithContext,
    validateFields: sentValidateFields,
    hasShape: sentHasShape,
    is: sentIs,
    createForm: sentCreateForm,
    fromFormData: sentFromFormData
} as const;
