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

export interface ServiceDefaults {
    price: number;

    description: string;
}

export function serviceDefaultsDefaultValue(): ServiceDefaults {
    return { price: 0, description: '' } as ServiceDefaults;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function serviceDefaultsSerialize(
    value: ServiceDefaults
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(serviceDefaultsSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function serviceDefaultsSerializeWithContext(
    value: ServiceDefaults,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'ServiceDefaults', __id };
    result['price'] = value.price;
    result['description'] = value.description;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function serviceDefaultsDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, ServiceDefaults> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = serviceDefaultsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'ServiceDefaults.deserialize: root cannot be a forward reference'
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
export function serviceDefaultsDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): ServiceDefaults | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message: 'ServiceDefaults.deserializeWithContext: expected an object'
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('price' in obj)) {
        errors.push({ field: 'price', message: 'missing required field' });
    }
    if (!('description' in obj)) {
        errors.push({ field: 'description', message: 'missing required field' });
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
        const __raw_price = obj['price'] as number;
        instance.price = __raw_price;
    }
    {
        const __raw_description = obj['description'] as string;
        if (__raw_description.length === 0) {
            errors.push({ field: 'description', message: 'must not be empty' });
        }
        instance.description = __raw_description;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as ServiceDefaults;
}
export function serviceDefaultsValidateField<K extends keyof ServiceDefaults>(
    field: K,
    value: ServiceDefaults[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'description': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'description', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function serviceDefaultsValidateFields(
    partial: Partial<ServiceDefaults>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('description' in partial && partial.description !== undefined) {
        const __val = partial.description as string;
        if (__val.length === 0) {
            errors.push({ field: 'description', message: 'must not be empty' });
        }
    }
    return errors;
}
export function serviceDefaultsHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'price' in o && 'description' in o;
}
export function serviceDefaultsIs(obj: unknown): obj is ServiceDefaults {
    if (!serviceDefaultsHasShape(obj)) {
        return false;
    }
    const result = serviceDefaultsDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type ServiceDefaultsErrors = {
    _errors: Option<Array<string>>;
    price: Option<Array<string>>;
    description: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type ServiceDefaultsTainted = {
    price: Option<boolean>;
    description: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface ServiceDefaultsFieldControllers {
    readonly price: FieldController<number>;
    readonly description: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface ServiceDefaultsGigaform {
    readonly data: ServiceDefaults;
    readonly errors: ServiceDefaultsErrors;
    readonly tainted: ServiceDefaultsTainted;
    readonly fields: ServiceDefaultsFieldControllers;
    validate(): Result<ServiceDefaults, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<ServiceDefaults>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function serviceDefaultsCreateForm(
    overrides?: Partial<ServiceDefaults>
): ServiceDefaultsGigaform {
    let data = $state({ ...serviceDefaultsDefaultValue(), ...overrides });
    let errors = $state<ServiceDefaultsErrors>({
        _errors: Option.none(),
        price: Option.none(),
        description: Option.none()
    });
    let tainted = $state<ServiceDefaultsTainted>({
        price: Option.none(),
        description: Option.none()
    });
    const fields: ServiceDefaultsFieldControllers = {
        price: {
            path: ['price'] as const,
            name: 'price',
            constraints: { required: true },
            label: 'Price',
            get: () => data.price,
            set: (value: number) => {
                data.price = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.price,
            setError: (value: Option<Array<string>>) => {
                errors.price = value;
            },
            getTainted: () => tainted.price,
            setTainted: (value: Option<boolean>) => {
                tainted.price = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = serviceDefaultsValidateField('price', data.price);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        description: {
            path: ['description'] as const,
            name: 'description',
            constraints: { required: true },
            label: 'Description',
            get: () => data.description,
            set: (value: string) => {
                data.description = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.description,
            setError: (value: Option<Array<string>>) => {
                errors.description = value;
            },
            getTainted: () => tainted.description,
            setTainted: (value: Option<boolean>) => {
                tainted.description = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = serviceDefaultsValidateField('description', data.description);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<ServiceDefaults, Array<{ field: string; message: string }>> {
        return serviceDefaultsDeserialize(data);
    }
    function reset(newOverrides?: Partial<ServiceDefaults>): void {
        data = { ...serviceDefaultsDefaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), price: Option.none(), description: Option.none() };
        tainted = { price: Option.none(), description: Option.none() };
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
export function serviceDefaultsFromFormData(
    formData: FormData
): Result<ServiceDefaults, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const priceStr = formData.get('price');
        obj.price = priceStr ? parseFloat(priceStr as string) : 0;
        if (obj.price !== undefined && isNaN(obj.price as number)) obj.price = 0;
    }
    obj.description = formData.get('description') ?? '';
    return serviceDefaultsDeserialize(obj);
}

export const ServiceDefaults = {
    defaultValue: serviceDefaultsDefaultValue,
    serialize: serviceDefaultsSerialize,
    serializeWithContext: serviceDefaultsSerializeWithContext,
    deserialize: serviceDefaultsDeserialize,
    deserializeWithContext: serviceDefaultsDeserializeWithContext,
    validateFields: serviceDefaultsValidateFields,
    hasShape: serviceDefaultsHasShape,
    is: serviceDefaultsIs,
    createForm: serviceDefaultsCreateForm,
    fromFormData: serviceDefaultsFromFormData
} as const;
