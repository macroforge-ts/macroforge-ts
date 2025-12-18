import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Email {
    canEmail: boolean;

    emailString: string;
}

export function emailDefaultValue(): Email {
    return { canEmail: false, emailString: '' } as Email;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function emailSerialize(
    value: Email
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(emailSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function emailSerializeWithContext(
    value: Email,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Email', __id };
    result['canEmail'] = value.canEmail;
    result['emailString'] = value.emailString;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function emailDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Email> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = emailDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                { field: '_root', message: 'Email.deserialize: root cannot be a forward reference' }
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
export function emailDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Email | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Email.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('canEmail' in obj)) {
        errors.push({ field: 'canEmail', message: 'missing required field' });
    }
    if (!('emailString' in obj)) {
        errors.push({ field: 'emailString', message: 'missing required field' });
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
        const __raw_canEmail = obj['canEmail'] as boolean;
        instance.canEmail = __raw_canEmail;
    }
    {
        const __raw_emailString = obj['emailString'] as string;
        if (__raw_emailString.length === 0) {
            errors.push({ field: 'emailString', message: 'must not be empty' });
        }

        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__raw_emailString)) {
            errors.push({ field: 'emailString', message: 'must be a valid email' });
        }
        instance.emailString = __raw_emailString;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Email;
}
export function emailValidateField<K extends keyof Email>(
    field: K,
    value: Email[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'emailString': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'emailString', message: 'must not be empty' });
            }

            if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
                errors.push({ field: 'emailString', message: 'must be a valid email' });
            }
            break;
        }
    }
    return errors;
}
export function emailValidateFields(
    partial: Partial<Email>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('emailString' in partial && partial.emailString !== undefined) {
        const __val = partial.emailString as string;
        if (__val.length === 0) {
            errors.push({ field: 'emailString', message: 'must not be empty' });
        }

        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
            errors.push({ field: 'emailString', message: 'must be a valid email' });
        }
    }
    return errors;
}
export function emailHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'canEmail' in o && 'emailString' in o;
}
export function emailIs(obj: unknown): obj is Email {
    if (!emailHasShape(obj)) {
        return false;
    }
    const result = emailDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type EmailErrors = {
    _errors: Option<Array<string>>;
    canEmail: Option<Array<string>>;
    emailString: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type EmailTainted = {
    canEmail: Option<boolean>;
    emailString: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface EmailFieldControllers {
    readonly canEmail: FieldController<boolean>;
    readonly emailString: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface EmailGigaform {
    readonly data: Email;
    readonly errors: EmailErrors;
    readonly tainted: EmailTainted;
    readonly fields: EmailFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, Email>;
    reset(overrides?: Partial<Email>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function emailCreateForm(overrides?: Partial<Email>): EmailGigaform {
    let data = $state({ ...emailDefaultValue(), ...overrides });
    let errors = $state<EmailErrors>({
        _errors: optionNone(),
        canEmail: optionNone(),
        emailString: optionNone()
    });
    let tainted = $state<EmailTainted>({ canEmail: optionNone(), emailString: optionNone() });
    const fields: EmailFieldControllers = {
        canEmail: {
            path: ['canEmail'] as const,
            name: 'canEmail',
            constraints: { required: true },
            label: 'Can Email',
            get: () => data.canEmail,
            set: (value: boolean) => {
                data.canEmail = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.canEmail,
            setError: (value: Option<Array<string>>) => {
                errors.canEmail = value;
            },
            getTainted: () => tainted.canEmail,
            setTainted: (value: Option<boolean>) => {
                tainted.canEmail = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = emailValidateField('canEmail', data.canEmail);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        emailString: {
            path: ['emailString'] as const,
            name: 'emailString',
            constraints: { required: true, type: 'email' },
            label: 'Email',
            get: () => data.emailString,
            set: (value: string) => {
                data.emailString = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.emailString,
            setError: (value: Option<Array<string>>) => {
                errors.emailString = value;
            },
            getTainted: () => tainted.emailString,
            setTainted: (value: Option<boolean>) => {
                tainted.emailString = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = emailValidateField('emailString', data.emailString);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, Email> {
        return emailDeserialize(data);
    }
    function reset(newOverrides?: Partial<Email>): void {
        data = { ...emailDefaultValue(), ...newOverrides };
        errors = { _errors: optionNone(), canEmail: optionNone(), emailString: optionNone() };
        tainted = { canEmail: optionNone(), emailString: optionNone() };
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
export function emailFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, Email> {
    const obj: Record<string, unknown> = {};
    {
        const canEmailVal = formData.get('canEmail');
        obj.canEmail = canEmailVal === 'true' || canEmailVal === 'on' || canEmailVal === '1';
    }
    obj.emailString = formData.get('emailString') ?? '';
    return emailDeserialize(obj);
}

export const Email = {
    defaultValue: emailDefaultValue,
    serialize: emailSerialize,
    serializeWithContext: emailSerializeWithContext,
    deserialize: emailDeserialize,
    deserializeWithContext: emailDeserializeWithContext,
    validateFields: emailValidateFields,
    hasShape: emailHasShape,
    is: emailIs,
    createForm: emailCreateForm,
    fromFormData: emailFromFormData
} as const;
