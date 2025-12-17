import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface EmailParts {
    local: string;

    domainName: string;

    topLevelDomain: string;
}

export function emailPartsDefaultValue(): EmailParts {
    return { local: '', domainName: '', topLevelDomain: '' } as EmailParts;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function emailPartsSerialize(
    value: EmailParts
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(emailPartsSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function emailPartsSerializeWithContext(
    value: EmailParts,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'EmailParts', __id };
    result['local'] = value.local;
    result['domainName'] = value.domainName;
    result['topLevelDomain'] = value.topLevelDomain;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function emailPartsDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Result<EmailParts, Array<{ field: string; message: string }>> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = emailPartsDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'EmailParts.deserialize: root cannot be a forward reference'
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
export function emailPartsDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): EmailParts | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'EmailParts.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('local' in obj)) {
        errors.push({ field: 'local', message: 'missing required field' });
    }
    if (!('domainName' in obj)) {
        errors.push({ field: 'domainName', message: 'missing required field' });
    }
    if (!('topLevelDomain' in obj)) {
        errors.push({ field: 'topLevelDomain', message: 'missing required field' });
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
        const __raw_local = obj['local'] as string;
        if (__raw_local.length === 0) {
            errors.push({ field: 'local', message: 'must not be empty' });
        }
        instance.local = __raw_local;
    }
    {
        const __raw_domainName = obj['domainName'] as string;
        if (__raw_domainName.length === 0) {
            errors.push({ field: 'domainName', message: 'must not be empty' });
        }
        instance.domainName = __raw_domainName;
    }
    {
        const __raw_topLevelDomain = obj['topLevelDomain'] as string;
        if (__raw_topLevelDomain.length === 0) {
            errors.push({ field: 'topLevelDomain', message: 'must not be empty' });
        }
        instance.topLevelDomain = __raw_topLevelDomain;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as EmailParts;
}
export function emailPartsValidateField<K extends keyof EmailParts>(
    field: K,
    value: EmailParts[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'local': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'local', message: 'must not be empty' });
            }
            break;
        }
        case 'domainName': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'domainName', message: 'must not be empty' });
            }
            break;
        }
        case 'topLevelDomain': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'topLevelDomain', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function emailPartsValidateFields(
    partial: Partial<EmailParts>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('local' in partial && partial.local !== undefined) {
        const __val = partial.local as string;
        if (__val.length === 0) {
            errors.push({ field: 'local', message: 'must not be empty' });
        }
    }
    if ('domainName' in partial && partial.domainName !== undefined) {
        const __val = partial.domainName as string;
        if (__val.length === 0) {
            errors.push({ field: 'domainName', message: 'must not be empty' });
        }
    }
    if ('topLevelDomain' in partial && partial.topLevelDomain !== undefined) {
        const __val = partial.topLevelDomain as string;
        if (__val.length === 0) {
            errors.push({ field: 'topLevelDomain', message: 'must not be empty' });
        }
    }
    return errors;
}
export function emailPartsHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'local' in o && 'domainName' in o && 'topLevelDomain' in o;
}
export function emailPartsIs(obj: unknown): obj is EmailParts {
    if (!emailPartsHasShape(obj)) {
        return false;
    }
    const result = emailPartsDeserialize(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type EmailPartsErrors = {
    _errors: Option<Array<string>>;
    local: Option<Array<string>>;
    domainName: Option<Array<string>>;
    topLevelDomain: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type EmailPartsTainted = {
    local: Option<boolean>;
    domainName: Option<boolean>;
    topLevelDomain: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface EmailPartsFieldControllers {
    readonly local: FieldController<string>;
    readonly domainName: FieldController<string>;
    readonly topLevelDomain: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface EmailPartsGigaform {
    readonly data: EmailParts;
    readonly errors: EmailPartsErrors;
    readonly tainted: EmailPartsTainted;
    readonly fields: EmailPartsFieldControllers;
    validate(): Result<EmailParts, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<EmailParts>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function emailPartsCreateForm(overrides?: Partial<EmailParts>): EmailPartsGigaform {
    let data = $state({ ...emailPartsDefaultValue(), ...overrides });
    let errors = $state<EmailPartsErrors>({
        _errors: Option.none(),
        local: Option.none(),
        domainName: Option.none(),
        topLevelDomain: Option.none()
    });
    let tainted = $state<EmailPartsTainted>({
        local: Option.none(),
        domainName: Option.none(),
        topLevelDomain: Option.none()
    });
    const fields: EmailPartsFieldControllers = {
        local: {
            path: ['local'] as const,
            name: 'local',
            constraints: { required: true },

            get: () => data.local,
            set: (value: string) => {
                data.local = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.local,
            setError: (value: Option<Array<string>>) => {
                errors.local = value;
            },
            getTainted: () => tainted.local,
            setTainted: (value: Option<boolean>) => {
                tainted.local = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = emailPartsValidateField('local', data.local);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        domainName: {
            path: ['domainName'] as const,
            name: 'domainName',
            constraints: { required: true },

            get: () => data.domainName,
            set: (value: string) => {
                data.domainName = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.domainName,
            setError: (value: Option<Array<string>>) => {
                errors.domainName = value;
            },
            getTainted: () => tainted.domainName,
            setTainted: (value: Option<boolean>) => {
                tainted.domainName = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = emailPartsValidateField('domainName', data.domainName);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        topLevelDomain: {
            path: ['topLevelDomain'] as const,
            name: 'topLevelDomain',
            constraints: { required: true },

            get: () => data.topLevelDomain,
            set: (value: string) => {
                data.topLevelDomain = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.topLevelDomain,
            setError: (value: Option<Array<string>>) => {
                errors.topLevelDomain = value;
            },
            getTainted: () => tainted.topLevelDomain,
            setTainted: (value: Option<boolean>) => {
                tainted.topLevelDomain = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = emailPartsValidateField('topLevelDomain', data.topLevelDomain);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<EmailParts, Array<{ field: string; message: string }>> {
        return emailPartsFromObject(data);
    }
    function reset(newOverrides?: Partial<EmailParts>): void {
        data = { ...emailPartsDefaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            local: Option.none(),
            domainName: Option.none(),
            topLevelDomain: Option.none()
        };
        tainted = {
            local: Option.none(),
            domainName: Option.none(),
            topLevelDomain: Option.none()
        };
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
export function emailPartsFromFormData(
    formData: FormData
): Result<EmailParts, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.local = formData.get('local') ?? '';
    obj.domainName = formData.get('domainName') ?? '';
    obj.topLevelDomain = formData.get('topLevelDomain') ?? '';
    return emailPartsFromStringifiedJSON(JSON.stringify(obj));
}

export const EmailParts = {
    defaultValue: emailPartsDefaultValue,
    serialize: emailPartsSerialize,
    serializeWithContext: emailPartsSerializeWithContext,
    deserialize: emailPartsDeserialize,
    deserializeWithContext: emailPartsDeserializeWithContext,
    validateFields: emailPartsValidateFields,
    hasShape: emailPartsHasShape,
    is: emailPartsIs,
    createForm: emailPartsCreateForm,
    fromFormData: emailPartsFromFormData
} as const;
