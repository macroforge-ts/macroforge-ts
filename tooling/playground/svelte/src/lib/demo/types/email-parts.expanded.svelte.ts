import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option as __gf_Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
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
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(emailPartsSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function emailPartsSerializeWithContext(
    value: EmailParts,
    ctx: __mf_SerializeContext
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
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: EmailParts }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = emailPartsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: '_root',
                        message: 'EmailParts.deserialize: root cannot be a forward reference'
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
        if (e instanceof __mf_DeserializeError) {
            return { success: false, errors: e.errors };
        }
        const message = e instanceof Error ? e.message : String(e);
        return { success: false, errors: [{ field: '_root', message }] };
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function emailPartsDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): EmailParts | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
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
        throw new __mf_DeserializeError(errors);
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
        throw new __mf_DeserializeError(errors);
    }
    return instance as EmailParts;
}
export function emailPartsValidateField<K extends keyof EmailParts>(
    _field: K,
    _value: EmailParts[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (_field) {
        case 'local': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'local', message: 'must not be empty' });
            }
            break;
        }
        case 'domainName': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'domainName', message: 'must not be empty' });
            }
            break;
        }
        case 'topLevelDomain': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'topLevelDomain', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function emailPartsValidateFields(
    _partial: Partial<EmailParts>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('local' in _partial && _partial.local !== undefined) {
        const __val = _partial.local as string;
        if (__val.length === 0) {
            errors.push({ field: 'local', message: 'must not be empty' });
        }
    }
    if ('domainName' in _partial && _partial.domainName !== undefined) {
        const __val = _partial.domainName as string;
        if (__val.length === 0) {
            errors.push({ field: 'domainName', message: 'must not be empty' });
        }
    }
    if ('topLevelDomain' in _partial && _partial.topLevelDomain !== undefined) {
        const __val = _partial.topLevelDomain as string;
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
    return result.success;
}

/** Nested error structure matching the data shape */ export type EmailPartsErrors = {
    _errors: __gf_Option<Array<string>>;
    local: __gf_Option<Array<string>>;
    domainName: __gf_Option<Array<string>>;
    topLevelDomain: __gf_Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type EmailPartsTainted = {
    local: __gf_Option<boolean>;
    domainName: __gf_Option<boolean>;
    topLevelDomain: __gf_Option<boolean>;
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
    validate(): Exit<EmailParts, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<EmailParts>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function emailPartsCreateForm(overrides?: Partial<EmailParts>): EmailPartsGigaform {
    let data = $state({ ...emailPartsDefaultValue(), ...overrides });
    let errors = $state<EmailPartsErrors>({
        _errors: optionNone(),
        local: optionNone(),
        domainName: optionNone(),
        topLevelDomain: optionNone()
    });
    let tainted = $state<EmailPartsTainted>({
        local: optionNone(),
        domainName: optionNone(),
        topLevelDomain: optionNone()
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
            setError: (value: __gf_Option<Array<string>>) => {
                errors.local = value;
            },
            getTainted: () => tainted.local,
            setTainted: (value: __gf_Option<boolean>) => {
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
            setError: (value: __gf_Option<Array<string>>) => {
                errors.domainName = value;
            },
            getTainted: () => tainted.domainName,
            setTainted: (value: __gf_Option<boolean>) => {
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
            setError: (value: __gf_Option<Array<string>>) => {
                errors.topLevelDomain = value;
            },
            getTainted: () => tainted.topLevelDomain,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.topLevelDomain = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = emailPartsValidateField('topLevelDomain', data.topLevelDomain);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<EmailParts, Array<{ field: string; message: string }>> {
        return toExit(emailPartsDeserialize(data));
    }
    function reset(newOverrides?: Partial<EmailParts>): void {
        data = { ...emailPartsDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            local: optionNone(),
            domainName: optionNone(),
            topLevelDomain: optionNone()
        };
        tainted = { local: optionNone(), domainName: optionNone(), topLevelDomain: optionNone() };
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
export function emailPartsFromFormData(
    formData: FormData
): Exit<EmailParts, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.local = formData.get('local') ?? '';
    obj.domainName = formData.get('domainName') ?? '';
    obj.topLevelDomain = formData.get('topLevelDomain') ?? '';
    return toExit(emailPartsDeserialize(obj));
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
