import { defaultValueUser } from './user.svelte';
import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { __deserializeAccount } from './account.svelte';
import { __deserializeEmployee } from './employee.svelte';
import { __deserializeUser } from './user.svelte';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
import { defaultValueAccount } from './account.svelte';
import { defaultValueEmployee } from './employee.svelte';
/** import macro {Gigaform} from "@playground/macro"; */

import type { User } from './user.svelte';
import type { Account } from './account.svelte';
import type { Employee } from './employee.svelte';

export type Actor = /** @default */ User | Employee | Account;

export function defaultValueActor(): Actor {
    return defaultValueUser();
}

export function toStringifiedJSONActor(value: Actor): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeActor(value, ctx));
}
export function toObjectActor(value: Actor): unknown {
    const ctx = SerializeContext.create();
    return __serializeActor(value, ctx);
}
export function __serializeActor(value: Actor, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONActor(
    json: string,
    opts?: DeserializeOptions
): Result<Actor, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectActor(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectActor(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Actor, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeActor(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Actor.fromObject: root cannot be a forward reference' }
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
export function __deserializeActor(value: any, ctx: DeserializeContext): Actor | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Actor | PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new DeserializeError([
            { field: '_root', message: 'Actor.__deserialize: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new DeserializeError([
            {
                field: '_root',
                message: 'Actor.__deserialize: missing __type field for union dispatch'
            }
        ]);
    }
    if (__typeName === 'User') {
        return __deserializeUser(value, ctx) as Actor;
    }
    if (__typeName === 'Employee') {
        return __deserializeEmployee(value, ctx) as Actor;
    }
    if (__typeName === 'Account') {
        return __deserializeAccount(value, ctx) as Actor;
    }
    throw new DeserializeError([
        {
            field: '_root',
            message:
                'Actor.__deserialize: unknown type "' +
                __typeName +
                '". Expected one of: User, Employee, Account'
        }
    ]);
}
export function isActor(value: unknown): value is Actor {
    if (typeof value !== 'object' || value === null) {
        return false;
    }
    const __typeName = (value as any).__type;
    return __typeName === 'User' || __typeName === 'Employee' || __typeName === 'Account';
}

/** Per-variant error types */ export type UserErrorsActor = { _errors: Option<Array<string>> };
export type EmployeeErrorsActor = { _errors: Option<Array<string>> };
export type AccountErrorsActor = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type UserTaintedActor = {};
export type EmployeeTaintedActor = {};
export type AccountTaintedActor = {}; /** Union error type */
export type ErrorsActor =
    | ({ _type: 'User' } & UserErrorsActor)
    | ({ _type: 'Employee' } & EmployeeErrorsActor)
    | ({ _type: 'Account' } & AccountErrorsActor); /** Union tainted type */
export type TaintedActor =
    | ({ _type: 'User' } & UserTaintedActor)
    | ({ _type: 'Employee' } & EmployeeTaintedActor)
    | ({ _type: 'Account' } & AccountTaintedActor); /** Per-variant field controller types */
export interface UserFieldControllersActor {}
export interface EmployeeFieldControllersActor {}
export interface AccountFieldControllersActor {} /** Union Gigaform interface with variant switching */
export interface GigaformActor {
    readonly currentVariant: 'User' | 'Employee' | 'Account';
    readonly data: Actor;
    readonly errors: ErrorsActor;
    readonly tainted: TaintedActor;
    readonly variants: VariantFieldsActor;
    switchVariant(variant: 'User' | 'Employee' | 'Account'): void;
    validate(): Result<Actor, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Actor>): void;
} /** Variant fields container */
export interface VariantFieldsActor {
    readonly User: { readonly fields: UserFieldControllersActor };
    readonly Employee: { readonly fields: EmployeeFieldControllersActor };
    readonly Account: { readonly fields: AccountFieldControllersActor };
} /** Gets default value for a specific variant */
function getDefaultForVariantActor(variant: string): Actor {
    switch (variant) {
        case 'User':
            return defaultValueUser() as Actor;
        case 'Employee':
            return defaultValueEmployee() as Actor;
        case 'Account':
            return defaultValueAccount() as Actor;
        default:
            return defaultValueUser() as Actor;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormActor(initial?: Actor): GigaformActor {
    const initialVariant: 'User' | 'Employee' | 'Account' = 'User';
    let currentVariant = $state<'User' | 'Employee' | 'Account'>(initialVariant);
    let data = $state<Actor>(initial ?? getDefaultForVariantActor(initialVariant));
    let errors = $state<ErrorsActor>({} as ErrorsActor);
    let tainted = $state<TaintedActor>({} as TaintedActor);
    const variants: VariantFieldsActor = {
        User: {
            fields: {} as UserFieldControllersActor
        },
        Employee: {
            fields: {} as EmployeeFieldControllersActor
        },
        Account: {
            fields: {} as AccountFieldControllersActor
        }
    };
    function switchVariant(variant: 'User' | 'Employee' | 'Account'): void {
        currentVariant = variant;
        data = getDefaultForVariantActor(variant);
        errors = {} as ErrorsActor;
        tainted = {} as TaintedActor;
    }
    function validate(): Result<Actor, Array<{ field: string; message: string }>> {
        return fromObjectActor(data);
    }
    function reset(overrides?: Partial<Actor>): void {
        data = overrides ? (overrides as typeof data) : getDefaultForVariantActor(currentVariant);
        errors = {} as ErrorsActor;
        tainted = {} as TaintedActor;
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
export function fromFormDataActor(
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
    return fromStringifiedJSONActor(JSON.stringify(obj));
}
