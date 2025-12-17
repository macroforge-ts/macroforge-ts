import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { User } from './user.svelte';
import type { Service } from './service.svelte';
import type { Did } from './did.svelte';
import type { Promotion } from './promotion.svelte';
import type { Site } from './site.svelte';
import type { Product } from './product.svelte';
import type { Represents } from './represents.svelte';
import type { Payment } from './payment.svelte';
import type { Appointment } from './appointment.svelte';
import type { Package } from './package.svelte';
import type { Account } from './account.svelte';
import type { Order } from './order.svelte';
import type { TaxRate } from './tax-rate.svelte';
import type { Lead } from './lead.svelte';
import type { Company } from './company.svelte';
import type { Employee } from './employee.svelte';
import type { Route } from './route.svelte';
import type { Ordered } from './ordered.svelte';

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

export function tableDefaultValue(): Table {
    return 'Account';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function tableSerialize(
    value: Table
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(tableSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function tableSerializeWithContext(value: Table, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function tableDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Result<Table, Array<{ field: string; message: string }>> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = tableDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Table.deserialize: root cannot be a forward reference' }
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
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function tableDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): Table | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Table | PendingRef;
    }
    const allowedValues = [
        'Account',
        'Did',
        'Appointment',
        'Lead',
        'TaxRate',
        'Site',
        'Employee',
        'Route',
        'Company',
        'Product',
        'Service',
        'User',
        'Order',
        'Payment',
        'Package',
        'Promotion',
        'Represents',
        'Ordered'
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for Table: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as Table;
}
export function tableIs(value: unknown): value is Table {
    const allowedValues = [
        'Account',
        'Did',
        'Appointment',
        'Lead',
        'TaxRate',
        'Site',
        'Employee',
        'Route',
        'Company',
        'Product',
        'Service',
        'User',
        'Order',
        'Payment',
        'Package',
        'Promotion',
        'Represents',
        'Ordered'
    ] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type TableAccountErrors = { _errors: Option<Array<string>> };
export type TableDidErrors = { _errors: Option<Array<string>> };
export type TableAppointmentErrors = { _errors: Option<Array<string>> };
export type TableLeadErrors = { _errors: Option<Array<string>> };
export type TableTaxRateErrors = { _errors: Option<Array<string>> };
export type TableSiteErrors = { _errors: Option<Array<string>> };
export type TableEmployeeErrors = { _errors: Option<Array<string>> };
export type TableRouteErrors = { _errors: Option<Array<string>> };
export type TableCompanyErrors = { _errors: Option<Array<string>> };
export type TableProductErrors = { _errors: Option<Array<string>> };
export type TableServiceErrors = { _errors: Option<Array<string>> };
export type TableUserErrors = { _errors: Option<Array<string>> };
export type TableOrderErrors = { _errors: Option<Array<string>> };
export type TablePaymentErrors = { _errors: Option<Array<string>> };
export type TablePackageErrors = { _errors: Option<Array<string>> };
export type TablePromotionErrors = { _errors: Option<Array<string>> };
export type TableRepresentsErrors = { _errors: Option<Array<string>> };
export type TableOrderedErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type TableAccountTainted = {};
export type TableDidTainted = {};
export type TableAppointmentTainted = {};
export type TableLeadTainted = {};
export type TableTaxRateTainted = {};
export type TableSiteTainted = {};
export type TableEmployeeTainted = {};
export type TableRouteTainted = {};
export type TableCompanyTainted = {};
export type TableProductTainted = {};
export type TableServiceTainted = {};
export type TableUserTainted = {};
export type TableOrderTainted = {};
export type TablePaymentTainted = {};
export type TablePackageTainted = {};
export type TablePromotionTainted = {};
export type TableRepresentsTainted = {};
export type TableOrderedTainted = {}; /** Union error type */
export type TableErrors =
    | ({ _value: 'Account' } & TableAccountErrors)
    | ({ _value: 'Did' } & TableDidErrors)
    | ({ _value: 'Appointment' } & TableAppointmentErrors)
    | ({ _value: 'Lead' } & TableLeadErrors)
    | ({ _value: 'TaxRate' } & TableTaxRateErrors)
    | ({ _value: 'Site' } & TableSiteErrors)
    | ({ _value: 'Employee' } & TableEmployeeErrors)
    | ({ _value: 'Route' } & TableRouteErrors)
    | ({ _value: 'Company' } & TableCompanyErrors)
    | ({ _value: 'Product' } & TableProductErrors)
    | ({ _value: 'Service' } & TableServiceErrors)
    | ({ _value: 'User' } & TableUserErrors)
    | ({ _value: 'Order' } & TableOrderErrors)
    | ({ _value: 'Payment' } & TablePaymentErrors)
    | ({ _value: 'Package' } & TablePackageErrors)
    | ({ _value: 'Promotion' } & TablePromotionErrors)
    | ({ _value: 'Represents' } & TableRepresentsErrors)
    | ({ _value: 'Ordered' } & TableOrderedErrors); /** Union tainted type */
export type TableTainted =
    | ({ _value: 'Account' } & TableAccountTainted)
    | ({ _value: 'Did' } & TableDidTainted)
    | ({ _value: 'Appointment' } & TableAppointmentTainted)
    | ({ _value: 'Lead' } & TableLeadTainted)
    | ({ _value: 'TaxRate' } & TableTaxRateTainted)
    | ({ _value: 'Site' } & TableSiteTainted)
    | ({ _value: 'Employee' } & TableEmployeeTainted)
    | ({ _value: 'Route' } & TableRouteTainted)
    | ({ _value: 'Company' } & TableCompanyTainted)
    | ({ _value: 'Product' } & TableProductTainted)
    | ({ _value: 'Service' } & TableServiceTainted)
    | ({ _value: 'User' } & TableUserTainted)
    | ({ _value: 'Order' } & TableOrderTainted)
    | ({ _value: 'Payment' } & TablePaymentTainted)
    | ({ _value: 'Package' } & TablePackageTainted)
    | ({ _value: 'Promotion' } & TablePromotionTainted)
    | ({ _value: 'Represents' } & TableRepresentsTainted)
    | ({ _value: 'Ordered' } & TableOrderedTainted); /** Per-variant field controller types */
export interface TableAccountFieldControllers {}
export interface TableDidFieldControllers {}
export interface TableAppointmentFieldControllers {}
export interface TableLeadFieldControllers {}
export interface TableTaxRateFieldControllers {}
export interface TableSiteFieldControllers {}
export interface TableEmployeeFieldControllers {}
export interface TableRouteFieldControllers {}
export interface TableCompanyFieldControllers {}
export interface TableProductFieldControllers {}
export interface TableServiceFieldControllers {}
export interface TableUserFieldControllers {}
export interface TableOrderFieldControllers {}
export interface TablePaymentFieldControllers {}
export interface TablePackageFieldControllers {}
export interface TablePromotionFieldControllers {}
export interface TableRepresentsFieldControllers {}
export interface TableOrderedFieldControllers {} /** Union Gigaform interface with variant switching */
export interface TableGigaform {
    readonly currentVariant:
        | 'Account'
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
    readonly data: Table;
    readonly errors: TableErrors;
    readonly tainted: TableTainted;
    readonly variants: TableVariantFields;
    switchVariant(
        variant:
            | 'Account'
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
            | 'Ordered'
    ): void;
    validate(): Result<Table, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Table>): void;
} /** Variant fields container */
export interface TableVariantFields {
    readonly Account: { readonly fields: TableAccountFieldControllers };
    readonly Did: { readonly fields: TableDidFieldControllers };
    readonly Appointment: { readonly fields: TableAppointmentFieldControllers };
    readonly Lead: { readonly fields: TableLeadFieldControllers };
    readonly TaxRate: { readonly fields: TableTaxRateFieldControllers };
    readonly Site: { readonly fields: TableSiteFieldControllers };
    readonly Employee: { readonly fields: TableEmployeeFieldControllers };
    readonly Route: { readonly fields: TableRouteFieldControllers };
    readonly Company: { readonly fields: TableCompanyFieldControllers };
    readonly Product: { readonly fields: TableProductFieldControllers };
    readonly Service: { readonly fields: TableServiceFieldControllers };
    readonly User: { readonly fields: TableUserFieldControllers };
    readonly Order: { readonly fields: TableOrderFieldControllers };
    readonly Payment: { readonly fields: TablePaymentFieldControllers };
    readonly Package: { readonly fields: TablePackageFieldControllers };
    readonly Promotion: { readonly fields: TablePromotionFieldControllers };
    readonly Represents: { readonly fields: TableRepresentsFieldControllers };
    readonly Ordered: { readonly fields: TableOrderedFieldControllers };
} /** Gets default value for a specific variant */
function tableGetDefaultForVariant(variant: string): Table {
    switch (variant) {
        case 'Account':
            return 'Account' as Table;
        case 'Did':
            return 'Did' as Table;
        case 'Appointment':
            return 'Appointment' as Table;
        case 'Lead':
            return 'Lead' as Table;
        case 'TaxRate':
            return 'TaxRate' as Table;
        case 'Site':
            return 'Site' as Table;
        case 'Employee':
            return 'Employee' as Table;
        case 'Route':
            return 'Route' as Table;
        case 'Company':
            return 'Company' as Table;
        case 'Product':
            return 'Product' as Table;
        case 'Service':
            return 'Service' as Table;
        case 'User':
            return 'User' as Table;
        case 'Order':
            return 'Order' as Table;
        case 'Payment':
            return 'Payment' as Table;
        case 'Package':
            return 'Package' as Table;
        case 'Promotion':
            return 'Promotion' as Table;
        case 'Represents':
            return 'Represents' as Table;
        case 'Ordered':
            return 'Ordered' as Table;
        default:
            return 'Account' as Table;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function tableCreateForm(initial?: Table): TableGigaform {
    const initialVariant:
        | 'Account'
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
        | 'Ordered' =
        (initial as
            | 'Account'
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
            | 'Ordered') ?? 'Account';
    let currentVariant = $state<
        | 'Account'
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
        | 'Ordered'
    >(initialVariant);
    let data = $state<Table>(initial ?? tableGetDefaultForVariant(initialVariant));
    let errors = $state<TableErrors>({} as TableErrors);
    let tainted = $state<TableTainted>({} as TableTainted);
    const variants: TableVariantFields = {
        Account: {
            fields: {} as TableAccountFieldControllers
        },
        Did: {
            fields: {} as TableDidFieldControllers
        },
        Appointment: {
            fields: {} as TableAppointmentFieldControllers
        },
        Lead: {
            fields: {} as TableLeadFieldControllers
        },
        TaxRate: {
            fields: {} as TableTaxRateFieldControllers
        },
        Site: {
            fields: {} as TableSiteFieldControllers
        },
        Employee: {
            fields: {} as TableEmployeeFieldControllers
        },
        Route: {
            fields: {} as TableRouteFieldControllers
        },
        Company: {
            fields: {} as TableCompanyFieldControllers
        },
        Product: {
            fields: {} as TableProductFieldControllers
        },
        Service: {
            fields: {} as TableServiceFieldControllers
        },
        User: {
            fields: {} as TableUserFieldControllers
        },
        Order: {
            fields: {} as TableOrderFieldControllers
        },
        Payment: {
            fields: {} as TablePaymentFieldControllers
        },
        Package: {
            fields: {} as TablePackageFieldControllers
        },
        Promotion: {
            fields: {} as TablePromotionFieldControllers
        },
        Represents: {
            fields: {} as TableRepresentsFieldControllers
        },
        Ordered: {
            fields: {} as TableOrderedFieldControllers
        }
    };
    function switchVariant(
        variant:
            | 'Account'
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
            | 'Ordered'
    ): void {
        currentVariant = variant;
        data = tableGetDefaultForVariant(variant);
        errors = {} as TableErrors;
        tainted = {} as TableTainted;
    }
    function validate(): Result<Table, Array<{ field: string; message: string }>> {
        return tableFromObject(data);
    }
    function reset(overrides?: Partial<Table>): void {
        data = overrides ? (overrides as typeof data) : tableGetDefaultForVariant(currentVariant);
        errors = {} as TableErrors;
        tainted = {} as TableTainted;
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
export function tableFromFormData(
    formData: FormData
): Result<Table, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as
        | 'Account'
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
        | 'Ordered'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Account') {
    } else if (discriminant === 'Did') {
    } else if (discriminant === 'Appointment') {
    } else if (discriminant === 'Lead') {
    } else if (discriminant === 'TaxRate') {
    } else if (discriminant === 'Site') {
    } else if (discriminant === 'Employee') {
    } else if (discriminant === 'Route') {
    } else if (discriminant === 'Company') {
    } else if (discriminant === 'Product') {
    } else if (discriminant === 'Service') {
    } else if (discriminant === 'User') {
    } else if (discriminant === 'Order') {
    } else if (discriminant === 'Payment') {
    } else if (discriminant === 'Package') {
    } else if (discriminant === 'Promotion') {
    } else if (discriminant === 'Represents') {
    } else if (discriminant === 'Ordered') {
    }
    return tableFromStringifiedJSON(JSON.stringify(obj));
}

export const Table = {
    defaultValue: tableDefaultValue,
    serialize: tableSerialize,
    serializeWithContext: tableSerializeWithContext,
    deserialize: tableDeserialize,
    deserializeWithContext: tableDeserializeWithContext,
    is: tableIs,
    createForm: tableCreateForm,
    fromFormData: tableFromFormData
} as const;
