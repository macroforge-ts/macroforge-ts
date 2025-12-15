import { defaultValueOverviewSettings } from './overview-settings.svelte';
import { defaultValueScheduleSettings } from './schedule-settings.svelte';
import { SerializeContext } from 'macroforge/serde';
import { __serializeAppointmentNotifications } from './appointment-notifications.svelte';
import { __serializeCommissions } from './commissions.svelte';
import { __serializeOverviewSettings } from './overview-settings.svelte';
import { __serializePage } from './page.svelte';
import { __serializeScheduleSettings } from './schedule-settings.svelte';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { __deserializeAppointmentNotifications } from './appointment-notifications.svelte';
import { __deserializeCommissions } from './commissions.svelte';
import { __deserializeOverviewSettings } from './overview-settings.svelte';
import { __deserializePage } from './page.svelte';
import { __deserializeScheduleSettings } from './schedule-settings.svelte';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { AppointmentNotifications } from './appointment-notifications.svelte';
import type { ScheduleSettings } from './schedule-settings.svelte';
import type { OverviewSettings } from './overview-settings.svelte';
import type { Commissions } from './commissions.svelte';
import type { Page } from './page.svelte';

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

export function defaultValueSettings(): Settings {
    return {
        appointmentNotifications: null,
        commissions: null,
        scheduleSettings: defaultValueScheduleSettings(),
        accountOverviewSettings: defaultValueOverviewSettings(),
        serviceOverviewSettings: defaultValueOverviewSettings(),
        appointmentOverviewSettings: defaultValueOverviewSettings(),
        leadOverviewSettings: defaultValueOverviewSettings(),
        packageOverviewSettings: defaultValueOverviewSettings(),
        productOverviewSettings: defaultValueOverviewSettings(),
        orderOverviewSettings: defaultValueOverviewSettings(),
        taxRateOverviewSettings: defaultValueOverviewSettings(),
        homePage: 'UserHome'
    } as Settings;
}

export function toStringifiedJSONSettings(value: Settings): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeSettings(value, ctx));
}
export function toObjectSettings(value: Settings): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeSettings(value, ctx);
}
export function __serializeSettings(
    value: Settings,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Settings', __id };
    if (value.appointmentNotifications !== null) {
        result['appointmentNotifications'] = __serializeAppointmentNotifications(
            value.appointmentNotifications,
            ctx
        );
    } else {
        result['appointmentNotifications'] = null;
    }
    if (value.commissions !== null) {
        result['commissions'] = __serializeCommissions(value.commissions, ctx);
    } else {
        result['commissions'] = null;
    }
    result['scheduleSettings'] = __serializeScheduleSettings(value.scheduleSettings, ctx);
    result['accountOverviewSettings'] = __serializeOverviewSettings(
        value.accountOverviewSettings,
        ctx
    );
    result['serviceOverviewSettings'] = __serializeOverviewSettings(
        value.serviceOverviewSettings,
        ctx
    );
    result['appointmentOverviewSettings'] = __serializeOverviewSettings(
        value.appointmentOverviewSettings,
        ctx
    );
    result['leadOverviewSettings'] = __serializeOverviewSettings(value.leadOverviewSettings, ctx);
    result['packageOverviewSettings'] = __serializeOverviewSettings(
        value.packageOverviewSettings,
        ctx
    );
    result['productOverviewSettings'] = __serializeOverviewSettings(
        value.productOverviewSettings,
        ctx
    );
    result['orderOverviewSettings'] = __serializeOverviewSettings(value.orderOverviewSettings, ctx);
    result['taxRateOverviewSettings'] = __serializeOverviewSettings(
        value.taxRateOverviewSettings,
        ctx
    );
    result['homePage'] = __serializePage(value.homePage, ctx);
    return result;
}

export function fromStringifiedJSONSettings(
    json: string,
    opts?: DeserializeOptions
): Result<Settings, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectSettings(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectSettings(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Settings, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeSettings(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Settings.fromObject: root cannot be a forward reference'
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
export function __deserializeSettings(value: any, ctx: DeserializeContext): Settings | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Settings.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('appointmentNotifications' in obj)) {
        errors.push({ field: 'appointmentNotifications', message: 'missing required field' });
    }
    if (!('commissions' in obj)) {
        errors.push({ field: 'commissions', message: 'missing required field' });
    }
    if (!('scheduleSettings' in obj)) {
        errors.push({ field: 'scheduleSettings', message: 'missing required field' });
    }
    if (!('accountOverviewSettings' in obj)) {
        errors.push({ field: 'accountOverviewSettings', message: 'missing required field' });
    }
    if (!('serviceOverviewSettings' in obj)) {
        errors.push({ field: 'serviceOverviewSettings', message: 'missing required field' });
    }
    if (!('appointmentOverviewSettings' in obj)) {
        errors.push({ field: 'appointmentOverviewSettings', message: 'missing required field' });
    }
    if (!('leadOverviewSettings' in obj)) {
        errors.push({ field: 'leadOverviewSettings', message: 'missing required field' });
    }
    if (!('packageOverviewSettings' in obj)) {
        errors.push({ field: 'packageOverviewSettings', message: 'missing required field' });
    }
    if (!('productOverviewSettings' in obj)) {
        errors.push({ field: 'productOverviewSettings', message: 'missing required field' });
    }
    if (!('orderOverviewSettings' in obj)) {
        errors.push({ field: 'orderOverviewSettings', message: 'missing required field' });
    }
    if (!('taxRateOverviewSettings' in obj)) {
        errors.push({ field: 'taxRateOverviewSettings', message: 'missing required field' });
    }
    if (!('homePage' in obj)) {
        errors.push({ field: 'homePage', message: 'missing required field' });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_appointmentNotifications = obj[
            'appointmentNotifications'
        ] as AppointmentNotifications | null;
        if (__raw_appointmentNotifications === null) {
            instance.appointmentNotifications = null;
        } else {
            const __result = __deserializeAppointmentNotifications(
                __raw_appointmentNotifications,
                ctx
            );
            ctx.assignOrDefer(instance, 'appointmentNotifications', __result);
        }
    }
    {
        const __raw_commissions = obj['commissions'] as Commissions | null;
        if (__raw_commissions === null) {
            instance.commissions = null;
        } else {
            const __result = __deserializeCommissions(__raw_commissions, ctx);
            ctx.assignOrDefer(instance, 'commissions', __result);
        }
    }
    {
        const __raw_scheduleSettings = obj['scheduleSettings'] as ScheduleSettings;
        {
            const __result = __deserializeScheduleSettings(__raw_scheduleSettings, ctx);
            ctx.assignOrDefer(instance, 'scheduleSettings', __result);
        }
    }
    {
        const __raw_accountOverviewSettings = obj['accountOverviewSettings'] as OverviewSettings;
        {
            const __result = __deserializeOverviewSettings(__raw_accountOverviewSettings, ctx);
            ctx.assignOrDefer(instance, 'accountOverviewSettings', __result);
        }
    }
    {
        const __raw_serviceOverviewSettings = obj['serviceOverviewSettings'] as OverviewSettings;
        {
            const __result = __deserializeOverviewSettings(__raw_serviceOverviewSettings, ctx);
            ctx.assignOrDefer(instance, 'serviceOverviewSettings', __result);
        }
    }
    {
        const __raw_appointmentOverviewSettings = obj[
            'appointmentOverviewSettings'
        ] as OverviewSettings;
        {
            const __result = __deserializeOverviewSettings(__raw_appointmentOverviewSettings, ctx);
            ctx.assignOrDefer(instance, 'appointmentOverviewSettings', __result);
        }
    }
    {
        const __raw_leadOverviewSettings = obj['leadOverviewSettings'] as OverviewSettings;
        {
            const __result = __deserializeOverviewSettings(__raw_leadOverviewSettings, ctx);
            ctx.assignOrDefer(instance, 'leadOverviewSettings', __result);
        }
    }
    {
        const __raw_packageOverviewSettings = obj['packageOverviewSettings'] as OverviewSettings;
        {
            const __result = __deserializeOverviewSettings(__raw_packageOverviewSettings, ctx);
            ctx.assignOrDefer(instance, 'packageOverviewSettings', __result);
        }
    }
    {
        const __raw_productOverviewSettings = obj['productOverviewSettings'] as OverviewSettings;
        {
            const __result = __deserializeOverviewSettings(__raw_productOverviewSettings, ctx);
            ctx.assignOrDefer(instance, 'productOverviewSettings', __result);
        }
    }
    {
        const __raw_orderOverviewSettings = obj['orderOverviewSettings'] as OverviewSettings;
        {
            const __result = __deserializeOverviewSettings(__raw_orderOverviewSettings, ctx);
            ctx.assignOrDefer(instance, 'orderOverviewSettings', __result);
        }
    }
    {
        const __raw_taxRateOverviewSettings = obj['taxRateOverviewSettings'] as OverviewSettings;
        {
            const __result = __deserializeOverviewSettings(__raw_taxRateOverviewSettings, ctx);
            ctx.assignOrDefer(instance, 'taxRateOverviewSettings', __result);
        }
    }
    {
        const __raw_homePage = obj['homePage'] as Page;
        {
            const __result = __deserializePage(__raw_homePage, ctx);
            ctx.assignOrDefer(instance, 'homePage', __result);
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Settings;
}
export function validateFieldSettings<K extends keyof Settings>(
    field: K,
    value: Settings[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsSettings(
    partial: Partial<Settings>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeSettings(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'appointmentNotifications' in o &&
        'commissions' in o &&
        'scheduleSettings' in o &&
        'accountOverviewSettings' in o &&
        'serviceOverviewSettings' in o &&
        'appointmentOverviewSettings' in o &&
        'leadOverviewSettings' in o &&
        'packageOverviewSettings' in o &&
        'productOverviewSettings' in o &&
        'orderOverviewSettings' in o &&
        'taxRateOverviewSettings' in o &&
        'homePage' in o
    );
}
export function isSettings(obj: unknown): obj is Settings {
    if (!hasShapeSettings(obj)) {
        return false;
    }
    const result = fromObjectSettings(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsSettings = {
    _errors: Option<Array<string>>;
    appointmentNotifications: Option<Array<string>>;
    commissions: Option<Array<string>>;
    scheduleSettings: Option<Array<string>>;
    accountOverviewSettings: Option<Array<string>>;
    serviceOverviewSettings: Option<Array<string>>;
    appointmentOverviewSettings: Option<Array<string>>;
    leadOverviewSettings: Option<Array<string>>;
    packageOverviewSettings: Option<Array<string>>;
    productOverviewSettings: Option<Array<string>>;
    orderOverviewSettings: Option<Array<string>>;
    taxRateOverviewSettings: Option<Array<string>>;
    homePage: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedSettings = {
    appointmentNotifications: Option<boolean>;
    commissions: Option<boolean>;
    scheduleSettings: Option<boolean>;
    accountOverviewSettings: Option<boolean>;
    serviceOverviewSettings: Option<boolean>;
    appointmentOverviewSettings: Option<boolean>;
    leadOverviewSettings: Option<boolean>;
    packageOverviewSettings: Option<boolean>;
    productOverviewSettings: Option<boolean>;
    orderOverviewSettings: Option<boolean>;
    taxRateOverviewSettings: Option<boolean>;
    homePage: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersSettings {
    readonly appointmentNotifications: FieldController<AppointmentNotifications | null>;
    readonly commissions: FieldController<Commissions | null>;
    readonly scheduleSettings: FieldController<ScheduleSettings>;
    readonly accountOverviewSettings: FieldController<OverviewSettings>;
    readonly serviceOverviewSettings: FieldController<OverviewSettings>;
    readonly appointmentOverviewSettings: FieldController<OverviewSettings>;
    readonly leadOverviewSettings: FieldController<OverviewSettings>;
    readonly packageOverviewSettings: FieldController<OverviewSettings>;
    readonly productOverviewSettings: FieldController<OverviewSettings>;
    readonly orderOverviewSettings: FieldController<OverviewSettings>;
    readonly taxRateOverviewSettings: FieldController<OverviewSettings>;
    readonly homePage: FieldController<Page>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformSettings {
    readonly data: Settings;
    readonly errors: ErrorsSettings;
    readonly tainted: TaintedSettings;
    readonly fields: FieldControllersSettings;
    validate(): Result<Settings, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Settings>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormSettings(overrides?: Partial<Settings>): GigaformSettings {
    let data = $state({ ...defaultValueSettings(), ...overrides });
    let errors = $state<ErrorsSettings>({
        _errors: Option.none(),
        appointmentNotifications: Option.none(),
        commissions: Option.none(),
        scheduleSettings: Option.none(),
        accountOverviewSettings: Option.none(),
        serviceOverviewSettings: Option.none(),
        appointmentOverviewSettings: Option.none(),
        leadOverviewSettings: Option.none(),
        packageOverviewSettings: Option.none(),
        productOverviewSettings: Option.none(),
        orderOverviewSettings: Option.none(),
        taxRateOverviewSettings: Option.none(),
        homePage: Option.none()
    });
    let tainted = $state<TaintedSettings>({
        appointmentNotifications: Option.none(),
        commissions: Option.none(),
        scheduleSettings: Option.none(),
        accountOverviewSettings: Option.none(),
        serviceOverviewSettings: Option.none(),
        appointmentOverviewSettings: Option.none(),
        leadOverviewSettings: Option.none(),
        packageOverviewSettings: Option.none(),
        productOverviewSettings: Option.none(),
        orderOverviewSettings: Option.none(),
        taxRateOverviewSettings: Option.none(),
        homePage: Option.none()
    });
    const fields: FieldControllersSettings = {
        appointmentNotifications: {
            path: ['appointmentNotifications'] as const,
            name: 'appointmentNotifications',
            constraints: { required: true },

            get: () => data.appointmentNotifications,
            set: (value: AppointmentNotifications | null) => {
                data.appointmentNotifications = value;
            },
            transform: (value: AppointmentNotifications | null): AppointmentNotifications | null =>
                value,
            getError: () => errors.appointmentNotifications,
            setError: (value: Option<Array<string>>) => {
                errors.appointmentNotifications = value;
            },
            getTainted: () => tainted.appointmentNotifications,
            setTainted: (value: Option<boolean>) => {
                tainted.appointmentNotifications = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldSettings(
                    'appointmentNotifications',
                    data.appointmentNotifications
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        commissions: {
            path: ['commissions'] as const,
            name: 'commissions',
            constraints: { required: true },

            get: () => data.commissions,
            set: (value: Commissions | null) => {
                data.commissions = value;
            },
            transform: (value: Commissions | null): Commissions | null => value,
            getError: () => errors.commissions,
            setError: (value: Option<Array<string>>) => {
                errors.commissions = value;
            },
            getTainted: () => tainted.commissions,
            setTainted: (value: Option<boolean>) => {
                tainted.commissions = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldSettings('commissions', data.commissions);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        scheduleSettings: {
            path: ['scheduleSettings'] as const,
            name: 'scheduleSettings',
            constraints: { required: true },

            get: () => data.scheduleSettings,
            set: (value: ScheduleSettings) => {
                data.scheduleSettings = value;
            },
            transform: (value: ScheduleSettings): ScheduleSettings => value,
            getError: () => errors.scheduleSettings,
            setError: (value: Option<Array<string>>) => {
                errors.scheduleSettings = value;
            },
            getTainted: () => tainted.scheduleSettings,
            setTainted: (value: Option<boolean>) => {
                tainted.scheduleSettings = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldSettings(
                    'scheduleSettings',
                    data.scheduleSettings
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        accountOverviewSettings: {
            path: ['accountOverviewSettings'] as const,
            name: 'accountOverviewSettings',
            constraints: { required: true },

            get: () => data.accountOverviewSettings,
            set: (value: OverviewSettings) => {
                data.accountOverviewSettings = value;
            },
            transform: (value: OverviewSettings): OverviewSettings => value,
            getError: () => errors.accountOverviewSettings,
            setError: (value: Option<Array<string>>) => {
                errors.accountOverviewSettings = value;
            },
            getTainted: () => tainted.accountOverviewSettings,
            setTainted: (value: Option<boolean>) => {
                tainted.accountOverviewSettings = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldSettings(
                    'accountOverviewSettings',
                    data.accountOverviewSettings
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        serviceOverviewSettings: {
            path: ['serviceOverviewSettings'] as const,
            name: 'serviceOverviewSettings',
            constraints: { required: true },

            get: () => data.serviceOverviewSettings,
            set: (value: OverviewSettings) => {
                data.serviceOverviewSettings = value;
            },
            transform: (value: OverviewSettings): OverviewSettings => value,
            getError: () => errors.serviceOverviewSettings,
            setError: (value: Option<Array<string>>) => {
                errors.serviceOverviewSettings = value;
            },
            getTainted: () => tainted.serviceOverviewSettings,
            setTainted: (value: Option<boolean>) => {
                tainted.serviceOverviewSettings = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldSettings(
                    'serviceOverviewSettings',
                    data.serviceOverviewSettings
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        appointmentOverviewSettings: {
            path: ['appointmentOverviewSettings'] as const,
            name: 'appointmentOverviewSettings',
            constraints: { required: true },

            get: () => data.appointmentOverviewSettings,
            set: (value: OverviewSettings) => {
                data.appointmentOverviewSettings = value;
            },
            transform: (value: OverviewSettings): OverviewSettings => value,
            getError: () => errors.appointmentOverviewSettings,
            setError: (value: Option<Array<string>>) => {
                errors.appointmentOverviewSettings = value;
            },
            getTainted: () => tainted.appointmentOverviewSettings,
            setTainted: (value: Option<boolean>) => {
                tainted.appointmentOverviewSettings = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldSettings(
                    'appointmentOverviewSettings',
                    data.appointmentOverviewSettings
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        leadOverviewSettings: {
            path: ['leadOverviewSettings'] as const,
            name: 'leadOverviewSettings',
            constraints: { required: true },

            get: () => data.leadOverviewSettings,
            set: (value: OverviewSettings) => {
                data.leadOverviewSettings = value;
            },
            transform: (value: OverviewSettings): OverviewSettings => value,
            getError: () => errors.leadOverviewSettings,
            setError: (value: Option<Array<string>>) => {
                errors.leadOverviewSettings = value;
            },
            getTainted: () => tainted.leadOverviewSettings,
            setTainted: (value: Option<boolean>) => {
                tainted.leadOverviewSettings = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldSettings(
                    'leadOverviewSettings',
                    data.leadOverviewSettings
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        packageOverviewSettings: {
            path: ['packageOverviewSettings'] as const,
            name: 'packageOverviewSettings',
            constraints: { required: true },

            get: () => data.packageOverviewSettings,
            set: (value: OverviewSettings) => {
                data.packageOverviewSettings = value;
            },
            transform: (value: OverviewSettings): OverviewSettings => value,
            getError: () => errors.packageOverviewSettings,
            setError: (value: Option<Array<string>>) => {
                errors.packageOverviewSettings = value;
            },
            getTainted: () => tainted.packageOverviewSettings,
            setTainted: (value: Option<boolean>) => {
                tainted.packageOverviewSettings = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldSettings(
                    'packageOverviewSettings',
                    data.packageOverviewSettings
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        productOverviewSettings: {
            path: ['productOverviewSettings'] as const,
            name: 'productOverviewSettings',
            constraints: { required: true },

            get: () => data.productOverviewSettings,
            set: (value: OverviewSettings) => {
                data.productOverviewSettings = value;
            },
            transform: (value: OverviewSettings): OverviewSettings => value,
            getError: () => errors.productOverviewSettings,
            setError: (value: Option<Array<string>>) => {
                errors.productOverviewSettings = value;
            },
            getTainted: () => tainted.productOverviewSettings,
            setTainted: (value: Option<boolean>) => {
                tainted.productOverviewSettings = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldSettings(
                    'productOverviewSettings',
                    data.productOverviewSettings
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        orderOverviewSettings: {
            path: ['orderOverviewSettings'] as const,
            name: 'orderOverviewSettings',
            constraints: { required: true },

            get: () => data.orderOverviewSettings,
            set: (value: OverviewSettings) => {
                data.orderOverviewSettings = value;
            },
            transform: (value: OverviewSettings): OverviewSettings => value,
            getError: () => errors.orderOverviewSettings,
            setError: (value: Option<Array<string>>) => {
                errors.orderOverviewSettings = value;
            },
            getTainted: () => tainted.orderOverviewSettings,
            setTainted: (value: Option<boolean>) => {
                tainted.orderOverviewSettings = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldSettings(
                    'orderOverviewSettings',
                    data.orderOverviewSettings
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        taxRateOverviewSettings: {
            path: ['taxRateOverviewSettings'] as const,
            name: 'taxRateOverviewSettings',
            constraints: { required: true },

            get: () => data.taxRateOverviewSettings,
            set: (value: OverviewSettings) => {
                data.taxRateOverviewSettings = value;
            },
            transform: (value: OverviewSettings): OverviewSettings => value,
            getError: () => errors.taxRateOverviewSettings,
            setError: (value: Option<Array<string>>) => {
                errors.taxRateOverviewSettings = value;
            },
            getTainted: () => tainted.taxRateOverviewSettings,
            setTainted: (value: Option<boolean>) => {
                tainted.taxRateOverviewSettings = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldSettings(
                    'taxRateOverviewSettings',
                    data.taxRateOverviewSettings
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        homePage: {
            path: ['homePage'] as const,
            name: 'homePage',
            constraints: { required: true },

            get: () => data.homePage,
            set: (value: Page) => {
                data.homePage = value;
            },
            transform: (value: Page): Page => value,
            getError: () => errors.homePage,
            setError: (value: Option<Array<string>>) => {
                errors.homePage = value;
            },
            getTainted: () => tainted.homePage,
            setTainted: (value: Option<boolean>) => {
                tainted.homePage = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldSettings('homePage', data.homePage);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Settings, Array<{ field: string; message: string }>> {
        return fromObjectSettings(data);
    }
    function reset(newOverrides?: Partial<Settings>): void {
        data = { ...defaultValueSettings(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            appointmentNotifications: Option.none(),
            commissions: Option.none(),
            scheduleSettings: Option.none(),
            accountOverviewSettings: Option.none(),
            serviceOverviewSettings: Option.none(),
            appointmentOverviewSettings: Option.none(),
            leadOverviewSettings: Option.none(),
            packageOverviewSettings: Option.none(),
            productOverviewSettings: Option.none(),
            orderOverviewSettings: Option.none(),
            taxRateOverviewSettings: Option.none(),
            homePage: Option.none()
        };
        tainted = {
            appointmentNotifications: Option.none(),
            commissions: Option.none(),
            scheduleSettings: Option.none(),
            accountOverviewSettings: Option.none(),
            serviceOverviewSettings: Option.none(),
            appointmentOverviewSettings: Option.none(),
            leadOverviewSettings: Option.none(),
            packageOverviewSettings: Option.none(),
            productOverviewSettings: Option.none(),
            orderOverviewSettings: Option.none(),
            taxRateOverviewSettings: Option.none(),
            homePage: Option.none()
        };
    }
    return {
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
        fields,
        validate,
        reset
    };
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function fromFormDataSettings(
    formData: FormData
): Result<Settings, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.appointmentNotifications = formData.get('appointmentNotifications') ?? '';
    obj.commissions = formData.get('commissions') ?? '';
    {
        // Collect nested object fields with prefix "scheduleSettings."
        const scheduleSettingsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('scheduleSettings.')) {
                const fieldName = key.slice('scheduleSettings.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = scheduleSettingsObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.scheduleSettings = scheduleSettingsObj;
    }
    {
        // Collect nested object fields with prefix "accountOverviewSettings."
        const accountOverviewSettingsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('accountOverviewSettings.')) {
                const fieldName = key.slice('accountOverviewSettings.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = accountOverviewSettingsObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.accountOverviewSettings = accountOverviewSettingsObj;
    }
    {
        // Collect nested object fields with prefix "serviceOverviewSettings."
        const serviceOverviewSettingsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('serviceOverviewSettings.')) {
                const fieldName = key.slice('serviceOverviewSettings.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = serviceOverviewSettingsObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.serviceOverviewSettings = serviceOverviewSettingsObj;
    }
    {
        // Collect nested object fields with prefix "appointmentOverviewSettings."
        const appointmentOverviewSettingsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('appointmentOverviewSettings.')) {
                const fieldName = key.slice('appointmentOverviewSettings.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = appointmentOverviewSettingsObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.appointmentOverviewSettings = appointmentOverviewSettingsObj;
    }
    {
        // Collect nested object fields with prefix "leadOverviewSettings."
        const leadOverviewSettingsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('leadOverviewSettings.')) {
                const fieldName = key.slice('leadOverviewSettings.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = leadOverviewSettingsObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.leadOverviewSettings = leadOverviewSettingsObj;
    }
    {
        // Collect nested object fields with prefix "packageOverviewSettings."
        const packageOverviewSettingsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('packageOverviewSettings.')) {
                const fieldName = key.slice('packageOverviewSettings.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = packageOverviewSettingsObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.packageOverviewSettings = packageOverviewSettingsObj;
    }
    {
        // Collect nested object fields with prefix "productOverviewSettings."
        const productOverviewSettingsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('productOverviewSettings.')) {
                const fieldName = key.slice('productOverviewSettings.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = productOverviewSettingsObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.productOverviewSettings = productOverviewSettingsObj;
    }
    {
        // Collect nested object fields with prefix "orderOverviewSettings."
        const orderOverviewSettingsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('orderOverviewSettings.')) {
                const fieldName = key.slice('orderOverviewSettings.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = orderOverviewSettingsObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.orderOverviewSettings = orderOverviewSettingsObj;
    }
    {
        // Collect nested object fields with prefix "taxRateOverviewSettings."
        const taxRateOverviewSettingsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('taxRateOverviewSettings.')) {
                const fieldName = key.slice('taxRateOverviewSettings.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = taxRateOverviewSettingsObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.taxRateOverviewSettings = taxRateOverviewSettingsObj;
    }
    {
        // Collect nested object fields with prefix "homePage."
        const homePageObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('homePage.')) {
                const fieldName = key.slice('homePage.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = homePageObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.homePage = homePageObj;
    }
    return fromStringifiedJSONSettings(JSON.stringify(obj));
}
