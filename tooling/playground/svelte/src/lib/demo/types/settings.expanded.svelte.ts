import { overviewSettingsDefaultValue } from "./overview-settings.svelte";
import { scheduleSettingsDefaultValue } from "./schedule-settings.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { appointmentNotificationsSerializeWithContext } from "./appointment-notifications.svelte";
import { commissionsSerializeWithContext } from "./commissions.svelte";
import { overviewSettingsSerializeWithContext } from "./overview-settings.svelte";
import { pageSerializeWithContext } from "./page.svelte";
import { scheduleSettingsSerializeWithContext } from "./schedule-settings.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { appointmentNotificationsDeserializeWithContext } from "./appointment-notifications.svelte";
import { commissionsDeserializeWithContext } from "./commissions.svelte";
import { overviewSettingsDeserializeWithContext } from "./overview-settings.svelte";
import { pageDeserializeWithContext } from "./page.svelte";
import { scheduleSettingsDeserializeWithContext } from "./schedule-settings.svelte";
/** import macro {Gigaform} from "@playground/macro"; */

import type { AppointmentNotifications } from './appointment-notifications.svelte';
import type { Commissions } from './commissions.svelte';
import type { OverviewSettings } from './overview-settings.svelte';
import type { Page } from './page.svelte';
import type { ScheduleSettings } from './schedule-settings.svelte';


export interface Settings {
    appointmentNotifications: AppointmentNotifications | null;
    commissions: Commissions | null;
    scheduleSettings: ScheduleSettings;
    accountOverviewSettings: OverviewSettings;
    serviceOverviewSettings: OverviewSettings;
    appointmentOverviewSettings: OverviewSettings;
    leadOverviewSettings: OverviewSettings;
    packageOverviewSettings: OverviewSettings;
    productOverviewSettings: OverviewSettings;
    orderOverviewSettings: OverviewSettings;
    taxRateOverviewSettings: OverviewSettings;
    
    homePage: Page;
}

export function settingsDefaultValue(): Settings {
    return {
        appointmentNotifications: null,
        commissions: null,
        scheduleSettings: scheduleSettingsDefaultValue(),
        accountOverviewSettings: overviewSettingsDefaultValue(),
        serviceOverviewSettings: overviewSettingsDefaultValue(),
        appointmentOverviewSettings: overviewSettingsDefaultValue(),
        leadOverviewSettings: overviewSettingsDefaultValue(),
        packageOverviewSettings: overviewSettingsDefaultValue(),
        productOverviewSettings: overviewSettingsDefaultValue(),
        orderOverviewSettings: overviewSettingsDefaultValue(),
        taxRateOverviewSettings: overviewSettingsDefaultValue(),
        homePage: "UserHome"
    } as Settings;
}

export function settingsSerialize(value: Settings): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(settingsSerializeWithContext(value, ctx));
}
export function settingsSerializeWithContext(value: Settings, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Settings"}`,
        __id
    };
    if (value.appointmentNotifications !== null) {
        result[`${"appointmentNotifications"}`] = appointmentNotificationsSerializeWithContext(value.appointmentNotifications, ctx);
    }
    if (value.commissions !== null) {
        result[`${"commissions"}`] = commissionsSerializeWithContext(value.commissions, ctx);
    }
    result[`${"scheduleSettings"}`] = scheduleSettingsSerializeWithContext(value.scheduleSettings, ctx);
    result[`${"accountOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.accountOverviewSettings, ctx);
    result[`${"serviceOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.serviceOverviewSettings, ctx);
    result[`${"appointmentOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.appointmentOverviewSettings, ctx);
    result[`${"leadOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.leadOverviewSettings, ctx);
    result[`${"packageOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.packageOverviewSettings, ctx);
    result[`${"productOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.productOverviewSettings, ctx);
    result[`${"orderOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.orderOverviewSettings, ctx);
    result[`${"taxRateOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.taxRateOverviewSettings, ctx);
    result[`${"homePage"}`] = pageSerializeWithContext(value.homePage, ctx);
    return result;
}

export function settingsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Settings } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = settingsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Settings.deserialize: root cannot be a forward reference"
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
export function settingsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Settings | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Settings"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"appointmentNotifications"}` in obj)) {
        errors.push({
            field: `${"appointmentNotifications"}`,
            message: "missing required field"
        });
    }
    if (!(`${"commissions"}` in obj)) {
        errors.push({
            field: `${"commissions"}`,
            message: "missing required field"
        });
    }
    if (!(`${"scheduleSettings"}` in obj)) {
        errors.push({
            field: `${"scheduleSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"accountOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"accountOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"serviceOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"serviceOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"appointmentOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"appointmentOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"leadOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"leadOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"packageOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"packageOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"productOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"productOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"orderOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"orderOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"taxRateOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"taxRateOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"homePage"}` in obj)) {
        errors.push({
            field: `${"homePage"}`,
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_appointmentNotifications = obj[`${"appointmentNotifications"}`] as AppointmentNotifications | null;
        if (__raw_appointmentNotifications === null) {
            instance.appointmentNotifications = null;
        } else {
            const __result = appointmentNotificationsDeserializeWithContext(__raw_appointmentNotifications, ctx);
            ctx.assignOrDefer(instance, `${"appointmentNotifications"}`, __result);
        }
    }
    {
        const __raw_commissions = obj[`${"commissions"}`] as Commissions | null;
        if (__raw_commissions === null) {
            instance.commissions = null;
        } else {
            const __result = commissionsDeserializeWithContext(__raw_commissions, ctx);
            ctx.assignOrDefer(instance, `${"commissions"}`, __result);
        }
    }
    {
        const __raw_scheduleSettings = obj[`${"scheduleSettings"}`] as ScheduleSettings;
        {
            const __result = scheduleSettingsDeserializeWithContext(__raw_scheduleSettings, ctx);
            ctx.assignOrDefer(instance, `${"scheduleSettings"}`, __result);
        }
    }
    {
        const __raw_accountOverviewSettings = obj[`${"accountOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_accountOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"accountOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_serviceOverviewSettings = obj[`${"serviceOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_serviceOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"serviceOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_appointmentOverviewSettings = obj[`${"appointmentOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_appointmentOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"appointmentOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_leadOverviewSettings = obj[`${"leadOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_leadOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"leadOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_packageOverviewSettings = obj[`${"packageOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_packageOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"packageOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_productOverviewSettings = obj[`${"productOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_productOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"productOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_orderOverviewSettings = obj[`${"orderOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_orderOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"orderOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_taxRateOverviewSettings = obj[`${"taxRateOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_taxRateOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"taxRateOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_homePage = obj[`${"homePage"}`] as Page;
        {
            const __result = pageDeserializeWithContext(__raw_homePage, ctx);
            ctx.assignOrDefer(instance, `${"homePage"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Settings;
}
export function settingsValidateField<K extends keyof Settings>(_field: K, _value: Settings[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function settingsValidateFields(_partial: Partial<Settings>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function settingsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"appointmentNotifications" in o && "commissions" in o && "scheduleSettings" in o && "accountOverviewSettings" in o && "serviceOverviewSettings" in o && "appointmentOverviewSettings" in o && "leadOverviewSettings" in o && "packageOverviewSettings" in o && "productOverviewSettings" in o && "orderOverviewSettings" in o && "taxRateOverviewSettings" in o && "homePage" in o';
}
export function settingsIs(obj: unknown): obj is Settings {
    if (!settingsHasShape(obj)) {
        return false;
    }
    const result = settingsDeserialize(obj);
    return result.success;
}

export const Settings = {
  defaultValue: settingsDefaultValue,
  serialize: settingsSerialize,
  serializeWithContext: settingsSerializeWithContext,
  deserialize: settingsDeserialize,
  deserializeWithContext: settingsDeserializeWithContext,
  validateFields: settingsValidateFields,
  hasShape: settingsHasShape,
  is: settingsIs
} as const;