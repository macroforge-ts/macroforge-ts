import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

export type UserRole =
    | /** @default */ 'Administrator'
    | 'SalesRepresentative'
    | 'Technician'
    | 'HumanResources'
    | 'InformationTechnology';

export function defaultValueUserRole(): UserRole {
    return 'Administrator';
}

export function toStringifiedJSONUserRole(value: UserRole): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeUserRole(value, ctx));
}
export function toObjectUserRole(value: UserRole): unknown {
    const ctx = SerializeContext.create();
    return __serializeUserRole(value, ctx);
}
export function __serializeUserRole(value: UserRole, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONUserRole(
    json: string,
    opts?: DeserializeOptions
): Result<UserRole, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectUserRole(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectUserRole(
    obj: unknown,
    opts?: DeserializeOptions
): Result<UserRole, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeUserRole(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'UserRole.fromObject: root cannot be a forward reference'
                }
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
export function __deserializeUserRole(value: any, ctx: DeserializeContext): UserRole | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as UserRole | PendingRef;
    }
    const allowedValues = [
        'Administrator',
        'SalesRepresentative',
        'Technician',
        'HumanResources',
        'InformationTechnology'
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
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
export function isUserRole(value: unknown): value is UserRole {
    const allowedValues = [
        'Administrator',
        'SalesRepresentative',
        'Technician',
        'HumanResources',
        'InformationTechnology'
    ] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type AdministratorErrorsUserRole = {
    _errors: Option<Array<string>>;
};
export type SalesRepresentativeErrorsUserRole = { _errors: Option<Array<string>> };
export type TechnicianErrorsUserRole = { _errors: Option<Array<string>> };
export type HumanResourcesErrorsUserRole = { _errors: Option<Array<string>> };
export type InformationTechnologyErrorsUserRole = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type AdministratorTaintedUserRole = {};
export type SalesRepresentativeTaintedUserRole = {};
export type TechnicianTaintedUserRole = {};
export type HumanResourcesTaintedUserRole = {};
export type InformationTechnologyTaintedUserRole = {}; /** Union error type */
export type ErrorsUserRole =
    | ({ _value: 'Administrator' } & AdministratorErrorsUserRole)
    | ({ _value: 'SalesRepresentative' } & SalesRepresentativeErrorsUserRole)
    | ({ _value: 'Technician' } & TechnicianErrorsUserRole)
    | ({ _value: 'HumanResources' } & HumanResourcesErrorsUserRole)
    | ({
          _value: 'InformationTechnology';
      } & InformationTechnologyErrorsUserRole); /** Union tainted type */
export type TaintedUserRole =
    | ({ _value: 'Administrator' } & AdministratorTaintedUserRole)
    | ({ _value: 'SalesRepresentative' } & SalesRepresentativeTaintedUserRole)
    | ({ _value: 'Technician' } & TechnicianTaintedUserRole)
    | ({ _value: 'HumanResources' } & HumanResourcesTaintedUserRole)
    | ({
          _value: 'InformationTechnology';
      } & InformationTechnologyTaintedUserRole); /** Per-variant field controller types */
export interface AdministratorFieldControllersUserRole {}
export interface SalesRepresentativeFieldControllersUserRole {}
export interface TechnicianFieldControllersUserRole {}
export interface HumanResourcesFieldControllersUserRole {}
export interface InformationTechnologyFieldControllersUserRole {} /** Union Gigaform interface with variant switching */
export interface GigaformUserRole {
    readonly currentVariant:
        | 'Administrator'
        | 'SalesRepresentative'
        | 'Technician'
        | 'HumanResources'
        | 'InformationTechnology';
    readonly data: UserRole;
    readonly errors: ErrorsUserRole;
    readonly tainted: TaintedUserRole;
    readonly variants: VariantFieldsUserRole;
    switchVariant(
        variant:
            | 'Administrator'
            | 'SalesRepresentative'
            | 'Technician'
            | 'HumanResources'
            | 'InformationTechnology'
    ): void;
    validate(): Result<UserRole, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<UserRole>): void;
} /** Variant fields container */
export interface VariantFieldsUserRole {
    readonly Administrator: { readonly fields: AdministratorFieldControllersUserRole };
    readonly SalesRepresentative: { readonly fields: SalesRepresentativeFieldControllersUserRole };
    readonly Technician: { readonly fields: TechnicianFieldControllersUserRole };
    readonly HumanResources: { readonly fields: HumanResourcesFieldControllersUserRole };
    readonly InformationTechnology: {
        readonly fields: InformationTechnologyFieldControllersUserRole;
    };
} /** Gets default value for a specific variant */
function getDefaultForVariantUserRole(variant: string): UserRole {
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
export function createFormUserRole(initial?: UserRole): GigaformUserRole {
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
    let data = $state<UserRole>(initial ?? getDefaultForVariantUserRole(initialVariant));
    let errors = $state<ErrorsUserRole>({} as ErrorsUserRole);
    let tainted = $state<TaintedUserRole>({} as TaintedUserRole);
    const variants: VariantFieldsUserRole = {
        Administrator: {
            fields: {} as AdministratorFieldControllersUserRole
        },
        SalesRepresentative: {
            fields: {} as SalesRepresentativeFieldControllersUserRole
        },
        Technician: {
            fields: {} as TechnicianFieldControllersUserRole
        },
        HumanResources: {
            fields: {} as HumanResourcesFieldControllersUserRole
        },
        InformationTechnology: {
            fields: {} as InformationTechnologyFieldControllersUserRole
        }
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
        data = getDefaultForVariantUserRole(variant);
        errors = {} as ErrorsUserRole;
        tainted = {} as TaintedUserRole;
    }
    function validate(): Result<UserRole, Array<{ field: string; message: string }>> {
        return fromObjectUserRole(data);
    }
    function reset(overrides?: Partial<UserRole>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantUserRole(currentVariant);
        errors = {} as ErrorsUserRole;
        tainted = {} as TaintedUserRole;
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
export function fromFormDataUserRole(
    formData: FormData
): Result<UserRole, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as
        | 'Administrator'
        | 'SalesRepresentative'
        | 'Technician'
        | 'HumanResources'
        | 'InformationTechnology'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Administrator') {
    } else if (discriminant === 'SalesRepresentative') {
    } else if (discriminant === 'Technician') {
    } else if (discriminant === 'HumanResources') {
    } else if (discriminant === 'InformationTechnology') {
    }
    return fromStringifiedJSONUserRole(JSON.stringify(obj));
}
