import { accountDefaultValue } from './account.svelte';
import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { accountDeserializeWithContext } from './account.svelte';
import { appointmentDeserializeWithContext } from './appointment.svelte';
import { companyDeserializeWithContext } from './company.svelte';
import { employeeDeserializeWithContext } from './employee.svelte';
import { leadDeserializeWithContext } from './lead.svelte';
import { orderDeserializeWithContext } from './order.svelte';
import { orderedDeserializeWithContext } from './ordered.svelte';
import { packageDeserializeWithContext } from './package.svelte';
import { paymentDeserializeWithContext } from './payment.svelte';
import { productDeserializeWithContext } from './product.svelte';
import { promotionDeserializeWithContext } from './promotion.svelte';
import { representsDeserializeWithContext } from './represents.svelte';
import { routeDeserializeWithContext } from './route.svelte';
import { serviceDeserializeWithContext } from './service.svelte';
import { siteDeserializeWithContext } from './site.svelte';
import { taxRateDeserializeWithContext } from './tax-rate.svelte';
import { userDeserializeWithContext } from './user.svelte';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
import { appointmentDefaultValue } from './appointment.svelte';
import { companyDefaultValue } from './company.svelte';
import { employeeDefaultValue } from './employee.svelte';
import { leadDefaultValue } from './lead.svelte';
import { orderDefaultValue } from './order.svelte';
import { orderedDefaultValue } from './ordered.svelte';
import { packageDefaultValue } from './package.svelte';
import { paymentDefaultValue } from './payment.svelte';
import { productDefaultValue } from './product.svelte';
import { promotionDefaultValue } from './promotion.svelte';
import { representsDefaultValue } from './represents.svelte';
import { routeDefaultValue } from './route.svelte';
import { serviceDefaultValue } from './service.svelte';
import { siteDefaultValue } from './site.svelte';
import { taxRateDefaultValue } from './tax-rate.svelte';
import { userDefaultValue } from './user.svelte';
/** import macro {Gigaform} from "@playground/macro"; */

import type { User } from './user.svelte';
import type { Service } from './service.svelte';
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

export function targetDefaultValue(): Target {
    return accountDefaultValue();
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function targetSerialize(
    value: Target
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(targetSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function targetSerializeWithContext(value: Target, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function targetDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Result<Target, Array<{ field: string; message: string }>> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = targetDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Target.deserialize: root cannot be a forward reference'
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
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function targetDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): Target | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Target | PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new DeserializeError([
            { field: '_root', message: 'Target.deserializeWithContext: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new DeserializeError([
            {
                field: '_root',
                message: 'Target.deserializeWithContext: missing __type field for union dispatch'
            }
        ]);
    }
    if (__typeName === 'Account') {
        return accountDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'User') {
        return userDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'Employee') {
        return employeeDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'Appointment') {
        return appointmentDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'Lead') {
        return leadDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'TaxRate') {
        return taxRateDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'Site') {
        return siteDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'Route') {
        return routeDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'Company') {
        return companyDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'Product') {
        return productDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'Service') {
        return serviceDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'Order') {
        return orderDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'Payment') {
        return paymentDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'Package') {
        return packageDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'Promotion') {
        return promotionDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'Represents') {
        return representsDeserializeWithContext(value, ctx) as Target;
    }
    if (__typeName === 'Ordered') {
        return orderedDeserializeWithContext(value, ctx) as Target;
    }
    throw new DeserializeError([
        {
            field: '_root',
            message:
                'Target.deserializeWithContext: unknown type "' +
                __typeName +
                '". Expected one of: Account, User, Employee, Appointment, Lead, TaxRate, Site, Route, Company, Product, Service, Order, Payment, Package, Promotion, Represents, Ordered'
        }
    ]);
}
export function targetIs(value: unknown): value is Target {
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

/** Per-variant error types */ export type TargetAccountErrors = { _errors: Option<Array<string>> };
export type TargetUserErrors = { _errors: Option<Array<string>> };
export type TargetEmployeeErrors = { _errors: Option<Array<string>> };
export type TargetAppointmentErrors = { _errors: Option<Array<string>> };
export type TargetLeadErrors = { _errors: Option<Array<string>> };
export type TargetTaxRateErrors = { _errors: Option<Array<string>> };
export type TargetSiteErrors = { _errors: Option<Array<string>> };
export type TargetRouteErrors = { _errors: Option<Array<string>> };
export type TargetCompanyErrors = { _errors: Option<Array<string>> };
export type TargetProductErrors = { _errors: Option<Array<string>> };
export type TargetServiceErrors = { _errors: Option<Array<string>> };
export type TargetOrderErrors = { _errors: Option<Array<string>> };
export type TargetPaymentErrors = { _errors: Option<Array<string>> };
export type TargetPackageErrors = { _errors: Option<Array<string>> };
export type TargetPromotionErrors = { _errors: Option<Array<string>> };
export type TargetRepresentsErrors = { _errors: Option<Array<string>> };
export type TargetOrderedErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type TargetAccountTainted = {};
export type TargetUserTainted = {};
export type TargetEmployeeTainted = {};
export type TargetAppointmentTainted = {};
export type TargetLeadTainted = {};
export type TargetTaxRateTainted = {};
export type TargetSiteTainted = {};
export type TargetRouteTainted = {};
export type TargetCompanyTainted = {};
export type TargetProductTainted = {};
export type TargetServiceTainted = {};
export type TargetOrderTainted = {};
export type TargetPaymentTainted = {};
export type TargetPackageTainted = {};
export type TargetPromotionTainted = {};
export type TargetRepresentsTainted = {};
export type TargetOrderedTainted = {}; /** Union error type */
export type TargetErrors =
    | ({ _type: 'Account' } & TargetAccountErrors)
    | ({ _type: 'User' } & TargetUserErrors)
    | ({ _type: 'Employee' } & TargetEmployeeErrors)
    | ({ _type: 'Appointment' } & TargetAppointmentErrors)
    | ({ _type: 'Lead' } & TargetLeadErrors)
    | ({ _type: 'TaxRate' } & TargetTaxRateErrors)
    | ({ _type: 'Site' } & TargetSiteErrors)
    | ({ _type: 'Route' } & TargetRouteErrors)
    | ({ _type: 'Company' } & TargetCompanyErrors)
    | ({ _type: 'Product' } & TargetProductErrors)
    | ({ _type: 'Service' } & TargetServiceErrors)
    | ({ _type: 'Order' } & TargetOrderErrors)
    | ({ _type: 'Payment' } & TargetPaymentErrors)
    | ({ _type: 'Package' } & TargetPackageErrors)
    | ({ _type: 'Promotion' } & TargetPromotionErrors)
    | ({ _type: 'Represents' } & TargetRepresentsErrors)
    | ({ _type: 'Ordered' } & TargetOrderedErrors); /** Union tainted type */
export type TargetTainted =
    | ({ _type: 'Account' } & TargetAccountTainted)
    | ({ _type: 'User' } & TargetUserTainted)
    | ({ _type: 'Employee' } & TargetEmployeeTainted)
    | ({ _type: 'Appointment' } & TargetAppointmentTainted)
    | ({ _type: 'Lead' } & TargetLeadTainted)
    | ({ _type: 'TaxRate' } & TargetTaxRateTainted)
    | ({ _type: 'Site' } & TargetSiteTainted)
    | ({ _type: 'Route' } & TargetRouteTainted)
    | ({ _type: 'Company' } & TargetCompanyTainted)
    | ({ _type: 'Product' } & TargetProductTainted)
    | ({ _type: 'Service' } & TargetServiceTainted)
    | ({ _type: 'Order' } & TargetOrderTainted)
    | ({ _type: 'Payment' } & TargetPaymentTainted)
    | ({ _type: 'Package' } & TargetPackageTainted)
    | ({ _type: 'Promotion' } & TargetPromotionTainted)
    | ({ _type: 'Represents' } & TargetRepresentsTainted)
    | ({ _type: 'Ordered' } & TargetOrderedTainted); /** Per-variant field controller types */
export interface TargetAccountFieldControllers {}
export interface TargetUserFieldControllers {}
export interface TargetEmployeeFieldControllers {}
export interface TargetAppointmentFieldControllers {}
export interface TargetLeadFieldControllers {}
export interface TargetTaxRateFieldControllers {}
export interface TargetSiteFieldControllers {}
export interface TargetRouteFieldControllers {}
export interface TargetCompanyFieldControllers {}
export interface TargetProductFieldControllers {}
export interface TargetServiceFieldControllers {}
export interface TargetOrderFieldControllers {}
export interface TargetPaymentFieldControllers {}
export interface TargetPackageFieldControllers {}
export interface TargetPromotionFieldControllers {}
export interface TargetRepresentsFieldControllers {}
export interface TargetOrderedFieldControllers {} /** Union Gigaform interface with variant switching */
export interface TargetGigaform {
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
    readonly errors: TargetErrors;
    readonly tainted: TargetTainted;
    readonly variants: TargetVariantFields;
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
export interface TargetVariantFields {
    readonly Account: { readonly fields: TargetAccountFieldControllers };
    readonly User: { readonly fields: TargetUserFieldControllers };
    readonly Employee: { readonly fields: TargetEmployeeFieldControllers };
    readonly Appointment: { readonly fields: TargetAppointmentFieldControllers };
    readonly Lead: { readonly fields: TargetLeadFieldControllers };
    readonly TaxRate: { readonly fields: TargetTaxRateFieldControllers };
    readonly Site: { readonly fields: TargetSiteFieldControllers };
    readonly Route: { readonly fields: TargetRouteFieldControllers };
    readonly Company: { readonly fields: TargetCompanyFieldControllers };
    readonly Product: { readonly fields: TargetProductFieldControllers };
    readonly Service: { readonly fields: TargetServiceFieldControllers };
    readonly Order: { readonly fields: TargetOrderFieldControllers };
    readonly Payment: { readonly fields: TargetPaymentFieldControllers };
    readonly Package: { readonly fields: TargetPackageFieldControllers };
    readonly Promotion: { readonly fields: TargetPromotionFieldControllers };
    readonly Represents: { readonly fields: TargetRepresentsFieldControllers };
    readonly Ordered: { readonly fields: TargetOrderedFieldControllers };
} /** Gets default value for a specific variant */
function targetGetDefaultForVariant(variant: string): Target {
    switch (variant) {
        case 'Account':
            return accountDefaultValue() as Target;
        case 'User':
            return userDefaultValue() as Target;
        case 'Employee':
            return employeeDefaultValue() as Target;
        case 'Appointment':
            return appointmentDefaultValue() as Target;
        case 'Lead':
            return leadDefaultValue() as Target;
        case 'TaxRate':
            return taxRateDefaultValue() as Target;
        case 'Site':
            return siteDefaultValue() as Target;
        case 'Route':
            return routeDefaultValue() as Target;
        case 'Company':
            return companyDefaultValue() as Target;
        case 'Product':
            return productDefaultValue() as Target;
        case 'Service':
            return serviceDefaultValue() as Target;
        case 'Order':
            return orderDefaultValue() as Target;
        case 'Payment':
            return paymentDefaultValue() as Target;
        case 'Package':
            return packageDefaultValue() as Target;
        case 'Promotion':
            return promotionDefaultValue() as Target;
        case 'Represents':
            return representsDefaultValue() as Target;
        case 'Ordered':
            return orderedDefaultValue() as Target;
        default:
            return accountDefaultValue() as Target;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function targetCreateForm(initial?: Target): TargetGigaform {
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
    let data = $state<Target>(initial ?? targetGetDefaultForVariant(initialVariant));
    let errors = $state<TargetErrors>({} as TargetErrors);
    let tainted = $state<TargetTainted>({} as TargetTainted);
    const variants: TargetVariantFields = {
        Account: {
            fields: {} as TargetAccountFieldControllers
        },
        User: {
            fields: {} as TargetUserFieldControllers
        },
        Employee: {
            fields: {} as TargetEmployeeFieldControllers
        },
        Appointment: {
            fields: {} as TargetAppointmentFieldControllers
        },
        Lead: {
            fields: {} as TargetLeadFieldControllers
        },
        TaxRate: {
            fields: {} as TargetTaxRateFieldControllers
        },
        Site: {
            fields: {} as TargetSiteFieldControllers
        },
        Route: {
            fields: {} as TargetRouteFieldControllers
        },
        Company: {
            fields: {} as TargetCompanyFieldControllers
        },
        Product: {
            fields: {} as TargetProductFieldControllers
        },
        Service: {
            fields: {} as TargetServiceFieldControllers
        },
        Order: {
            fields: {} as TargetOrderFieldControllers
        },
        Payment: {
            fields: {} as TargetPaymentFieldControllers
        },
        Package: {
            fields: {} as TargetPackageFieldControllers
        },
        Promotion: {
            fields: {} as TargetPromotionFieldControllers
        },
        Represents: {
            fields: {} as TargetRepresentsFieldControllers
        },
        Ordered: {
            fields: {} as TargetOrderedFieldControllers
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
        data = targetGetDefaultForVariant(variant);
        errors = {} as TargetErrors;
        tainted = {} as TargetTainted;
    }
    function validate(): Result<Target, Array<{ field: string; message: string }>> {
        return targetFromObject(data);
    }
    function reset(overrides?: Partial<Target>): void {
        data = overrides ? (overrides as typeof data) : targetGetDefaultForVariant(currentVariant);
        errors = {} as TargetErrors;
        tainted = {} as TargetTainted;
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
export function targetFromFormData(
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
    return targetFromStringifiedJSON(JSON.stringify(obj));
}

export const Target = {
    defaultValue: targetDefaultValue,
    serialize: targetSerialize,
    serializeWithContext: targetSerializeWithContext,
    deserialize: targetDeserialize,
    deserializeWithContext: targetDeserializeWithContext,
    is: targetIs,
    createForm: targetCreateForm,
    fromFormData: targetFromFormData
} as const;
