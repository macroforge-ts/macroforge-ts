import { companyNameDefaultValue } from "./company-name.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { companyNameDeserializeWithContext } from "./company-name.svelte";
import { personNameDeserializeWithContext } from "./person-name.svelte";
/** import macro {Gigaform} from "@playground/macro"; */

import type { CompanyName } from './company-name.svelte';
import type { PersonName } from './person-name.svelte';


export type AccountName = /** @default */ CompanyName | PersonName;

export function accountNameDefaultValue#0#0(): AccountName {
    return companyNameDefaultValue();
}

export function accountNameSerialize#0#0(value: AccountName): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(accountNameSerializeWithContext(value, ctx));
}
export function accountNameSerializeWithContext#0#0(value: AccountName, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function accountNameDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: AccountName } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = accountNameDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "AccountName.deserialize: root cannot be a forward reference"
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
export function accountNameDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): AccountName | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as AccountName | __mf_PendingRef;
    }
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if (typeof __typeName === "string") {}
        if (__typeName === `${"CompanyName"}`) {
            return companyNameDeserializeWithContext(value, ctx) as AccountName;
        }
        if (__typeName === `${"PersonName"}`) {
            return personNameDeserializeWithContext(value, ctx) as AccountName;
        }
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"AccountName"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function accountNameIs(value: unknown): value is AccountName {
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if ('__typeName === "CompanyName" || __typeName === "PersonName"') return true;
    }
    return false;
}
     }

export const AccountName = {
  deserialize: accountNameDeserialize,
  deserializeWithContext: accountNameDeserializeWithContext,
  is: accountNameIs
} as const;