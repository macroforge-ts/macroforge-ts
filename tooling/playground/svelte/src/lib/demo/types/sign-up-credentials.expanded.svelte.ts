import { emailPartsDefaultValue } from "./email-parts.svelte";
import { firstNameDefaultValue } from "./first-name.svelte";
import { lastNameDefaultValue } from "./last-name.svelte";
import { passwordDefaultValue } from "./password.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { emailPartsSerializeWithContext } from "./email-parts.svelte";
import { firstNameSerializeWithContext } from "./first-name.svelte";
import { lastNameSerializeWithContext } from "./last-name.svelte";
import { passwordSerializeWithContext } from "./password.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { emailPartsDeserializeWithContext } from "./email-parts.svelte";
import { firstNameDeserializeWithContext } from "./first-name.svelte";
import { lastNameDeserializeWithContext } from "./last-name.svelte";
import { passwordDeserializeWithContext } from "./password.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { EmailParts } from './email-parts.svelte';
import type { FirstName } from './first-name.svelte';
import type { LastName } from './last-name.svelte';
import type { Password } from './password.svelte';


export interface SignUpCredentials {
    firstName: FirstName;
    lastName: LastName;
    email: EmailParts;
    password: Password;
    rememberMe: boolean;
}

export function signUpCredentialsDefaultValue(): SignUpCredentials {
    return {
        firstName: firstNameDefaultValue(),
        lastName: lastNameDefaultValue(),
        email: emailPartsDefaultValue(),
        password: passwordDefaultValue(),
        rememberMe: false
    } as SignUpCredentials;
}

export function signUpCredentialsSerialize(value: SignUpCredentials): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(signUpCredentialsSerializeWithContext(value, ctx));
}
export function signUpCredentialsSerializeWithContext(value: SignUpCredentials, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"SignUpCredentials"}`,
        __id
    };
    result[`${"firstName"}`] = firstNameSerializeWithContext(value.firstName, ctx);
    result[`${"lastName"}`] = lastNameSerializeWithContext(value.lastName, ctx);
    result[`${"email"}`] = emailPartsSerializeWithContext(value.email, ctx);
    result[`${"password"}`] = passwordSerializeWithContext(value.password, ctx);
    result[`${"rememberMe"}`] = value.rememberMe;
    return result;
}

export function signUpCredentialsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: SignUpCredentials } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = signUpCredentialsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "SignUpCredentials.deserialize: root cannot be a forward reference"
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
export function signUpCredentialsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): SignUpCredentials | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"SignUpCredentials"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"firstName"}` in obj)) {
        errors.push({
            field: `${"firstName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"lastName"}` in obj)) {
        errors.push({
            field: `${"lastName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"email"}` in obj)) {
        errors.push({
            field: `${"email"}`,
            message: "missing required field"
        });
    }
    if (!(`${"password"}` in obj)) {
        errors.push({
            field: `${"password"}`,
            message: "missing required field"
        });
    }
    if (!(`${"rememberMe"}` in obj)) {
        errors.push({
            field: `${"rememberMe"}`,
            message: "missing required field"
        });
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
        const __raw_firstName = obj[`${"firstName"}`] as FirstName;
        {
            const __result = firstNameDeserializeWithContext(__raw_firstName, ctx);
            ctx.assignOrDefer(instance, `${"firstName"}`, __result);
        }
    }
    {
        const __raw_lastName = obj[`${"lastName"}`] as LastName;
        {
            const __result = lastNameDeserializeWithContext(__raw_lastName, ctx);
            ctx.assignOrDefer(instance, `${"lastName"}`, __result);
        }
    }
    {
        const __raw_email = obj[`${"email"}`] as EmailParts;
        {
            const __result = emailPartsDeserializeWithContext(__raw_email, ctx);
            ctx.assignOrDefer(instance, `${"email"}`, __result);
        }
    }
    {
        const __raw_password = obj[`${"password"}`] as Password;
        {
            const __result = passwordDeserializeWithContext(__raw_password, ctx);
            ctx.assignOrDefer(instance, `${"password"}`, __result);
        }
    }
    {
        const __raw_rememberMe = obj[`${"rememberMe"}`] as boolean;
        instance.rememberMe = __raw_rememberMe;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as SignUpCredentials;
}
export function signUpCredentialsValidateField<K extends keyof SignUpCredentials>(_field: K, _value: SignUpCredentials[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function signUpCredentialsValidateFields(_partial: Partial<SignUpCredentials>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function signUpCredentialsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"firstName" in o && "lastName" in o && "email" in o && "password" in o && "rememberMe" in o';
}
export function signUpCredentialsIs(obj: unknown): obj is SignUpCredentials {
    if (!signUpCredentialsHasShape(obj)) {
        return false;
    }
    const result = signUpCredentialsDeserialize(obj);
    return result.success;
}

export function signUpCredentialsFromFormData(formData: FormData): Exit<SignUpCredentials, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    {
        const firstNameObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"firstName"}.`)) {
                const fieldName = key.slice(`${"firstName"}.`.length);
                const parts = fieldName.split(".");
                let current = firstNameObj;
                for(let i = 0; i < parts.length - 1; i++){
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.firstName = firstNameObj;
    }
    {
        const lastNameObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"lastName"}.`)) {
                const fieldName = key.slice(`${"lastName"}.`.length);
                const parts = fieldName.split(".");
                let current = lastNameObj;
                for(let i = 0; i < parts.length - 1; i++){
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.lastName = lastNameObj;
    }
    {
        const emailObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"email"}.`)) {
                const fieldName = key.slice(`${"email"}.`.length);
                const parts = fieldName.split(".");
                let current = emailObj;
                for(let i = 0; i < parts.length - 1; i++){
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.email = emailObj;
    }
    {
        const passwordObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"password"}.`)) {
                const fieldName = key.slice(`${"password"}.`.length);
                const parts = fieldName.split(".");
                let current = passwordObj;
                for(let i = 0; i < parts.length - 1; i++){
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.password = passwordObj;
    }
    {
        const rememberMeVal = formData.get(`${"rememberMe"}`);
        obj.rememberMe = rememberMeVal === "true" || rememberMeVal === "on" || rememberMeVal === "1";
    }
    return toExit("signUpCredentialsDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: SignUpCredentials;
    readonly errors: SignUpCredentialsErrors;
    readonly tainted: SignUpCredentialsTainted;
    readonly fields: SignUpCredentialsFieldControllers;
    validate(): Exit<SignUpCredentials, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<SignUpCredentials>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function signUpCredentialsCreateForm(overrides: Partial<SignUpCredentials>): SignUpCredentialsGigaform {}
let data = $state({
    ...signUpCredentialsDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as SignUpCredentialsErrors);
let tainted = $state<$MfPh3>({} as SignUpCredentialsTainted);
const fields = {} as SignUpCredentialsFieldControllers;
fields.firstName = {
    label: `${"firstName"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.lastName = {
    label: `${"lastName"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.email = {
    label: `${"email"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.password = {
    label: `${"password"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.rememberMe = {
    label: `${"rememberMe"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
function validate(): Exit<SignUpCredentials, Array<{
    field: string;
    message: string;
}>> {
    return toExit("signUpCredentialsDeserialize(data)");
    data = {
        ...signUpCredentialsDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const SignUpCredentials = {
  defaultValue: signUpCredentialsDefaultValue,
  serialize: signUpCredentialsSerialize,
  serializeWithContext: signUpCredentialsSerializeWithContext,
  deserialize: signUpCredentialsDeserialize,
  deserializeWithContext: signUpCredentialsDeserializeWithContext,
  validateFields: signUpCredentialsValidateFields,
  hasShape: signUpCredentialsHasShape,
  is: signUpCredentialsIs,
  fromFormData: signUpCredentialsFromFormData,
  createForm: signUpCredentialsCreateForm
} as const;