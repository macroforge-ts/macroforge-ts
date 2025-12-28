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

export type Table =
    | /** @default */ 'Account'
    | 'Did'
    | 'Appointment'
    | 'Lead'
    | 'TaxRate'
    | 'Site'
    | 'Employee'
    | 'Route'
    | 'Company'
    | 'Product'
    | 'Service'
    | 'User'
    | 'Order'
    | 'Payment'
    | 'Package'
    | 'Promotion'
    | 'Represents'
    | 'Ordered';

export function tableDefaultValue#0#0(): Table {
    return 'Account';
}

export function tableSerialize(value: Table): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(tableSerializeWithContext(value, ctx));
}
export function tableSerializeWithContext(value: Table, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function tableDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Table } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = tableDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Table.deserialize: root cannot be a forward reference"
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
export function tableDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Table | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Table | __mf_PendingRef;
    }
    const allowedValues = [
        "'Account', 'Did', 'Appointment', 'Lead', 'TaxRate', 'Site', 'Employee', 'Route', 'Company', 'Product', 'Service', 'User', 'Order', 'Payment', 'Package', 'Promotion', 'Represents', 'Ordered'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Table"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Table;
}
export function tableIs(value: unknown): value is Table {
    const allowedValues = [
        "'Account', 'Did', 'Appointment', 'Lead', 'TaxRate', 'Site', 'Employee', 'Route', 'Company', 'Product', 'Service', 'User', 'Order', 'Payment', 'Package', 'Promotion', 'Represents', 'Ordered'"
    ] as const;
    return allowedValues.includes(value as any);
}

export type TableAccountErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableDidErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableAppointmentErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableLeadErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableTaxRateErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableSiteErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableEmployeeErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableRouteErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableCompanyErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableProductErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableServiceErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableUserErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableOrderErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TablePaymentErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TablePackageErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TablePromotionErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableRepresentsErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableOrderedErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TableAccountTainted = {
};
export type TableDidTainted = {
};
export type TableAppointmentTainted = {
};
export type TableLeadTainted = {
};
export type TableTaxRateTainted = {
};
export type TableSiteTainted = {
};
export type TableEmployeeTainted = {
};
export type TableRouteTainted = {
};
export type TableCompanyTainted = {
};
export type TableProductTainted = {
};
export type TableServiceTainted = {
};
export type TableUserTainted = {
};
export type TableOrderTainted = {
};
export type TablePaymentTainted = {
};
export type TablePackageTainted = {
};
export type TablePromotionTainted = {
};
export type TableRepresentsTainted = {
};
export type TableOrderedTainted = {
};
export type TableErrors = ({
    _value: "Account";
} & TableAccountErrors) | ({
    _value: "Did";
} & TableDidErrors) | ({
    _value: "Appointment";
} & TableAppointmentErrors) | ({
    _value: "Lead";
} & TableLeadErrors) | ({
    _value: "TaxRate";
} & TableTaxRateErrors) | ({
    _value: "Site";
} & TableSiteErrors) | ({
    _value: "Employee";
} & TableEmployeeErrors) | ({
    _value: "Route";
} & TableRouteErrors) | ({
    _value: "Company";
} & TableCompanyErrors) | ({
    _value: "Product";
} & TableProductErrors) | ({
    _value: "Service";
} & TableServiceErrors) | ({
    _value: "User";
} & TableUserErrors) | ({
    _value: "Order";
} & TableOrderErrors) | ({
    _value: "Payment";
} & TablePaymentErrors) | ({
    _value: "Package";
} & TablePackageErrors) | ({
    _value: "Promotion";
} & TablePromotionErrors) | ({
    _value: "Represents";
} & TableRepresentsErrors) | ({
    _value: "Ordered";
} & TableOrderedErrors);
export type TableTainted = ({
    _value: "Account";
} & TableAccountTainted) | ({
    _value: "Did";
} & TableDidTainted) | ({
    _value: "Appointment";
} & TableAppointmentTainted) | ({
    _value: "Lead";
} & TableLeadTainted) | ({
    _value: "TaxRate";
} & TableTaxRateTainted) | ({
    _value: "Site";
} & TableSiteTainted) | ({
    _value: "Employee";
} & TableEmployeeTainted) | ({
    _value: "Route";
} & TableRouteTainted) | ({
    _value: "Company";
} & TableCompanyTainted) | ({
    _value: "Product";
} & TableProductTainted) | ({
    _value: "Service";
} & TableServiceTainted) | ({
    _value: "User";
} & TableUserTainted) | ({
    _value: "Order";
} & TableOrderTainted) | ({
    _value: "Payment";
} & TablePaymentTainted) | ({
    _value: "Package";
} & TablePackageTainted) | ({
    _value: "Promotion";
} & TablePromotionTainted) | ({
    _value: "Represents";
} & TableRepresentsTainted) | ({
    _value: "Ordered";
} & TableOrderedTainted);
export interface TableAccountFieldControllers {
}
export interface TableDidFieldControllers {
}
export interface TableAppointmentFieldControllers {
}
export interface TableLeadFieldControllers {
}
export interface TableTaxRateFieldControllers {
}
export interface TableSiteFieldControllers {
}
export interface TableEmployeeFieldControllers {
}
export interface TableRouteFieldControllers {
}
export interface TableCompanyFieldControllers {
}
export interface TableProductFieldControllers {
}
export interface TableServiceFieldControllers {
}
export interface TableUserFieldControllers {
}
export interface TableOrderFieldControllers {
}
export interface TablePaymentFieldControllers {
}
export interface TablePackageFieldControllers {
}
export interface TablePromotionFieldControllers {
}
export interface TableRepresentsFieldControllers {
}
export interface TableOrderedFieldControllers {
}
export interface TableGigaform {
    readonly currentVariant: "Account" | "Did" | "Appointment" | "Lead" | "TaxRate" | "Site" | "Employee" | "Route" | "Company" | "Product" | "Service" | "User" | "Order" | "Payment" | "Package" | "Promotion" | "Represents" | "Ordered";
    readonly data: Table;
    readonly errors: TableErrors;
    readonly tainted: TableTainted;
    readonly variants: TableVariantFields;
    switchVariant(variant: "Account" | "Did" | "Appointment" | "Lead" | "TaxRate" | "Site" | "Employee" | "Route" | "Company" | "Product" | "Service" | "User" | "Order" | "Payment" | "Package" | "Promotion" | "Represents" | "Ordered"): void;
    validate(): Exit<Table, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Table>): void;
}
export interface TableVariantFields {
    readonly Account: {
        readonly fields: TableAccountFieldControllers;
    };
    readonly Did: {
        readonly fields: TableDidFieldControllers;
    };
    readonly Appointment: {
        readonly fields: TableAppointmentFieldControllers;
    };
    readonly Lead: {
        readonly fields: TableLeadFieldControllers;
    };
    readonly TaxRate: {
        readonly fields: TableTaxRateFieldControllers;
    };
    readonly Site: {
        readonly fields: TableSiteFieldControllers;
    };
    readonly Employee: {
        readonly fields: TableEmployeeFieldControllers;
    };
    readonly Route: {
        readonly fields: TableRouteFieldControllers;
    };
    readonly Company: {
        readonly fields: TableCompanyFieldControllers;
    };
    readonly Product: {
        readonly fields: TableProductFieldControllers;
    };
    readonly Service: {
        readonly fields: TableServiceFieldControllers;
    };
    readonly User: {
        readonly fields: TableUserFieldControllers;
    };
    readonly Order: {
        readonly fields: TableOrderFieldControllers;
    };
    readonly Payment: {
        readonly fields: TablePaymentFieldControllers;
    };
    readonly Package: {
        readonly fields: TablePackageFieldControllers;
    };
    readonly Promotion: {
        readonly fields: TablePromotionFieldControllers;
    };
    readonly Represents: {
        readonly fields: TableRepresentsFieldControllers;
    };
    readonly Ordered: {
        readonly fields: TableOrderedFieldControllers;
    };
}
function tableGetDefaultForVariant(variant: string): Table {
    if (variant === "Account") {
        return "Account" as Table;
    }
    if (variant === "Did") {
        return "Did" as Table;
    }
    if (variant === "Appointment") {
        return "Appointment" as Table;
    }
    if (variant === "Lead") {
        return "Lead" as Table;
    }
    if (variant === "TaxRate") {
        return "TaxRate" as Table;
    }
    if (variant === "Site") {
        return "Site" as Table;
    }
    if (variant === "Employee") {
        return "Employee" as Table;
    }
    if (variant === "Route") {
        return "Route" as Table;
    }
    if (variant === "Company") {
        return "Company" as Table;
    }
    if (variant === "Product") {
        return "Product" as Table;
    }
    if (variant === "Service") {
        return "Service" as Table;
    }
    if (variant === "User") {
        return "User" as Table;
    }
    if (variant === "Order") {
        return "Order" as Table;
    }
    if (variant === "Payment") {
        return "Payment" as Table;
    }
    if (variant === "Package") {
        return "Package" as Table;
    }
    if (variant === "Promotion") {
        return "Promotion" as Table;
    }
    if (variant === "Represents") {
        return "Represents" as Table;
    }
    if (variant === "Ordered") {
        return "Ordered" as Table;
    }
    return "Account" as Table;
}
export function tableCreateForm(initial: Table): TableGigaform {
    const initialVariant: "Account" | "Did" | "Appointment" | "Lead" | "TaxRate" | "Site" | "Employee" | "Route" | "Company" | "Product" | "Service" | "User" | "Order" | "Payment" | "Package" | "Promotion" | "Represents" | "Ordered" = (initial as "Account" | "Did" | "Appointment" | "Lead" | "TaxRate" | "Site" | "Employee" | "Route" | "Company" | "Product" | "Service" | "User" | "Order" | "Payment" | "Package" | "Promotion" | "Represents" | "Ordered") ?? "Account";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "tableGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as TableErrors);
    let tainted = $state<$MfPh10>({} as TableTainted);
    const variants = {} as TableVariantFields;
    variants[__expr__] = {
        fields: {} as TableAccountFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TableDidFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TableAppointmentFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TableLeadFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TableTaxRateFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TableSiteFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TableEmployeeFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TableRouteFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TableCompanyFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TableProductFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TableServiceFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TableUserFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TableOrderFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TablePaymentFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TablePackageFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TablePromotionFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TableRepresentsFieldControllers
    };
    variants[__expr__] = {
        fields: {} as TableOrderedFieldControllers
    };
    function switchVariant(variant: "Account" | "Did" | "Appointment" | "Lead" | "TaxRate" | "Site" | "Employee" | "Route" | "Company" | "Product" | "Service" | "User" | "Order" | "Payment" | "Package" | "Promotion" | "Represents" | "Ordered"): void {
        currentVariant = variant;
        data = "tableGetDefaultForVariant"(variant);
        errors = {} as TableErrors;
        tainted = {} as TableTainted;
    }
    function validate(): Exit<Table, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(tableDeserialize(data));
    }
    function reset(overrides: Partial<Table>): void {
        data = overrides ? overrides as typeof data : tableGetDefaultForVariant(currentVariant);
        errors = {} as TableErrors;
        tainted = {} as TableTainted;
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
export function tableFromFormData(formData: FormData): Exit<Table, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "Account" | "Did" | "Appointment" | "Lead" | "TaxRate" | "Site" | "Employee" | "Route" | "Company" | "Product" | "Service" | "User" | "Order" | "Payment" | "Package" | "Promotion" | "Represents" | "Ordered" | null;
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
    return toExit(tableDeserialize(obj));
}

export const Table = {
  serialize: tableSerialize,
  serializeWithContext: tableSerializeWithContext,
  deserialize: tableDeserialize,
  deserializeWithContext: tableDeserializeWithContext,
  is: tableIs,
  createForm: tableCreateForm,
  fromFormData: tableFromFormData
} as const;