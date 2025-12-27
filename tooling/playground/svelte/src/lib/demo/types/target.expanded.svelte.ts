import { accountDefaultValue } from "./account.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { accountDeserializeWithContext } from "./account.svelte";
import { appointmentDeserializeWithContext } from "./appointment.svelte";
import { companyDeserializeWithContext } from "./company.svelte";
import { employeeDeserializeWithContext } from "./employee.svelte";
import { leadDeserializeWithContext } from "./lead.svelte";
import { orderDeserializeWithContext } from "./order.svelte";
import { orderedDeserializeWithContext } from "./ordered.svelte";
import { packageDeserializeWithContext } from "./package.svelte";
import { paymentDeserializeWithContext } from "./payment.svelte";
import { productDeserializeWithContext } from "./product.svelte";
import { promotionDeserializeWithContext } from "./promotion.svelte";
import { representsDeserializeWithContext } from "./represents.svelte";
import { routeDeserializeWithContext } from "./route.svelte";
import { serviceDeserializeWithContext } from "./service.svelte";
import { siteDeserializeWithContext } from "./site.svelte";
import { taxRateDeserializeWithContext } from "./tax-rate.svelte";
import { userDeserializeWithContext } from "./user.svelte";
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

export function targetDefaultValue#0#0(): Target {
    return accountDefaultValue();
}

export function targetSerialize#0#0(value: Target): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(targetSerializeWithContext(value, ctx));
}
export function targetSerializeWithContext#0#0(value: Target, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function targetDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Target } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = targetDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Target.deserialize: root cannot be a forward reference"
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
export function targetDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Target | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Target | __mf_PendingRef;
    }
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if (typeof __typeName === "string") {}
        if (__typeName === `${"Account"}`) {
            return accountDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"User"}`) {
            return userDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Employee"}`) {
            return employeeDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Appointment"}`) {
            return appointmentDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Lead"}`) {
            return leadDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"TaxRate"}`) {
            return taxRateDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Site"}`) {
            return siteDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Route"}`) {
            return routeDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Company"}`) {
            return companyDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Product"}`) {
            return productDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Service"}`) {
            return serviceDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Order"}`) {
            return orderDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Payment"}`) {
            return paymentDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Package"}`) {
            return packageDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Promotion"}`) {
            return promotionDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Represents"}`) {
            return representsDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Ordered"}`) {
            return orderedDeserializeWithContext(value, ctx) as Target;
        }
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"Target"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function targetIs(value: unknown): value is Target {
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if ('__typeName === "Account" || __typeName === "User" || __typeName === "Employee" || __typeName === "Appointment" || __typeName === "Lead" || __typeName === "TaxRate" || __typeName === "Site" || __typeName === "Route" || __typeName === "Company" || __typeName === "Product" || __typeName === "Service" || __typeName === "Order" || __typeName === "Payment" || __typeName === "Package" || __typeName === "Promotion" || __typeName === "Represents" || __typeName === "Ordered"') return true;
    }
    return false;
}
     }

export const Target = {
  deserialize: targetDeserialize,
  deserializeWithContext: targetDeserializeWithContext,
  is: targetIs
} as const;