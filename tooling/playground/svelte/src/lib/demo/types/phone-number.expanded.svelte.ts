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

import type { Number } from './number.svelte';

export interface PhoneNumber {
    main: boolean;

    phoneType: string;

    number: string;

    canText: boolean;

    canCall: boolean;
}

export function phoneNumberDefaultValue(): PhoneNumber {
    return {
        main: false,
        phoneType: '',
        number: '',
        canText: false,
        canCall: false
    } as PhoneNumber;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function phoneNumberSerialize(
    value: PhoneNumber
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(phoneNumberSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function phoneNumberSerializeWithContext(
    value: PhoneNumber,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'PhoneNumber', __id };
    result['main'] = value.main;
    result['phoneType'] = value.phoneType;
    result['number'] = value.number;
    result['canText'] = value.canText;
    result['canCall'] = value.canCall;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function phoneNumberDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: PhoneNumber }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = phoneNumberDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: '_root',
                        message: 'PhoneNumber.deserialize: root cannot be a forward reference'
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
export function phoneNumberDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): PhoneNumber | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'PhoneNumber.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('main' in obj)) {
        errors.push({ field: 'main', message: 'missing required field' });
    }
    if (!('phoneType' in obj)) {
        errors.push({ field: 'phoneType', message: 'missing required field' });
    }
    if (!('number' in obj)) {
        errors.push({ field: 'number', message: 'missing required field' });
    }
    if (!('canText' in obj)) {
        errors.push({ field: 'canText', message: 'missing required field' });
    }
    if (!('canCall' in obj)) {
        errors.push({ field: 'canCall', message: 'missing required field' });
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
        const __raw_main = obj['main'] as boolean;
        instance.main = __raw_main;
    }
    {
        const __raw_phoneType = obj['phoneType'] as string;
        if (__raw_phoneType.length === 0) {
            errors.push({ field: 'phoneType', message: 'must not be empty' });
        }
        instance.phoneType = __raw_phoneType;
    }
    {
        const __raw_number = obj['number'] as string;
        if (__raw_number.length === 0) {
            errors.push({ field: 'number', message: 'must not be empty' });
        }
        instance.number = __raw_number;
    }
    {
        const __raw_canText = obj['canText'] as boolean;
        instance.canText = __raw_canText;
    }
    {
        const __raw_canCall = obj['canCall'] as boolean;
        instance.canCall = __raw_canCall;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as PhoneNumber;
}
export function phoneNumberValidateField<K extends keyof PhoneNumber>(
    _field: K,
    _value: PhoneNumber[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (_field) {
        case 'phoneType': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'phoneType', message: 'must not be empty' });
            }
            break;
        }
        case 'number': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({ field: 'number', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function phoneNumberValidateFields(
    _partial: Partial<PhoneNumber>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('phoneType' in _partial && _partial.phoneType !== undefined) {
        const __val = _partial.phoneType as string;
        if (__val.length === 0) {
            errors.push({ field: 'phoneType', message: 'must not be empty' });
        }
    }
    if ('number' in _partial && _partial.number !== undefined) {
        const __val = _partial.number as string;
        if (__val.length === 0) {
            errors.push({ field: 'number', message: 'must not be empty' });
        }
    }
    return errors;
}
export function phoneNumberHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'main' in o && 'phoneType' in o && 'number' in o && 'canText' in o && 'canCall' in o;
}
export function phoneNumberIs(obj: unknown): obj is PhoneNumber {
    if (!phoneNumberHasShape(obj)) {
        return false;
    }
    const result = phoneNumberDeserialize(obj);
    return result.success;
}

/** Nested error structure matching the data shape */ export type PhoneNumberErrors = {
    _errors: __gf_Option<Array<string>>;
    main: __gf_Option<Array<string>>;
    phoneType: __gf_Option<Array<string>>;
    number: __gf_Option<Array<string>>;
    canText: __gf_Option<Array<string>>;
    canCall: __gf_Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type PhoneNumberTainted = {
    main: __gf_Option<boolean>;
    phoneType: __gf_Option<boolean>;
    number: __gf_Option<boolean>;
    canText: __gf_Option<boolean>;
    canCall: __gf_Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface PhoneNumberFieldControllers {
    readonly main: FieldController<boolean>;
    readonly phoneType: FieldController<string>;
    readonly number: FieldController<string>;
    readonly canText: FieldController<boolean>;
    readonly canCall: FieldController<boolean>;
} /** Gigaform instance containing reactive state and field controllers */
export interface PhoneNumberGigaform {
    readonly data: PhoneNumber;
    readonly errors: PhoneNumberErrors;
    readonly tainted: PhoneNumberTainted;
    readonly fields: PhoneNumberFieldControllers;
    validate(): Exit<PhoneNumber, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<PhoneNumber>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function phoneNumberCreateForm(overrides?: Partial<PhoneNumber>): PhoneNumberGigaform {
    let data = $state({ ...phoneNumberDefaultValue(), ...overrides });
    let errors = $state<PhoneNumberErrors>({
        _errors: optionNone(),
        main: optionNone(),
        phoneType: optionNone(),
        number: optionNone(),
        canText: optionNone(),
        canCall: optionNone()
    });
    let tainted = $state<PhoneNumberTainted>({
        main: optionNone(),
        phoneType: optionNone(),
        number: optionNone(),
        canText: optionNone(),
        canCall: optionNone()
    });
    const fields: PhoneNumberFieldControllers = {
        main: {
            path: ['main'] as const,
            name: 'main',
            constraints: { required: true },
            label: 'Main',
            get: () => data.main,
            set: (value: boolean) => {
                data.main = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.main,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.main = value;
            },
            getTainted: () => tainted.main,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.main = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = phoneNumberValidateField('main', data.main);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        phoneType: {
            path: ['phoneType'] as const,
            name: 'phoneType',
            constraints: { required: true },
            label: 'Phone Type',
            get: () => data.phoneType,
            set: (value: string) => {
                data.phoneType = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.phoneType,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.phoneType = value;
            },
            getTainted: () => tainted.phoneType,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.phoneType = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = phoneNumberValidateField('phoneType', data.phoneType);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        number: {
            path: ['number'] as const,
            name: 'number',
            constraints: { required: true },
            label: 'Number',
            get: () => data.number,
            set: (value: string) => {
                data.number = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.number,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.number = value;
            },
            getTainted: () => tainted.number,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.number = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = phoneNumberValidateField('number', data.number);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        canText: {
            path: ['canText'] as const,
            name: 'canText',
            constraints: { required: true },
            label: 'Can Text',
            get: () => data.canText,
            set: (value: boolean) => {
                data.canText = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.canText,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.canText = value;
            },
            getTainted: () => tainted.canText,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.canText = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = phoneNumberValidateField('canText', data.canText);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        canCall: {
            path: ['canCall'] as const,
            name: 'canCall',
            constraints: { required: true },
            label: 'Can Call',
            get: () => data.canCall,
            set: (value: boolean) => {
                data.canCall = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.canCall,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.canCall = value;
            },
            getTainted: () => tainted.canCall,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.canCall = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = phoneNumberValidateField('canCall', data.canCall);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<PhoneNumber, Array<{ field: string; message: string }>> {
        return toExit(phoneNumberDeserialize(data));
    }
    function reset(newOverrides?: Partial<PhoneNumber>): void {
        data = { ...phoneNumberDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            main: optionNone(),
            phoneType: optionNone(),
            number: optionNone(),
            canText: optionNone(),
            canCall: optionNone()
        };
        tainted = {
            main: optionNone(),
            phoneType: optionNone(),
            number: optionNone(),
            canText: optionNone(),
            canCall: optionNone()
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */
export function phoneNumberFromFormData(
    formData: FormData
): Exit<PhoneNumber, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const mainVal = formData.get('main');
        obj.main = mainVal === 'true' || mainVal === 'on' || mainVal === '1';
    }
    obj.phoneType = formData.get('phoneType') ?? '';
    obj.number = formData.get('number') ?? '';
    {
        const canTextVal = formData.get('canText');
        obj.canText = canTextVal === 'true' || canTextVal === 'on' || canTextVal === '1';
    }
    {
        const canCallVal = formData.get('canCall');
        obj.canCall = canCallVal === 'true' || canCallVal === 'on' || canCallVal === '1';
    }
    return toExit(phoneNumberDeserialize(obj));
}

export const PhoneNumber = {
    defaultValue: phoneNumberDefaultValue,
    serialize: phoneNumberSerialize,
    serializeWithContext: phoneNumberSerializeWithContext,
    deserialize: phoneNumberDeserialize,
    deserializeWithContext: phoneNumberDeserializeWithContext,
    validateFields: phoneNumberValidateFields,
    hasShape: phoneNumberHasShape,
    is: phoneNumberIs,
    createForm: phoneNumberCreateForm,
    fromFormData: phoneNumberFromFormData
} as const;
