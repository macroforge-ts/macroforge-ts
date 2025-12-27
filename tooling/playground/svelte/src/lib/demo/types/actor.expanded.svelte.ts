import { userDefaultValue } from "./user.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { accountDeserializeWithContext } from "./account.svelte";
import { employeeDeserializeWithContext } from "./employee.svelte";
import { userDeserializeWithContext } from "./user.svelte";
/** import macro {Gigaform} from "@playground/macro"; */

import type { Account } from './account.svelte';
import type { Employee } from './employee.svelte';
import type { User } from './user.svelte';


export type Actor = /** @default */ User | Employee | Account;

export function actorDefaultValue#0#0(): Actor {
    return userDefaultValue();
}

export function actorSerialize#0#0(value: Actor): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(actorSerializeWithContext(value, ctx));
}
export function actorSerializeWithContext#0#0(value: Actor, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function actorDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Actor } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = actorDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Actor.deserialize: root cannot be a forward reference"
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
export function actorDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Actor | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Actor | __mf_PendingRef;
    }
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if (typeof __typeName === "string") {}
        if (__typeName === `${"User"}`) {
            return userDeserializeWithContext(value, ctx) as Actor;
        }
        if (__typeName === `${"Employee"}`) {
            return employeeDeserializeWithContext(value, ctx) as Actor;
        }
        if (__typeName === `${"Account"}`) {
            return accountDeserializeWithContext(value, ctx) as Actor;
        }
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"Actor"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function actorIs(value: unknown): value is Actor {
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if ('__typeName === "User" || __typeName === "Employee" || __typeName === "Account"') return true;
    }
    return false;
}
     }

export const Actor = {
  deserialize: actorDeserialize,
  deserializeWithContext: actorDeserializeWithContext,
  is: actorIs
} as const;