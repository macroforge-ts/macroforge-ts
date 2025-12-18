import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { itemSerializeWithContext } from './item.svelte';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { itemDeserializeWithContext } from './item.svelte';
import type { Exit } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { Item } from './item.svelte';

export interface BilledItem {
    item: Item;

    quantity: number;

    taxed: boolean;

    upsale: boolean;
}

export function billedItemDefaultValue(): BilledItem {
    return { item: '', quantity: 0, taxed: false, upsale: false } as BilledItem;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function billedItemSerialize(
    value: BilledItem
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(billedItemSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function billedItemSerializeWithContext(
    value: BilledItem,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'BilledItem', __id };
    result['item'] = itemSerializeWithContext(value.item, ctx);
    result['quantity'] = value.quantity;
    result['taxed'] = value.taxed;
    result['upsale'] = value.upsale;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function billedItemDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, BilledItem> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = billedItemDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'BilledItem.deserialize: root cannot be a forward reference'
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
export function billedItemDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): BilledItem | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'BilledItem.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('item' in obj)) {
        errors.push({ field: 'item', message: 'missing required field' });
    }
    if (!('quantity' in obj)) {
        errors.push({ field: 'quantity', message: 'missing required field' });
    }
    if (!('taxed' in obj)) {
        errors.push({ field: 'taxed', message: 'missing required field' });
    }
    if (!('upsale' in obj)) {
        errors.push({ field: 'upsale', message: 'missing required field' });
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
        const __raw_item = obj['item'] as Item;
        {
            const __result = itemDeserializeWithContext(__raw_item, ctx);
            ctx.assignOrDefer(instance, 'item', __result);
        }
    }
    {
        const __raw_quantity = obj['quantity'] as number;
        instance.quantity = __raw_quantity;
    }
    {
        const __raw_taxed = obj['taxed'] as boolean;
        instance.taxed = __raw_taxed;
    }
    {
        const __raw_upsale = obj['upsale'] as boolean;
        instance.upsale = __raw_upsale;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as BilledItem;
}
export function billedItemValidateField<K extends keyof BilledItem>(
    field: K,
    value: BilledItem[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function billedItemValidateFields(
    partial: Partial<BilledItem>
): Array<{ field: string; message: string }> {
    return [];
}
export function billedItemHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'item' in o && 'quantity' in o && 'taxed' in o && 'upsale' in o;
}
export function billedItemIs(obj: unknown): obj is BilledItem {
    if (!billedItemHasShape(obj)) {
        return false;
    }
    const result = billedItemDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type BilledItemErrors = {
    _errors: Option<Array<string>>;
    item: Option<Array<string>>;
    quantity: Option<Array<string>>;
    taxed: Option<Array<string>>;
    upsale: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type BilledItemTainted = {
    item: Option<boolean>;
    quantity: Option<boolean>;
    taxed: Option<boolean>;
    upsale: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface BilledItemFieldControllers {
    readonly item: FieldController<Item>;
    readonly quantity: FieldController<number>;
    readonly taxed: FieldController<boolean>;
    readonly upsale: FieldController<boolean>;
} /** Gigaform instance containing reactive state and field controllers */
export interface BilledItemGigaform {
    readonly data: BilledItem;
    readonly errors: BilledItemErrors;
    readonly tainted: BilledItemTainted;
    readonly fields: BilledItemFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, BilledItem>;
    reset(overrides?: Partial<BilledItem>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function billedItemCreateForm(overrides?: Partial<BilledItem>): BilledItemGigaform {
    let data = $state({ ...billedItemDefaultValue(), ...overrides });
    let errors = $state<BilledItemErrors>({
        _errors: optionNone(),
        item: optionNone(),
        quantity: optionNone(),
        taxed: optionNone(),
        upsale: optionNone()
    });
    let tainted = $state<BilledItemTainted>({
        item: optionNone(),
        quantity: optionNone(),
        taxed: optionNone(),
        upsale: optionNone()
    });
    const fields: BilledItemFieldControllers = {
        item: {
            path: ['item'] as const,
            name: 'item',
            constraints: { required: true },
            label: 'Item',
            get: () => data.item,
            set: (value: Item) => {
                data.item = value;
            },
            transform: (value: Item): Item => value,
            getError: () => errors.item,
            setError: (value: Option<Array<string>>) => {
                errors.item = value;
            },
            getTainted: () => tainted.item,
            setTainted: (value: Option<boolean>) => {
                tainted.item = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = billedItemValidateField('item', data.item);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        quantity: {
            path: ['quantity'] as const,
            name: 'quantity',
            constraints: { required: true },
            label: 'Quantity',
            get: () => data.quantity,
            set: (value: number) => {
                data.quantity = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.quantity,
            setError: (value: Option<Array<string>>) => {
                errors.quantity = value;
            },
            getTainted: () => tainted.quantity,
            setTainted: (value: Option<boolean>) => {
                tainted.quantity = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = billedItemValidateField('quantity', data.quantity);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        taxed: {
            path: ['taxed'] as const,
            name: 'taxed',
            constraints: { required: true },
            label: 'Taxed',
            get: () => data.taxed,
            set: (value: boolean) => {
                data.taxed = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.taxed,
            setError: (value: Option<Array<string>>) => {
                errors.taxed = value;
            },
            getTainted: () => tainted.taxed,
            setTainted: (value: Option<boolean>) => {
                tainted.taxed = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = billedItemValidateField('taxed', data.taxed);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        upsale: {
            path: ['upsale'] as const,
            name: 'upsale',
            constraints: { required: true },
            label: 'Upsale',
            get: () => data.upsale,
            set: (value: boolean) => {
                data.upsale = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.upsale,
            setError: (value: Option<Array<string>>) => {
                errors.upsale = value;
            },
            getTainted: () => tainted.upsale,
            setTainted: (value: Option<boolean>) => {
                tainted.upsale = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = billedItemValidateField('upsale', data.upsale);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, BilledItem> {
        return billedItemDeserialize(data);
    }
    function reset(newOverrides?: Partial<BilledItem>): void {
        data = { ...billedItemDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            item: optionNone(),
            quantity: optionNone(),
            taxed: optionNone(),
            upsale: optionNone()
        };
        tainted = {
            item: optionNone(),
            quantity: optionNone(),
            taxed: optionNone(),
            upsale: optionNone()
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
export function billedItemFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, BilledItem> {
    const obj: Record<string, unknown> = {};
    {
        const itemObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('item.')) {
                const fieldName = key.slice('item.'.length);
                const parts = fieldName.split('.');
                let current = itemObj;
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
        obj.item = itemObj;
    }
    {
        const quantityStr = formData.get('quantity');
        obj.quantity = quantityStr ? parseFloat(quantityStr as string) : 0;
        if (obj.quantity !== undefined && isNaN(obj.quantity as number)) obj.quantity = 0;
    }
    {
        const taxedVal = formData.get('taxed');
        obj.taxed = taxedVal === 'true' || taxedVal === 'on' || taxedVal === '1';
    }
    {
        const upsaleVal = formData.get('upsale');
        obj.upsale = upsaleVal === 'true' || upsaleVal === 'on' || upsaleVal === '1';
    }
    return billedItemDeserialize(obj);
}

export const BilledItem = {
    defaultValue: billedItemDefaultValue,
    serialize: billedItemSerialize,
    serializeWithContext: billedItemSerializeWithContext,
    deserialize: billedItemDeserialize,
    deserializeWithContext: billedItemDeserializeWithContext,
    validateFields: billedItemValidateFields,
    hasShape: billedItemHasShape,
    is: billedItemIs,
    createForm: billedItemCreateForm,
    fromFormData: billedItemFromFormData
} as const;
