import { productDefaultsDefaultValue } from './product-defaults.svelte';
import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { productDefaultsSerializeWithContext } from './product-defaults.svelte';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { productDefaultsDeserializeWithContext } from './product-defaults.svelte';
import type { Exit } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { ProductDefaults } from './product-defaults.svelte';

export interface Product {
    id: string;

    name: string;

    quickCode: string;

    group: string | null;

    subgroup: string | null;

    unit: string | null;

    active: boolean;

    commission: boolean;

    favorite: boolean;
    defaults: ProductDefaults;
}

export function productDefaultValue(): Product {
    return {
        id: '',
        name: '',
        quickCode: '',
        group: null,
        subgroup: null,
        unit: null,
        active: false,
        commission: false,
        favorite: false,
        defaults: productDefaultsDefaultValue()
    } as Product;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function productSerialize(
    value: Product
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(productSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function productSerializeWithContext(
    value: Product,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Product', __id };
    result['id'] = value.id;
    result['name'] = value.name;
    result['quickCode'] = value.quickCode;
    result['group'] = value.group;
    result['subgroup'] = value.subgroup;
    result['unit'] = value.unit;
    result['active'] = value.active;
    result['commission'] = value.commission;
    result['favorite'] = value.favorite;
    result['defaults'] = productDefaultsSerializeWithContext(value.defaults, ctx);
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function productDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Product> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = productDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'Product.deserialize: root cannot be a forward reference'
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
export function productDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Product | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Product.deserializeWithContext: expected an object' }
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
    if (!('quickCode' in obj)) {
        errors.push({ field: 'quickCode', message: 'missing required field' });
    }
    if (!('group' in obj)) {
        errors.push({ field: 'group', message: 'missing required field' });
    }
    if (!('subgroup' in obj)) {
        errors.push({ field: 'subgroup', message: 'missing required field' });
    }
    if (!('unit' in obj)) {
        errors.push({ field: 'unit', message: 'missing required field' });
    }
    if (!('active' in obj)) {
        errors.push({ field: 'active', message: 'missing required field' });
    }
    if (!('commission' in obj)) {
        errors.push({ field: 'commission', message: 'missing required field' });
    }
    if (!('favorite' in obj)) {
        errors.push({ field: 'favorite', message: 'missing required field' });
    }
    if (!('defaults' in obj)) {
        errors.push({ field: 'defaults', message: 'missing required field' });
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
        const __raw_quickCode = obj['quickCode'] as string;
        if (__raw_quickCode.length === 0) {
            errors.push({ field: 'quickCode', message: 'must not be empty' });
        }
        instance.quickCode = __raw_quickCode;
    }
    {
        const __raw_group = obj['group'] as string | null;
        instance.group = __raw_group;
    }
    {
        const __raw_subgroup = obj['subgroup'] as string | null;
        instance.subgroup = __raw_subgroup;
    }
    {
        const __raw_unit = obj['unit'] as string | null;
        instance.unit = __raw_unit;
    }
    {
        const __raw_active = obj['active'] as boolean;
        instance.active = __raw_active;
    }
    {
        const __raw_commission = obj['commission'] as boolean;
        instance.commission = __raw_commission;
    }
    {
        const __raw_favorite = obj['favorite'] as boolean;
        instance.favorite = __raw_favorite;
    }
    {
        const __raw_defaults = obj['defaults'] as ProductDefaults;
        {
            const __result = productDefaultsDeserializeWithContext(__raw_defaults, ctx);
            ctx.assignOrDefer(instance, 'defaults', __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Product;
}
export function productValidateField<K extends keyof Product>(
    field: K,
    value: Product[K]
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
        case 'quickCode': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'quickCode', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function productValidateFields(
    partial: Partial<Product>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('name' in partial && partial.name !== undefined) {
        const __val = partial.name as string;
        if (__val.length === 0) {
            errors.push({ field: 'name', message: 'must not be empty' });
        }
    }
    if ('quickCode' in partial && partial.quickCode !== undefined) {
        const __val = partial.quickCode as string;
        if (__val.length === 0) {
            errors.push({ field: 'quickCode', message: 'must not be empty' });
        }
    }
    return errors;
}
export function productHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'id' in o &&
        'name' in o &&
        'quickCode' in o &&
        'group' in o &&
        'subgroup' in o &&
        'unit' in o &&
        'active' in o &&
        'commission' in o &&
        'favorite' in o &&
        'defaults' in o
    );
}
export function productIs(obj: unknown): obj is Product {
    if (!productHasShape(obj)) {
        return false;
    }
    const result = productDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type ProductErrors = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    name: Option<Array<string>>;
    quickCode: Option<Array<string>>;
    group: Option<Array<string>>;
    subgroup: Option<Array<string>>;
    unit: Option<Array<string>>;
    active: Option<Array<string>>;
    commission: Option<Array<string>>;
    favorite: Option<Array<string>>;
    defaults: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type ProductTainted = {
    id: Option<boolean>;
    name: Option<boolean>;
    quickCode: Option<boolean>;
    group: Option<boolean>;
    subgroup: Option<boolean>;
    unit: Option<boolean>;
    active: Option<boolean>;
    commission: Option<boolean>;
    favorite: Option<boolean>;
    defaults: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface ProductFieldControllers {
    readonly id: FieldController<string>;
    readonly name: FieldController<string>;
    readonly quickCode: FieldController<string>;
    readonly group: FieldController<string | null>;
    readonly subgroup: FieldController<string | null>;
    readonly unit: FieldController<string | null>;
    readonly active: FieldController<boolean>;
    readonly commission: FieldController<boolean>;
    readonly favorite: FieldController<boolean>;
    readonly defaults: FieldController<ProductDefaults>;
} /** Gigaform instance containing reactive state and field controllers */
export interface ProductGigaform {
    readonly data: Product;
    readonly errors: ProductErrors;
    readonly tainted: ProductTainted;
    readonly fields: ProductFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, Product>;
    reset(overrides?: Partial<Product>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function productCreateForm(overrides?: Partial<Product>): ProductGigaform {
    let data = $state({ ...productDefaultValue(), ...overrides });
    let errors = $state<ProductErrors>({
        _errors: optionNone(),
        id: optionNone(),
        name: optionNone(),
        quickCode: optionNone(),
        group: optionNone(),
        subgroup: optionNone(),
        unit: optionNone(),
        active: optionNone(),
        commission: optionNone(),
        favorite: optionNone(),
        defaults: optionNone()
    });
    let tainted = $state<ProductTainted>({
        id: optionNone(),
        name: optionNone(),
        quickCode: optionNone(),
        group: optionNone(),
        subgroup: optionNone(),
        unit: optionNone(),
        active: optionNone(),
        commission: optionNone(),
        favorite: optionNone(),
        defaults: optionNone()
    });
    const fields: ProductFieldControllers = {
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
                const fieldErrors = productValidateField('id', data.id);
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
                const fieldErrors = productValidateField('name', data.name);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        quickCode: {
            path: ['quickCode'] as const,
            name: 'quickCode',
            constraints: { required: true },
            label: 'Quick Code',
            get: () => data.quickCode,
            set: (value: string) => {
                data.quickCode = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.quickCode,
            setError: (value: Option<Array<string>>) => {
                errors.quickCode = value;
            },
            getTainted: () => tainted.quickCode,
            setTainted: (value: Option<boolean>) => {
                tainted.quickCode = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = productValidateField('quickCode', data.quickCode);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        group: {
            path: ['group'] as const,
            name: 'group',
            constraints: { required: true },
            label: 'Group',
            get: () => data.group,
            set: (value: string | null) => {
                data.group = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.group,
            setError: (value: Option<Array<string>>) => {
                errors.group = value;
            },
            getTainted: () => tainted.group,
            setTainted: (value: Option<boolean>) => {
                tainted.group = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = productValidateField('group', data.group);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        subgroup: {
            path: ['subgroup'] as const,
            name: 'subgroup',
            constraints: { required: true },
            label: 'Subgroup',
            get: () => data.subgroup,
            set: (value: string | null) => {
                data.subgroup = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.subgroup,
            setError: (value: Option<Array<string>>) => {
                errors.subgroup = value;
            },
            getTainted: () => tainted.subgroup,
            setTainted: (value: Option<boolean>) => {
                tainted.subgroup = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = productValidateField('subgroup', data.subgroup);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        unit: {
            path: ['unit'] as const,
            name: 'unit',
            constraints: { required: true },
            label: 'Unit',
            get: () => data.unit,
            set: (value: string | null) => {
                data.unit = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.unit,
            setError: (value: Option<Array<string>>) => {
                errors.unit = value;
            },
            getTainted: () => tainted.unit,
            setTainted: (value: Option<boolean>) => {
                tainted.unit = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = productValidateField('unit', data.unit);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        active: {
            path: ['active'] as const,
            name: 'active',
            constraints: { required: true },
            label: 'Active',
            get: () => data.active,
            set: (value: boolean) => {
                data.active = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.active,
            setError: (value: Option<Array<string>>) => {
                errors.active = value;
            },
            getTainted: () => tainted.active,
            setTainted: (value: Option<boolean>) => {
                tainted.active = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = productValidateField('active', data.active);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        commission: {
            path: ['commission'] as const,
            name: 'commission',
            constraints: { required: true },
            label: 'Commission',
            get: () => data.commission,
            set: (value: boolean) => {
                data.commission = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.commission,
            setError: (value: Option<Array<string>>) => {
                errors.commission = value;
            },
            getTainted: () => tainted.commission,
            setTainted: (value: Option<boolean>) => {
                tainted.commission = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = productValidateField('commission', data.commission);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        favorite: {
            path: ['favorite'] as const,
            name: 'favorite',
            constraints: { required: true },
            label: 'Favorite',
            get: () => data.favorite,
            set: (value: boolean) => {
                data.favorite = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.favorite,
            setError: (value: Option<Array<string>>) => {
                errors.favorite = value;
            },
            getTainted: () => tainted.favorite,
            setTainted: (value: Option<boolean>) => {
                tainted.favorite = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = productValidateField('favorite', data.favorite);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        defaults: {
            path: ['defaults'] as const,
            name: 'defaults',
            constraints: { required: true },
            get: () => data.defaults,
            set: (value: ProductDefaults) => {
                data.defaults = value;
            },
            transform: (value: ProductDefaults): ProductDefaults => value,
            getError: () => errors.defaults,
            setError: (value: Option<Array<string>>) => {
                errors.defaults = value;
            },
            getTainted: () => tainted.defaults,
            setTainted: (value: Option<boolean>) => {
                tainted.defaults = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = productValidateField('defaults', data.defaults);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, Product> {
        return productDeserialize(data);
    }
    function reset(newOverrides?: Partial<Product>): void {
        data = { ...productDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            id: optionNone(),
            name: optionNone(),
            quickCode: optionNone(),
            group: optionNone(),
            subgroup: optionNone(),
            unit: optionNone(),
            active: optionNone(),
            commission: optionNone(),
            favorite: optionNone(),
            defaults: optionNone()
        };
        tainted = {
            id: optionNone(),
            name: optionNone(),
            quickCode: optionNone(),
            group: optionNone(),
            subgroup: optionNone(),
            unit: optionNone(),
            active: optionNone(),
            commission: optionNone(),
            favorite: optionNone(),
            defaults: optionNone()
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
export function productFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, Product> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.name = formData.get('name') ?? '';
    obj.quickCode = formData.get('quickCode') ?? '';
    obj.group = formData.get('group') ?? '';
    obj.subgroup = formData.get('subgroup') ?? '';
    obj.unit = formData.get('unit') ?? '';
    {
        const activeVal = formData.get('active');
        obj.active = activeVal === 'true' || activeVal === 'on' || activeVal === '1';
    }
    {
        const commissionVal = formData.get('commission');
        obj.commission =
            commissionVal === 'true' || commissionVal === 'on' || commissionVal === '1';
    }
    {
        const favoriteVal = formData.get('favorite');
        obj.favorite = favoriteVal === 'true' || favoriteVal === 'on' || favoriteVal === '1';
    }
    {
        const defaultsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('defaults.')) {
                const fieldName = key.slice('defaults.'.length);
                const parts = fieldName.split('.');
                let current = defaultsObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.defaults = defaultsObj;
    }
    return productDeserialize(obj);
}

export const Product = {
    defaultValue: productDefaultValue,
    serialize: productSerialize,
    serializeWithContext: productSerializeWithContext,
    deserialize: productDeserialize,
    deserializeWithContext: productDeserializeWithContext,
    validateFields: productValidateFields,
    hasShape: productHasShape,
    is: productIs,
    createForm: productCreateForm,
    fromFormData: productFromFormData
} as const;
