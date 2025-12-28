import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";

export type UserRole =
    | /** @default */ 'Administrator'
    | 'SalesRepresentative'
    | 'Technician'
    | 'HumanResources'
    | 'InformationTechnology';

export function userRoleDefaultValue#0#0(): UserRole {
    return 'Administrator';
}

export function userRoleSerialize(value: UserRole): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(userRoleSerializeWithContext(value, ctx));
}
export function userRoleSerializeWithContext(value: UserRole, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function userRoleDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: UserRole } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = userRoleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "UserRole.deserialize: root cannot be a forward reference"
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
export function userRoleDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): UserRole | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as UserRole | __mf_PendingRef;
    }
    const allowedValues = [
        "'Administrator', 'SalesRepresentative', 'Technician', 'HumanResources', 'InformationTechnology'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"UserRole"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as UserRole;
}
export function userRoleIs(value: unknown): value is UserRole {
    const allowedValues = [
        "'Administrator', 'SalesRepresentative', 'Technician', 'HumanResources', 'InformationTechnology'"
    ] as const;
    return allowedValues.includes(value as any);
}

export type UserRoleAdministratorErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type UserRoleSalesRepresentativeErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type UserRoleTechnicianErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type UserRoleHumanResourcesErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type UserRoleInformationTechnologyErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type UserRoleAdministratorTainted = {
};
export type UserRoleSalesRepresentativeTainted = {
};
export type UserRoleTechnicianTainted = {
};
export type UserRoleHumanResourcesTainted = {
};
export type UserRoleInformationTechnologyTainted = {
};
export type UserRoleErrors = ({
    _value: "Administrator";
} & UserRoleAdministratorErrors) | ({
    _value: "SalesRepresentative";
} & UserRoleSalesRepresentativeErrors) | ({
    _value: "Technician";
} & UserRoleTechnicianErrors) | ({
    _value: "HumanResources";
} & UserRoleHumanResourcesErrors) | ({
    _value: "InformationTechnology";
} & UserRoleInformationTechnologyErrors);
export type UserRoleTainted = ({
    _value: "Administrator";
} & UserRoleAdministratorTainted) | ({
    _value: "SalesRepresentative";
} & UserRoleSalesRepresentativeTainted) | ({
    _value: "Technician";
} & UserRoleTechnicianTainted) | ({
    _value: "HumanResources";
} & UserRoleHumanResourcesTainted) | ({
    _value: "InformationTechnology";
} & UserRoleInformationTechnologyTainted);
export interface UserRoleAdministratorFieldControllers {
}
export interface UserRoleSalesRepresentativeFieldControllers {
}
export interface UserRoleTechnicianFieldControllers {
}
export interface UserRoleHumanResourcesFieldControllers {
}
export interface UserRoleInformationTechnologyFieldControllers {
}
export interface UserRoleGigaform {
    readonly currentVariant: "Administrator" | "SalesRepresentative" | "Technician" | "HumanResources" | "InformationTechnology";
    readonly data: UserRole;
    readonly errors: UserRoleErrors;
    readonly tainted: UserRoleTainted;
    readonly variants: UserRoleVariantFields;
    switchVariant(variant: "Administrator" | "SalesRepresentative" | "Technician" | "HumanResources" | "InformationTechnology"): void;
    validate(): Exit<UserRole, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<UserRole>): void;
}
export interface UserRoleVariantFields {
    readonly Administrator: {
        readonly fields: UserRoleAdministratorFieldControllers;
    };
    readonly SalesRepresentative: {
        readonly fields: UserRoleSalesRepresentativeFieldControllers;
    };
    readonly Technician: {
        readonly fields: UserRoleTechnicianFieldControllers;
    };
    readonly HumanResources: {
        readonly fields: UserRoleHumanResourcesFieldControllers;
    };
    readonly InformationTechnology: {
        readonly fields: UserRoleInformationTechnologyFieldControllers;
    };
}
function userRoleGetDefaultForVariant(variant: string): UserRole {
    if (variant === "Administrator") {
        return "Administrator" as UserRole;
    }
    if (variant === "SalesRepresentative") {
        return "SalesRepresentative" as UserRole;
    }
    if (variant === "Technician") {
        return "Technician" as UserRole;
    }
    if (variant === "HumanResources") {
        return "HumanResources" as UserRole;
    }
    if (variant === "InformationTechnology") {
        return "InformationTechnology" as UserRole;
    }
    return "Administrator" as UserRole;
}
export function userRoleCreateForm(initial: UserRole): UserRoleGigaform {
    const initialVariant: "Administrator" | "SalesRepresentative" | "Technician" | "HumanResources" | "InformationTechnology" = (initial as "Administrator" | "SalesRepresentative" | "Technician" | "HumanResources" | "InformationTechnology") ?? "Administrator";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "userRoleGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as UserRoleErrors);
    let tainted = $state<$MfPh10>({} as UserRoleTainted);
    const variants = {} as UserRoleVariantFields;
    variants[__expr__] = {
        fields: {} as UserRoleAdministratorFieldControllers
    };
    variants[__expr__] = {
        fields: {} as UserRoleSalesRepresentativeFieldControllers
    };
    variants[__expr__] = {
        fields: {} as UserRoleTechnicianFieldControllers
    };
    variants[__expr__] = {
        fields: {} as UserRoleHumanResourcesFieldControllers
    };
    variants[__expr__] = {
        fields: {} as UserRoleInformationTechnologyFieldControllers
    };
    function switchVariant(variant: "Administrator" | "SalesRepresentative" | "Technician" | "HumanResources" | "InformationTechnology"): void {
        currentVariant = variant;
        data = "userRoleGetDefaultForVariant"(variant);
        errors = {} as UserRoleErrors;
        tainted = {} as UserRoleTainted;
    }
    function validate(): Exit<UserRole, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(userRoleDeserialize(data));
    }
    function reset(overrides: Partial<UserRole>): void {
        data = overrides ? overrides as typeof data : userRoleGetDefaultForVariant(currentVariant);
        errors = {} as UserRoleErrors;
        tainted = {} as UserRoleTainted;
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
export function userRoleFromFormData(formData: FormData): Exit<UserRole, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "Administrator" | "SalesRepresentative" | "Technician" | "HumanResources" | "InformationTechnology" | null;
    if (!discriminant) {
        return toExit({
            success: false,
            errors: [
                {
                    field: `${"_value"}`,
                    message: "Missing discriminant field"
                }
            ]
        });
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    return toExit(userRoleDeserialize(obj));
}

export const UserRole = {
  serialize: userRoleSerialize,
  serializeWithContext: userRoleSerializeWithContext,
  deserialize: userRoleDeserialize,
  deserializeWithContext: userRoleDeserializeWithContext,
  is: userRoleIs,
  createForm: userRoleCreateForm,
  fromFormData: userRoleFromFormData
} as const;