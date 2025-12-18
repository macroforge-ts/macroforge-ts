import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { Result } from '@playground/macro/gigaform';
import { Option } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Number {
    countryCode: string;

    areaCode: string;

    localNumber: string;
}

export function numberDefaultValue(): Number {
    return { countryCode: '', areaCode: '', localNumber: '' } as Number;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function numberSerialize(
    value: Number
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(numberSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function numberSerializeWithContext(
    value: Number,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Number', __id };
    result['countryCode'] = value.countryCode;
    result['areaCode'] = value.areaCode;
    result['localNumber'] = value.localNumber;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function numberDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Number> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = numberDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'Number.deserialize: root cannot be a forward reference'
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
export function numberDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Number | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Number.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('countryCode' in obj)) {
        errors.push({ field: 'countryCode', message: 'missing required field' });
    }
    if (!('areaCode' in obj)) {
        errors.push({ field: 'areaCode', message: 'missing required field' });
    }
    if (!('localNumber' in obj)) {
        errors.push({ field: 'localNumber', message: 'missing required field' });
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
        const __raw_countryCode = obj['countryCode'] as string;
        if (__raw_countryCode.length === 0) {
            errors.push({ field: 'countryCode', message: 'must not be empty' });
        }
        instance.countryCode = __raw_countryCode;
    }
    {
        const __raw_areaCode = obj['areaCode'] as string;
        if (__raw_areaCode.length === 0) {
            errors.push({ field: 'areaCode', message: 'must not be empty' });
        }
        instance.areaCode = __raw_areaCode;
    }
    {
        const __raw_localNumber = obj['localNumber'] as string;
        if (__raw_localNumber.length === 0) {
            errors.push({ field: 'localNumber', message: 'must not be empty' });
        }
        instance.localNumber = __raw_localNumber;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Number;
}
export function numberValidateField<K extends keyof Number>(
    field: K,
    value: Number[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'countryCode': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'countryCode', message: 'must not be empty' });
            }
            break;
        }
        case 'areaCode': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'areaCode', message: 'must not be empty' });
            }
            break;
        }
        case 'localNumber': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'localNumber', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function numberValidateFields(
    partial: Partial<Number>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('countryCode' in partial && partial.countryCode !== undefined) {
        const __val = partial.countryCode as string;
        if (__val.length === 0) {
            errors.push({ field: 'countryCode', message: 'must not be empty' });
        }
    }
    if ('areaCode' in partial && partial.areaCode !== undefined) {
        const __val = partial.areaCode as string;
        if (__val.length === 0) {
            errors.push({ field: 'areaCode', message: 'must not be empty' });
        }
    }
    if ('localNumber' in partial && partial.localNumber !== undefined) {
        const __val = partial.localNumber as string;
        if (__val.length === 0) {
            errors.push({ field: 'localNumber', message: 'must not be empty' });
        }
    }
    return errors;
}
export function numberHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'countryCode' in o && 'areaCode' in o && 'localNumber' in o;
}
export function numberIs(obj: unknown): obj is Number {
    if (!numberHasShape(obj)) {
        return false;
    }
    const result = numberDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type NumberErrors = {
    _errors: Option<Array<string>>;
    countryCode: Option<Array<string>>;
    areaCode: Option<Array<string>>;
    localNumber: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type NumberTainted = {
    countryCode: Option<boolean>;
    areaCode: Option<boolean>;
    localNumber: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface NumberFieldControllers {
    readonly countryCode: FieldController<string>;
    readonly areaCode: FieldController<string>;
    readonly localNumber: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface NumberGigaform {
    readonly data: Number;
    readonly errors: NumberErrors;
    readonly tainted: NumberTainted;
    readonly fields: NumberFieldControllers;
    validate(): Result<Number, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Number>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function numberCreateForm(overrides?: Partial<Number>): NumberGigaform {
    let data = $state({ ...numberDefaultValue(), ...overrides });
    let errors = $state<NumberErrors>({
        _errors: Option.none(),
        countryCode: Option.none(),
        areaCode: Option.none(),
        localNumber: Option.none()
    });
    let tainted = $state<NumberTainted>({
        countryCode: Option.none(),
        areaCode: Option.none(),
        localNumber: Option.none()
    });
    const fields: NumberFieldControllers = {
        countryCode: {
            path: ['countryCode'] as const,
            name: 'countryCode',
            constraints: { required: true },
            get: () => data.countryCode,
            set: (value: string) => {
                data.countryCode = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.countryCode,
            setError: (value: Option<Array<string>>) => {
                errors.countryCode = value;
            },
            getTainted: () => tainted.countryCode,
            setTainted: (value: Option<boolean>) => {
                tainted.countryCode = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = numberValidateField('countryCode', data.countryCode);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        areaCode: {
            path: ['areaCode'] as const,
            name: 'areaCode',
            constraints: { required: true },
            get: () => data.areaCode,
            set: (value: string) => {
                data.areaCode = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.areaCode,
            setError: (value: Option<Array<string>>) => {
                errors.areaCode = value;
            },
            getTainted: () => tainted.areaCode,
            setTainted: (value: Option<boolean>) => {
                tainted.areaCode = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = numberValidateField('areaCode', data.areaCode);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        localNumber: {
            path: ['localNumber'] as const,
            name: 'localNumber',
            constraints: { required: true },
            get: () => data.localNumber,
            set: (value: string) => {
                data.localNumber = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.localNumber,
            setError: (value: Option<Array<string>>) => {
                errors.localNumber = value;
            },
            getTainted: () => tainted.localNumber,
            setTainted: (value: Option<boolean>) => {
                tainted.localNumber = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = numberValidateField('localNumber', data.localNumber);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Number, Array<{ field: string; message: string }>> {
        return numberDeserialize(data);
    }
    function reset(newOverrides?: Partial<Number>): void {
        data = { ...numberDefaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            countryCode: Option.none(),
            areaCode: Option.none(),
            localNumber: Option.none()
        };
        tainted = {
            countryCode: Option.none(),
            areaCode: Option.none(),
            localNumber: Option.none()
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
export function numberFromFormData(
    formData: FormData
): Result<Number, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.countryCode = formData.get('countryCode') ?? '';
    obj.areaCode = formData.get('areaCode') ?? '';
    obj.localNumber = formData.get('localNumber') ?? '';
    return numberDeserialize(obj);
}

export const Number = {
    defaultValue: numberDefaultValue,
    serialize: numberSerialize,
    serializeWithContext: numberSerializeWithContext,
    deserialize: numberDeserialize,
    deserializeWithContext: numberDeserializeWithContext,
    validateFields: numberValidateFields,
    hasShape: numberHasShape,
    is: numberIs,
    createForm: numberCreateForm,
    fromFormData: numberFromFormData
} as const;
