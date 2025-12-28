import { userDefaultValue } from "./user.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { accountDeserializeWithContext } from "./account.svelte";
import { employeeDeserializeWithContext } from "./employee.svelte";
import { userDeserializeWithContext } from "./user.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
import { accountDefaultValue } from "./account.svelte";
import { employeeDefaultValue } from "./employee.svelte";
/** import macro {Gigaform} from "@playground/macro"; */

import type { Account } from './account.svelte';
import type { Employee } from './employee.svelte';
import type { User } from './user.svelte';


export type Actor = /** @default */ User | Employee | Account;

export function actorDefaultValue#0#0(): Actor {
    return userDefaultValue();
}

export function actorSerialize(value: Actor): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(actorSerializeWithContext(value, ctx));
}
export function actorSerializeWithContext(value: Actor, ctx: __mf_SerializeContext): unknown {
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
        if (__typeName === "User") {
            return userDeserializeWithContext(value, ctx) as Actor;
        }
        if (__typeName === "Employee") {
            return employeeDeserializeWithContext(value, ctx) as Actor;
        }
        if (__typeName === "Account") {
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

export type ActorUserErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ActorEmployeeErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ActorAccountErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ActorUserTainted = {
};
export type ActorEmployeeTainted = {
};
export type ActorAccountTainted = {
};
export type ActorErrors = ({
    _type: "User";
} & ActorUserErrors) | ({
    _type: "Employee";
} & ActorEmployeeErrors) | ({
    _type: "Account";
} & ActorAccountErrors);
export type ActorTainted = ({
    _type: "User";
} & ActorUserTainted) | ({
    _type: "Employee";
} & ActorEmployeeTainted) | ({
    _type: "Account";
} & ActorAccountTainted);
export interface ActorUserFieldControllers {
}
export interface ActorEmployeeFieldControllers {
}
export interface ActorAccountFieldControllers {
}
export interface ActorGigaform {
    readonly currentVariant: "User" | "Employee" | "Account";
    readonly data: Actor;
    readonly errors: ActorErrors;
    readonly tainted: ActorTainted;
    readonly variants: ActorVariantFields;
    switchVariant(variant: "User" | "Employee" | "Account"): void;
    validate(): Exit<Actor, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Actor>): void;
}
export interface ActorVariantFields {
    readonly User: {
        readonly fields: ActorUserFieldControllers;
    };
    readonly Employee: {
        readonly fields: ActorEmployeeFieldControllers;
    };
    readonly Account: {
        readonly fields: ActorAccountFieldControllers;
    };
}
function actorGetDefaultForVariant(variant: string): Actor {
    if (variant === "User") {
        return userDefaultValue() as Actor;
    }
    if (variant === "Employee") {
        return employeeDefaultValue() as Actor;
    }
    if (variant === "Account") {
        return accountDefaultValue() as Actor;
    }
    return userDefaultValue() as Actor;
}
export function actorCreateForm(initial: Actor): ActorGigaform {
    const initialVariant: "User" | "Employee" | "Account" = "User";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "actorGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as ActorErrors);
    let tainted = $state<$MfPh10>({} as ActorTainted);
    const variants = {} as ActorVariantFields;
    variants[__expr__] = {
        fields: {} as ActorUserFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ActorEmployeeFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ActorAccountFieldControllers
    };
    function switchVariant(variant: "User" | "Employee" | "Account"): void {
        currentVariant = variant;
        data = "actorGetDefaultForVariant"(variant);
        errors = {} as ActorErrors;
        tainted = {} as ActorTainted;
    }
    function validate(): Exit<Actor, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(actorDeserialize(data));
    }
    function reset(overrides: Partial<Actor>): void {
        data = overrides ? overrides as typeof data : actorGetDefaultForVariant(currentVariant);
        errors = {} as ActorErrors;
        tainted = {} as ActorTainted;
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
export function actorFromFormData(formData: FormData): Exit<Actor, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_type"}`) as "User" | "Employee" | "Account" | null;
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
    return toExit(actorDeserialize(obj));
}

export const Actor = {
  serialize: actorSerialize,
  serializeWithContext: actorSerializeWithContext,
  deserialize: actorDeserialize,
  deserializeWithContext: actorDeserializeWithContext,
  is: actorIs,
  createForm: actorCreateForm,
  fromFormData: actorFromFormData
} as const;