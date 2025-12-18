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

export interface Address {
    street: string;

    city: string;

    state: string;

    zipcode: string;
}

export function addressDefaultValue(): Address {
    return { street: '', city: '', state: '', zipcode: '' } as Address;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function addressSerialize(
    value: Address
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(addressSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function addressSerializeWithContext(
    value: Address,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Address', __id };
    result['street'] = value.street;
    result['city'] = value.city;
    result['state'] = value.state;
    result['zipcode'] = value.zipcode;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function addressDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Address> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = addressDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'Address.deserialize: root cannot be a forward reference'
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
export function addressDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Address | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Address.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('street' in obj)) {
        errors.push({ field: 'street', message: 'missing required field' });
    }
    if (!('city' in obj)) {
        errors.push({ field: 'city', message: 'missing required field' });
    }
    if (!('state' in obj)) {
        errors.push({ field: 'state', message: 'missing required field' });
    }
    if (!('zipcode' in obj)) {
        errors.push({ field: 'zipcode', message: 'missing required field' });
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
        const __raw_street = obj['street'] as string;
        if (__raw_street.length === 0) {
            errors.push({ field: 'street', message: 'must not be empty' });
        }
        instance.street = __raw_street;
    }
    {
        const __raw_city = obj['city'] as string;
        if (__raw_city.length === 0) {
            errors.push({ field: 'city', message: 'must not be empty' });
        }
        instance.city = __raw_city;
    }
    {
        const __raw_state = obj['state'] as string;
        if (__raw_state.length === 0) {
            errors.push({ field: 'state', message: 'must not be empty' });
        }
        instance.state = __raw_state;
    }
    {
        const __raw_zipcode = obj['zipcode'] as string;
        if (__raw_zipcode.length === 0) {
            errors.push({ field: 'zipcode', message: 'must not be empty' });
        }
        instance.zipcode = __raw_zipcode;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Address;
}
export function addressValidateField<K extends keyof Address>(
    field: K,
    value: Address[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'street': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'street', message: 'must not be empty' });
            }
            break;
        }
        case 'city': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'city', message: 'must not be empty' });
            }
            break;
        }
        case 'state': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'state', message: 'must not be empty' });
            }
            break;
        }
        case 'zipcode': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'zipcode', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function addressValidateFields(
    partial: Partial<Address>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('street' in partial && partial.street !== undefined) {
        const __val = partial.street as string;
        if (__val.length === 0) {
            errors.push({ field: 'street', message: 'must not be empty' });
        }
    }
    if ('city' in partial && partial.city !== undefined) {
        const __val = partial.city as string;
        if (__val.length === 0) {
            errors.push({ field: 'city', message: 'must not be empty' });
        }
    }
    if ('state' in partial && partial.state !== undefined) {
        const __val = partial.state as string;
        if (__val.length === 0) {
            errors.push({ field: 'state', message: 'must not be empty' });
        }
    }
    if ('zipcode' in partial && partial.zipcode !== undefined) {
        const __val = partial.zipcode as string;
        if (__val.length === 0) {
            errors.push({ field: 'zipcode', message: 'must not be empty' });
        }
    }
    return errors;
}
export function addressHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'street' in o && 'city' in o && 'state' in o && 'zipcode' in o;
}
export function addressIs(obj: unknown): obj is Address {
    if (!addressHasShape(obj)) {
        return false;
    }
    const result = addressDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type AddressErrors = {
    _errors: Option<Array<string>>;
    street: Option<Array<string>>;
    city: Option<Array<string>>;
    state: Option<Array<string>>;
    zipcode: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type AddressTainted = {
    street: Option<boolean>;
    city: Option<boolean>;
    state: Option<boolean>;
    zipcode: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface AddressFieldControllers {
    readonly street: FieldController<string>;
    readonly city: FieldController<string>;
    readonly state: FieldController<string>;
    readonly zipcode: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface AddressGigaform {
    readonly data: Address;
    readonly errors: AddressErrors;
    readonly tainted: AddressTainted;
    readonly fields: AddressFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, Address>;
    reset(overrides?: Partial<Address>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function addressCreateForm(overrides?: Partial<Address>): AddressGigaform {
    let data = $state({ ...addressDefaultValue(), ...overrides });
    let errors = $state<AddressErrors>({
        _errors: optionNone(),
        street: optionNone(),
        city: optionNone(),
        state: optionNone(),
        zipcode: optionNone()
    });
    let tainted = $state<AddressTainted>({
        street: optionNone(),
        city: optionNone(),
        state: optionNone(),
        zipcode: optionNone()
    });
    const fields: AddressFieldControllers = {
        street: {
            path: ['street'] as const,
            name: 'street',
            constraints: { required: true },
            get: () => data.street,
            set: (value: string) => {
                data.street = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.street,
            setError: (value: Option<Array<string>>) => {
                errors.street = value;
            },
            getTainted: () => tainted.street,
            setTainted: (value: Option<boolean>) => {
                tainted.street = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = addressValidateField('street', data.street);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        city: {
            path: ['city'] as const,
            name: 'city',
            constraints: { required: true },
            get: () => data.city,
            set: (value: string) => {
                data.city = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.city,
            setError: (value: Option<Array<string>>) => {
                errors.city = value;
            },
            getTainted: () => tainted.city,
            setTainted: (value: Option<boolean>) => {
                tainted.city = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = addressValidateField('city', data.city);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        state: {
            path: ['state'] as const,
            name: 'state',
            constraints: { required: true },
            get: () => data.state,
            set: (value: string) => {
                data.state = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.state,
            setError: (value: Option<Array<string>>) => {
                errors.state = value;
            },
            getTainted: () => tainted.state,
            setTainted: (value: Option<boolean>) => {
                tainted.state = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = addressValidateField('state', data.state);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        zipcode: {
            path: ['zipcode'] as const,
            name: 'zipcode',
            constraints: { required: true },
            get: () => data.zipcode,
            set: (value: string) => {
                data.zipcode = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.zipcode,
            setError: (value: Option<Array<string>>) => {
                errors.zipcode = value;
            },
            getTainted: () => tainted.zipcode,
            setTainted: (value: Option<boolean>) => {
                tainted.zipcode = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = addressValidateField('zipcode', data.zipcode);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, Address> {
        return addressDeserialize(data);
    }
    function reset(newOverrides?: Partial<Address>): void {
        data = { ...addressDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            street: optionNone(),
            city: optionNone(),
            state: optionNone(),
            zipcode: optionNone()
        };
        tainted = {
            street: optionNone(),
            city: optionNone(),
            state: optionNone(),
            zipcode: optionNone()
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
export function addressFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, Address> {
    const obj: Record<string, unknown> = {};
    obj.street = formData.get('street') ?? '';
    obj.city = formData.get('city') ?? '';
    obj.state = formData.get('state') ?? '';
    obj.zipcode = formData.get('zipcode') ?? '';
    return addressDeserialize(obj);
}

export const Address = {
    defaultValue: addressDefaultValue,
    serialize: addressSerialize,
    serializeWithContext: addressSerializeWithContext,
    deserialize: addressDeserialize,
    deserializeWithContext: addressDeserializeWithContext,
    validateFields: addressValidateFields,
    hasShape: addressHasShape,
    is: addressIs,
    createForm: addressCreateForm,
    fromFormData: addressFromFormData
} as const;
