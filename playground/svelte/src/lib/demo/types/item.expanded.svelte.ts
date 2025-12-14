import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import { Service } from './service.svelte';
import { Product } from './product.svelte';
import { RecordLink } from './record-link.svelte';

export type Item = RecordLink<Product> | /** @default */ RecordLink<Service>;

export function defaultValueItem(): Item {
    return RecordLink.defaultValue<Service>();
}

export function toStringifiedJSONItem(value: Item): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeItem(value, ctx));
}
export function toObjectItem(value: Item): unknown {
    const ctx = SerializeContext.create();
    return __serializeItem(value, ctx);
}
export function __serializeItem(value: Item, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONItem(
    json: string,
    opts?: DeserializeOptions
): Result<Item, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectItem(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectItem(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Item, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeItem(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Item.fromObject: root cannot be a forward reference' }
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
}
export function __deserializeItem(value: any, ctx: DeserializeContext): Item | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Item | PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new DeserializeError([
            { field: '_root', message: 'Item.__deserialize: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new DeserializeError([
            {
                field: '_root',
                message: 'Item.__deserialize: missing __type field for union dispatch'
            }
        ]);
    }
    if (__typeName === 'RecordLink<Product>') {
        if (typeof (RecordLink as any)?.__deserialize === 'function') {
            return (RecordLink as any).__deserialize(value, ctx) as Item;
        }
        return value as Item;
    }
    if (__typeName === 'RecordLink<Service>') {
        if (typeof (RecordLink as any)?.__deserialize === 'function') {
            return (RecordLink as any).__deserialize(value, ctx) as Item;
        }
        return value as Item;
    }
    throw new DeserializeError([
        {
            field: '_root',
            message:
                'Item.__deserialize: unknown type "' +
                __typeName +
                '". Expected one of: RecordLink<Product>, RecordLink<Service>'
        }
    ]);
}
export function isItem(value: unknown): value is Item {
    if (typeof value !== 'object' || value === null) {
        return false;
    }
    const __typeName = (value as any).__type;
    return __typeName === 'RecordLink<Product>' || __typeName === 'RecordLink<Service>';
}

/** Per-variant error types */ export type RecordLinkProductErrorsItem = {
    _errors: Option<Array<string>>;
};
export type RecordLinkServiceErrorsItem = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type RecordLinkProductTaintedItem = {};
export type RecordLinkServiceTaintedItem = {}; /** Union error type */
export type ErrorsItem =
    | ({ _type: 'RecordLink<Product>' } & RecordLinkProductErrorsItem)
    | ({ _type: 'RecordLink<Service>' } & RecordLinkServiceErrorsItem); /** Union tainted type */
export type TaintedItem =
    | ({ _type: 'RecordLink<Product>' } & RecordLinkProductTaintedItem)
    | ({
          _type: 'RecordLink<Service>';
      } & RecordLinkServiceTaintedItem); /** Per-variant field controller types */
export interface RecordLinkProductFieldControllersItem {}
export interface RecordLinkServiceFieldControllersItem {} /** Union Gigaform interface with variant switching */
export interface GigaformItem {
    readonly currentVariant: 'RecordLink<Product>' | 'RecordLink<Service>';
    readonly data: Item;
    readonly errors: ErrorsItem;
    readonly tainted: TaintedItem;
    readonly variants: VariantFieldsItem;
    switchVariant(variant: 'RecordLink<Product>' | 'RecordLink<Service>'): void;
    validate(): Result<Item, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Item>): void;
} /** Variant fields container */
export interface VariantFieldsItem {
    readonly 'RecordLink<Product>': { readonly fields: RecordLinkProductFieldControllersItem };
    readonly 'RecordLink<Service>': { readonly fields: RecordLinkServiceFieldControllersItem };
} /** Gets default value for a specific variant */
function getDefaultForVariantItem(variant: string): Item {
    switch (variant) {
        case 'RecordLink<Product>':
            return RecordLink.defaultValue<Product>() as Item;
        case 'RecordLink<Service>':
            return RecordLink.defaultValue<Service>() as Item;
        default:
            return RecordLink.defaultValue<Product>() as Item;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormItem(initial?: Item): GigaformItem {
    const initialVariant: 'RecordLink<Product>' | 'RecordLink<Service>' = 'RecordLink<Product>';
    let currentVariant = $state<'RecordLink<Product>' | 'RecordLink<Service>'>(initialVariant);
    let data = $state<Item>(initial ?? getDefaultForVariantItem(initialVariant));
    let errors = $state<ErrorsItem>({} as ErrorsItem);
    let tainted = $state<TaintedItem>({} as TaintedItem);
    const variants: VariantFieldsItem = {
        'RecordLink<Product>': {
            fields: {} as RecordLinkProductFieldControllersItem
        },
        'RecordLink<Service>': {
            fields: {} as RecordLinkServiceFieldControllersItem
        }
    };
    function switchVariant(variant: 'RecordLink<Product>' | 'RecordLink<Service>'): void {
        currentVariant = variant;
        data = getDefaultForVariantItem(variant);
        errors = {} as ErrorsItem;
        tainted = {} as TaintedItem;
    }
    function validate(): Result<Item, Array<{ field: string; message: string }>> {
        return Item.fromObject(data);
    }
    function reset(overrides?: Partial<Item>): void {
        data = overrides ? (overrides as typeof data) : getDefaultForVariantItem(currentVariant);
        errors = {} as ErrorsItem;
        tainted = {} as TaintedItem;
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
export function fromFormDataItem(
    formData: FormData
): Result<Item, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_type') as
        | 'RecordLink<Product>'
        | 'RecordLink<Service>'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_type', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._type = discriminant;
    if (discriminant === 'RecordLink<Product>') {
    } else if (discriminant === 'RecordLink<Service>') {
    }
    return Item.fromStringifiedJSON(JSON.stringify(obj));
}
