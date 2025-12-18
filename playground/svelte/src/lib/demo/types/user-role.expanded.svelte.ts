import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { exitFail } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';

export type UserRole =
    | /** @default */ 'Administrator'
    | 'SalesRepresentative'
    | 'Technician'
    | 'HumanResources'
    | 'InformationTechnology';

export function userRoleDefaultValue(): UserRole {
    return 'Administrator';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function userRoleSerialize(
    value: UserRole
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(userRoleSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function userRoleSerializeWithContext(value: UserRole, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function userRoleDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, UserRole> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = userRoleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'UserRole.deserialize: root cannot be a forward reference'
                }
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
export function userRoleDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): UserRole | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as UserRole | __mf_PendingRef;
    }
    const allowedValues = [
        'Administrator',
        'SalesRepresentative',
        'Technician',
        'HumanResources',
        'InformationTechnology'
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for UserRole: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as UserRole;
}
export function userRoleIs(value: unknown): value is UserRole {
    const allowedValues = [
        'Administrator',
        'SalesRepresentative',
        'Technician',
        'HumanResources',
        'InformationTechnology'
    ] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type UserRoleAdministratorErrors = {
    _errors: Option<Array<string>>;
};
export type UserRoleSalesRepresentativeErrors = { _errors: Option<Array<string>> };
export type UserRoleTechnicianErrors = { _errors: Option<Array<string>> };
export type UserRoleHumanResourcesErrors = { _errors: Option<Array<string>> };
export type UserRoleInformationTechnologyErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type UserRoleAdministratorTainted = {};
export type UserRoleSalesRepresentativeTainted = {};
export type UserRoleTechnicianTainted = {};
export type UserRoleHumanResourcesTainted = {};
export type UserRoleInformationTechnologyTainted = {}; /** Union error type */
export type UserRoleErrors =
    | ({ _value: 'Administrator' } & UserRoleAdministratorErrors)
    | ({ _value: 'SalesRepresentative' } & UserRoleSalesRepresentativeErrors)
    | ({ _value: 'Technician' } & UserRoleTechnicianErrors)
    | ({ _value: 'HumanResources' } & UserRoleHumanResourcesErrors)
    | ({
          _value: 'InformationTechnology';
      } & UserRoleInformationTechnologyErrors); /** Union tainted type */
export type UserRoleTainted =
    | ({ _value: 'Administrator' } & UserRoleAdministratorTainted)
    | ({ _value: 'SalesRepresentative' } & UserRoleSalesRepresentativeTainted)
    | ({ _value: 'Technician' } & UserRoleTechnicianTainted)
    | ({ _value: 'HumanResources' } & UserRoleHumanResourcesTainted)
    | ({
          _value: 'InformationTechnology';
      } & UserRoleInformationTechnologyTainted); /** Per-variant field controller types */
export interface UserRoleAdministratorFieldControllers {}
export interface UserRoleSalesRepresentativeFieldControllers {}
export interface UserRoleTechnicianFieldControllers {}
export interface UserRoleHumanResourcesFieldControllers {}
export interface UserRoleInformationTechnologyFieldControllers {} /** Union Gigaform interface with variant switching */
export interface UserRoleGigaform {
    readonly currentVariant:
        | 'Administrator'
        | 'SalesRepresentative'
        | 'Technician'
        | 'HumanResources'
        | 'InformationTechnology';
    readonly data: UserRole;
    readonly errors: UserRoleErrors;
    readonly tainted: UserRoleTainted;
    readonly variants: UserRoleVariantFields;
    switchVariant(
        variant:
            | 'Administrator'
            | 'SalesRepresentative'
            | 'Technician'
            | 'HumanResources'
            | 'InformationTechnology'
    ): void;
    validate(): Exit<Array<{ field: string; message: string }>, UserRole>;
    reset(overrides?: Partial<UserRole>): void;
} /** Variant fields container */
export interface UserRoleVariantFields {
    readonly Administrator: { readonly fields: UserRoleAdministratorFieldControllers };
    readonly SalesRepresentative: { readonly fields: UserRoleSalesRepresentativeFieldControllers };
    readonly Technician: { readonly fields: UserRoleTechnicianFieldControllers };
    readonly HumanResources: { readonly fields: UserRoleHumanResourcesFieldControllers };
    readonly InformationTechnology: {
        readonly fields: UserRoleInformationTechnologyFieldControllers;
    };
} /** Gets default value for a specific variant */
function userRoleGetDefaultForVariant(variant: string): UserRole {
    switch (variant) {
        case 'Administrator':
            return 'Administrator' as UserRole;
        case 'SalesRepresentative':
            return 'SalesRepresentative' as UserRole;
        case 'Technician':
            return 'Technician' as UserRole;
        case 'HumanResources':
            return 'HumanResources' as UserRole;
        case 'InformationTechnology':
            return 'InformationTechnology' as UserRole;
        default:
            return 'Administrator' as UserRole;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function userRoleCreateForm(initial?: UserRole): UserRoleGigaform {
    const initialVariant:
        | 'Administrator'
        | 'SalesRepresentative'
        | 'Technician'
        | 'HumanResources'
        | 'InformationTechnology' =
        (initial as
            | 'Administrator'
            | 'SalesRepresentative'
            | 'Technician'
            | 'HumanResources'
            | 'InformationTechnology') ?? 'Administrator';
    let currentVariant = $state<
        | 'Administrator'
        | 'SalesRepresentative'
        | 'Technician'
        | 'HumanResources'
        | 'InformationTechnology'
    >(initialVariant);
    let data = $state<UserRole>(initial ?? userRoleGetDefaultForVariant(initialVariant));
    let errors = $state<UserRoleErrors>({} as UserRoleErrors);
    let tainted = $state<UserRoleTainted>({} as UserRoleTainted);
    const variants: UserRoleVariantFields = {
        Administrator: { fields: {} as UserRoleAdministratorFieldControllers },
        SalesRepresentative: { fields: {} as UserRoleSalesRepresentativeFieldControllers },
        Technician: { fields: {} as UserRoleTechnicianFieldControllers },
        HumanResources: { fields: {} as UserRoleHumanResourcesFieldControllers },
        InformationTechnology: { fields: {} as UserRoleInformationTechnologyFieldControllers }
    };
    function switchVariant(
        variant:
            | 'Administrator'
            | 'SalesRepresentative'
            | 'Technician'
            | 'HumanResources'
            | 'InformationTechnology'
    ): void {
        currentVariant = variant;
        data = userRoleGetDefaultForVariant(variant);
        errors = {} as UserRoleErrors;
        tainted = {} as UserRoleTainted;
    }
    function validate(): Exit<Array<{ field: string; message: string }>, UserRole> {
        return userRoleDeserialize(data);
    }
    function reset(overrides?: Partial<UserRole>): void {
        data = overrides
            ? (overrides as typeof data)
            : userRoleGetDefaultForVariant(currentVariant);
        errors = {} as UserRoleErrors;
        tainted = {} as UserRoleTainted;
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
export function userRoleFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, UserRole> {
    const discriminant = formData.get('_value') as
        | 'Administrator'
        | 'SalesRepresentative'
        | 'Technician'
        | 'HumanResources'
        | 'InformationTechnology'
        | null;
    if (!discriminant) {
        return exitFail([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Administrator') {
    } else if (discriminant === 'SalesRepresentative') {
    } else if (discriminant === 'Technician') {
    } else if (discriminant === 'HumanResources') {
    } else if (discriminant === 'InformationTechnology') {
    }
    return userRoleDeserialize(obj);
}

export const UserRole = {
    defaultValue: userRoleDefaultValue,
    serialize: userRoleSerialize,
    serializeWithContext: userRoleSerializeWithContext,
    deserialize: userRoleDeserialize,
    deserializeWithContext: userRoleDeserializeWithContext,
    is: userRoleIs,
    createForm: userRoleCreateForm,
    fromFormData: userRoleFromFormData
} as const;
