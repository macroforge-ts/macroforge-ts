import { SerializeContext } from 'macroforge/serde';
import { Exit } from 'macroforge/utils/effect';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface TaxRate {
    id: string;

    name: string;

    taxAgency: string;

    zip: number;

    city: string;

    county: string;

    state: string;

    isActive: boolean;

    description: string;

    taxComponents: { [key: string]: number };
}

export function taxRateDefaultValue(): TaxRate {
    return {
        id: '',
        name: '',
        taxAgency: '',
        zip: 0,
        city: '',
        county: '',
        state: '',
        isActive: false,
        description: '',
        taxComponents: {}
    } as TaxRate;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function taxRateSerialize(
    value: TaxRate
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(taxRateSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function taxRateSerializeWithContext(
    value: TaxRate,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'TaxRate', __id };
    result['id'] = value.id;
    result['name'] = value.name;
    result['taxAgency'] = value.taxAgency;
    result['zip'] = value.zip;
    result['city'] = value.city;
    result['county'] = value.county;
    result['state'] = value.state;
    result['isActive'] = value.isActive;
    result['description'] = value.description;
    result['taxComponents'] = value.taxComponents;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function taxRateDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Exit.Exit<Array<{ field: string; message: string }>, TaxRate> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = taxRateDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Exit.fail([
                {
                    field: '_root',
                    message: 'TaxRate.deserialize: root cannot be a forward reference'
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Exit.succeed(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Exit.fail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Exit.fail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function taxRateDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): TaxRate | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'TaxRate.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('name' in obj)) {
        errors.push({ field: 'name', message: 'missing required field' });
    }
    if (!('taxAgency' in obj)) {
        errors.push({ field: 'taxAgency', message: 'missing required field' });
    }
    if (!('zip' in obj)) {
        errors.push({ field: 'zip', message: 'missing required field' });
    }
    if (!('city' in obj)) {
        errors.push({ field: 'city', message: 'missing required field' });
    }
    if (!('county' in obj)) {
        errors.push({ field: 'county', message: 'missing required field' });
    }
    if (!('state' in obj)) {
        errors.push({ field: 'state', message: 'missing required field' });
    }
    if (!('isActive' in obj)) {
        errors.push({ field: 'isActive', message: 'missing required field' });
    }
    if (!('description' in obj)) {
        errors.push({ field: 'description', message: 'missing required field' });
    }
    if (!('taxComponents' in obj)) {
        errors.push({ field: 'taxComponents', message: 'missing required field' });
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
        const __raw_name = obj['name'] as string;
        if (__raw_name.length === 0) {
            errors.push({ field: 'name', message: 'must not be empty' });
        }
        instance.name = __raw_name;
    }
    {
        const __raw_taxAgency = obj['taxAgency'] as string;
        if (__raw_taxAgency.length === 0) {
            errors.push({ field: 'taxAgency', message: 'must not be empty' });
        }
        instance.taxAgency = __raw_taxAgency;
    }
    {
        const __raw_zip = obj['zip'] as number;
        instance.zip = __raw_zip;
    }
    {
        const __raw_city = obj['city'] as string;
        if (__raw_city.length === 0) {
            errors.push({ field: 'city', message: 'must not be empty' });
        }
        instance.city = __raw_city;
    }
    {
        const __raw_county = obj['county'] as string;
        if (__raw_county.length === 0) {
            errors.push({ field: 'county', message: 'must not be empty' });
        }
        instance.county = __raw_county;
    }
    {
        const __raw_state = obj['state'] as string;
        if (__raw_state.length === 0) {
            errors.push({ field: 'state', message: 'must not be empty' });
        }
        instance.state = __raw_state;
    }
    {
        const __raw_isActive = obj['isActive'] as boolean;
        instance.isActive = __raw_isActive;
    }
    {
        const __raw_description = obj['description'] as string;
        if (__raw_description.length === 0) {
            errors.push({ field: 'description', message: 'must not be empty' });
        }
        instance.description = __raw_description;
    }
    {
        const __raw_taxComponents = obj['taxComponents'] as { [key: string]: number };
        instance.taxComponents = __raw_taxComponents;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as TaxRate;
}
export function taxRateValidateField<K extends keyof TaxRate>(
    field: K,
    value: TaxRate[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'name': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'name', message: 'must not be empty' });
            }
            break;
        }
        case 'taxAgency': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'taxAgency', message: 'must not be empty' });
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
        case 'county': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'county', message: 'must not be empty' });
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
export function taxRateValidateFields(
    partial: Partial<TaxRate>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('name' in partial && partial.name !== undefined) {
        const __val = partial.name as string;
        if (__val.length === 0) {
            errors.push({ field: 'name', message: 'must not be empty' });
        }
    }
    if ('taxAgency' in partial && partial.taxAgency !== undefined) {
        const __val = partial.taxAgency as string;
        if (__val.length === 0) {
            errors.push({ field: 'taxAgency', message: 'must not be empty' });
        }
    }
    if ('city' in partial && partial.city !== undefined) {
        const __val = partial.city as string;
        if (__val.length === 0) {
            errors.push({ field: 'city', message: 'must not be empty' });
        }
    }
    if ('county' in partial && partial.county !== undefined) {
        const __val = partial.county as string;
        if (__val.length === 0) {
            errors.push({ field: 'county', message: 'must not be empty' });
        }
    }
    if ('state' in partial && partial.state !== undefined) {
        const __val = partial.state as string;
        if (__val.length === 0) {
            errors.push({ field: 'state', message: 'must not be empty' });
        }
    }
    if ('description' in partial && partial.description !== undefined) {
        const __val = partial.description as string;
        if (__val.length === 0) {
            errors.push({ field: 'description', message: 'must not be empty' });
        }
    }
    return errors;
}
export function taxRateHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'id' in o &&
        'name' in o &&
        'taxAgency' in o &&
        'zip' in o &&
        'city' in o &&
        'county' in o &&
        'state' in o &&
        'isActive' in o &&
        'description' in o &&
        'taxComponents' in o
    );
}
export function taxRateIs(obj: unknown): obj is TaxRate {
    if (!taxRateHasShape(obj)) {
        return false;
    }
    const result = taxRateDeserialize(obj);
    return Exit.isSuccess(result);
}

/** Nested error structure matching the data shape */ export type TaxRateErrors = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    name: Option<Array<string>>;
    taxAgency: Option<Array<string>>;
    zip: Option<Array<string>>;
    city: Option<Array<string>>;
    county: Option<Array<string>>;
    state: Option<Array<string>>;
    isActive: Option<Array<string>>;
    description: Option<Array<string>>;
    taxComponents: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaxRateTainted = {
    id: Option<boolean>;
    name: Option<boolean>;
    taxAgency: Option<boolean>;
    zip: Option<boolean>;
    city: Option<boolean>;
    county: Option<boolean>;
    state: Option<boolean>;
    isActive: Option<boolean>;
    description: Option<boolean>;
    taxComponents: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface TaxRateFieldControllers {
    readonly id: FieldController<string>;
    readonly name: FieldController<string>;
    readonly taxAgency: FieldController<string>;
    readonly zip: FieldController<number>;
    readonly city: FieldController<string>;
    readonly county: FieldController<string>;
    readonly state: FieldController<string>;
    readonly isActive: FieldController<boolean>;
    readonly description: FieldController<string>;
    readonly taxComponents: FieldController<{ [key: string]: number }>;
} /** Gigaform instance containing reactive state and field controllers */
export interface TaxRateGigaform {
    readonly data: TaxRate;
    readonly errors: TaxRateErrors;
    readonly tainted: TaxRateTainted;
    readonly fields: TaxRateFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, TaxRate>;
    reset(overrides?: Partial<TaxRate>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function taxRateCreateForm(overrides?: Partial<TaxRate>): TaxRateGigaform {
    let data = $state({ ...taxRateDefaultValue(), ...overrides });
    let errors = $state<TaxRateErrors>({
        _errors: optionNone(),
        id: optionNone(),
        name: optionNone(),
        taxAgency: optionNone(),
        zip: optionNone(),
        city: optionNone(),
        county: optionNone(),
        state: optionNone(),
        isActive: optionNone(),
        description: optionNone(),
        taxComponents: optionNone()
    });
    let tainted = $state<TaxRateTainted>({
        id: optionNone(),
        name: optionNone(),
        taxAgency: optionNone(),
        zip: optionNone(),
        city: optionNone(),
        county: optionNone(),
        state: optionNone(),
        isActive: optionNone(),
        description: optionNone(),
        taxComponents: optionNone()
    });
    const fields: TaxRateFieldControllers = {
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
                const fieldErrors = taxRateValidateField('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        name: {
            path: ['name'] as const,
            name: 'name',
            constraints: { required: true },
            label: 'Name',
            get: () => data.name,
            set: (value: string) => {
                data.name = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.name,
            setError: (value: Option<Array<string>>) => {
                errors.name = value;
            },
            getTainted: () => tainted.name,
            setTainted: (value: Option<boolean>) => {
                tainted.name = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = taxRateValidateField('name', data.name);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        taxAgency: {
            path: ['taxAgency'] as const,
            name: 'taxAgency',
            constraints: { required: true },
            label: 'Tax Agency',
            get: () => data.taxAgency,
            set: (value: string) => {
                data.taxAgency = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.taxAgency,
            setError: (value: Option<Array<string>>) => {
                errors.taxAgency = value;
            },
            getTainted: () => tainted.taxAgency,
            setTainted: (value: Option<boolean>) => {
                tainted.taxAgency = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = taxRateValidateField('taxAgency', data.taxAgency);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        zip: {
            path: ['zip'] as const,
            name: 'zip',
            constraints: { required: true },
            label: 'Zip',
            get: () => data.zip,
            set: (value: number) => {
                data.zip = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.zip,
            setError: (value: Option<Array<string>>) => {
                errors.zip = value;
            },
            getTainted: () => tainted.zip,
            setTainted: (value: Option<boolean>) => {
                tainted.zip = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = taxRateValidateField('zip', data.zip);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        city: {
            path: ['city'] as const,
            name: 'city',
            constraints: { required: true },
            label: 'City',
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
                const fieldErrors = taxRateValidateField('city', data.city);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        county: {
            path: ['county'] as const,
            name: 'county',
            constraints: { required: true },
            label: 'County',
            get: () => data.county,
            set: (value: string) => {
                data.county = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.county,
            setError: (value: Option<Array<string>>) => {
                errors.county = value;
            },
            getTainted: () => tainted.county,
            setTainted: (value: Option<boolean>) => {
                tainted.county = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = taxRateValidateField('county', data.county);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        state: {
            path: ['state'] as const,
            name: 'state',
            constraints: { required: true },
            label: 'State',
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
                const fieldErrors = taxRateValidateField('state', data.state);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        isActive: {
            path: ['isActive'] as const,
            name: 'isActive',
            constraints: { required: true },
            label: 'Active',
            get: () => data.isActive,
            set: (value: boolean) => {
                data.isActive = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.isActive,
            setError: (value: Option<Array<string>>) => {
                errors.isActive = value;
            },
            getTainted: () => tainted.isActive,
            setTainted: (value: Option<boolean>) => {
                tainted.isActive = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = taxRateValidateField('isActive', data.isActive);
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
                const fieldErrors = taxRateValidateField('description', data.description);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        taxComponents: {
            path: ['taxComponents'] as const,
            name: 'taxComponents',
            constraints: { required: true },
            get: () => data.taxComponents,
            set: (value: { [key: string]: number }) => {
                data.taxComponents = value;
            },
            transform: (value: { [key: string]: number }): { [key: string]: number } => value,
            getError: () => errors.taxComponents,
            setError: (value: Option<Array<string>>) => {
                errors.taxComponents = value;
            },
            getTainted: () => tainted.taxComponents,
            setTainted: (value: Option<boolean>) => {
                tainted.taxComponents = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = taxRateValidateField('taxComponents', data.taxComponents);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, TaxRate> {
        return toExit(taxRateDeserialize(data));
    }
    function reset(newOverrides?: Partial<TaxRate>): void {
        data = { ...taxRateDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            id: optionNone(),
            name: optionNone(),
            taxAgency: optionNone(),
            zip: optionNone(),
            city: optionNone(),
            county: optionNone(),
            state: optionNone(),
            isActive: optionNone(),
            description: optionNone(),
            taxComponents: optionNone()
        };
        tainted = {
            id: optionNone(),
            name: optionNone(),
            taxAgency: optionNone(),
            zip: optionNone(),
            city: optionNone(),
            county: optionNone(),
            state: optionNone(),
            isActive: optionNone(),
            description: optionNone(),
            taxComponents: optionNone()
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
export function taxRateFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, TaxRate> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.name = formData.get('name') ?? '';
    obj.taxAgency = formData.get('taxAgency') ?? '';
    {
        const zipStr = formData.get('zip');
        obj.zip = zipStr ? parseFloat(zipStr as string) : 0;
        if (obj.zip !== undefined && isNaN(obj.zip as number)) obj.zip = 0;
    }
    obj.city = formData.get('city') ?? '';
    obj.county = formData.get('county') ?? '';
    obj.state = formData.get('state') ?? '';
    {
        const isActiveVal = formData.get('isActive');
        obj.isActive = isActiveVal === 'true' || isActiveVal === 'on' || isActiveVal === '1';
    }
    obj.description = formData.get('description') ?? '';
    obj.taxComponents = formData.get('taxComponents') ?? '';
    return toExit(taxRateDeserialize(obj));
}

export const TaxRate = {
    defaultValue: taxRateDefaultValue,
    serialize: taxRateSerialize,
    serializeWithContext: taxRateSerializeWithContext,
    deserialize: taxRateDeserialize,
    deserializeWithContext: taxRateDeserializeWithContext,
    validateFields: taxRateValidateFields,
    hasShape: taxRateHasShape,
    is: taxRateIs,
    createForm: taxRateCreateForm,
    fromFormData: taxRateFromFormData
} as const;
