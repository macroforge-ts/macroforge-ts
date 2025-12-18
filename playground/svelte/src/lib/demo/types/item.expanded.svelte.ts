import { recordLinkDefaultValue } from './record-link.svelte';
import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { recordLinkDeserializeWithContext } from './record-link.svelte';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option as __gf_Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { Service } from './service.svelte';
import type { Product } from './product.svelte';
import type { RecordLink } from './record-link.svelte';

export type Item = RecordLink<Product> | /** @default */ RecordLink<Service>;

export function itemDefaultValue(): Item {
    return recordLinkDefaultValue<Service>();
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function itemSerialize(
    value: Item
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(itemSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function itemSerializeWithContext(value: Item, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function itemDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: Item }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = itemDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: '_root',
                        message: 'Item.deserialize: root cannot be a forward reference'
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
export function itemDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Item | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Item | __mf_PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Item.deserializeWithContext: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message: 'Item.deserializeWithContext: missing __type field for union dispatch'
            }
        ]);
    }
    if (__typeName === 'RecordLink<Product>') {
        return recordLinkDeserializeWithContext(value, ctx) as Item;
    }
    if (__typeName === 'RecordLink<Service>') {
        return recordLinkDeserializeWithContext(value, ctx) as Item;
    }
    throw new __mf_DeserializeError([
        {
            field: '_root',
            message:
                'Item.deserializeWithContext: unknown type "' +
                __typeName +
                '". Expected one of: RecordLink<Product>, RecordLink<Service>'
        }
    ]);
}
export function itemIs(value: unknown): value is Item {
    if (typeof value !== 'object' || value === null) {
        return false;
    }
    const __typeName = (value as any).__type;
    return __typeName === 'RecordLink<Product>' || __typeName === 'RecordLink<Service>';
}

/** Per-variant error types */ export type ItemRecordLinkProductErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ItemRecordLinkServiceErrors = {
    _errors: __gf_Option<Array<string>>;
}; /** Per-variant tainted types */
export type ItemRecordLinkProductTainted = {};
export type ItemRecordLinkServiceTainted = {}; /** Union error type */
export type ItemErrors =
    | ({ _type: 'RecordLink<Product>' } & ItemRecordLinkProductErrors)
    | ({ _type: 'RecordLink<Service>' } & ItemRecordLinkServiceErrors); /** Union tainted type */
export type ItemTainted =
    | ({ _type: 'RecordLink<Product>' } & ItemRecordLinkProductTainted)
    | ({
          _type: 'RecordLink<Service>';
      } & ItemRecordLinkServiceTainted); /** Per-variant field controller types */
export interface ItemRecordLinkProductFieldControllers {}
export interface ItemRecordLinkServiceFieldControllers {} /** Union Gigaform interface with variant switching */
export interface ItemGigaform {
    readonly currentVariant: 'RecordLink<Product>' | 'RecordLink<Service>';
    readonly data: Item;
    readonly errors: ItemErrors;
    readonly tainted: ItemTainted;
    readonly variants: ItemVariantFields;
    switchVariant(variant: 'RecordLink<Product>' | 'RecordLink<Service>'): void;
    validate(): Exit<Item, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Item>): void;
} /** Variant fields container */
export interface ItemVariantFields {
    readonly 'RecordLink<Product>': { readonly fields: ItemRecordLinkProductFieldControllers };
    readonly 'RecordLink<Service>': { readonly fields: ItemRecordLinkServiceFieldControllers };
} /** Gets default value for a specific variant */
function itemGetDefaultForVariant(variant: string): Item {
    switch (variant) {
        case 'RecordLink<Product>':
            return recordLinkDefaultValue<Product>() as Item;
        case 'RecordLink<Service>':
            return recordLinkDefaultValue<Service>() as Item;
        default:
            return recordLinkDefaultValue<Product>() as Item;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function itemCreateForm(initial?: Item): ItemGigaform {
    const initialVariant: 'RecordLink<Product>' | 'RecordLink<Service>' = 'RecordLink<Product>';
    let currentVariant = $state<'RecordLink<Product>' | 'RecordLink<Service>'>(initialVariant);
    let data = $state<Item>(initial ?? itemGetDefaultForVariant(initialVariant));
    let errors = $state<ItemErrors>({} as ItemErrors);
    let tainted = $state<ItemTainted>({} as ItemTainted);
    const variants: ItemVariantFields = {
        'RecordLink<Product>': { fields: {} as ItemRecordLinkProductFieldControllers },
        'RecordLink<Service>': { fields: {} as ItemRecordLinkServiceFieldControllers }
    };
    function switchVariant(variant: 'RecordLink<Product>' | 'RecordLink<Service>'): void {
        currentVariant = variant;
        data = itemGetDefaultForVariant(variant);
        errors = {} as ItemErrors;
        tainted = {} as ItemTainted;
    }
    function validate(): Exit<Item, Array<{ field: string; message: string }>> {
        return toExit(itemDeserialize(data));
    }
    function reset(overrides?: Partial<Item>): void {
        data = overrides ? (overrides as typeof data) : itemGetDefaultForVariant(currentVariant);
        errors = {} as ItemErrors;
        tainted = {} as ItemTainted;
    }
    return {
        get currentVariant() {
            return currentVariant;
        },
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
        variants,
        switchVariant,
        validate,
        reset
    };
} /** Parses FormData for union type, determining variant from discriminant field */
export function itemFromFormData(
    formData: FormData
): Exit<Item, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_type') as
        | 'RecordLink<Product>'
        | 'RecordLink<Service>'
        | null;
    if (!discriminant) {
        return toExit({
            success: false,
            errors: [{ field: '_type', message: 'Missing discriminant field' }]
        });
    }
    const obj: Record<string, unknown> = {};
    obj._type = discriminant;
    if (discriminant === 'RecordLink<Product>') {
    } else if (discriminant === 'RecordLink<Service>') {
    }
    return toExit(itemDeserialize(obj));
}

export const Item = {
    defaultValue: itemDefaultValue,
    serialize: itemSerialize,
    serializeWithContext: itemSerializeWithContext,
    deserialize: itemDeserialize,
    deserializeWithContext: itemDeserializeWithContext,
    is: itemIs,
    createForm: itemCreateForm,
    fromFormData: itemFromFormData
} as const;
