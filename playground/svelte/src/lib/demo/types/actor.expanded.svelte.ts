import { userDefaultValue } from './user.svelte';
import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { accountDeserializeWithContext } from './account.svelte';
import { employeeDeserializeWithContext } from './employee.svelte';
import { userDeserializeWithContext } from './user.svelte';
import { Result } from 'macroforge/reexports';
import { Option } from 'macroforge/reexports';
import type { FieldController } from '@playground/macro/gigaform';
import { accountDefaultValue } from './account.svelte';
import { employeeDefaultValue } from './employee.svelte';
/** import macro {Gigaform} from "@playground/macro"; */

import type { User } from './user.svelte';
import type { Account } from './account.svelte';
import type { Employee } from './employee.svelte';

export type Actor = /** @default */ User | Employee | Account;

export function actorDefaultValue(): Actor {
    return userDefaultValue();
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function actorSerialize(
    value: Actor
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(actorSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function actorSerializeWithContext(value: Actor, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function actorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Actor> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = actorDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                { field: '_root', message: 'Actor.deserialize: root cannot be a forward reference' }
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
export function actorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Actor | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Actor | __mf_PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Actor.deserializeWithContext: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message: 'Actor.deserializeWithContext: missing __type field for union dispatch'
            }
        ]);
    }
    if (__typeName === 'User') {
        return userDeserializeWithContext(value, ctx) as Actor;
    }
    if (__typeName === 'Employee') {
        return employeeDeserializeWithContext(value, ctx) as Actor;
    }
    if (__typeName === 'Account') {
        return accountDeserializeWithContext(value, ctx) as Actor;
    }
    throw new __mf_DeserializeError([
        {
            field: '_root',
            message:
                'Actor.deserializeWithContext: unknown type "' +
                __typeName +
                '". Expected one of: User, Employee, Account'
        }
    ]);
}
export function actorIs(value: unknown): value is Actor {
    if (typeof value !== 'object' || value === null) {
        return false;
    }
    const __typeName = (value as any).__type;
    return __typeName === 'User' || __typeName === 'Employee' || __typeName === 'Account';
}

/** Per-variant error types */ export type ActorUserErrors = { _errors: Option<Array<string>> };
export type ActorEmployeeErrors = { _errors: Option<Array<string>> };
export type ActorAccountErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type ActorUserTainted = {};
export type ActorEmployeeTainted = {};
export type ActorAccountTainted = {}; /** Union error type */
export type ActorErrors =
    | ({ _type: 'User' } & ActorUserErrors)
    | ({ _type: 'Employee' } & ActorEmployeeErrors)
    | ({ _type: 'Account' } & ActorAccountErrors); /** Union tainted type */
export type ActorTainted =
    | ({ _type: 'User' } & ActorUserTainted)
    | ({ _type: 'Employee' } & ActorEmployeeTainted)
    | ({ _type: 'Account' } & ActorAccountTainted); /** Per-variant field controller types */
export interface ActorUserFieldControllers {}
export interface ActorEmployeeFieldControllers {}
export interface ActorAccountFieldControllers {} /** Union Gigaform interface with variant switching */
export interface ActorGigaform {
    readonly currentVariant: 'User' | 'Employee' | 'Account';
    readonly data: Actor;
    readonly errors: ActorErrors;
    readonly tainted: ActorTainted;
    readonly variants: ActorVariantFields;
    switchVariant(variant: 'User' | 'Employee' | 'Account'): void;
    validate(): Result<Actor, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Actor>): void;
} /** Variant fields container */
export interface ActorVariantFields {
    readonly User: { readonly fields: ActorUserFieldControllers };
    readonly Employee: { readonly fields: ActorEmployeeFieldControllers };
    readonly Account: { readonly fields: ActorAccountFieldControllers };
} /** Gets default value for a specific variant */
function actorGetDefaultForVariant(variant: string): Actor {
    switch (variant) {
        case 'User':
            return userDefaultValue() as Actor;
        case 'Employee':
            return employeeDefaultValue() as Actor;
        case 'Account':
            return accountDefaultValue() as Actor;
        default:
            return userDefaultValue() as Actor;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function actorCreateForm(initial?: Actor): ActorGigaform {
    const initialVariant: 'User' | 'Employee' | 'Account' = 'User';
    let currentVariant = $state<'User' | 'Employee' | 'Account'>(initialVariant);
    let data = $state<Actor>(initial ?? actorGetDefaultForVariant(initialVariant));
    let errors = $state<ActorErrors>({} as ActorErrors);
    let tainted = $state<ActorTainted>({} as ActorTainted);
    const variants: ActorVariantFields = {
        User: { fields: {} as ActorUserFieldControllers },
        Employee: { fields: {} as ActorEmployeeFieldControllers },
        Account: { fields: {} as ActorAccountFieldControllers }
    };
    function switchVariant(variant: 'User' | 'Employee' | 'Account'): void {
        currentVariant = variant;
        data = actorGetDefaultForVariant(variant);
        errors = {} as ActorErrors;
        tainted = {} as ActorTainted;
    }
    function validate(): Result<Actor, Array<{ field: string; message: string }>> {
        return actorDeserialize(data);
    }
    function reset(overrides?: Partial<Actor>): void {
        data = overrides ? (overrides as typeof data) : actorGetDefaultForVariant(currentVariant);
        errors = {} as ActorErrors;
        tainted = {} as ActorTainted;
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
export function actorFromFormData(
    formData: FormData
): Result<Actor, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_type') as 'User' | 'Employee' | 'Account' | null;
    if (!discriminant) {
        return Result.err([{ field: '_type', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._type = discriminant;
    if (discriminant === 'User') {
    } else if (discriminant === 'Employee') {
    } else if (discriminant === 'Account') {
    }
    return actorDeserialize(obj);
}

export const Actor = {
    defaultValue: actorDefaultValue,
    serialize: actorSerialize,
    serializeWithContext: actorSerializeWithContext,
    deserialize: actorDeserialize,
    deserializeWithContext: actorDeserializeWithContext,
    is: actorIs,
    createForm: actorCreateForm,
    fromFormData: actorFromFormData
} as const;
