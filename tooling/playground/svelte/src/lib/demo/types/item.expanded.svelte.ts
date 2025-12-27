import { recordLinkDefaultValue } from "./record-link.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { recordLinkDeserializeWithContext } from "./record-link.svelte";
/** import macro {Gigaform} from "@playground/macro"; */

import type { Product } from './product.svelte';
import type { RecordLink } from './record-link.svelte';
import type { Service } from './service.svelte';


export type Item = RecordLink<Product> | /** @default */ RecordLink<Service>;

export function itemDefaultValue#0#0(): Item {
    return recordLinkDefaultValue<Service>();
}

export function itemSerialize#0#0(value: Item): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(itemSerializeWithContext(value, ctx));
}
export function itemSerializeWithContext#0#0(value: Item, ctx: __mf_SerializeContext): unknown {
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
        if (__typeName === `${"RecordLink<Product>"}`) {
            return recordLinkDeserializeWithContext(value, ctx) as Item;
        }
        if (__typeName === `${"RecordLink<Service>"}`) {
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

export const Item = {
  deserialize: itemDeserialize,
  deserializeWithContext: itemDeserializeWithContext,
  is: itemIs
} as const;