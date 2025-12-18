import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { Result } from '@playground/macro/gigaform';
import { Option } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';

export type Page =
    | /** @default */ 'SalesHomeDashboard'
    | 'SalesHomeProducts'
    | 'SalesHomeServices'
    | 'SalesHomePackages'
    | 'SalesHomeTaxRates'
    | 'SalesLeadsOverview'
    | 'SalesLeadsActivities'
    | 'SalesLeadsCampaigns'
    | 'SalesLeadsDripCampaigns'
    | 'SalesLeadsOpportunities'
    | 'SalesLeadsPromotions'
    | 'SalesAccountsOverview'
    | 'SalesAccountsActivities'
    | 'SalesAccountsBilling'
    | 'SalesAccountsContracts'
    | 'SalesOrdersOverview'
    | 'SalesOrdersActivities'
    | 'SalesOrdersPayments'
    | 'SalesOrdersCommissions'
    | 'SalesSchedulingSchedule'
    | 'SalesSchedulingAppointments'
    | 'SalesSchedulingRecurring'
    | 'SalesSchedulingRoutes'
    | 'SalesSchedulingReminders'
    | 'UserHome';

export function pageDefaultValue(): Page {
    return 'SalesHomeDashboard';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function pageSerialize(
    value: Page
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(pageSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function pageSerializeWithContext(value: Page, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function pageDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Page> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = pageDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                { field: '_root', message: 'Page.deserialize: root cannot be a forward reference' }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return __mf_exitSucceed(resultOrRef);
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return __mf_exitFail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return __mf_exitFail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function pageDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Page | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Page | __mf_PendingRef;
    }
    const allowedValues = [
        'SalesHomeDashboard',
        'SalesHomeProducts',
        'SalesHomeServices',
        'SalesHomePackages',
        'SalesHomeTaxRates',
        'SalesLeadsOverview',
        'SalesLeadsActivities',
        'SalesLeadsCampaigns',
        'SalesLeadsDripCampaigns',
        'SalesLeadsOpportunities',
        'SalesLeadsPromotions',
        'SalesAccountsOverview',
        'SalesAccountsActivities',
        'SalesAccountsBilling',
        'SalesAccountsContracts',
        'SalesOrdersOverview',
        'SalesOrdersActivities',
        'SalesOrdersPayments',
        'SalesOrdersCommissions',
        'SalesSchedulingSchedule',
        'SalesSchedulingAppointments',
        'SalesSchedulingRecurring',
        'SalesSchedulingRoutes',
        'SalesSchedulingReminders',
        'UserHome'
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for Page: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as Page;
}
export function pageIs(value: unknown): value is Page {
    const allowedValues = [
        'SalesHomeDashboard',
        'SalesHomeProducts',
        'SalesHomeServices',
        'SalesHomePackages',
        'SalesHomeTaxRates',
        'SalesLeadsOverview',
        'SalesLeadsActivities',
        'SalesLeadsCampaigns',
        'SalesLeadsDripCampaigns',
        'SalesLeadsOpportunities',
        'SalesLeadsPromotions',
        'SalesAccountsOverview',
        'SalesAccountsActivities',
        'SalesAccountsBilling',
        'SalesAccountsContracts',
        'SalesOrdersOverview',
        'SalesOrdersActivities',
        'SalesOrdersPayments',
        'SalesOrdersCommissions',
        'SalesSchedulingSchedule',
        'SalesSchedulingAppointments',
        'SalesSchedulingRecurring',
        'SalesSchedulingRoutes',
        'SalesSchedulingReminders',
        'UserHome'
    ] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type PageSalesHomeDashboardErrors = {
    _errors: Option<Array<string>>;
};
export type PageSalesHomeProductsErrors = { _errors: Option<Array<string>> };
export type PageSalesHomeServicesErrors = { _errors: Option<Array<string>> };
export type PageSalesHomePackagesErrors = { _errors: Option<Array<string>> };
export type PageSalesHomeTaxRatesErrors = { _errors: Option<Array<string>> };
export type PageSalesLeadsOverviewErrors = { _errors: Option<Array<string>> };
export type PageSalesLeadsActivitiesErrors = { _errors: Option<Array<string>> };
export type PageSalesLeadsCampaignsErrors = { _errors: Option<Array<string>> };
export type PageSalesLeadsDripCampaignsErrors = { _errors: Option<Array<string>> };
export type PageSalesLeadsOpportunitiesErrors = { _errors: Option<Array<string>> };
export type PageSalesLeadsPromotionsErrors = { _errors: Option<Array<string>> };
export type PageSalesAccountsOverviewErrors = { _errors: Option<Array<string>> };
export type PageSalesAccountsActivitiesErrors = { _errors: Option<Array<string>> };
export type PageSalesAccountsBillingErrors = { _errors: Option<Array<string>> };
export type PageSalesAccountsContractsErrors = { _errors: Option<Array<string>> };
export type PageSalesOrdersOverviewErrors = { _errors: Option<Array<string>> };
export type PageSalesOrdersActivitiesErrors = { _errors: Option<Array<string>> };
export type PageSalesOrdersPaymentsErrors = { _errors: Option<Array<string>> };
export type PageSalesOrdersCommissionsErrors = { _errors: Option<Array<string>> };
export type PageSalesSchedulingScheduleErrors = { _errors: Option<Array<string>> };
export type PageSalesSchedulingAppointmentsErrors = { _errors: Option<Array<string>> };
export type PageSalesSchedulingRecurringErrors = { _errors: Option<Array<string>> };
export type PageSalesSchedulingRoutesErrors = { _errors: Option<Array<string>> };
export type PageSalesSchedulingRemindersErrors = { _errors: Option<Array<string>> };
export type PageUserHomeErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type PageSalesHomeDashboardTainted = {};
export type PageSalesHomeProductsTainted = {};
export type PageSalesHomeServicesTainted = {};
export type PageSalesHomePackagesTainted = {};
export type PageSalesHomeTaxRatesTainted = {};
export type PageSalesLeadsOverviewTainted = {};
export type PageSalesLeadsActivitiesTainted = {};
export type PageSalesLeadsCampaignsTainted = {};
export type PageSalesLeadsDripCampaignsTainted = {};
export type PageSalesLeadsOpportunitiesTainted = {};
export type PageSalesLeadsPromotionsTainted = {};
export type PageSalesAccountsOverviewTainted = {};
export type PageSalesAccountsActivitiesTainted = {};
export type PageSalesAccountsBillingTainted = {};
export type PageSalesAccountsContractsTainted = {};
export type PageSalesOrdersOverviewTainted = {};
export type PageSalesOrdersActivitiesTainted = {};
export type PageSalesOrdersPaymentsTainted = {};
export type PageSalesOrdersCommissionsTainted = {};
export type PageSalesSchedulingScheduleTainted = {};
export type PageSalesSchedulingAppointmentsTainted = {};
export type PageSalesSchedulingRecurringTainted = {};
export type PageSalesSchedulingRoutesTainted = {};
export type PageSalesSchedulingRemindersTainted = {};
export type PageUserHomeTainted = {}; /** Union error type */
export type PageErrors =
    | ({ _value: 'SalesHomeDashboard' } & PageSalesHomeDashboardErrors)
    | ({ _value: 'SalesHomeProducts' } & PageSalesHomeProductsErrors)
    | ({ _value: 'SalesHomeServices' } & PageSalesHomeServicesErrors)
    | ({ _value: 'SalesHomePackages' } & PageSalesHomePackagesErrors)
    | ({ _value: 'SalesHomeTaxRates' } & PageSalesHomeTaxRatesErrors)
    | ({ _value: 'SalesLeadsOverview' } & PageSalesLeadsOverviewErrors)
    | ({ _value: 'SalesLeadsActivities' } & PageSalesLeadsActivitiesErrors)
    | ({ _value: 'SalesLeadsCampaigns' } & PageSalesLeadsCampaignsErrors)
    | ({ _value: 'SalesLeadsDripCampaigns' } & PageSalesLeadsDripCampaignsErrors)
    | ({ _value: 'SalesLeadsOpportunities' } & PageSalesLeadsOpportunitiesErrors)
    | ({ _value: 'SalesLeadsPromotions' } & PageSalesLeadsPromotionsErrors)
    | ({ _value: 'SalesAccountsOverview' } & PageSalesAccountsOverviewErrors)
    | ({ _value: 'SalesAccountsActivities' } & PageSalesAccountsActivitiesErrors)
    | ({ _value: 'SalesAccountsBilling' } & PageSalesAccountsBillingErrors)
    | ({ _value: 'SalesAccountsContracts' } & PageSalesAccountsContractsErrors)
    | ({ _value: 'SalesOrdersOverview' } & PageSalesOrdersOverviewErrors)
    | ({ _value: 'SalesOrdersActivities' } & PageSalesOrdersActivitiesErrors)
    | ({ _value: 'SalesOrdersPayments' } & PageSalesOrdersPaymentsErrors)
    | ({ _value: 'SalesOrdersCommissions' } & PageSalesOrdersCommissionsErrors)
    | ({ _value: 'SalesSchedulingSchedule' } & PageSalesSchedulingScheduleErrors)
    | ({ _value: 'SalesSchedulingAppointments' } & PageSalesSchedulingAppointmentsErrors)
    | ({ _value: 'SalesSchedulingRecurring' } & PageSalesSchedulingRecurringErrors)
    | ({ _value: 'SalesSchedulingRoutes' } & PageSalesSchedulingRoutesErrors)
    | ({ _value: 'SalesSchedulingReminders' } & PageSalesSchedulingRemindersErrors)
    | ({ _value: 'UserHome' } & PageUserHomeErrors); /** Union tainted type */
export type PageTainted =
    | ({ _value: 'SalesHomeDashboard' } & PageSalesHomeDashboardTainted)
    | ({ _value: 'SalesHomeProducts' } & PageSalesHomeProductsTainted)
    | ({ _value: 'SalesHomeServices' } & PageSalesHomeServicesTainted)
    | ({ _value: 'SalesHomePackages' } & PageSalesHomePackagesTainted)
    | ({ _value: 'SalesHomeTaxRates' } & PageSalesHomeTaxRatesTainted)
    | ({ _value: 'SalesLeadsOverview' } & PageSalesLeadsOverviewTainted)
    | ({ _value: 'SalesLeadsActivities' } & PageSalesLeadsActivitiesTainted)
    | ({ _value: 'SalesLeadsCampaigns' } & PageSalesLeadsCampaignsTainted)
    | ({ _value: 'SalesLeadsDripCampaigns' } & PageSalesLeadsDripCampaignsTainted)
    | ({ _value: 'SalesLeadsOpportunities' } & PageSalesLeadsOpportunitiesTainted)
    | ({ _value: 'SalesLeadsPromotions' } & PageSalesLeadsPromotionsTainted)
    | ({ _value: 'SalesAccountsOverview' } & PageSalesAccountsOverviewTainted)
    | ({ _value: 'SalesAccountsActivities' } & PageSalesAccountsActivitiesTainted)
    | ({ _value: 'SalesAccountsBilling' } & PageSalesAccountsBillingTainted)
    | ({ _value: 'SalesAccountsContracts' } & PageSalesAccountsContractsTainted)
    | ({ _value: 'SalesOrdersOverview' } & PageSalesOrdersOverviewTainted)
    | ({ _value: 'SalesOrdersActivities' } & PageSalesOrdersActivitiesTainted)
    | ({ _value: 'SalesOrdersPayments' } & PageSalesOrdersPaymentsTainted)
    | ({ _value: 'SalesOrdersCommissions' } & PageSalesOrdersCommissionsTainted)
    | ({ _value: 'SalesSchedulingSchedule' } & PageSalesSchedulingScheduleTainted)
    | ({ _value: 'SalesSchedulingAppointments' } & PageSalesSchedulingAppointmentsTainted)
    | ({ _value: 'SalesSchedulingRecurring' } & PageSalesSchedulingRecurringTainted)
    | ({ _value: 'SalesSchedulingRoutes' } & PageSalesSchedulingRoutesTainted)
    | ({ _value: 'SalesSchedulingReminders' } & PageSalesSchedulingRemindersTainted)
    | ({ _value: 'UserHome' } & PageUserHomeTainted); /** Per-variant field controller types */
export interface PageSalesHomeDashboardFieldControllers {}
export interface PageSalesHomeProductsFieldControllers {}
export interface PageSalesHomeServicesFieldControllers {}
export interface PageSalesHomePackagesFieldControllers {}
export interface PageSalesHomeTaxRatesFieldControllers {}
export interface PageSalesLeadsOverviewFieldControllers {}
export interface PageSalesLeadsActivitiesFieldControllers {}
export interface PageSalesLeadsCampaignsFieldControllers {}
export interface PageSalesLeadsDripCampaignsFieldControllers {}
export interface PageSalesLeadsOpportunitiesFieldControllers {}
export interface PageSalesLeadsPromotionsFieldControllers {}
export interface PageSalesAccountsOverviewFieldControllers {}
export interface PageSalesAccountsActivitiesFieldControllers {}
export interface PageSalesAccountsBillingFieldControllers {}
export interface PageSalesAccountsContractsFieldControllers {}
export interface PageSalesOrdersOverviewFieldControllers {}
export interface PageSalesOrdersActivitiesFieldControllers {}
export interface PageSalesOrdersPaymentsFieldControllers {}
export interface PageSalesOrdersCommissionsFieldControllers {}
export interface PageSalesSchedulingScheduleFieldControllers {}
export interface PageSalesSchedulingAppointmentsFieldControllers {}
export interface PageSalesSchedulingRecurringFieldControllers {}
export interface PageSalesSchedulingRoutesFieldControllers {}
export interface PageSalesSchedulingRemindersFieldControllers {}
export interface PageUserHomeFieldControllers {} /** Union Gigaform interface with variant switching */
export interface PageGigaform {
    readonly currentVariant:
        | 'SalesHomeDashboard'
        | 'SalesHomeProducts'
        | 'SalesHomeServices'
        | 'SalesHomePackages'
        | 'SalesHomeTaxRates'
        | 'SalesLeadsOverview'
        | 'SalesLeadsActivities'
        | 'SalesLeadsCampaigns'
        | 'SalesLeadsDripCampaigns'
        | 'SalesLeadsOpportunities'
        | 'SalesLeadsPromotions'
        | 'SalesAccountsOverview'
        | 'SalesAccountsActivities'
        | 'SalesAccountsBilling'
        | 'SalesAccountsContracts'
        | 'SalesOrdersOverview'
        | 'SalesOrdersActivities'
        | 'SalesOrdersPayments'
        | 'SalesOrdersCommissions'
        | 'SalesSchedulingSchedule'
        | 'SalesSchedulingAppointments'
        | 'SalesSchedulingRecurring'
        | 'SalesSchedulingRoutes'
        | 'SalesSchedulingReminders'
        | 'UserHome';
    readonly data: Page;
    readonly errors: PageErrors;
    readonly tainted: PageTainted;
    readonly variants: PageVariantFields;
    switchVariant(
        variant:
            | 'SalesHomeDashboard'
            | 'SalesHomeProducts'
            | 'SalesHomeServices'
            | 'SalesHomePackages'
            | 'SalesHomeTaxRates'
            | 'SalesLeadsOverview'
            | 'SalesLeadsActivities'
            | 'SalesLeadsCampaigns'
            | 'SalesLeadsDripCampaigns'
            | 'SalesLeadsOpportunities'
            | 'SalesLeadsPromotions'
            | 'SalesAccountsOverview'
            | 'SalesAccountsActivities'
            | 'SalesAccountsBilling'
            | 'SalesAccountsContracts'
            | 'SalesOrdersOverview'
            | 'SalesOrdersActivities'
            | 'SalesOrdersPayments'
            | 'SalesOrdersCommissions'
            | 'SalesSchedulingSchedule'
            | 'SalesSchedulingAppointments'
            | 'SalesSchedulingRecurring'
            | 'SalesSchedulingRoutes'
            | 'SalesSchedulingReminders'
            | 'UserHome'
    ): void;
    validate(): Result<Page, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Page>): void;
} /** Variant fields container */
export interface PageVariantFields {
    readonly SalesHomeDashboard: { readonly fields: PageSalesHomeDashboardFieldControllers };
    readonly SalesHomeProducts: { readonly fields: PageSalesHomeProductsFieldControllers };
    readonly SalesHomeServices: { readonly fields: PageSalesHomeServicesFieldControllers };
    readonly SalesHomePackages: { readonly fields: PageSalesHomePackagesFieldControllers };
    readonly SalesHomeTaxRates: { readonly fields: PageSalesHomeTaxRatesFieldControllers };
    readonly SalesLeadsOverview: { readonly fields: PageSalesLeadsOverviewFieldControllers };
    readonly SalesLeadsActivities: { readonly fields: PageSalesLeadsActivitiesFieldControllers };
    readonly SalesLeadsCampaigns: { readonly fields: PageSalesLeadsCampaignsFieldControllers };
    readonly SalesLeadsDripCampaigns: {
        readonly fields: PageSalesLeadsDripCampaignsFieldControllers;
    };
    readonly SalesLeadsOpportunities: {
        readonly fields: PageSalesLeadsOpportunitiesFieldControllers;
    };
    readonly SalesLeadsPromotions: { readonly fields: PageSalesLeadsPromotionsFieldControllers };
    readonly SalesAccountsOverview: { readonly fields: PageSalesAccountsOverviewFieldControllers };
    readonly SalesAccountsActivities: {
        readonly fields: PageSalesAccountsActivitiesFieldControllers;
    };
    readonly SalesAccountsBilling: { readonly fields: PageSalesAccountsBillingFieldControllers };
    readonly SalesAccountsContracts: {
        readonly fields: PageSalesAccountsContractsFieldControllers;
    };
    readonly SalesOrdersOverview: { readonly fields: PageSalesOrdersOverviewFieldControllers };
    readonly SalesOrdersActivities: { readonly fields: PageSalesOrdersActivitiesFieldControllers };
    readonly SalesOrdersPayments: { readonly fields: PageSalesOrdersPaymentsFieldControllers };
    readonly SalesOrdersCommissions: {
        readonly fields: PageSalesOrdersCommissionsFieldControllers;
    };
    readonly SalesSchedulingSchedule: {
        readonly fields: PageSalesSchedulingScheduleFieldControllers;
    };
    readonly SalesSchedulingAppointments: {
        readonly fields: PageSalesSchedulingAppointmentsFieldControllers;
    };
    readonly SalesSchedulingRecurring: {
        readonly fields: PageSalesSchedulingRecurringFieldControllers;
    };
    readonly SalesSchedulingRoutes: { readonly fields: PageSalesSchedulingRoutesFieldControllers };
    readonly SalesSchedulingReminders: {
        readonly fields: PageSalesSchedulingRemindersFieldControllers;
    };
    readonly UserHome: { readonly fields: PageUserHomeFieldControllers };
} /** Gets default value for a specific variant */
function pageGetDefaultForVariant(variant: string): Page {
    switch (variant) {
        case 'SalesHomeDashboard':
            return 'SalesHomeDashboard' as Page;
        case 'SalesHomeProducts':
            return 'SalesHomeProducts' as Page;
        case 'SalesHomeServices':
            return 'SalesHomeServices' as Page;
        case 'SalesHomePackages':
            return 'SalesHomePackages' as Page;
        case 'SalesHomeTaxRates':
            return 'SalesHomeTaxRates' as Page;
        case 'SalesLeadsOverview':
            return 'SalesLeadsOverview' as Page;
        case 'SalesLeadsActivities':
            return 'SalesLeadsActivities' as Page;
        case 'SalesLeadsCampaigns':
            return 'SalesLeadsCampaigns' as Page;
        case 'SalesLeadsDripCampaigns':
            return 'SalesLeadsDripCampaigns' as Page;
        case 'SalesLeadsOpportunities':
            return 'SalesLeadsOpportunities' as Page;
        case 'SalesLeadsPromotions':
            return 'SalesLeadsPromotions' as Page;
        case 'SalesAccountsOverview':
            return 'SalesAccountsOverview' as Page;
        case 'SalesAccountsActivities':
            return 'SalesAccountsActivities' as Page;
        case 'SalesAccountsBilling':
            return 'SalesAccountsBilling' as Page;
        case 'SalesAccountsContracts':
            return 'SalesAccountsContracts' as Page;
        case 'SalesOrdersOverview':
            return 'SalesOrdersOverview' as Page;
        case 'SalesOrdersActivities':
            return 'SalesOrdersActivities' as Page;
        case 'SalesOrdersPayments':
            return 'SalesOrdersPayments' as Page;
        case 'SalesOrdersCommissions':
            return 'SalesOrdersCommissions' as Page;
        case 'SalesSchedulingSchedule':
            return 'SalesSchedulingSchedule' as Page;
        case 'SalesSchedulingAppointments':
            return 'SalesSchedulingAppointments' as Page;
        case 'SalesSchedulingRecurring':
            return 'SalesSchedulingRecurring' as Page;
        case 'SalesSchedulingRoutes':
            return 'SalesSchedulingRoutes' as Page;
        case 'SalesSchedulingReminders':
            return 'SalesSchedulingReminders' as Page;
        case 'UserHome':
            return 'UserHome' as Page;
        default:
            return 'SalesHomeDashboard' as Page;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function pageCreateForm(initial?: Page): PageGigaform {
    const initialVariant:
        | 'SalesHomeDashboard'
        | 'SalesHomeProducts'
        | 'SalesHomeServices'
        | 'SalesHomePackages'
        | 'SalesHomeTaxRates'
        | 'SalesLeadsOverview'
        | 'SalesLeadsActivities'
        | 'SalesLeadsCampaigns'
        | 'SalesLeadsDripCampaigns'
        | 'SalesLeadsOpportunities'
        | 'SalesLeadsPromotions'
        | 'SalesAccountsOverview'
        | 'SalesAccountsActivities'
        | 'SalesAccountsBilling'
        | 'SalesAccountsContracts'
        | 'SalesOrdersOverview'
        | 'SalesOrdersActivities'
        | 'SalesOrdersPayments'
        | 'SalesOrdersCommissions'
        | 'SalesSchedulingSchedule'
        | 'SalesSchedulingAppointments'
        | 'SalesSchedulingRecurring'
        | 'SalesSchedulingRoutes'
        | 'SalesSchedulingReminders'
        | 'UserHome' =
        (initial as
            | 'SalesHomeDashboard'
            | 'SalesHomeProducts'
            | 'SalesHomeServices'
            | 'SalesHomePackages'
            | 'SalesHomeTaxRates'
            | 'SalesLeadsOverview'
            | 'SalesLeadsActivities'
            | 'SalesLeadsCampaigns'
            | 'SalesLeadsDripCampaigns'
            | 'SalesLeadsOpportunities'
            | 'SalesLeadsPromotions'
            | 'SalesAccountsOverview'
            | 'SalesAccountsActivities'
            | 'SalesAccountsBilling'
            | 'SalesAccountsContracts'
            | 'SalesOrdersOverview'
            | 'SalesOrdersActivities'
            | 'SalesOrdersPayments'
            | 'SalesOrdersCommissions'
            | 'SalesSchedulingSchedule'
            | 'SalesSchedulingAppointments'
            | 'SalesSchedulingRecurring'
            | 'SalesSchedulingRoutes'
            | 'SalesSchedulingReminders'
            | 'UserHome') ?? 'SalesHomeDashboard';
    let currentVariant = $state<
        | 'SalesHomeDashboard'
        | 'SalesHomeProducts'
        | 'SalesHomeServices'
        | 'SalesHomePackages'
        | 'SalesHomeTaxRates'
        | 'SalesLeadsOverview'
        | 'SalesLeadsActivities'
        | 'SalesLeadsCampaigns'
        | 'SalesLeadsDripCampaigns'
        | 'SalesLeadsOpportunities'
        | 'SalesLeadsPromotions'
        | 'SalesAccountsOverview'
        | 'SalesAccountsActivities'
        | 'SalesAccountsBilling'
        | 'SalesAccountsContracts'
        | 'SalesOrdersOverview'
        | 'SalesOrdersActivities'
        | 'SalesOrdersPayments'
        | 'SalesOrdersCommissions'
        | 'SalesSchedulingSchedule'
        | 'SalesSchedulingAppointments'
        | 'SalesSchedulingRecurring'
        | 'SalesSchedulingRoutes'
        | 'SalesSchedulingReminders'
        | 'UserHome'
    >(initialVariant);
    let data = $state<Page>(initial ?? pageGetDefaultForVariant(initialVariant));
    let errors = $state<PageErrors>({} as PageErrors);
    let tainted = $state<PageTainted>({} as PageTainted);
    const variants: PageVariantFields = {
        SalesHomeDashboard: { fields: {} as PageSalesHomeDashboardFieldControllers },
        SalesHomeProducts: { fields: {} as PageSalesHomeProductsFieldControllers },
        SalesHomeServices: { fields: {} as PageSalesHomeServicesFieldControllers },
        SalesHomePackages: { fields: {} as PageSalesHomePackagesFieldControllers },
        SalesHomeTaxRates: { fields: {} as PageSalesHomeTaxRatesFieldControllers },
        SalesLeadsOverview: { fields: {} as PageSalesLeadsOverviewFieldControllers },
        SalesLeadsActivities: { fields: {} as PageSalesLeadsActivitiesFieldControllers },
        SalesLeadsCampaigns: { fields: {} as PageSalesLeadsCampaignsFieldControllers },
        SalesLeadsDripCampaigns: { fields: {} as PageSalesLeadsDripCampaignsFieldControllers },
        SalesLeadsOpportunities: { fields: {} as PageSalesLeadsOpportunitiesFieldControllers },
        SalesLeadsPromotions: { fields: {} as PageSalesLeadsPromotionsFieldControllers },
        SalesAccountsOverview: { fields: {} as PageSalesAccountsOverviewFieldControllers },
        SalesAccountsActivities: { fields: {} as PageSalesAccountsActivitiesFieldControllers },
        SalesAccountsBilling: { fields: {} as PageSalesAccountsBillingFieldControllers },
        SalesAccountsContracts: { fields: {} as PageSalesAccountsContractsFieldControllers },
        SalesOrdersOverview: { fields: {} as PageSalesOrdersOverviewFieldControllers },
        SalesOrdersActivities: { fields: {} as PageSalesOrdersActivitiesFieldControllers },
        SalesOrdersPayments: { fields: {} as PageSalesOrdersPaymentsFieldControllers },
        SalesOrdersCommissions: { fields: {} as PageSalesOrdersCommissionsFieldControllers },
        SalesSchedulingSchedule: { fields: {} as PageSalesSchedulingScheduleFieldControllers },
        SalesSchedulingAppointments: {
            fields: {} as PageSalesSchedulingAppointmentsFieldControllers
        },
        SalesSchedulingRecurring: { fields: {} as PageSalesSchedulingRecurringFieldControllers },
        SalesSchedulingRoutes: { fields: {} as PageSalesSchedulingRoutesFieldControllers },
        SalesSchedulingReminders: { fields: {} as PageSalesSchedulingRemindersFieldControllers },
        UserHome: { fields: {} as PageUserHomeFieldControllers }
    };
    function switchVariant(
        variant:
            | 'SalesHomeDashboard'
            | 'SalesHomeProducts'
            | 'SalesHomeServices'
            | 'SalesHomePackages'
            | 'SalesHomeTaxRates'
            | 'SalesLeadsOverview'
            | 'SalesLeadsActivities'
            | 'SalesLeadsCampaigns'
            | 'SalesLeadsDripCampaigns'
            | 'SalesLeadsOpportunities'
            | 'SalesLeadsPromotions'
            | 'SalesAccountsOverview'
            | 'SalesAccountsActivities'
            | 'SalesAccountsBilling'
            | 'SalesAccountsContracts'
            | 'SalesOrdersOverview'
            | 'SalesOrdersActivities'
            | 'SalesOrdersPayments'
            | 'SalesOrdersCommissions'
            | 'SalesSchedulingSchedule'
            | 'SalesSchedulingAppointments'
            | 'SalesSchedulingRecurring'
            | 'SalesSchedulingRoutes'
            | 'SalesSchedulingReminders'
            | 'UserHome'
    ): void {
        currentVariant = variant;
        data = pageGetDefaultForVariant(variant);
        errors = {} as PageErrors;
        tainted = {} as PageTainted;
    }
    function validate(): Result<Page, Array<{ field: string; message: string }>> {
        return pageDeserialize(data);
    }
    function reset(overrides?: Partial<Page>): void {
        data = overrides ? (overrides as typeof data) : pageGetDefaultForVariant(currentVariant);
        errors = {} as PageErrors;
        tainted = {} as PageTainted;
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
export function pageFromFormData(
    formData: FormData
): Result<Page, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as
        | 'SalesHomeDashboard'
        | 'SalesHomeProducts'
        | 'SalesHomeServices'
        | 'SalesHomePackages'
        | 'SalesHomeTaxRates'
        | 'SalesLeadsOverview'
        | 'SalesLeadsActivities'
        | 'SalesLeadsCampaigns'
        | 'SalesLeadsDripCampaigns'
        | 'SalesLeadsOpportunities'
        | 'SalesLeadsPromotions'
        | 'SalesAccountsOverview'
        | 'SalesAccountsActivities'
        | 'SalesAccountsBilling'
        | 'SalesAccountsContracts'
        | 'SalesOrdersOverview'
        | 'SalesOrdersActivities'
        | 'SalesOrdersPayments'
        | 'SalesOrdersCommissions'
        | 'SalesSchedulingSchedule'
        | 'SalesSchedulingAppointments'
        | 'SalesSchedulingRecurring'
        | 'SalesSchedulingRoutes'
        | 'SalesSchedulingReminders'
        | 'UserHome'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'SalesHomeDashboard') {
    } else if (discriminant === 'SalesHomeProducts') {
    } else if (discriminant === 'SalesHomeServices') {
    } else if (discriminant === 'SalesHomePackages') {
    } else if (discriminant === 'SalesHomeTaxRates') {
    } else if (discriminant === 'SalesLeadsOverview') {
    } else if (discriminant === 'SalesLeadsActivities') {
    } else if (discriminant === 'SalesLeadsCampaigns') {
    } else if (discriminant === 'SalesLeadsDripCampaigns') {
    } else if (discriminant === 'SalesLeadsOpportunities') {
    } else if (discriminant === 'SalesLeadsPromotions') {
    } else if (discriminant === 'SalesAccountsOverview') {
    } else if (discriminant === 'SalesAccountsActivities') {
    } else if (discriminant === 'SalesAccountsBilling') {
    } else if (discriminant === 'SalesAccountsContracts') {
    } else if (discriminant === 'SalesOrdersOverview') {
    } else if (discriminant === 'SalesOrdersActivities') {
    } else if (discriminant === 'SalesOrdersPayments') {
    } else if (discriminant === 'SalesOrdersCommissions') {
    } else if (discriminant === 'SalesSchedulingSchedule') {
    } else if (discriminant === 'SalesSchedulingAppointments') {
    } else if (discriminant === 'SalesSchedulingRecurring') {
    } else if (discriminant === 'SalesSchedulingRoutes') {
    } else if (discriminant === 'SalesSchedulingReminders') {
    } else if (discriminant === 'UserHome') {
    }
    return pageDeserialize(obj);
}

export const Page = {
    defaultValue: pageDefaultValue,
    serialize: pageSerialize,
    serializeWithContext: pageSerializeWithContext,
    deserialize: pageDeserialize,
    deserializeWithContext: pageDeserializeWithContext,
    is: pageIs,
    createForm: pageCreateForm,
    fromFormData: pageFromFormData
} as const;
