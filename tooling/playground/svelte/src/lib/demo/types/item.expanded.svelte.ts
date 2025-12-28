import { recordLinkDefaultValue } from "./record-link.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { recordLinkDeserializeWithContext } from "./record-link.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { Product } from './product.svelte';
import type { RecordLink } from './record-link.svelte';
import type { Service } from './service.svelte';


export type Item = RecordLink<Product> | /** @default */ RecordLink<Service>;

export function itemDefaultValue#0#0(): Item {
    return recordLinkDefaultValue<Service>();
}

export function itemSerialize(value: Item): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(itemSerializeWithContext(value, ctx));
}
export function itemSerializeWithContext(value: Item, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function itemDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Item } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = itemDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Item.deserialize: root cannot be a forward reference"
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return {
            success: true,
            value: resultOrRef
        };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return {
                success: false,
                errors: e.errors
            };
        }
        const message = e instanceof Error ? e.message : String(e);
        return {
            success: false,
            errors: [
                {
                    field: "_root",
                    message
                }
            ]
        };
    }
}
export function itemDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Item | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Item | __mf_PendingRef;
    }
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if (typeof __typeName === "string") {}
        if (__typeName === "RecordLink<Product>") {
            return recordLinkDeserializeWithContext(value, ctx) as Item;
        }
        if (__typeName === "RecordLink<Service>") {
            return recordLinkDeserializeWithContext(value, ctx) as Item;
        }
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"Item"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function itemIs(value: unknown): value is Item {
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if ('__typeName === "RecordLink<Product>" || __typeName === "RecordLink<Service>"') return true;
    }
    return false;
}
     }

export type ItemRecordLinkProductErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ItemRecordLinkServiceErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ItemRecordLinkProductTainted = {
};
export type ItemRecordLinkServiceTainted = {
};
export type ItemErrors = ({
    _type: "RecordLink<Product>";
} & ItemRecordLinkProductErrors) | ({
    _type: "RecordLink<Service>";
} & ItemRecordLinkServiceErrors);
export type ItemTainted = ({
    _type: "RecordLink<Product>";
} & ItemRecordLinkProductTainted) | ({
    _type: "RecordLink<Service>";
} & ItemRecordLinkServiceTainted);
export interface ItemRecordLinkProductFieldControllers {
}
export interface ItemRecordLinkServiceFieldControllers {
}
export interface ItemGigaform {
    readonly currentVariant: "RecordLink<Product>" | "RecordLink<Service>";
    readonly data: Item;
    readonly errors: ItemErrors;
    readonly tainted: ItemTainted;
    readonly variants: ItemVariantFields;
    switchVariant(variant: "RecordLink<Product>" | "RecordLink<Service>"): void;
    validate(): Exit<Item, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Item>): void;
}
export interface ItemVariantFields {
    readonly "RecordLink<Product>": {
        readonly fields: ItemRecordLinkProductFieldControllers;
    };
    readonly "RecordLink<Service>": {
        readonly fields: ItemRecordLinkServiceFieldControllers;
    };
}
function itemGetDefaultForVariant(variant: string): Item {
    if (variant === "RecordLink<Product>") {
        return recordLinkDefaultValue<Product>() as Item;
    }
    if (variant === "RecordLink<Service>") {
        return recordLinkDefaultValue<Service>() as Item;
    }
    return recordLinkDefaultValue<Product>() as Item;
}
export function itemCreateForm(initial: Item): ItemGigaform {
    const initialVariant: "RecordLink<Product>" | "RecordLink<Service>" = "RecordLink<Product>";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "itemGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as ItemErrors);
    let tainted = $state<$MfPh10>({} as ItemTainted);
    const variants = {} as ItemVariantFields;
    variants[__expr__] = {
        fields: {} as ItemRecordLinkProductFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ItemRecordLinkServiceFieldControllers
    };
    function switchVariant(variant: "RecordLink<Product>" | "RecordLink<Service>"): void {
        currentVariant = variant;
        data = "itemGetDefaultForVariant"(variant);
        errors = {} as ItemErrors;
        tainted = {} as ItemTainted;
    }
    function validate(): Exit<Item, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(itemDeserialize(data));
    }
    function reset(overrides: Partial<Item>): void {
        data = overrides ? overrides as typeof data : itemGetDefaultForVariant(currentVariant);
        errors = {} as ItemErrors;
        tainted = {} as ItemTainted;
    }
    return {
        get currentVariant () {
            return currentVariant;
        },
        get data () {
            return data;
        },
        set data (v){
            data = v;
        },
        get errors () {
            return errors;
        },
        set errors (v){
            errors = v;
        },
        get tainted () {
            return tainted;
        },
        set tainted (v){
            tainted = v;
        },
        variants,
        switchVariant,
        validate,
        reset
    };
}
export function itemFromFormData(formData: FormData): Exit<Item, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_type"}`) as "RecordLink<Product>" | "RecordLink<Service>" | null;
    if (!discriminant) {
        return toExit({
            success: false,
            errors: [
                {
                    field: `${"_type"}`,
                    message: "Missing discriminant field"
                }
            ]
        });
    }
    const obj: Record<string, unknown> = {};
    obj._type = discriminant;
    return toExit(itemDeserialize(obj));
}

export const Item = {
  serialize: itemSerialize,
  serializeWithContext: itemSerializeWithContext,
  deserialize: itemDeserialize,
  deserializeWithContext: itemDeserializeWithContext,
  is: itemIs,
  createForm: itemCreateForm,
  fromFormData: itemFromFormData
} as const;