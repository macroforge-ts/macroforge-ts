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

export type Target =
    | /** @default */ Account
    | User
    | Employee
    | Appointment
    | Lead
    | TaxRate
    | Site
    | Route
    | Company
    | Product
    | Service
    | Order
    | Payment
    | Package
    | Promotion
    | Represents
    | Ordered;

export function defaultValueTarget(): Target {
    return Account.defaultValue();
}

export function toStringifiedJSONTarget(value: Target): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeTarget(value, ctx));
}
export function toObjectTarget(value: Target): unknown {
    const ctx = SerializeContext.create();
    return __serializeTarget(value, ctx);
}
export function __serializeTarget(value: Target, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONTarget(
    json: string,
    opts?: DeserializeOptions
): Result<Target, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectTarget(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectTarget(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Target, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeTarget(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Target.fromObject: root cannot be a forward reference' }
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
export function __deserializeTarget(value: any, ctx: DeserializeContext): Target | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Target | PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new DeserializeError([
            { field: '_root', message: 'Target.__deserialize: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new DeserializeError([
            {
                field: '_root',
                message: 'Target.__deserialize: missing __type field for union dispatch'
            }
        ]);
    }
    if (__typeName === 'Account') {
        if (typeof (Account as any)?.__deserialize === 'function') {
            return (Account as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'User') {
        if (typeof (User as any)?.__deserialize === 'function') {
            return (User as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'Employee') {
        if (typeof (Employee as any)?.__deserialize === 'function') {
            return (Employee as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'Appointment') {
        if (typeof (Appointment as any)?.__deserialize === 'function') {
            return (Appointment as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'Lead') {
        if (typeof (Lead as any)?.__deserialize === 'function') {
            return (Lead as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'TaxRate') {
        if (typeof (TaxRate as any)?.__deserialize === 'function') {
            return (TaxRate as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'Site') {
        if (typeof (Site as any)?.__deserialize === 'function') {
            return (Site as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'Route') {
        if (typeof (Route as any)?.__deserialize === 'function') {
            return (Route as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'Company') {
        if (typeof (Company as any)?.__deserialize === 'function') {
            return (Company as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'Product') {
        if (typeof (Product as any)?.__deserialize === 'function') {
            return (Product as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'Service') {
        if (typeof (Service as any)?.__deserialize === 'function') {
            return (Service as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'Order') {
        if (typeof (Order as any)?.__deserialize === 'function') {
            return (Order as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'Payment') {
        if (typeof (Payment as any)?.__deserialize === 'function') {
            return (Payment as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'Package') {
        if (typeof (Package as any)?.__deserialize === 'function') {
            return (Package as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'Promotion') {
        if (typeof (Promotion as any)?.__deserialize === 'function') {
            return (Promotion as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'Represents') {
        if (typeof (Represents as any)?.__deserialize === 'function') {
            return (Represents as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    if (__typeName === 'Ordered') {
        if (typeof (Ordered as any)?.__deserialize === 'function') {
            return (Ordered as any).__deserialize(value, ctx) as Target;
        }
        return value as Target;
    }
    throw new DeserializeError([
        {
            field: '_root',
            message:
                'Target.__deserialize: unknown type "' +
                __typeName +
                '". Expected one of: Account, User, Employee, Appointment, Lead, TaxRate, Site, Route, Company, Product, Service, Order, Payment, Package, Promotion, Represents, Ordered'
        }
    ]);
}
export function isTarget(value: unknown): value is Target {
    if (typeof value !== 'object' || value === null) {
        return false;
    }
    const __typeName = (value as any).__type;
    return (
        __typeName === 'Account' ||
        __typeName === 'User' ||
        __typeName === 'Employee' ||
        __typeName === 'Appointment' ||
        __typeName === 'Lead' ||
        __typeName === 'TaxRate' ||
        __typeName === 'Site' ||
        __typeName === 'Route' ||
        __typeName === 'Company' ||
        __typeName === 'Product' ||
        __typeName === 'Service' ||
        __typeName === 'Order' ||
        __typeName === 'Payment' ||
        __typeName === 'Package' ||
        __typeName === 'Promotion' ||
        __typeName === 'Represents' ||
        __typeName === 'Ordered'
    );
}

/** Per-variant error types */ export type AccountErrorsTarget = { _errors: Option<Array<string>> };
export type UserErrorsTarget = { _errors: Option<Array<string>> };
export type EmployeeErrorsTarget = { _errors: Option<Array<string>> };
export type AppointmentErrorsTarget = { _errors: Option<Array<string>> };
export type LeadErrorsTarget = { _errors: Option<Array<string>> };
export type TaxRateErrorsTarget = { _errors: Option<Array<string>> };
export type SiteErrorsTarget = { _errors: Option<Array<string>> };
export type RouteErrorsTarget = { _errors: Option<Array<string>> };
export type CompanyErrorsTarget = { _errors: Option<Array<string>> };
export type ProductErrorsTarget = { _errors: Option<Array<string>> };
export type ServiceErrorsTarget = { _errors: Option<Array<string>> };
export type OrderErrorsTarget = { _errors: Option<Array<string>> };
export type PaymentErrorsTarget = { _errors: Option<Array<string>> };
export type PackageErrorsTarget = { _errors: Option<Array<string>> };
export type PromotionErrorsTarget = { _errors: Option<Array<string>> };
export type RepresentsErrorsTarget = { _errors: Option<Array<string>> };
export type OrderedErrorsTarget = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type AccountTaintedTarget = {};
export type UserTaintedTarget = {};
export type EmployeeTaintedTarget = {};
export type AppointmentTaintedTarget = {};
export type LeadTaintedTarget = {};
export type TaxRateTaintedTarget = {};
export type SiteTaintedTarget = {};
export type RouteTaintedTarget = {};
export type CompanyTaintedTarget = {};
export type ProductTaintedTarget = {};
export type ServiceTaintedTarget = {};
export type OrderTaintedTarget = {};
export type PaymentTaintedTarget = {};
export type PackageTaintedTarget = {};
export type PromotionTaintedTarget = {};
export type RepresentsTaintedTarget = {};
export type OrderedTaintedTarget = {}; /** Union error type */
export type ErrorsTarget =
    | ({ _type: 'Account' } & AccountErrorsTarget)
    | ({ _type: 'User' } & UserErrorsTarget)
    | ({ _type: 'Employee' } & EmployeeErrorsTarget)
    | ({ _type: 'Appointment' } & AppointmentErrorsTarget)
    | ({ _type: 'Lead' } & LeadErrorsTarget)
    | ({ _type: 'TaxRate' } & TaxRateErrorsTarget)
    | ({ _type: 'Site' } & SiteErrorsTarget)
    | ({ _type: 'Route' } & RouteErrorsTarget)
    | ({ _type: 'Company' } & CompanyErrorsTarget)
    | ({ _type: 'Product' } & ProductErrorsTarget)
    | ({ _type: 'Service' } & ServiceErrorsTarget)
    | ({ _type: 'Order' } & OrderErrorsTarget)
    | ({ _type: 'Payment' } & PaymentErrorsTarget)
    | ({ _type: 'Package' } & PackageErrorsTarget)
    | ({ _type: 'Promotion' } & PromotionErrorsTarget)
    | ({ _type: 'Represents' } & RepresentsErrorsTarget)
    | ({ _type: 'Ordered' } & OrderedErrorsTarget); /** Union tainted type */
export type TaintedTarget =
    | ({ _type: 'Account' } & AccountTaintedTarget)
    | ({ _type: 'User' } & UserTaintedTarget)
    | ({ _type: 'Employee' } & EmployeeTaintedTarget)
    | ({ _type: 'Appointment' } & AppointmentTaintedTarget)
    | ({ _type: 'Lead' } & LeadTaintedTarget)
    | ({ _type: 'TaxRate' } & TaxRateTaintedTarget)
    | ({ _type: 'Site' } & SiteTaintedTarget)
    | ({ _type: 'Route' } & RouteTaintedTarget)
    | ({ _type: 'Company' } & CompanyTaintedTarget)
    | ({ _type: 'Product' } & ProductTaintedTarget)
    | ({ _type: 'Service' } & ServiceTaintedTarget)
    | ({ _type: 'Order' } & OrderTaintedTarget)
    | ({ _type: 'Payment' } & PaymentTaintedTarget)
    | ({ _type: 'Package' } & PackageTaintedTarget)
    | ({ _type: 'Promotion' } & PromotionTaintedTarget)
    | ({ _type: 'Represents' } & RepresentsTaintedTarget)
    | ({ _type: 'Ordered' } & OrderedTaintedTarget); /** Per-variant field controller types */
export interface AccountFieldControllersTarget {}
export interface UserFieldControllersTarget {}
export interface EmployeeFieldControllersTarget {}
export interface AppointmentFieldControllersTarget {}
export interface LeadFieldControllersTarget {}
export interface TaxRateFieldControllersTarget {}
export interface SiteFieldControllersTarget {}
export interface RouteFieldControllersTarget {}
export interface CompanyFieldControllersTarget {}
export interface ProductFieldControllersTarget {}
export interface ServiceFieldControllersTarget {}
export interface OrderFieldControllersTarget {}
export interface PaymentFieldControllersTarget {}
export interface PackageFieldControllersTarget {}
export interface PromotionFieldControllersTarget {}
export interface RepresentsFieldControllersTarget {}
export interface OrderedFieldControllersTarget {} /** Union Gigaform interface with variant switching */
export interface GigaformTarget {
    readonly currentVariant:
        | 'Account'
        | 'User'
        | 'Employee'
        | 'Appointment'
        | 'Lead'
        | 'TaxRate'
        | 'Site'
        | 'Route'
        | 'Company'
        | 'Product'
        | 'Service'
        | 'Order'
        | 'Payment'
        | 'Package'
        | 'Promotion'
        | 'Represents'
        | 'Ordered';
    readonly data: Target;
    readonly errors: ErrorsTarget;
    readonly tainted: TaintedTarget;
    readonly variants: VariantFieldsTarget;
    switchVariant(
        variant:
            | 'Account'
            | 'User'
            | 'Employee'
            | 'Appointment'
            | 'Lead'
            | 'TaxRate'
            | 'Site'
            | 'Route'
            | 'Company'
            | 'Product'
            | 'Service'
            | 'Order'
            | 'Payment'
            | 'Package'
            | 'Promotion'
            | 'Represents'
            | 'Ordered'
    ): void;
    validate(): Result<Target, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Target>): void;
} /** Variant fields container */
export interface VariantFieldsTarget {
    readonly Account: { readonly fields: AccountFieldControllersTarget };
    readonly User: { readonly fields: UserFieldControllersTarget };
    readonly Employee: { readonly fields: EmployeeFieldControllersTarget };
    readonly Appointment: { readonly fields: AppointmentFieldControllersTarget };
    readonly Lead: { readonly fields: LeadFieldControllersTarget };
    readonly TaxRate: { readonly fields: TaxRateFieldControllersTarget };
    readonly Site: { readonly fields: SiteFieldControllersTarget };
    readonly Route: { readonly fields: RouteFieldControllersTarget };
    readonly Company: { readonly fields: CompanyFieldControllersTarget };
    readonly Product: { readonly fields: ProductFieldControllersTarget };
    readonly Service: { readonly fields: ServiceFieldControllersTarget };
    readonly Order: { readonly fields: OrderFieldControllersTarget };
    readonly Payment: { readonly fields: PaymentFieldControllersTarget };
    readonly Package: { readonly fields: PackageFieldControllersTarget };
    readonly Promotion: { readonly fields: PromotionFieldControllersTarget };
    readonly Represents: { readonly fields: RepresentsFieldControllersTarget };
    readonly Ordered: { readonly fields: OrderedFieldControllersTarget };
} /** Gets default value for a specific variant */
function getDefaultForVariantTarget(variant: string): Target {
    switch (variant) {
        case 'Account':
            return Account.defaultValue() as Target;
        case 'User':
            return User.defaultValue() as Target;
        case 'Employee':
            return Employee.defaultValue() as Target;
        case 'Appointment':
            return Appointment.defaultValue() as Target;
        case 'Lead':
            return Lead.defaultValue() as Target;
        case 'TaxRate':
            return TaxRate.defaultValue() as Target;
        case 'Site':
            return Site.defaultValue() as Target;
        case 'Route':
            return Route.defaultValue() as Target;
        case 'Company':
            return Company.defaultValue() as Target;
        case 'Product':
            return Product.defaultValue() as Target;
        case 'Service':
            return Service.defaultValue() as Target;
        case 'Order':
            return Order.defaultValue() as Target;
        case 'Payment':
            return Payment.defaultValue() as Target;
        case 'Package':
            return Package.defaultValue() as Target;
        case 'Promotion':
            return Promotion.defaultValue() as Target;
        case 'Represents':
            return Represents.defaultValue() as Target;
        case 'Ordered':
            return Ordered.defaultValue() as Target;
        default:
            return Account.defaultValue() as Target;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormTarget(initial?: Target): GigaformTarget {
    const initialVariant:
        | 'Account'
        | 'User'
        | 'Employee'
        | 'Appointment'
        | 'Lead'
        | 'TaxRate'
        | 'Site'
        | 'Route'
        | 'Company'
        | 'Product'
        | 'Service'
        | 'Order'
        | 'Payment'
        | 'Package'
        | 'Promotion'
        | 'Represents'
        | 'Ordered' = 'Account';
    let currentVariant = $state<
        | 'Account'
        | 'User'
        | 'Employee'
        | 'Appointment'
        | 'Lead'
        | 'TaxRate'
        | 'Site'
        | 'Route'
        | 'Company'
        | 'Product'
        | 'Service'
        | 'Order'
        | 'Payment'
        | 'Package'
        | 'Promotion'
        | 'Represents'
        | 'Ordered'
    >(initialVariant);
    let data = $state<Target>(initial ?? getDefaultForVariantTarget(initialVariant));
    let errors = $state<ErrorsTarget>({} as ErrorsTarget);
    let tainted = $state<TaintedTarget>({} as TaintedTarget);
    const variants: VariantFieldsTarget = {
        Account: {
            fields: {} as AccountFieldControllersTarget
        },
        User: {
            fields: {} as UserFieldControllersTarget
        },
        Employee: {
            fields: {} as EmployeeFieldControllersTarget
        },
        Appointment: {
            fields: {} as AppointmentFieldControllersTarget
        },
        Lead: {
            fields: {} as LeadFieldControllersTarget
        },
        TaxRate: {
            fields: {} as TaxRateFieldControllersTarget
        },
        Site: {
            fields: {} as SiteFieldControllersTarget
        },
        Route: {
            fields: {} as RouteFieldControllersTarget
        },
        Company: {
            fields: {} as CompanyFieldControllersTarget
        },
        Product: {
            fields: {} as ProductFieldControllersTarget
        },
        Service: {
            fields: {} as ServiceFieldControllersTarget
        },
        Order: {
            fields: {} as OrderFieldControllersTarget
        },
        Payment: {
            fields: {} as PaymentFieldControllersTarget
        },
        Package: {
            fields: {} as PackageFieldControllersTarget
        },
        Promotion: {
            fields: {} as PromotionFieldControllersTarget
        },
        Represents: {
            fields: {} as RepresentsFieldControllersTarget
        },
        Ordered: {
            fields: {} as OrderedFieldControllersTarget
        }
    };
    function switchVariant(
        variant:
            | 'Account'
            | 'User'
            | 'Employee'
            | 'Appointment'
            | 'Lead'
            | 'TaxRate'
            | 'Site'
            | 'Route'
            | 'Company'
            | 'Product'
            | 'Service'
            | 'Order'
            | 'Payment'
            | 'Package'
            | 'Promotion'
            | 'Represents'
            | 'Ordered'
    ): void {
        currentVariant = variant;
        data = getDefaultForVariantTarget(variant);
        errors = {} as ErrorsTarget;
        tainted = {} as TaintedTarget;
    }
    function validate(): Result<Target, Array<{ field: string; message: string }>> {
        return Target.fromObject(data);
    }
    function reset(overrides?: Partial<Target>): void {
        data = overrides ? (overrides as typeof data) : getDefaultForVariantTarget(currentVariant);
        errors = {} as ErrorsTarget;
        tainted = {} as TaintedTarget;
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
export function fromFormDataTarget(
    formData: FormData
): Result<Target, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_type') as
        | 'Account'
        | 'User'
        | 'Employee'
        | 'Appointment'
        | 'Lead'
        | 'TaxRate'
        | 'Site'
        | 'Route'
        | 'Company'
        | 'Product'
        | 'Service'
        | 'Order'
        | 'Payment'
        | 'Package'
        | 'Promotion'
        | 'Represents'
        | 'Ordered'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_type', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._type = discriminant;
    if (discriminant === 'Account') {
    } else if (discriminant === 'User') {
    } else if (discriminant === 'Employee') {
    } else if (discriminant === 'Appointment') {
    } else if (discriminant === 'Lead') {
    } else if (discriminant === 'TaxRate') {
    } else if (discriminant === 'Site') {
    } else if (discriminant === 'Route') {
    } else if (discriminant === 'Company') {
    } else if (discriminant === 'Product') {
    } else if (discriminant === 'Service') {
    } else if (discriminant === 'Order') {
    } else if (discriminant === 'Payment') {
    } else if (discriminant === 'Package') {
    } else if (discriminant === 'Promotion') {
    } else if (discriminant === 'Represents') {
    } else if (discriminant === 'Ordered') {
    }
    return Target.fromStringifiedJSON(JSON.stringify(obj));
}
