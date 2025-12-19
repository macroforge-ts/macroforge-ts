import type { Option as __gf_Option, Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import {
    DeserializeContext as __mf_DeserializeContext,
    DeserializeError as __mf_DeserializeError,
    PendingRef as __mf_PendingRef,
    SerializeContext as __mf_SerializeContext
} from 'macroforge/serde';
import { accountDefaultValue, accountDeserializeWithContext } from './account.svelte';
import { appointmentDefaultValue, appointmentDeserializeWithContext } from './appointment.svelte';
import { companyDefaultValue, companyDeserializeWithContext } from './company.svelte';
import { employeeDefaultValue, employeeDeserializeWithContext } from './employee.svelte';
import { leadDefaultValue, leadDeserializeWithContext } from './lead.svelte';
import { orderDefaultValue, orderDeserializeWithContext } from './order.svelte';
import { orderedDefaultValue, orderedDeserializeWithContext } from './ordered.svelte';
import { packageDefaultValue, packageDeserializeWithContext } from './package.svelte';
import { paymentDefaultValue, paymentDeserializeWithContext } from './payment.svelte';
import { productDefaultValue, productDeserializeWithContext } from './product.svelte';
import { promotionDefaultValue, promotionDeserializeWithContext } from './promotion.svelte';
import { representsDefaultValue, representsDeserializeWithContext } from './represents.svelte';
import { routeDefaultValue, routeDeserializeWithContext } from './route.svelte';
import { serviceDefaultValue, serviceDeserializeWithContext } from './service.svelte';
import { siteDefaultValue, siteDeserializeWithContext } from './site.svelte';
import { taxRateDefaultValue, taxRateDeserializeWithContext } from './tax-rate.svelte';
import { userDefaultValue, userDeserializeWithContext } from './user.svelte';

/** import macro {Gigaform} from "@playground/macro"; */

import type { Account } from './account.svelte';
import type { Appointment } from './appointment.svelte';
import type { Company } from './company.svelte';
import type { Employee } from './employee.svelte';
import type { Lead } from './lead.svelte';
import type { Order } from './order.svelte';
import type { Ordered } from './ordered.svelte';
import type { Package } from './package.svelte';
import type { Payment } from './payment.svelte';
import type { Product } from './product.svelte';
import type { Promotion } from './promotion.svelte';
import type { Represents } from './represents.svelte';
import type { Route } from './route.svelte';
import type { Service } from './service.svelte';
import type { Site } from './site.svelte';
import type { TaxRate } from './tax-rate.svelte';
import type { User } from './user.svelte';

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
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(targetSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function targetSerializeWithContext(value: Target, ctx: __mf_SerializeContext): unknown {
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
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: Target }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = targetDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: '_root',
                        message: 'Target.deserialize: root cannot be a forward reference'
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return { success: true, value: resultOrRef };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return { success: false, errors: e.errors };
        }
        const message = e instanceof Error ? e.message : String(e);
        return { success: false, errors: [{ field: '_root', message }] };
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function targetDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Target | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Target | __mf_PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Target.deserializeWithContext: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new __mf_DeserializeError([
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
    throw new __mf_DeserializeError([
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

/** Per-variant error types */ export type TargetAccountErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type TargetUserErrors = { _errors: __gf_Option<Array<string>> };
export type TargetEmployeeErrors = { _errors: __gf_Option<Array<string>> };
export type TargetAppointmentErrors = { _errors: __gf_Option<Array<string>> };
export type TargetLeadErrors = { _errors: __gf_Option<Array<string>> };
export type TargetTaxRateErrors = { _errors: __gf_Option<Array<string>> };
export type TargetSiteErrors = { _errors: __gf_Option<Array<string>> };
export type TargetRouteErrors = { _errors: __gf_Option<Array<string>> };
export type TargetCompanyErrors = { _errors: __gf_Option<Array<string>> };
export type TargetProductErrors = { _errors: __gf_Option<Array<string>> };
export type TargetServiceErrors = { _errors: __gf_Option<Array<string>> };
export type TargetOrderErrors = { _errors: __gf_Option<Array<string>> };
export type TargetPaymentErrors = { _errors: __gf_Option<Array<string>> };
export type TargetPackageErrors = { _errors: __gf_Option<Array<string>> };
export type TargetPromotionErrors = { _errors: __gf_Option<Array<string>> };
export type TargetRepresentsErrors = { _errors: __gf_Option<Array<string>> };
export type TargetOrderedErrors = {
    _errors: __gf_Option<Array<string>>;
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
export type TargetAccountFieldControllers = {};
export type TargetUserFieldControllers = {};
export type TargetEmployeeFieldControllers = {};
export type TargetAppointmentFieldControllers = {};
export type TargetLeadFieldControllers = {};
export type TargetTaxRateFieldControllers = {};
export type TargetSiteFieldControllers = {};
export type TargetRouteFieldControllers = {};
export type TargetCompanyFieldControllers = {};
export type TargetProductFieldControllers = {};
export type TargetServiceFieldControllers = {};
export type TargetOrderFieldControllers = {};
export type TargetPaymentFieldControllers = {};
export type TargetPackageFieldControllers = {};
export type TargetPromotionFieldControllers = {};
export type TargetRepresentsFieldControllers = {};
export type TargetOrderedFieldControllers =
    {}; /** Union Gigaform interface with variant switching */
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
    validate(): Exit<Target, Array<{ field: string; message: string }>>;
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
        Account: { fields: {} as TargetAccountFieldControllers },
        User: { fields: {} as TargetUserFieldControllers },
        Employee: { fields: {} as TargetEmployeeFieldControllers },
        Appointment: { fields: {} as TargetAppointmentFieldControllers },
        Lead: { fields: {} as TargetLeadFieldControllers },
        TaxRate: { fields: {} as TargetTaxRateFieldControllers },
        Site: { fields: {} as TargetSiteFieldControllers },
        Route: { fields: {} as TargetRouteFieldControllers },
        Company: { fields: {} as TargetCompanyFieldControllers },
        Product: { fields: {} as TargetProductFieldControllers },
        Service: { fields: {} as TargetServiceFieldControllers },
        Order: { fields: {} as TargetOrderFieldControllers },
        Payment: { fields: {} as TargetPaymentFieldControllers },
        Package: { fields: {} as TargetPackageFieldControllers },
        Promotion: { fields: {} as TargetPromotionFieldControllers },
        Represents: { fields: {} as TargetRepresentsFieldControllers },
        Ordered: { fields: {} as TargetOrderedFieldControllers }
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
    function validate(): Exit<Target, Array<{ field: string; message: string }>> {
        return toExit(targetDeserialize(data));
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
): Exit<Target, Array<{ field: string; message: string }>> {
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
        return toExit({
            success: false,
            errors: [{ field: '_type', message: 'Missing discriminant field' }]
        });
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
    return toExit(targetDeserialize(obj));
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
