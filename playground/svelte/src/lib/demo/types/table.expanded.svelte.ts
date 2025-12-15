import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import { User } from './user.svelte';
import { Service } from './service.svelte';
import { Did } from './did.svelte';
import { Promotion } from './promotion.svelte';
import { Site } from './site.svelte';
import { Product } from './product.svelte';
import { Represents } from './represents.svelte';
import { Payment } from './payment.svelte';
import { Appointment } from './appointment.svelte';
import { Package } from './package.svelte';
import { Account } from './account.svelte';
import { Order } from './order.svelte';
import { TaxRate } from './tax-rate.svelte';
import { Lead } from './lead.svelte';
import { Company } from './company.svelte';
import { Employee } from './employee.svelte';
import { Route } from './route.svelte';
import { Ordered } from './ordered.svelte';

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

export function defaultValueTable(): Table {
    return 'Account';
}

export function toStringifiedJSONTable(value: Table): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeTable(value, ctx));
}
export function toObjectTable(value: Table): unknown {
    const ctx = SerializeContext.create();
    return __serializeTable(value, ctx);
}
export function __serializeTable(value: Table, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONTable(
    json: string,
    opts?: DeserializeOptions
): Result<Table, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectTable(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectTable(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Table, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeTable(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Table.fromObject: root cannot be a forward reference' }
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
export function __deserializeTable(value: any, ctx: DeserializeContext): Table | PendingRef {
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
export function isTable(value: unknown): value is Table {
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

/** Per-variant error types */ export type AccountErrorsTable = { _errors: Option<Array<string>> };
export type DidErrorsTable = { _errors: Option<Array<string>> };
export type AppointmentErrorsTable = { _errors: Option<Array<string>> };
export type LeadErrorsTable = { _errors: Option<Array<string>> };
export type TaxRateErrorsTable = { _errors: Option<Array<string>> };
export type SiteErrorsTable = { _errors: Option<Array<string>> };
export type EmployeeErrorsTable = { _errors: Option<Array<string>> };
export type RouteErrorsTable = { _errors: Option<Array<string>> };
export type CompanyErrorsTable = { _errors: Option<Array<string>> };
export type ProductErrorsTable = { _errors: Option<Array<string>> };
export type ServiceErrorsTable = { _errors: Option<Array<string>> };
export type UserErrorsTable = { _errors: Option<Array<string>> };
export type OrderErrorsTable = { _errors: Option<Array<string>> };
export type PaymentErrorsTable = { _errors: Option<Array<string>> };
export type PackageErrorsTable = { _errors: Option<Array<string>> };
export type PromotionErrorsTable = { _errors: Option<Array<string>> };
export type RepresentsErrorsTable = { _errors: Option<Array<string>> };
export type OrderedErrorsTable = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type AccountTaintedTable = {};
export type DidTaintedTable = {};
export type AppointmentTaintedTable = {};
export type LeadTaintedTable = {};
export type TaxRateTaintedTable = {};
export type SiteTaintedTable = {};
export type EmployeeTaintedTable = {};
export type RouteTaintedTable = {};
export type CompanyTaintedTable = {};
export type ProductTaintedTable = {};
export type ServiceTaintedTable = {};
export type UserTaintedTable = {};
export type OrderTaintedTable = {};
export type PaymentTaintedTable = {};
export type PackageTaintedTable = {};
export type PromotionTaintedTable = {};
export type RepresentsTaintedTable = {};
export type OrderedTaintedTable = {}; /** Union error type */
export type ErrorsTable =
    | ({ _value: 'Account' } & AccountErrorsTable)
    | ({ _value: 'Did' } & DidErrorsTable)
    | ({ _value: 'Appointment' } & AppointmentErrorsTable)
    | ({ _value: 'Lead' } & LeadErrorsTable)
    | ({ _value: 'TaxRate' } & TaxRateErrorsTable)
    | ({ _value: 'Site' } & SiteErrorsTable)
    | ({ _value: 'Employee' } & EmployeeErrorsTable)
    | ({ _value: 'Route' } & RouteErrorsTable)
    | ({ _value: 'Company' } & CompanyErrorsTable)
    | ({ _value: 'Product' } & ProductErrorsTable)
    | ({ _value: 'Service' } & ServiceErrorsTable)
    | ({ _value: 'User' } & UserErrorsTable)
    | ({ _value: 'Order' } & OrderErrorsTable)
    | ({ _value: 'Payment' } & PaymentErrorsTable)
    | ({ _value: 'Package' } & PackageErrorsTable)
    | ({ _value: 'Promotion' } & PromotionErrorsTable)
    | ({ _value: 'Represents' } & RepresentsErrorsTable)
    | ({ _value: 'Ordered' } & OrderedErrorsTable); /** Union tainted type */
export type TaintedTable =
    | ({ _value: 'Account' } & AccountTaintedTable)
    | ({ _value: 'Did' } & DidTaintedTable)
    | ({ _value: 'Appointment' } & AppointmentTaintedTable)
    | ({ _value: 'Lead' } & LeadTaintedTable)
    | ({ _value: 'TaxRate' } & TaxRateTaintedTable)
    | ({ _value: 'Site' } & SiteTaintedTable)
    | ({ _value: 'Employee' } & EmployeeTaintedTable)
    | ({ _value: 'Route' } & RouteTaintedTable)
    | ({ _value: 'Company' } & CompanyTaintedTable)
    | ({ _value: 'Product' } & ProductTaintedTable)
    | ({ _value: 'Service' } & ServiceTaintedTable)
    | ({ _value: 'User' } & UserTaintedTable)
    | ({ _value: 'Order' } & OrderTaintedTable)
    | ({ _value: 'Payment' } & PaymentTaintedTable)
    | ({ _value: 'Package' } & PackageTaintedTable)
    | ({ _value: 'Promotion' } & PromotionTaintedTable)
    | ({ _value: 'Represents' } & RepresentsTaintedTable)
    | ({ _value: 'Ordered' } & OrderedTaintedTable); /** Per-variant field controller types */
export interface AccountFieldControllersTable {}
export interface DidFieldControllersTable {}
export interface AppointmentFieldControllersTable {}
export interface LeadFieldControllersTable {}
export interface TaxRateFieldControllersTable {}
export interface SiteFieldControllersTable {}
export interface EmployeeFieldControllersTable {}
export interface RouteFieldControllersTable {}
export interface CompanyFieldControllersTable {}
export interface ProductFieldControllersTable {}
export interface ServiceFieldControllersTable {}
export interface UserFieldControllersTable {}
export interface OrderFieldControllersTable {}
export interface PaymentFieldControllersTable {}
export interface PackageFieldControllersTable {}
export interface PromotionFieldControllersTable {}
export interface RepresentsFieldControllersTable {}
export interface OrderedFieldControllersTable {} /** Union Gigaform interface with variant switching */
export interface GigaformTable {
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
    readonly errors: ErrorsTable;
    readonly tainted: TaintedTable;
    readonly variants: VariantFieldsTable;
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
export interface VariantFieldsTable {
    readonly Account: { readonly fields: AccountFieldControllersTable };
    readonly Did: { readonly fields: DidFieldControllersTable };
    readonly Appointment: { readonly fields: AppointmentFieldControllersTable };
    readonly Lead: { readonly fields: LeadFieldControllersTable };
    readonly TaxRate: { readonly fields: TaxRateFieldControllersTable };
    readonly Site: { readonly fields: SiteFieldControllersTable };
    readonly Employee: { readonly fields: EmployeeFieldControllersTable };
    readonly Route: { readonly fields: RouteFieldControllersTable };
    readonly Company: { readonly fields: CompanyFieldControllersTable };
    readonly Product: { readonly fields: ProductFieldControllersTable };
    readonly Service: { readonly fields: ServiceFieldControllersTable };
    readonly User: { readonly fields: UserFieldControllersTable };
    readonly Order: { readonly fields: OrderFieldControllersTable };
    readonly Payment: { readonly fields: PaymentFieldControllersTable };
    readonly Package: { readonly fields: PackageFieldControllersTable };
    readonly Promotion: { readonly fields: PromotionFieldControllersTable };
    readonly Represents: { readonly fields: RepresentsFieldControllersTable };
    readonly Ordered: { readonly fields: OrderedFieldControllersTable };
} /** Gets default value for a specific variant */
function getDefaultForVariantTable(variant: string): Table {
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
export function createFormTable(initial?: Table): GigaformTable {
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
    let data = $state<Table>(initial ?? getDefaultForVariantTable(initialVariant));
    let errors = $state<ErrorsTable>({} as ErrorsTable);
    let tainted = $state<TaintedTable>({} as TaintedTable);
    const variants: VariantFieldsTable = {
        Account: {
            fields: {} as AccountFieldControllersTable
        },
        Did: {
            fields: {} as DidFieldControllersTable
        },
        Appointment: {
            fields: {} as AppointmentFieldControllersTable
        },
        Lead: {
            fields: {} as LeadFieldControllersTable
        },
        TaxRate: {
            fields: {} as TaxRateFieldControllersTable
        },
        Site: {
            fields: {} as SiteFieldControllersTable
        },
        Employee: {
            fields: {} as EmployeeFieldControllersTable
        },
        Route: {
            fields: {} as RouteFieldControllersTable
        },
        Company: {
            fields: {} as CompanyFieldControllersTable
        },
        Product: {
            fields: {} as ProductFieldControllersTable
        },
        Service: {
            fields: {} as ServiceFieldControllersTable
        },
        User: {
            fields: {} as UserFieldControllersTable
        },
        Order: {
            fields: {} as OrderFieldControllersTable
        },
        Payment: {
            fields: {} as PaymentFieldControllersTable
        },
        Package: {
            fields: {} as PackageFieldControllersTable
        },
        Promotion: {
            fields: {} as PromotionFieldControllersTable
        },
        Represents: {
            fields: {} as RepresentsFieldControllersTable
        },
        Ordered: {
            fields: {} as OrderedFieldControllersTable
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
        data = getDefaultForVariantTable(variant);
        errors = {} as ErrorsTable;
        tainted = {} as TaintedTable;
    }
    function validate(): Result<Table, Array<{ field: string; message: string }>> {
        return fromObjectTable(data);
    }
    function reset(overrides?: Partial<Table>): void {
        data = overrides ? (overrides as typeof data) : getDefaultForVariantTable(currentVariant);
        errors = {} as ErrorsTable;
        tainted = {} as TaintedTable;
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
export function fromFormDataTable(
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
    return fromStringifiedJSONTable(JSON.stringify(obj));
}
